use std::ops::Deref;
use std::sync::Arc;

use crate::constants::GITHUB_API_BASE_URL;
use crate::domains::issues::{GithubIssue, GithubIssueComment};
use crate::infrastructure::api_client::GithubAPIClient;
use crate::infrastructure::error::GithubError;

#[derive(Clone)]
pub struct GithubIssueAPI {
    client: Arc<GithubAPIClient>,
}

impl GithubIssueAPI {
    pub fn new(client: Arc<GithubAPIClient>) -> Self {
        Self { client }
    }

    pub async fn list_repository_issues(
        &self,
        owner: impl Into<String>,
        repo: impl Into<String>,
    ) -> Result<Vec<GithubIssue>, GithubError> {
        let request_url = format!(
            "{}/repos/{}/{}/issues?state=all&per_page=30",
            GITHUB_API_BASE_URL,
            owner.into(),
            repo.into()
        );

        self.client
            .deref()
            .get(request_url)
            .respond_json::<Vec<GithubIssue>>()
            .await
    }

    pub async fn create_issue_comment(
        &self,
        owner: impl Into<String>,
        repo: impl Into<String>,
        issue_number: impl Into<u64>,
        comment: impl Into<String>,
    ) -> Result<GithubIssueComment, GithubError> {
        let request_url = format!(
            "{}/repos/{}/{}/issues/{}/comments",
            GITHUB_API_BASE_URL,
            owner.into(),
            repo.into(),
            issue_number.into()
        );

        let body = serde_json::json!({
            "body": comment.into(),
        });

        self.client
            .deref()
            .post(request_url)
            .json(&body)
            .respond_json::<GithubIssueComment>()
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
    async fn test_list_repository_issues() {
        let mut app = create_app();
        let installation_id = env::var("TEST_GITHUB_INSTALLATION_ID").unwrap();
        let repo_owner = env::var("TEST_GITHUB_REPO_OWNER").unwrap();
        let repo_name = env::var("TEST_GITHUB_REPO_NAME").unwrap();

        let token = app
            .get_installation_access_token(installation_id)
            .await
            .unwrap();

        let api_client = GithubAPIClient::new(token);
        let issues_api = GithubIssueAPI::new(Arc::new(api_client));
        let issues = issues_api
            .list_repository_issues(repo_owner, repo_name)
            .await
            .unwrap();

        println!("issues {:#?}", issues);

        assert!(issues.len() > 0);
    }

    #[tokio::test]
    async fn test_create_issue_comment() {
        let mut app = create_app();
        let installation_id = env::var("TEST_GITHUB_INSTALLATION_ID").unwrap();
        let repo_owner = env::var("TEST_GITHUB_REPO_OWNER").unwrap();
        let repo_name = env::var("TEST_GITHUB_REPO_NAME").unwrap();
        let issue_number = env::var("TEST_GITHUB_ISSUE_NUMBER").unwrap();

        let token = app
            .get_installation_access_token(installation_id)
            .await
            .unwrap();

        let api_client = GithubAPIClient::new(token);
        let issues_api = GithubIssueAPI::new(Arc::new(api_client));
        let issue_comment = issues_api
            .create_issue_comment(
                repo_owner,
                repo_name,
                issue_number.parse::<u64>().unwrap(),
                "test",
            )
            .await
            .unwrap();

        println!("issue comment {:#?}", issue_comment);
    }
}
