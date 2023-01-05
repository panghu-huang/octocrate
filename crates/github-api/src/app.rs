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
use infrastructure::{ExpirableToken, GithubAPIClient, GithubError, GithubResult};
use std::collections::HashMap;
use websockets::WebSocket;

pub type GithubWebhookEventListener = Box<
    dyn Fn(
            GithubWebhookEvent,
            GithubAPI<GithubInstallationAccessToken>,
        ) -> Result<(), Box<dyn std::error::Error>>
        + Send
        + Sync,
>;

pub struct GithubApp {
    webhook_event_listeners: Vec<GithubWebhookEventListener>,
    tokens: HashMap<u64, GithubInstallationAccessToken>,
    api_client: GithubAPIClient<GithubInstallationExpirableToken>,
}

impl GithubApp {
    pub fn new(id: impl Into<String>, private_key: impl Into<String>) -> Self {
        let mut token = GithubInstallationExpirableToken::new(id.into(), private_key.into());

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

    pub async fn start(&mut self) -> GithubResult<()> {
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
                    eprintln!("Error: {}", error);
                }
            }
        }
    }

    pub async fn get_installations(&self) -> GithubResult<Vec<GithubInstallation>> {
        let request_url = format!("{}/app/installations", GITHUB_API_BASE_URL);

        self.api_client
            .get(request_url)
            .respond_json::<Vec<GithubInstallation>>()
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
            .get(request_url)
            .respond_json::<GithubInstallation>()
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
        payload: &String,
    ) -> GithubResult<(GithubWebhookInstallation, GithubWebhookEvent)> {
        let event: serde_json::Value = serde_json::from_str(payload)?;

        if let Some(event_name) = event["event"].as_str() {
            match event_name {
                "issue_comment" => {
                    let evt = serde_json::from_value::<GithubWebhookIssueCommentEvent>(
                        event["data"].clone(),
                    )?;

                    return Ok((
                        evt.installation.clone(),
                        GithubWebhookEvent::IssueComment(evt),
                    ));
                }
                "pull_request" => {
                    let evt = serde_json::from_value::<GithubWebhookPullRequestEvent>(
                        event["data"].clone(),
                    )?;

                    return Ok((
                        evt.installation.clone(),
                        GithubWebhookEvent::PullRequest(evt),
                    ));
                }
                "push" => {
                    let evt =
                        serde_json::from_value::<GithubWebhookPushEvent>(event["data"].clone())?;

                    return Ok((evt.installation.clone(), GithubWebhookEvent::Push(evt)));
                }
                _ => {
                    let installation = event
                        .get("data")
                        .ok_or(GithubError::new("No data field on webhook event"))?
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

        Err(GithubError::new("No event name found"))
    }
}

#[cfg(test)]
mod tests {
    use infrastructure::GithubResult;
    use crate::test_utils;

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
