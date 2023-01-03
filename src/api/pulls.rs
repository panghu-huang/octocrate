use crate::constants::GITHUB_API_BASE_URL;
use crate::domains::pulls::GithubPullRequest;
use crate::infrastructure::{ExpirableToken, GithubAPIClient, GithubError};
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::Arc;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GithubMergePullRequestResponse {
    pub sha: String,
    pub merged: bool,
    pub message: String,
}

#[derive(Clone, Debug)]
pub struct GithubPullRequestAPI<T: ExpirableToken + Clone> {
    client: Arc<GithubAPIClient<T>>,
}

impl<T: ExpirableToken + Clone> GithubPullRequestAPI<T> {
    pub fn new(client: Arc<GithubAPIClient<T>>) -> Self {
        Self { client }
    }

    pub async fn merge_pull_request(
        &self,
        owner: impl Into<String>,
        repo: impl Into<String>,
        pull_number: impl Into<u64>,
    ) -> Result<GithubMergePullRequestResponse, GithubError> {
        let request_url = format!(
            "{}/repos/{}/{}/pulls/{}/merge",
            GITHUB_API_BASE_URL,
            owner.into(),
            repo.into(),
            pull_number.into()
        );

        self.client
            .deref()
            .put(request_url)
            .respond_json::<GithubMergePullRequestResponse>()
            .await
    }

    pub async fn get_pull_request(
        &self,
        owner: impl Into<String>,
        repo: impl Into<String>,
        pull_number: impl Into<u64>,
    ) -> Result<GithubPullRequest, GithubError> {
        let request_url = format!(
            "{}/repos/{}/{}/pulls/{}",
            GITHUB_API_BASE_URL,
            owner.into(),
            repo.into(),
            pull_number.into()
        );

        self.client
            .deref()
            .get(request_url)
            .respond_json::<GithubPullRequest>()
            .await
    }

    pub async fn list_pull_requests(
        &self,
        owner: impl Into<String>,
        repo: impl Into<String>,
    ) -> Result<Vec<GithubPullRequest>, GithubError> {
        let request_url = format!(
            "{}/repos/{}/{}/pulls?state=all&per_page=30",
            GITHUB_API_BASE_URL,
            owner.into(),
            repo.into()
        );

        self.client
            .deref()
            .get(request_url)
            .respond_json::<Vec<GithubPullRequest>>()
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::GithubPullRequestAPI;
    use crate::infrastructure::GithubResult;
    use crate::utils::test_utils;
    use std::sync::Arc;

    #[tokio::test]
    async fn list_pull_requests() -> GithubResult<()> {
        let envs = test_utils::load_test_envs()?;
        let client = test_utils::create_api_client()?;
        let api = GithubPullRequestAPI::new(Arc::new(client));

        let result = api
            .list_pull_requests(envs.repo_owner, envs.repo_name)
            .await?;

        assert!(result.len() > 0);

        Ok(())
    }

    #[tokio::test]
    async fn get_pull_request() -> GithubResult<()> {
        let envs = test_utils::load_test_envs()?;
        let client = test_utils::create_api_client()?;
        let api = GithubPullRequestAPI::new(Arc::new(client));

        let result = api
            .get_pull_request(envs.repo_owner, envs.repo_name, envs.issue_number)
            .await?;

        assert!(result.merged.is_some());

        Ok(())
    }
}
