use std::ops::Deref;
use std::sync::Arc;

use crate::constants::GITHUB_API_BASE_URL;
use crate::domains::issues::{GithubIssue, GithubIssueComment};
use crate::infrastructure::{ExpirableToken, GithubAPIClient, GithubError};

#[derive(Clone, Debug)]
pub struct GithubIssueAPI<T: ExpirableToken + Clone> {
    client: Arc<GithubAPIClient<T>>,
}

impl<T: ExpirableToken + Clone> GithubIssueAPI<T> {
    pub fn new(client: Arc<GithubAPIClient<T>>) -> Self {
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

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use crate::app::GithubApp;
    use crate::infrastructure::GithubResult;
    use crate::utils::test_utils;

    #[tokio::test]
    async fn list_repository_issues() -> GithubResult<()> {
        let envs = test_utils::load_test_envs()?;
        let api_client = test_utils::create_api_client()?;

        let issues_api = GithubIssueAPI::new(Arc::new(api_client));
        let issues = issues_api
            .list_repository_issues(envs.repo_owner, envs.repo_name)
            .await?;

        assert!(issues.len() > 0);

        Ok(())
    }

    #[tokio::test]
    async fn create_issue_comment() -> GithubResult<()> {
        let envs = test_utils::load_test_envs()?;
        let api_client = test_utils::create_api_client()?;

        let issues_api = GithubIssueAPI::new(Arc::new(api_client));
        let comment_body = "test";
        let issue_comment = issues_api
            .create_issue_comment(envs.repo_owner, envs.repo_name, envs.issue_number, comment_body)
            .await?;

        assert_eq!(issue_comment.body, comment_body);

        Ok(())
    }
}
