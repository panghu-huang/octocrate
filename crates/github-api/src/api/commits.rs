use crate::constants::GITHUB_API_BASE_URL;
use crate::domains::commits::GithubCommit;
use infrastructure::{ExpirableToken, GithubAPIClient, GithubAPIRequest};
use std::ops::Deref;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct GithubCommitAPI<T: ExpirableToken + Clone> {
    client: Arc<GithubAPIClient<T>>,
}

impl<T: ExpirableToken + Clone> GithubCommitAPI<T> {
    pub fn new(client: Arc<GithubAPIClient<T>>) -> Self {
        Self { client }
    }

    pub fn list_repository_commits(
        &self,
        owner: impl Into<String>,
        repo: impl Into<String>,
    ) -> GithubAPIRequest<Vec<GithubCommit>> {
        let request_url = format!(
            "{}/repos/{}/{}/commits",
            GITHUB_API_BASE_URL,
            owner.into(),
            repo.into(),
        );

        self.client.deref().get::<Vec<GithubCommit>>(request_url)
    }
}

#[cfg(test)]
mod tests {
    use super::GithubCommitAPI;
    use crate::utils::test_utils;
    use infrastructure::GithubResult;
    use std::sync::Arc;

    #[tokio::test]
    async fn list_repository_commits() -> GithubResult<()> {
        let envs = test_utils::load_test_envs()?;
        let api_client = test_utils::create_api_client()?;

        let api = GithubCommitAPI::new(Arc::new(api_client));

        let commits = api
            .list_repository_commits(envs.repo_owner, envs.repo_name)
            .send()
            .await?;

        assert!(commits.len() > 0);

        Ok(())
    }
}
