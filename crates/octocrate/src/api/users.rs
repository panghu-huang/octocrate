use crate::domains::users::GithubUser;
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
  }
}
