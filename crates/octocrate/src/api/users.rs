use crate::{GithubUser, GithubUserInstallationRepositories, GithubUserInstallations};
use octocrate_api_builder::github_api;

github_api! {
  GithubUserAPI {
    get_authenticated_user {
      path "/user"
      response GithubUser
      test {
        assert assert_eq!(res.login, envs.repo_owner)
      }
    }

    get_user {
      path "/users/{}"
      params {
        username String
      }
      response GithubUser
      test {
        params {
          envs.repo_owner.clone()
        }
        assert assert_eq!(res.login, envs.repo_owner)
      }
    }

    list_user_installations {
      path "/user/installations"
      response GithubUserInstallations
    }

    list_user_installation_repositories {
      path "/user/installations/{}/repositories"
      params {
        installation_id u64
      }
      response GithubUserInstallationRepositories
      test {
        params {
          envs.installation_id
        }
        assert assert!(res.total_count > 0)
      }
    }
  }
}
