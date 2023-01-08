use crate::domains::branches::GithubBranch;
use api_builder::github_api;

github_api! {
  GithubBranchAPI {
    list_branches {
      path "/repos/{}/{}/branches"
      params {
        owner String
        repo String
      }
      response Vec<GithubBranch>
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
          envs.repo_name.clone()
          envs.branch_name.clone()
        }
        assert assert_eq!(res.name, envs.branch_name)
      }
    }
  }
}

// #[cfg(test)]
// mod tests {
//     use super::GithubBranchAPI;
//     use crate::utils::test_utils;
//     use infrastructure::GithubResult;
//     use std::sync::Arc;

//     #[tokio::test]
//     async fn list_branches() -> GithubResult<()> {
//         let envs = test_utils::load_test_envs()?;
//         let api_client = test_utils::create_api_client()?;
//         let github_api = GithubBranchAPI::new(Arc::new(api_client));

//         let branches = github_api
//             .list_branches(envs.repo_owner, envs.repo_name.clone())
//             .await?;

//         assert!(branches.len() > 0);

//         Ok(())
//     }

//     #[tokio::test]
//     async fn get_branch() -> GithubResult<()> {
//         let envs = test_utils::load_test_envs()?;
//         let api_client = test_utils::create_api_client()?;
//         let github_api = GithubBranchAPI::new(Arc::new(api_client));

//         let branch = github_api
//             .get_branch(
//                 envs.repo_owner,
//                 envs.repo_name.clone(),
//                 envs.branch_name.clone(),
//             )
//             .await?;

//         assert!(branch.name == envs.branch_name);

//         Ok(())
//     }
// }
