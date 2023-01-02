use crate::constants::GITHUB_API_BASE_URL;
use crate::infrastructure::api_client::GithubAPIClient;
use crate::infrastructure::error::GithubError;
use crate::infrastructure::expirable_token::ExpirableToken;
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
