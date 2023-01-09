# Github API Builder with tests

## Usage
```rust
use api_builder::github_api;
use crate::domains::issues::GithubIssue;

github_api! {
  GithubIssueAPI {
    list_repository_issues {
      path "/repos/{}/{}/issues"
      params {
        owner String
        repo String
      }
      response Vec<GithubIssue>
    }
  }
}
```

Will compile as below
```rust
use std::ops::Deref;
use std::sync::Arc;

use crate::constants::GITHUB_API_BASE_URL;
use crate::domains::issues::{GithubIssue};
use infrastructure::{ExpirableToken, GithubAPIClient, GithubError};

#[derive(Clone, Debug)]
pub struct GithubIssueAPI<T: ExpirableToken + Clone> {
    client: Arc<GithubAPIClient<T>>,
}

impl<T: ExpirableToken + Clone> GithubIssueAPI<T> {
    pub fn new(client: Arc<GithubAPIClient<T>>) -> Self {
        Self { client }
    }

    pub async fn list_repository_issues(
        &self,
        owner: impl Into<String>,
        repo: impl Into<String>,
    ) -> Result<Vec<GithubIssue>, GithubError> {
        let request_url = format!(
            "{}/repos/{}/{}/issues",
            GITHUB_API_BASE_URL,
            owner.into(),
            repo.into()
        );

        self.client
            .deref()
            .get(request_url)
            .respond_json::<Vec<GithubIssue>>()
            .await
    }
}
```
If you want add tests
```rust
github_api! {
  GithubIssueAPI {
    list_repository_issues {
      /// ...
      /// Add test block
      test {
        params {
          envs.repo_owner
          envs.repo_name
        }
        assert assert!(res.len() > 0)
      }
    }
  }
}
```

Will compile as below

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test_utils;
    use infrastructure::GithubResult;

    #[tokio::test]
    async fn list_repository_issues() -> GithubResult<()> {
        let envs = test_utils::load_test_envs()?;
        let api_client = test_utils::create_api_client()?;

        let api = GithubIssueAPI::new(Arc::new(api_client));
        let res = api
            .list_repository_issues(envs.repo_owner, envs.repo_name)
            .await?;

        assert!(res.len() > 0);

        Ok(())
    }
}
```