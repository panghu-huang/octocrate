use std::ops::Deref;
use std::sync::Arc;

use crate::constants::GITHUB_API_BASE_URL;
use crate::domains::repositories::GithubRepository;
use crate::infrastructure::{ExpirableToken, GithubAPIClient, GithubError};

#[derive(Clone, Debug)]
pub struct GithubRepositoryAPI<T: ExpirableToken + Clone> {
    client: Arc<GithubAPIClient<T>>,
}

impl<T: ExpirableToken + Clone> GithubRepositoryAPI<T> {
    pub fn new(client: Arc<GithubAPIClient<T>>) -> Self {
        Self { client }
    }

    pub async fn get_repository(
        &self,
        owner: impl Into<String>,
        repo: impl Into<String>,
    ) -> Result<GithubRepository, GithubError> {
        let request_url = format!(
            "{}/repos/{}/{}",
            GITHUB_API_BASE_URL,
            owner.into(),
            repo.into()
        );

        self.client
            .deref()
            .get(request_url)
            .respond_json::<GithubRepository>()
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::GithubRepositoryAPI;
    use crate::infrastructure::GithubResult;
    use crate::utils::test_utils;
    use std::sync::Arc;

    #[tokio::test]
    async fn get_repository() -> GithubResult<()> {
        let envs = test_utils::load_test_envs()?;
        let api_client = test_utils::create_api_client()?;

        let repo_api = GithubRepositoryAPI::new(Arc::new(api_client));
        let repo = repo_api
            .get_repository(envs.repo_owner, envs.repo_name.clone())
            .await?;

        assert_eq!(repo.name, envs.repo_name);

        Ok(())
    }
}
