use crate::domains::repositories::GithubRepository;

use request_builder::github_api;

github_api! {
  GithubRepositoryAPI {
    get_repository {
      path "/repos/{}/{}"
      params {
        owner String
        name String
      }
      response GithubRepository
    }
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
