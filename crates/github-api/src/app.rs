use crate::api::GithubAPI;
use crate::constants::GITHUB_API_BASE_URL;
use crate::domains::{
    events::{
        GithubWebhookEvent, GithubWebhookInstallation, GithubWebhookIssueCommentEvent,
        GithubWebhookPullRequestEvent, GithubWebhookPushEvent,
    },
    installation_token::GithubInstallationExpirableToken,
    installations::{GithubInstallation, GithubInstallationAccessToken},
};
use actix_web::{post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use infrastructure::{ExpirableToken, GithubAPIClient, GithubError, GithubResult};
use std::collections::HashMap;
use tokio::sync::{mpsc, oneshot};

pub type GithubWebhookEventListener = Box<
    dyn Fn(
            GithubWebhookEvent,
            GithubAPI<GithubInstallationAccessToken>,
        ) -> Result<(), Box<dyn std::error::Error>>
        + Send
        + Sync,
>;

#[derive(Debug)]
pub enum ChannelMessage {
    WebhookEvent(String, String),
    ServerHandle(actix_web::dev::ServerHandle),
    Stop(oneshot::Sender<()>),
}

#[derive(Clone, Debug)]
pub struct Handle {
    msg_tx: mpsc::UnboundedSender<ChannelMessage>,
}

pub struct GithubApp {
    handle: Handle,
    msg_rx: mpsc::UnboundedReceiver<ChannelMessage>,
    webhook_event_listeners: Vec<GithubWebhookEventListener>,
    tokens: HashMap<u64, GithubInstallationAccessToken>,
    api_client: GithubAPIClient<GithubInstallationExpirableToken>,
}

impl Handle {
    pub fn new(msg_tx: mpsc::UnboundedSender<ChannelMessage>) -> Self {
        Self { msg_tx }
    }

    pub async fn stop(&self) -> GithubResult<()> {
        let (tx, rx) = oneshot::channel::<()>();

        self.msg_tx.send(ChannelMessage::Stop(tx)).unwrap();

        rx.await.unwrap();

        Ok(())
    }
}

impl GithubApp {
    pub fn new(id: impl Into<String>, private_key: impl Into<String>) -> Self {
        let mut token = GithubInstallationExpirableToken::new(id.into(), private_key.into());

        let tokens = HashMap::new();

        if let Err(error) = token.generate_token_if_expired() {
            panic!("Error generating token: {}", error);
        }

        let (tx, rx) = mpsc::unbounded_channel::<ChannelMessage>();

        Self {
            tokens,
            api_client: GithubAPIClient::new(token),
            webhook_event_listeners: Vec::new(),
            handle: Handle::new(tx),
            msg_rx: rx,
        }
    }

    pub fn handle(&self) -> Handle {
        self.handle.clone()
    }

    pub async fn start(&mut self) -> GithubResult<()> {
        let msg_tx = self.handle.msg_tx.clone();

        tokio::spawn(async move {
            #[post("/webhook")]
            async fn webhook(
                payload: web::Bytes,
                req: HttpRequest,
                sender: web::Data<mpsc::UnboundedSender<ChannelMessage>>,
            ) -> impl Responder {
                let event = req.headers().get("X-GitHub-Event").unwrap();
                let payload = String::from_utf8(payload.to_vec()).unwrap();

                sender
                    .send(ChannelMessage::WebhookEvent(
                        event.to_str().unwrap().to_string(),
                        payload,
                    ))
                    .unwrap();
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

            tx.send(ChannelMessage::ServerHandle(handle.clone()))
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
                      ChannelMessage::WebhookEvent(event, payload) => {
                          let (installation, event) = self.parse_webhook_event_payload(event, payload)?;

                          let api = self.get_api(installation.id).await?;
                          for listener in &self.webhook_event_listeners {
                              if let Err(error) = listener(event.clone(), api.clone()) {
                                  eprintln!("Error: {}", error);
                              }
                          }
                      }
                      #[allow(unused_assignments)]
                      ChannelMessage::ServerHandle(handle) => {
                          server_handle = Some(handle);
                      }
                      ChannelMessage::Stop(tx) => {
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

    pub async fn get_installations(&self) -> GithubResult<Vec<GithubInstallation>> {
        let request_url = format!("{}/app/installations", GITHUB_API_BASE_URL);

        self.api_client
            .get::<Vec<GithubInstallation>>(request_url)
            .send()
            .await
    }

    pub async fn get_installation(
        &self,
        installation_id: impl Into<u64>,
    ) -> GithubResult<GithubInstallation> {
        let request_url = format!(
            "{}/app/installations/{}",
            GITHUB_API_BASE_URL,
            installation_id.into()
        );
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

        let request_url = format!(
            "{}/app/installations/{}/access_tokens",
            GITHUB_API_BASE_URL, installation_id
        );

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
        F: Fn(
            GithubWebhookEvent,
            GithubAPI<GithubInstallationAccessToken>,
        ) -> Result<(), Box<dyn std::error::Error>>,
        F: Send + Sync + 'static,
    {
        self.webhook_event_listeners.push(Box::new(listener));

        self
    }

    pub async fn get_api(
        &mut self,
        installation_id: u64,
    ) -> GithubResult<GithubAPI<GithubInstallationAccessToken>> {
        if let Some(token) = self.tokens.get(&installation_id) {
            if !token.is_expired() {
                return Ok(GithubAPI::new(token.clone()));
            }
        }

        let token = self.get_installation_access_token(installation_id).await?;

        Ok(GithubAPI::new(token))
    }

    fn parse_webhook_event_payload(
        &self,
        event_name: String,
        payload: String,
    ) -> GithubResult<(GithubWebhookInstallation, GithubWebhookEvent)> {
        match event_name.as_str() {
            "issue_comment" => {
                let evt = serde_json::from_str::<GithubWebhookIssueCommentEvent>(&payload)?;

                return Ok((
                    evt.installation.clone(),
                    GithubWebhookEvent::IssueComment(evt),
                ));
            }
            "pull_request" => {
                let evt = serde_json::from_str::<GithubWebhookPullRequestEvent>(&payload)?;

                return Ok((
                    evt.installation.clone(),
                    GithubWebhookEvent::PullRequest(evt),
                ));
            }
            "push" => {
                let evt = serde_json::from_str::<GithubWebhookPushEvent>(&payload)?;

                return Ok((evt.installation.clone(), GithubWebhookEvent::Push(evt)));
            }
            _ => {
                let event = serde_json::from_str::<serde_json::Value>(&payload)?;
                let installation = event
                    .get("installation")
                    .ok_or(GithubError::new("No installation field on webhook event"))?;

                let installation =
                    serde_json::from_value::<GithubWebhookInstallation>(installation.clone())?;
                return Ok((
                    installation,
                    GithubWebhookEvent::Unsupported(payload.clone()),
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils;
    use infrastructure::GithubResult;

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
}
