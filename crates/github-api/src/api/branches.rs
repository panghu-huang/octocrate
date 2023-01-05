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
  }
}

#[cfg(test)]
mod tests {
    use super::GithubBranchAPI;
    use crate::utils::test_utils;
    use infrastructure::GithubResult;
    use std::sync::Arc;

    #[tokio::test]
    async fn list_branches() -> GithubResult<()> {
        let envs = test_utils::load_test_envs()?;
        let api_client = test_utils::create_api_client()?;
        let github_api = GithubBranchAPI::new(Arc::new(api_client));

        let branches = github_api
            .list_branches(envs.repo_owner, envs.repo_name.clone())
            .await?;

        println!("{:?}", branches);

        assert!(branches.len() > 0);

        Ok(())
    }
}
