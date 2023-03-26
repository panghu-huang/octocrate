mod actions;
mod branches;
mod commits;
mod deployments;
mod git_database;
mod issues;
mod pulls;
mod repositories;
mod users;

use infrastructure::{ExpirableToken, GithubAPIClient, GithubAPIConfig};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct GithubAPI {
    pub issues: issues::GithubIssueAPI,
    pub repositories: repositories::GithubRepositoryAPI,
    pub pulls: pulls::GithubPullRequestAPI,
    pub commits: commits::GithubCommitAPI,
    pub branches: branches::GithubBranchAPI,
    pub deployments: deployments::GithubDeploymentAPI,
    pub git_database: git_database::GithubGitDatabaseAPI,
}

impl GithubAPI {
    pub fn new(api_config: GithubAPIConfig) -> Self {
        let client = Arc::new(GithubAPIClient::new(api_config));

        Self {
            issues: issues::GithubIssueAPI::new(client.clone()),
            repositories: repositories::GithubRepositoryAPI::new(client.clone()),
            pulls: pulls::GithubPullRequestAPI::new(client.clone()),
            commits: commits::GithubCommitAPI::new(client.clone()),
            branches: branches::GithubBranchAPI::new(client.clone()),
            deployments: deployments::GithubDeploymentAPI::new(client.clone()),
            git_database: git_database::GithubGitDatabaseAPI::new(client.clone()),
        }
    }

    pub fn with_token<T>(token: T) -> Self
    where
        T: ExpirableToken + 'static,
    {
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
