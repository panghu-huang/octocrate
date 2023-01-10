use crate::domains::users::GithubUser;
use api_builder::github_api;

github_api! {
  GithubUserAPI {
    get_authenticated_user {
      path "/user"
      response GithubUser
    }

    get_user {
      path "/users/{}"
      params {
        username String
      }
      response GithubUser
    }
  }
}

#[cfg(test)]
mod tests {
    use super::GithubUserAPI;
    use crate::utils::test_utils;
    use infrastructure::GithubResult;
    use std::sync::Arc;

    #[tokio::test]
    async fn get_authenticated_user() -> GithubResult<()> {
        let envs = test_utils::load_test_envs()?;
        let api_client = test_utils::create_api_client()?;
        let github_api = GithubUserAPI::new(Arc::new(api_client));

        let user = github_api.get_authenticated_user().send().await?;

        assert_eq!(user.login, envs.repo_owner);

        Ok(())
    }

    #[tokio::test]
    async fn get_user() -> GithubResult<()> {
        let envs = test_utils::load_test_envs()?;
        let api_client = test_utils::create_api_client()?;
        let github_api = GithubUserAPI::new(Arc::new(api_client));

        let user = github_api.get_user(envs.repo_owner.clone()).send().await?;

        assert_eq!(user.login, envs.repo_owner);

        Ok(())
    }
}
