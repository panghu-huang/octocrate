use std::collections::HashMap;

use crate::domains::{
    account::GithubAccount,
    jwt_token::GithubJWTExpirableToken,
    installation::{GithubInstallation, GithubInstallationAccessToken},
    issue::{GithubComment, GithubIssue},
    repository::GithubRepository,
};
use crate::infrastructure::api_client::GithubAPIClient;
use crate::infrastructure::error::GithubError;
use crate::infrastructure::expirable_token::ExpirableToken;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GithubCommentEvent {
    action: String,
    issue: GithubIssue,
    comment: GithubComment,
    repository: GithubRepository,
    sender: GithubAccount,
}

pub enum GithubWebhookEvent {
    IssueComment(GithubCommentEvent),
}

pub struct GithubWebHookEventPayload {
    event: GithubWebhookEvent,
    api: String,
}

pub type GithubWebhookEventListener = Box<dyn Fn(GithubWebHookEventPayload) + Send + Sync>;

pub struct GithubApp {
    id: String,
    listeners: Vec<GithubWebhookEventListener>,
    tokens: HashMap<u64, GithubInstallationAccessToken>,
    api_client: GithubAPIClient,
}

const GITHUB_API_BASE_URL: &str = "https://api.github.com";

impl GithubApp {
    pub fn new(id: String, private_key: String) -> Self {
        let token = GithubJWTExpirableToken::new(id.clone(), private_key.clone());

        let tokens = HashMap::new();

        Self {
            id,
            tokens,
            api_client: GithubAPIClient::new(token),
            listeners: Vec::new(),
        }
    }

    pub async fn get_installations(&mut self) -> Result<Vec<GithubInstallation>, GithubError> {
        let request_url = format!("{}/app/installations", GITHUB_API_BASE_URL);

        self.api_client.get(request_url).respond_json::<Vec<GithubInstallation>>().await
    }

    pub async fn get_installation(
        &mut self,
        installation_id: impl Into<String>,
    ) -> Result<GithubInstallation, GithubError> {
        let request_url = format!(
            "{}/app/installations/{}",
            GITHUB_API_BASE_URL,
            installation_id.into()
        );
        self.api_client.get(request_url).respond_json::<GithubInstallation>().await
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

        let token: GithubInstallationAccessToken =
            self.api_client.post(request_url).respond_json::<GithubInstallationAccessToken>().await?;

        self.tokens.insert(installation_id, token.clone());

        Ok(token)
    }

    pub fn on_event<F>(&mut self, listener: F)
    where
        F: Fn(GithubWebHookEventPayload) + Send + Sync + 'static,
    {
        self.listeners.push(Box::new(listener));
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
    async fn get_installations() {
        let mut app = create_app();

        let installations = app.get_installations().await.unwrap();
        println!("installations {:#?}", installations);
    }

    #[tokio::test]
    async fn get_installation() {
        let mut app = create_app();

        let installation_id = env::var("TEST_GITHUB_INSTALLATION_ID").unwrap();

        let installation = app.get_installation(installation_id).await.unwrap();
        
        println!("installation {:#?}", installation);
    }

    #[tokio::test]
    async fn get_installation_access_token() {
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
