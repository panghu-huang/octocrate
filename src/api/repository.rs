use std::ops::Deref;
use std::sync::Arc;

use crate::constants::GITHUB_API_BASE_URL;
use crate::domains::repositories::GithubRepository;
use crate::infrastructure::api_client::GithubAPIClient;
use crate::infrastructure::error::GithubError;

#[derive(Clone, Debug)]
pub struct GithubRepositoryAPI {
    client: Arc<GithubAPIClient>,
}

impl GithubRepositoryAPI {
    pub fn new(client: Arc<GithubAPIClient>) -> Self {
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
        let repo = repo_api
            .get(repo_owner, repo_name)
            .await
            .unwrap();

        println!("repo {:#?}", repo);
    }
}
