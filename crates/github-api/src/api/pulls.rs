use crate::domains::pulls::{
    GithubMergePullRequestResponse, GithubPullRequest, GithubPullRequestFile,
};
use api_builder::github_api;

// TODO
// check if the pull request is mergeable
// 1. 返回 404 代表没有合并，但现在 API client 代表出错
// 2. 要使用 response_text 代替 send 方法，API 设计不统一
github_api! {
  GithubPullRequestAPI {
    get_pull_request {
      path "/repos/{}/{}/pulls/{}"
      params {
        owner String
        repo String
        pull_number u64
      }
      response GithubPullRequest
      test {
        params {
          envs.repo_owner
          envs.repo_name
          envs.issue_number.clone()
        }
        assert assert_eq!(res.number, envs.issue_number)
      }
    }

    list_pull_requests {
      path "/repos/{}/{}/pulls"
      params {
        owner String
        repo String
      }
      response Vec<GithubPullRequest>
      test {
        params {
          envs.repo_owner
          envs.repo_name
        }
        query {
          state "closed"
        }
        assert assert!(res.len() > 0)
      }
    }

    merge_pull_request {
      path "/repos/{}/{}/pulls/{}/merge"
      method PUT
      params {
        owner String
        repo String
        pull_number u64
      }
      response GithubMergePullRequestResponse
      test {
        params {
          envs.repo_owner
          envs.repo_name
          envs.issue_number
        }
        assert assert_eq!(res.merged, true)
      }
    }

    list_pull_request_files {
      path "/repos/{}/{}/pulls/{}/files"
      params {
        owner String
        repo String
        pull_number u64
      }
      response Vec<GithubPullRequestFile>
      test {
        params {
          envs.repo_owner
          envs.repo_name
          envs.issue_number
        }
        assert assert!(res.len() > 0)
      }
    }

    check_if_pull_request_is_merged {
      path "/repos/{}/{}/pulls/{}/merge"
      method GET
      params {
        owner String
        repo String
        pull_number u64
      }
      response ()
    }
  }
}

#[cfg(test)]
mod tests {
    use super::GithubPullRequestAPI;
    use crate::utils::test_utils;
    use infrastructure::GithubResult;
    use std::sync::Arc;

    #[tokio::test]
    async fn check_if_pull_request_is_merged() -> GithubResult<()> {
        let envs = test_utils::load_test_envs()?;
        let api_client = test_utils::create_api_client()?;

        let api = GithubPullRequestAPI::new(Arc::new(api_client));
        api.check_if_pull_request_is_merged(envs.repo_owner, envs.repo_name, envs.issue_number)
            .respond_text()
            .await?;

        Ok(())
    }
}
