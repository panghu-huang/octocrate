use crate::api::GithubAPI;
use crate::domains::{
    events::GithubWebhookEvent,
    installation_token::GithubInstallationExpirableToken,
    installations::{GithubInstallation, GithubInstallationAccessToken},
};
#[cfg(feature = "webhook-server")]
use actix_web::{post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use octocrate_infra::{ExpirableToken, GithubAPIClient, GithubAPIConfig, GithubError, GithubResult};
use std::collections::HashMap;
use tokio::sync::{mpsc, oneshot};

pub type GithubWebhookEventListener = Box<
    dyn Fn(GithubWebhookEvent, GithubAPI) -> Result<(), Box<dyn std::error::Error>> + Send + Sync,
>;

#[derive(Debug)]
pub enum Message {
    WebhookEvent(GithubWebhookEvent),
    #[cfg(feature = "webhook-server")]
    ServerHandle(actix_web::dev::ServerHandle),
    Stop(oneshot::Sender<()>),
}

#[derive(Clone, Debug)]
pub struct AppHandle {
    msg_tx: mpsc::Sender<Message>,
}

pub struct GithubApp {
    base_url: String,
    app_handle: AppHandle,
    msg_rx: mpsc::Receiver<Message>,
    webhook_event_listeners: Vec<GithubWebhookEventListener>,
    tokens: HashMap<u64, GithubInstallationAccessToken>,
    api_client: GithubAPIClient,
}

pub struct GithubAppBuilder {
    app_id: Option<String>,
    private_key: Option<String>,
    base_url: Option<String>,
}

impl AppHandle {
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

impl GithubApp {
    pub fn builder() -> GithubAppBuilder {
        GithubAppBuilder::new()
    }

    pub fn new(
        app_id: impl Into<String>,
        private_key: impl Into<String>,
        base_url: impl Into<String>,
    ) -> Self {
        let token = GithubInstallationExpirableToken::new(app_id.into(), private_key.into());

        let tokens = HashMap::new();

        let base_url = base_url.into();
        let api_config = GithubAPIConfig::new(base_url.clone(), token);

        let (tx, rx) = mpsc::channel::<Message>(3);

        Self {
            base_url: base_url,
            tokens,
            api_client: GithubAPIClient::new(api_config),
            webhook_event_listeners: Vec::new(),
            app_handle: AppHandle::new(tx),
            msg_rx: rx,
        }
    }

    pub fn app_handle(&self) -> AppHandle {
        self.app_handle.clone()
    }

    #[cfg(feature = "webhook-server")]
    pub async fn serve(&mut self) -> GithubResult<()> {
        let msg_tx = self.app_handle.msg_tx.clone();

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

            tx.send(Message::ServerHandle(handle.clone())).unwrap();

            println!("Server started on http://localhost:{}", port);

            srv.await.unwrap();
        });

        self.start().await
    }

    pub async fn start(&mut self) -> GithubResult<()> {
        #[cfg(feature = "webhook-server")]
        let mut server_handle: Option<actix_web::dev::ServerHandle> = None;

        loop {
            tokio::select! {
              msg = self.msg_rx.recv() => {
                  if let Some(msg) = msg {
                    match msg {
                      Message::WebhookEvent(event) => {
                          let installation = event.installation();
                          match self.get_api(installation.id).await {
                            Ok(api) => {
                              for listener in &self.webhook_event_listeners {
                                if let Err(error) = listener(event.clone(), api.clone()) {
                                    eprintln!("Error: {}", error);
                                }
                              }
                            }
                            Err(error) => {
                              eprintln!("Error on GithubApp.start(): {}", error);
                            }
                          };
                      }
                      #[cfg(feature = "webhook-server")]
                      #[allow(unused_assignments)]
                      Message::ServerHandle(handle) => {
                          server_handle = Some(handle);
                      }
                      Message::Stop(tx) => {
                        #[cfg(feature = "webhook-server")]
                        {
                          if let Some(handle) = server_handle.clone() {
                            handle.stop(true).await;
                          }
                        }
                        tx.send(()).unwrap();

                        break;
                      }
                  }
                }
              }

              v = tokio::signal::ctrl_c() => {
                if let Ok(_) = v {
                  #[cfg(feature = "webhook-server")]
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

    pub async fn get_installations(&self) -> GithubResult<Vec<GithubInstallation>> {
        self.api_client
            .get::<Vec<GithubInstallation>>("/app/installations")
            .send()
            .await
    }

    pub async fn get_repository_installation(
        &self,
        owner: impl Into<String>,
        repo: impl Into<String>,
    ) -> GithubResult<GithubInstallation> {
        let owner = owner.into();
        let repo = repo.into();

        let request_url = format!("/repos/{}/{}/installation", owner, repo);

        self.api_client
            .get::<GithubInstallation>(request_url)
            .send()
            .await
    }

    pub async fn get_installation(
        &self,
        installation_id: impl Into<u64>,
    ) -> GithubResult<GithubInstallation> {
        let request_url = format!("/app/installations/{}", installation_id.into());
        self.api_client
            .get::<GithubInstallation>(request_url)
            .send()
            .await
    }

    pub async fn get_installation_access_token(
        &mut self,
        installation_id: impl Into<u64>,
    ) -> GithubResult<GithubInstallationAccessToken> {
        let installation_id: u64 = installation_id.into();

        if let Some(token) = self.tokens.get(&installation_id) {
            if !token.is_expired() {
                return Ok(token.clone());
            }
        }

        let request_url = format!("/app/installations/{}/access_tokens", installation_id);

        let token = self
            .api_client
            .post::<GithubInstallationAccessToken>(request_url)
            .send()
            .await?;

        self.tokens.insert(installation_id, token.clone());

        Ok(token)
    }

    pub fn on_webhook_event<F>(&mut self, listener: F) -> &mut Self
    where
        F: Fn(GithubWebhookEvent, GithubAPI) -> Result<(), Box<dyn std::error::Error>>,
        F: Send + Sync + 'static,
    {
        self.webhook_event_listeners.push(Box::new(listener));

        self
    }

    pub async fn get_api(&mut self, installation_id: u64) -> GithubResult<GithubAPI> {
        if let Some(token) = self.tokens.get(&installation_id) {
            if !token.is_expired() {
                let api_config = GithubAPIConfig::new(self.base_url.clone(), token.clone());
                return Ok(GithubAPI::new(api_config));
            }
        }

        let token = self.get_installation_access_token(installation_id).await?;
        let api_config = GithubAPIConfig::new(self.base_url.clone(), token.clone());
        return Ok(GithubAPI::new(api_config));
    }
}

impl GithubAppBuilder {
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

    pub fn build(self) -> GithubResult<GithubApp> {
        let app_id = self.app_id.ok_or_else(|| {
            GithubError::new("app_id is required to create a GithubApp".to_string())
        })?;

        let private_key = self.private_key.ok_or_else(|| {
            GithubError::new("private_key is required to create a GithubApp".to_string())
        })?;

        let base_url = self
            .base_url
            .unwrap_or_else(|| "https://api.github.com".to_string());

        let app = GithubApp::new(app_id, private_key, base_url);

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
