use crate::constants::GITHUB_API_BASE_URL;
use crate::domains::commits::GithubCommit;
use crate::infrastructure::api_client::GithubAPIClient;
use crate::infrastructure::error::GithubError;
use crate::infrastructure::expirable_token::ExpirableToken;
use std::ops::Deref;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct GithubCommitAPI<T: ExpirableToken + Clone> {
    client: Arc<GithubAPIClient<T>>,
}

impl<T: ExpirableToken + Clone> GithubCommitAPI<T> {
    pub fn new(client: Arc<GithubAPIClient<T>>) -> Self {
        Self { client }
    }

    pub async fn list_repository_commits(
        &self,
        owner: impl Into<String>,
        repo: impl Into<String>,
    ) -> Result<Vec<GithubCommit>, GithubError> {
        let request_url = format!(
            "{}/repos/{}/{}/commits",
            GITHUB_API_BASE_URL,
            owner.into(),
            repo.into(),
        );

        self.client
            .deref()
            .get(request_url)
            .respond_json::<Vec<GithubCommit>>()
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::api::commits::GithubCommitAPI;
    use crate::app::GithubApp;
    use crate::infrastructure::api_client::GithubAPIClient;
    use dotenv::dotenv;
    use std::env;
    use std::fs;
    use std::sync::Arc;

    fn create_app() -> GithubApp {
        dotenv().ok();
        let github_app_id = env::var("TEST_GITHUB_APP_ID").unwrap();
        let github_app_private_key_path = env::var("TEST_GITHUB_APP_PRIVATE_KEY_PATH").unwrap();

        let private_key = fs::read_to_string(github_app_private_key_path).unwrap();

        GithubApp::new(github_app_id, private_key)
    }

    #[tokio::test]
    async fn test_list_repository_commits() {
        let mut app = create_app();
        let installation_id = env::var("TEST_GITHUB_INSTALLATION_ID").unwrap();
        let repo_owner = env::var("TEST_GITHUB_REPO_OWNER").unwrap();
        let repo_name = env::var("TEST_GITHUB_REPO_NAME").unwrap();

        let token = app
            .get_installation_access_token(installation_id)
            .await
            .unwrap();
        let client = Arc::new(GithubAPIClient::new(token));
        let api = GithubCommitAPI::new(client);

        let commits = api
            .list_repository_commits(repo_owner, repo_name)
            .await
            .unwrap();

        println!("{:?}", commits);

        assert!(commits.len() > 0);
    }
}
