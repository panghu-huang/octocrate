pub mod issues;
pub mod repository;
pub mod pulls;

use std::sync::Arc;

use crate::infrastructure::api_client::GithubAPIClient;
use crate::domains::installations::GithubInstallationAccessToken;

#[derive(Clone, Debug)]
pub struct GithubAPI {
  pub issues: issues::GithubIssueAPI,
  pub repository: repository::GithubRepositoryAPI,
  pub pulls: pulls::GithubPullRequestAPI,
}

impl GithubAPI {
    pub fn new(token: GithubInstallationAccessToken) -> Self {
        let client = Arc::new(GithubAPIClient::new(token));

        Self {
            issues: issues::GithubIssueAPI::new(client.clone()),
            repository: repository::GithubRepositoryAPI::new(client.clone()),
            pulls: pulls::GithubPullRequestAPI::new(client.clone()),
        }
    }
}