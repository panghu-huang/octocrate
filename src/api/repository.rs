use std::ops::Deref;
use std::sync::Arc;

use crate::constants::GITHUB_API_BASE_URL;
use crate::domains::repositories::GithubRepository;
use crate::infrastructure::api_client::GithubAPIClient;
use crate::infrastructure::error::GithubError;
use crate::infrastructure::expirable_token::ExpirableToken;

#[derive(Clone, Debug)]
pub struct GithubRepositoryAPI<T: ExpirableToken + Clone> {
    client: Arc<GithubAPIClient<T>>,
}

impl<T: ExpirableToken + Clone> GithubRepositoryAPI<T> {
    pub fn new(client: Arc<GithubAPIClient<T>>) -> Self {
        Self { client }
    }

    pub async fn get(
        &self,
        owner: impl Into<String>,
        repo: impl Into<String>,
    ) -> Result<GithubRepository, GithubError> {
        let request_url = format!(
            "{}/repos/{}/{}",
            GITHUB_API_BASE_URL,
            owner.into(),
            repo.into()
        );

        self.client
            .deref()
            .get(request_url)
            .respond_json::<GithubRepository>()
            .await
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use crate::app::GithubApp;
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
    async fn test_get_repository() {
        let mut app = create_app();
        let installation_id = env::var("TEST_GITHUB_INSTALLATION_ID").unwrap();
        let repo_owner = env::var("TEST_GITHUB_REPO_OWNER").unwrap();
        let repo_name = env::var("TEST_GITHUB_REPO_NAME").unwrap();

        let token = app
            .get_installation_access_token(installation_id)
            .await
            .unwrap();

        let api_client = GithubAPIClient::new(token);
        let repo_api = GithubRepositoryAPI::new(Arc::new(api_client));
        let repo = repo_api.get(repo_owner, repo_name).await.unwrap();

        println!("repo {:#?}", repo);
    }
}
