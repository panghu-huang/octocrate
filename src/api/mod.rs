pub mod issues;
pub mod repository;

use std::sync::Arc;

use crate::infrastructure::api_client::GithubAPIClient;
use crate::domains::installations::GithubInstallationAccessToken;


pub struct GithubAPI {
  pub issues: issues::GithubIssueAPI,
  pub repository: repository::GithubRepositoryAPI,
}

impl GithubAPI {
    pub fn new(token: GithubInstallationAccessToken) -> Self {
        let client = Arc::new(GithubAPIClient::new(token));

        Self {
            issues: issues::GithubIssueAPI::new(client.clone()),
            repository: repository::GithubRepositoryAPI::new(client.clone()),
        }
    }
}