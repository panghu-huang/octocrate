use crate::{GithubDeployment, GithubDeploymentStatus};
use octocrate_api_builder::github_api;

github_api! {
  GithubDeploymentAPI {
    get_deployment {
      path "/repos/{}/{}/deployments/{}"
      params {
        owner String
        repo String
        deployment_id u32
      }
      response GithubDeployment
    }

    list_deployments {
      path "/repos/{}/{}/deployments"
      params {
        owner String
        repo String
      }
      response Vec<GithubDeployment>
      test {
        params {
          envs.repo_owner
          envs.repo_name
        }
        assert assert!(res.len() > 0)
      }
    }

    create_deployment {
      method POST
      path "/repos/{}/{}/deployments"
      params {
        owner String
        repo String
      }
      response GithubDeployment
    }

    list_deployment_statuses {
      path "/repos/{}/{}/deployments/{}/statuses"
      params {
        owner String
        repo String
        deployment_id u64
      }
      response Vec<GithubDeploymentStatus>
      test {
        params {
          envs.repo_owner
          envs.repo_name
          envs.deployment_id
        }
        assert println!("{:#?}", res)
      }
    }

    create_deployment_status {
      method POST
      path "/repos/{}/{}/deployments/{}/statuses"
      params {
        owner String
        repo String
        deployment_id u64
      }
      response GithubDeploymentStatus
      test {
        ignore
        params {
          envs.repo_owner
          envs.repo_name
          envs.deployment_id
        }
        body {
          state crate::GithubDeploymentState::Success
          environment "production"
          description "Deployed to production"
          log_url "https://example.com"
          environment_url "https://example.com"
          target_url "https://example.com"
        }
        assert assert_eq!(res.state, crate::GithubDeploymentState::Success)
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::GithubDeploymentAPI;
  use crate::utils::test_utils;
  use octocrate_infra::GithubResult;
  use std::sync::Arc;

  #[ignore]
  #[tokio::test]
  async fn test_create_deployment() -> GithubResult<()> {
    let envs = test_utils::load_test_envs()?;
    let api_client = test_utils::create_api_client()?;

    let api = GithubDeploymentAPI::new(Arc::new(api_client));

    let res = api
      .create_deployment(envs.repo_owner, envs.repo_name)
      .body(&serde_json::json!({
        "ref": "88d3bd59fe5c1c1f46d0424418a87b63614106f9",
        "environment": "production",
        "description": "Deploying to production",
        "payload": {"foo": "bar"},
      }))
      .send()
      .await?;

    println!("{:#?}", res);

    Ok(())
  }
}
