use crate::domains::branches::GithubBranch;
use octocrate_api_builder::github_api;

github_api! {
  /// see https://docs.github.com/en/rest/reference/repos#branches
  GithubBranchAPI {
    list_branches {
      path "/repos/{}/{}/branches"
      params {
        owner String
        repo String
      }
      response Vec<GithubBranch>
      test {
        params {
          envs.repo_owner
          envs.repo_name
        }
        assert assert!(res.len() > 1)
      }
    }

    get_branch {
      path "/repos/{}/{}/branches/{}"
      params {
        owner String
        repo String
        branch String
      }
      response GithubBranch
      test {
        params {
          envs.repo_owner
          envs.repo_name
          envs.branch_name.clone()
        }
        assert assert_eq!(res.name, envs.branch_name)
      }
    }

    rename_branch {
      path "/repos/{}/{}/branches/{}/rename"
      method POST
      params {
        owner String
        repo String
        branch String
      }
      response GithubBranch
    }
  }
}
