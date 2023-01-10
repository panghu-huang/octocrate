use crate::constants::GITHUB_API_BASE_URL;
use crate::domains::pulls::GithubPullRequest;
use infrastructure::{ExpirableToken, GithubAPIClient, GithubAPIRequest};
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

    pub fn merge_pull_request(
        &self,
        owner: impl Into<String>,
        repo: impl Into<String>,
        pull_number: impl Into<u64>,
    ) -> GithubAPIRequest<GithubMergePullRequestResponse> {
        let request_url = format!(
            "{}/repos/{}/{}/pulls/{}/merge",
            GITHUB_API_BASE_URL,
            owner.into(),
            repo.into(),
            pull_number.into()
        );

        self.client
            .deref()
            .put::<GithubMergePullRequestResponse>(request_url)
    }

    pub fn get_pull_request(
        &self,
        owner: impl Into<String>,
        repo: impl Into<String>,
        pull_number: impl Into<u64>,
    ) -> GithubAPIRequest<GithubPullRequest> {
        let request_url = format!(
            "{}/repos/{}/{}/pulls/{}",
            GITHUB_API_BASE_URL,
            owner.into(),
            repo.into(),
            pull_number.into()
        );

        self.client.deref().get::<GithubPullRequest>(request_url)
    }

    pub fn list_pull_requests(
        &self,
        owner: impl Into<String>,
        repo: impl Into<String>,
    ) -> GithubAPIRequest<Vec<GithubPullRequest>> {
        let request_url = format!(
            "{}/repos/{}/{}/pulls?state=all&per_page=30",
            GITHUB_API_BASE_URL,
            owner.into(),
            repo.into()
        );

        self.client
            .deref()
            .get::<Vec<GithubPullRequest>>(request_url)
    }
}

#[cfg(test)]
mod tests {
    use super::GithubPullRequestAPI;
    use crate::utils::test_utils;
    use infrastructure::GithubResult;
    use std::sync::Arc;

    #[tokio::test]
    async fn list_pull_requests() -> GithubResult<()> {
        let envs = test_utils::load_test_envs()?;
        let client = test_utils::create_api_client()?;
        let api = GithubPullRequestAPI::new(Arc::new(client));

        let result = api
            .list_pull_requests(envs.repo_owner, envs.repo_name)
            .send()
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
            .send()
            .await?;

        assert!(result.merged.is_some());

        Ok(())
    }
}
