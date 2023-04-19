use crate::{
    GithubAPI, GithubInstallation, GithubInstallationAccessToken, GithubInstallationExpirableToken,
};
use octocrate_infra::{
    ExpirableToken, GithubAPIClient, GithubAPIConfig, GithubError, GithubResult,
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex, MutexGuard},
};

pub struct GithubAppInner {
    tokens: HashMap<u64, GithubInstallationAccessToken>,
}

#[derive(Clone)]
pub struct GithubApp {
    base_url: String,
    api_client: GithubAPIClient,
    inner: Arc<Mutex<GithubAppInner>>,
}

pub struct GithubAppBuilder {
    app_id: Option<String>,
    private_key: Option<String>,
    base_url: Option<String>,
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

        let inner = GithubAppInner { tokens };

        Self {
            base_url: base_url,
            api_client: GithubAPIClient::new(api_config),
            inner: Arc::new(Mutex::new(inner)),
        }
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
        &self,
        installation_id: impl Into<u64>,
    ) -> GithubResult<GithubInstallationAccessToken> {
        let installation_id: u64 = installation_id.into();

        let inner = self.get_locked_inner()?.tokens.clone();

        if let Some(token) = inner.get(&installation_id) {
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

        let mut inner = self.get_locked_inner()?;
        inner.tokens.insert(installation_id, token.clone());

        Ok(token)
    }

    pub async fn get_api(&self, installation_id: u64) -> GithubResult<GithubAPI> {
        let tokens = self.get_locked_inner()?.tokens.clone();
        if let Some(token) = tokens.get(&installation_id) {
            if !token.is_expired() {
                let api_config = GithubAPIConfig::new(self.base_url.clone(), token.clone());
                return Ok(GithubAPI::new(api_config));
            }
        }
        drop(tokens);

        let token = self.get_installation_access_token(installation_id).await?;
        let api_config = GithubAPIConfig::new(self.base_url.clone(), token.clone());
        return Ok(GithubAPI::new(api_config));
    }

    fn get_locked_inner(&self) -> Result<MutexGuard<GithubAppInner>, GithubError> {
        match self.inner.lock() {
            Ok(inner) => Ok(inner),
            Err(_) => Err(GithubError::new("Failed to get inner")),
        }
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

    #[cfg(feature = "webhook-events")]
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
        let app = test_utils::create_github_app()?;
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

    #[tokio::test]
    async fn get_api() -> GithubResult<()> {
        let app = test_utils::create_github_app()?;
        let envs = test_utils::load_test_envs()?;
        let installation_id = envs.installation_id;
        let repo_installation = app
            .get_repository_installation("panghu-huang", "octocrate")
            .await?;

        let api1 = app.get_api(installation_id);
        let api2 = app.get_api(repo_installation.id);
        let _res = tokio::join!(api1, api2);

        Ok(())
    }
}
