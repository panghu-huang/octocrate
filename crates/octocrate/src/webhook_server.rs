use crate::{GithubAPI, GithubApp, GithubWebhookEvent};
use actix_web::{post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use octocrate_infra::{GithubError, GithubResult};
use tokio::sync::{mpsc, oneshot};

pub type GithubWebhookEventListener = Box<
    dyn Fn(GithubWebhookEvent, GithubAPI) -> Result<(), Box<dyn std::error::Error>> + Send + Sync,
>;

#[derive(Debug)]
pub enum Message {
    WebhookEvent(GithubWebhookEvent),
    ServerHandle(actix_web::dev::ServerHandle),
    Stop(oneshot::Sender<()>),
}

#[derive(Clone, Debug)]
pub struct ServerHandle {
    msg_tx: mpsc::Sender<Message>,
}

pub struct WebhookServer {
    github_app: GithubApp,
    server_handle: ServerHandle,
    msg_rx: mpsc::Receiver<Message>,
    listeners: Vec<GithubWebhookEventListener>,
}

pub struct WebhookServerBuilder {
    app_id: Option<String>,
    private_key: Option<String>,
    base_url: Option<String>,
}

impl ServerHandle {
    pub fn new(msg_tx: mpsc::Sender<Message>) -> Self {
        Self { msg_tx }
    }

    pub async fn trigger_webhook_event(&self, event: GithubWebhookEvent) -> GithubResult<()> {
        self.msg_tx.send(Message::WebhookEvent(event)).await?;

        Ok(())
    }

    pub async fn stop(&self) -> GithubResult<()> {
        let (tx, rx) = oneshot::channel::<()>();

        self.msg_tx.send(Message::Stop(tx)).await?;

        rx.await?;

        Ok(())
    }
}

impl WebhookServer {
    pub fn builder() -> WebhookServerBuilder {
        WebhookServerBuilder::new()
    }

    pub fn new(
        app_id: impl Into<String>,
        private_key: impl Into<String>,
        base_url: impl Into<String>,
    ) -> Self {
        let github_app = GithubApp::new(app_id, private_key, base_url);

        let (tx, rx) = mpsc::channel::<Message>(20);
        Self {
            github_app,
            listeners: Vec::new(),
            server_handle: ServerHandle::new(tx),
            msg_rx: rx,
        }
    }

    pub fn server_handle(&self) -> ServerHandle {
        self.server_handle.clone()
    }

    pub async fn start(&mut self) -> GithubResult<()> {
        let msg_tx = self.server_handle.msg_tx.clone();

        tokio::spawn(async move {
            #[post("/github/webhook")]
            async fn webhook(
                payload: web::Bytes,
                req: HttpRequest,
                sender: web::Data<mpsc::UnboundedSender<Message>>,
            ) -> impl Responder {
                let event_name = req.headers().get("X-GitHub-Event").unwrap();
                let payload = String::from_utf8(payload.to_vec()).unwrap();

                let event_name = event_name.to_str().unwrap();
                let event = GithubWebhookEvent::try_parse(event_name.to_string(), payload).unwrap();

                sender.send(Message::WebhookEvent(event)).unwrap();

                HttpResponse::Ok()
            }

            let port = std::env::var("PORT").unwrap_or(String::from("3000"));
            let port = port.parse::<u16>().unwrap();

            let tx = msg_tx.clone();

            let srv = HttpServer::new(move || {
                return App::new()
                    .service(webhook)
                    .app_data(web::Data::new(msg_tx.clone()));
            })
            .bind(("127.0.0.1", port))
            .unwrap()
            .run();

            let handle = srv.handle();

            tx.send(Message::ServerHandle(handle.clone()))
                .await
                .unwrap();

            println!("Server started on http://localhost:{}", port);

            srv.await.unwrap();
        });

        let mut server_handle: Option<actix_web::dev::ServerHandle> = None;

        loop {
            tokio::select! {
              msg = self.msg_rx.recv() => {
                  if let Some(msg) = msg {
                    match msg {
                      Message::WebhookEvent(event) => {
                          let installation = event.installation();
                          match self.github_app.get_api(installation.id).await {
                            Ok(api) => {
                              for listener in &self.listeners {
                                if let Err(error) = listener(event.clone(), api.clone()) {
                                    eprintln!("Error: {}", error);
                                }
                              }
                            }
                            Err(error) => {
                              eprintln!("Error on WebhookServer.start(): {}", error);
                            }
                          };
                      }
                      #[allow(unused_assignments)]
                      Message::ServerHandle(handle) => {
                          server_handle = Some(handle);
                      }
                      Message::Stop(tx) => {
                        if let Some(handle) = server_handle.clone() {
                          handle.stop(true).await;
                        }
                        tx.send(()).unwrap();

                        break;
                      }
                  }
                }
              }

              v = tokio::signal::ctrl_c() => {
                if let Ok(_) = v {
                  if let Some(handle) = server_handle.clone() {
                    handle.stop(true).await;
                  }
                  break;
                }
              }
            };
        }

        Ok(())
    }

    pub fn on_webhook_event<F>(&mut self, listener: F) -> &mut Self
    where
        F: Fn(GithubWebhookEvent, GithubAPI) -> Result<(), Box<dyn std::error::Error>>,
        F: Send + Sync + 'static,
    {
        self.listeners.push(Box::new(listener));

        self
    }
}

impl WebhookServerBuilder {
    pub fn new() -> Self {
        Self {
            app_id: None,
            private_key: None,
            base_url: None,
        }
    }

    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.app_id = Some(app_id.into());
        self
    }

    pub fn private_key(mut self, private_key: impl Into<String>) -> Self {
        self.private_key = Some(private_key.into());
        self
    }

    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = Some(base_url.into());
        self
    }

    pub fn build(self) -> GithubResult<WebhookServer> {
        let app_id = self.app_id.ok_or_else(|| {
            GithubError::new("app_id is required to create a WebhookServer".to_string())
        })?;

        let private_key = self.private_key.ok_or_else(|| {
            GithubError::new("private_key is required to create a WebhookServer".to_string())
        })?;

        let base_url = self
            .base_url
            .unwrap_or_else(|| "https://api.github.com".to_string());

        let app = WebhookServer::new(app_id, private_key, base_url);

        Ok(app)
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils;
    use octocrate_infra::GithubResult;

    #[tokio::test]
    #[ignore]
    async fn start() -> GithubResult<()> {
        let mut app = test_utils::create_github_app()?;

        app.start().await?;

        Ok(())
    }

    #[tokio::test]
    async fn get_installations() -> GithubResult<()> {
        let app = test_utils::create_github_app()?;

        let _installations = app.get_installations().await?;

        Ok(())
    }

    #[tokio::test]
    async fn get_installation() -> GithubResult<()> {
        let app = test_utils::create_github_app()?;
        let envs = test_utils::load_test_envs()?;

        let installation = app.get_installation(envs.installation_id).await?;

        assert_eq!(installation.id, envs.installation_id);
        Ok(())
    }

    #[tokio::test]
    async fn get_installation_access_token() -> GithubResult<()> {
        let mut app = test_utils::create_github_app()?;
        let envs = test_utils::load_test_envs()?;
        let installation_id = envs.installation_id;

        let access_token = app
            .get_installation_access_token(installation_id.clone())
            .await?;

        let access_token1 = app.get_installation_access_token(installation_id).await?;

        assert_eq!(access_token.expires_at, access_token1.expires_at);

        Ok(())
    }

    #[tokio::test]
    async fn get_repository_installation() -> GithubResult<()> {
        let app = test_utils::create_github_app()?;
        let envs = test_utils::load_test_envs()?;
        let github_app_id = envs.github_app_id;

        let installation = app
            .get_repository_installation(envs.repo_owner.clone(), envs.repo_name.clone())
            .await?;

        assert_eq!(installation.app_id.to_string(), github_app_id);

        Ok(())
    }
}
