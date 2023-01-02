use crate::api::GithubAPI;
use crate::constants::GITHUB_API_BASE_URL;
use crate::domains::{
    events::{GithubWebhookEvent, GithubWebhookInstallation, GithubWebhookIssueCommentEvent},
    installation_token::GithubInstallationExpirableToken,
    installations::{GithubInstallation, GithubInstallationAccessToken},
};
use crate::infrastructure::api_client::GithubAPIClient;
use crate::infrastructure::error::GithubError;
use crate::infrastructure::expirable_token::ExpirableToken;
use std::collections::HashMap;
use websockets::WebSocket;

pub type GithubWebhookEventListener = Box<
    dyn Fn(GithubWebhookEvent, GithubAPI) -> Result<(), Box<dyn std::error::Error>>
        + Send
        + Sync,
>;

pub struct GithubApp {
    webhook_event_listeners: Vec<GithubWebhookEventListener>,
    tokens: HashMap<u64, GithubInstallationAccessToken>,
    api_client: GithubAPIClient,
}

impl GithubApp {
    pub fn new(id: String, private_key: String) -> Self {
        let mut token = GithubInstallationExpirableToken::new(id.clone(), private_key.clone());

        let tokens = HashMap::new();

        if let Err(error) = token.generate_token_if_expired() {
            panic!("Error generating token: {}", error);
        }

        Self {
            tokens,
            api_client: GithubAPIClient::new(token),
            webhook_event_listeners: Vec::new(),
        }
    }

    pub async fn connect(&mut self) -> Result<(), GithubError> {
        let ws_url = std::env::var("WEBHOOK_WEBSOCKET_URL").unwrap();
        let ws = WebSocket::connect(ws_url.as_str()).await?;

        let (mut ws, _) = ws.split();
        loop {
            let s = ws.receive().await?;
            let (msg, _, _) = s.as_text().unwrap();
            let (installation, event) = self.parse_webhook_event_payload(msg)?;

            let api = self.get_api(installation.id).await?;
            for listener in &self.webhook_event_listeners {
                if let Err(error) = listener(event.clone(), api.clone()) {
                    println!("Error: {}", error);
                }
            }
        }
    }

    pub async fn get_installations(&self) -> Result<Vec<GithubInstallation>, GithubError> {
        let request_url = format!("{}/app/installations", GITHUB_API_BASE_URL);

        self.api_client
            .get(request_url)
            .respond_json::<Vec<GithubInstallation>>()
            .await
    }

    pub async fn get_installation(
        &self,
        installation_id: impl Into<String>,
    ) -> Result<GithubInstallation, GithubError> {
        let request_url = format!(
            "{}/app/installations/{}",
            GITHUB_API_BASE_URL,
            installation_id.into()
        );
        self.api_client
            .get(request_url)
            .respond_json::<GithubInstallation>()
            .await
    }

    pub async fn get_installation_access_token(
        &mut self,
        installation_id: impl Into<String>,
    ) -> Result<GithubInstallationAccessToken, GithubError> {
        let installation_id: String = installation_id.into();
        let installation_id: u64 = installation_id.parse().unwrap();

        if let Some(token) = self.tokens.get(&installation_id) {
            if !token.is_expired() {
                return Ok(token.clone());
            }
        }

        let request_url = format!(
            "{}/app/installations/{}/access_tokens",
            GITHUB_API_BASE_URL, installation_id
        );

        let token: GithubInstallationAccessToken = self
            .api_client
            .post(request_url)
            .respond_json::<GithubInstallationAccessToken>()
            .await?;

        self.tokens.insert(installation_id, token.clone());

        Ok(token)
    }

    pub fn on_webhook_event<F>(&mut self, listener: F) -> &mut Self
    where
        F: Fn(
            GithubWebhookEvent,
            GithubAPI,
        ) -> Result<(), Box<dyn std::error::Error>>,
        F: Send + Sync + 'static,
    {
        self.webhook_event_listeners.push(Box::new(listener));

        self
    }

    pub async fn get_api(&mut self, installation_id: u64) -> Result<GithubAPI, GithubError> {
        if let Some(token) = self.tokens.get(&installation_id) {
            if !token.is_expired() {
                return Ok(GithubAPI::new(token.clone()));
            }
        }

        let token = self
            .get_installation_access_token(installation_id.to_string())
            .await?;

        Ok(GithubAPI::new(token))
    }

    fn parse_webhook_event_payload(
        &self,
        payload: &String,
    ) -> Result<(GithubWebhookInstallation, GithubWebhookEvent), GithubError> {
        let event: serde_json::Value = serde_json::from_str(payload)?;
        let installation = event
            .get("data")
            .ok_or(GithubError::new("No data field on webhook event"))?
            .get("installation")
            .ok_or(GithubError::new("No installation field on webhook event"))?;

        let installation =
            serde_json::from_value::<GithubWebhookInstallation>(installation.clone())?;

        if let Some(event_name) = event["event"].as_str() {
            match event_name {
                "issue_comment" => {
                    let evt = serde_json::from_value::<GithubWebhookIssueCommentEvent>(
                        event["data"].clone(),
                    )?;

                    return Ok((installation, GithubWebhookEvent::IssueComment(evt)));
                }
                _ => {
                    return Err(GithubError::new(format!(
                        "Unknown event name: {}",
                        event_name
                    )));
                }
            }
        }

        Err(GithubError::new("No event name found"))
    }
}

// tests
mod tests {

    use super::GithubApp;
    use dotenv::dotenv;
    use std::env;
    use std::fs;

    fn create_app() -> GithubApp {
        dotenv().ok();
        let github_app_id = env::var("TEST_GITHUB_APP_ID").unwrap();
        let github_app_private_key_path = env::var("TEST_GITHUB_APP_PRIVATE_KEY_PATH").unwrap();

        let private_key = fs::read_to_string(github_app_private_key_path).unwrap();

        GithubApp::new(github_app_id, private_key)
    }

    #[tokio::test]
    async fn test_connect() {
        let mut app = create_app();

        app.connect().await.unwrap();
    }

    #[tokio::test]
    async fn test_get_installations() {
        let app = create_app();

        let installations = app.get_installations().await.unwrap();
        println!("installations {:#?}", installations);
    }

    #[tokio::test]
    async fn test_get_installation() {
        let app = create_app();

        let installation_id = env::var("TEST_GITHUB_INSTALLATION_ID").unwrap();

        let installation = app.get_installation(installation_id).await.unwrap();

        println!("installation {:#?}", installation);
    }

    #[tokio::test]
    async fn test_get_installation_access_token() {
        let mut app = create_app();

        let installation_id = env::var("TEST_GITHUB_INSTALLATION_ID").unwrap();

        let access_token = app
            .get_installation_access_token(installation_id.clone())
            .await
            .unwrap();

        let access_token1 = app
            .get_installation_access_token(installation_id)
            .await
            .unwrap();

        assert_eq!(access_token.expires_at, access_token1.expires_at);
        println!("installation access token {:#?}", access_token);
    }
}
