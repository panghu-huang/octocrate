use crate::GithubReference;
use octocrate_api_builder::github_api;

github_api! {
  GithubGitDatabaseAPI {
    get_reference {
      path "/repos/{}/{}/git/ref/{}"
      method GET
      params {
        owner String
        repo String
        ref_name String
      }
      response GithubReference
      test {
        params {
          envs.repo_owner
          envs.repo_name
          format!("heads/{}", envs.branch_name)
        }
        assert assert_eq!(res.ref_name, format!("refs/heads/{}", envs.branch_name))
      }
    }

    create_reference {
      path "/repos/{}/{}/git/refs"
      method POST
      params {
        owner String
        repo String
      }
      response GithubReference
    }

    update_reference {
      path "/repos/{}/{}/git/refs/{}"
      method PATCH
      params {
        owner String
        repo String
        ref_name String
      }
      response GithubReference
    }

    delete_reference {
      path "/repos/{}/{}/git/refs/{}"
      method DELETE
      params {
        owner String
        repo String
        ref_name String
      }
      response GithubReference
    }

    list_matching_references {
      path "/repos/{}/{}/git/matching-refs/{}"
      method GET
      params {
        owner String
        repo String
        ref_name String
      }
      response Vec<GithubReference>
      test {
        params {
          envs.repo_owner
          envs.repo_name
          format!("heads/{}", envs.branch_name)
        }
        assert assert_eq!(res.len(), 1)
      }
    }
  }
}

#[cfg(test)]
mod tests {
    use super::GithubGitDatabaseAPI;
    use crate::utils::test_utils;
    use octocrate_infra::GithubResult;
    use std::sync::Arc;

    #[tokio::test]
    #[ignore]
    async fn delete_reference() -> GithubResult<()> {
        let envs = test_utils::load_test_envs()?;
        let api_client = test_utils::create_api_client()?;

        let api = GithubGitDatabaseAPI::new(Arc::new(api_client));
        api.delete_reference(envs.repo_owner, envs.repo_name, "heads/test-branch")
            // Use `respond_text` instead of `send`
            .respond_text()
            .await?;

        Ok(())
    }
}
