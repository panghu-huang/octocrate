pub mod issues;
pub mod pulls;
pub mod repository;
pub mod commits;

use crate::infrastructure::api_client::GithubAPIClient;
use crate::infrastructure::expirable_token::ExpirableToken;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct GithubAPI<T: ExpirableToken + Clone> {
    pub issues: issues::GithubIssueAPI<T>,
    pub repositories: repository::GithubRepositoryAPI<T>,
    pub pulls: pulls::GithubPullRequestAPI<T>,
    pub commits: commits::GithubCommitAPI<T>,
}

impl<T: ExpirableToken + Clone + 'static> GithubAPI<T> {
    pub fn new(token: T) -> Self {
        let client = Arc::new(GithubAPIClient::new(token));

        Self {
            issues: issues::GithubIssueAPI::new(client.clone()),
            repositories: repository::GithubRepositoryAPI::new(client.clone()),
            pulls: pulls::GithubPullRequestAPI::new(client.clone()),
            commits: commits::GithubCommitAPI::new(client.clone()),
        }
    }
}
