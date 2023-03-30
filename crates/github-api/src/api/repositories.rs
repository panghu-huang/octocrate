use crate::domains::repositories::{
    GithubRepository, GithubRepositoryContent, GithubRepositoryContentResponse,
};
use api_builder::github_api;

github_api! {
  GithubRepositoryAPI {
    get_repository {
      path "/repos/{}/{}"
      params {
        owner String
        name String
      }
      response GithubRepository
      test {
        params {
          envs.repo_owner
          envs.repo_name.clone()
        }
        assert assert_eq!(res.name, envs.repo_name)
      }
    }

    list_organization_repositories {
      path "/orgs/{}/repos"
      params {
        org String
      }
      response Vec<GithubRepository>
      test {
        params {
          "coodevjs"
        }
        assert assert!(res.len() > 0)
      }
    }

    list_user_repositories {
      path "/users/{}/repos"
      params {
        username String
      }
      response Vec<GithubRepository>
      test {
        params {
          "panghu-huang"
        }
        assert assert!(res.len() > 0)
      }
    }

    list_repository_language {
      path "/repos/{}/{}/languages"
      params {
        owner String
        repo String
      }
      response std::collections::HashMap<String, u64>
      test {
        params {
          envs.repo_owner
          "github-api"
        }
        assert assert!(res.len() > 0)
      }
    }

    list_authenticated_user_repositories {
      path "/user/repos"
      response Vec<GithubRepository>
      test {
        assert assert!(res.len() > 0)
      }
    }

    list_repository_contents {
      path "/repos/{}/{}/contents/{}"
      params {
        owner String
        repo String
        path String
      }
      response GithubRepositoryContentResponse
      test {
        params {
          envs.repo_owner
          envs.repo_name
          "README.md"
        }
        assert println!("{:?}", res)
      }
    }

    get_repository_readme {
      path "/repos/{}/{}/readme"
      params {
        owner String
        repo String
      }
      response GithubRepositoryContent
      test {
        params {
          envs.repo_owner
          envs.repo_name
        }
        assert assert_eq!(res.content_type, "file")
      }
    }

    get_repository_readme_for_dir {
      path "/repos/{}/{}/readme/{}"
      params {
        owner String
        repo String
        path String
      }
      response GithubRepositoryContent
      test {
        params {
          envs.repo_owner
          envs.repo_name
          "."
        }
        assert println!("{:?}", res)
      }
    }
  }
}
