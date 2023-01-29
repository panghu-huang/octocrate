use crate::domains::deployments::GithubDeployment;
use api_builder::github_api;

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
    }
  }
}