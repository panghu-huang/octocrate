use std::ops::Deref;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::constants::GITHUB_API_BASE_URL;
use crate::infrastructure::api_client::GithubAPIClient;
use crate::infrastructure::error::GithubError;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GithubMergePullRequestResponse {
    pub sha: String,
    pub merged: bool,
    pub message: String,
}

#[derive(Clone, Debug)]
pub struct GithubPullRequestAPI {
    client: Arc<GithubAPIClient>,
}

impl GithubPullRequestAPI {
    pub fn new(client: Arc<GithubAPIClient>) -> Self {
        Self { client }
    }

    pub async fn merge(
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
}