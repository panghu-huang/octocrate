mod actions;
mod branches;
mod commits;
mod issues;
mod pulls;
mod repositories;
mod users;

use infrastructure::{ExpirableToken, GithubAPIClient, GithubAPIConfig};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct GithubAPI<T: ExpirableToken + Clone> {
    pub issues: issues::GithubIssueAPI<T>,
    pub repositories: repositories::GithubRepositoryAPI<T>,
    pub pulls: pulls::GithubPullRequestAPI<T>,
    pub commits: commits::GithubCommitAPI<T>,
    pub branches: branches::GithubBranchAPI<T>,
}

impl<T: ExpirableToken + Clone + 'static> GithubAPI<T> {
    pub fn new(api_config: GithubAPIConfig<T>) -> Self {
        let client = Arc::new(GithubAPIClient::new(api_config));

        Self {
            issues: issues::GithubIssueAPI::new(client.clone()),
            repositories: repositories::GithubRepositoryAPI::new(client.clone()),
            pulls: pulls::GithubPullRequestAPI::new(client.clone()),
            commits: commits::GithubCommitAPI::new(client.clone()),
            branches: branches::GithubBranchAPI::new(client.clone()),
        }
    }

    pub fn with_token(token: T) -> Self {
        let api_config = GithubAPIConfig::with_token(token);

        Self::new(api_config)
    }
}

#[cfg(test)]
mod tests {
    use super::GithubAPI;
    use crate::domains::personal_access_token::GithubPersonalAccessToken;
    use crate::utils::test_utils;
    use infrastructure::{GithubAPIConfig, GithubResult};

    #[tokio::test]
    async fn get_repository() -> GithubResult<()> {
        let envs = test_utils::load_test_envs()?;
        let token = GithubPersonalAccessToken::new(envs.personal_access_token);
        let api_config = GithubAPIConfig::with_token(token);
        let github_api = GithubAPI::new(api_config);

        let repo = github_api
            .repositories
            .get_repository(envs.repo_owner, envs.repo_name.clone())
            .send()
            .await?;

        assert_eq!(repo.name, envs.repo_name);

        Ok(())
    }
}
