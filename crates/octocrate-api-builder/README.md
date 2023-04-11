# Github API Builder with tests

## Usage
```rust
use octocrate_api_builder::github_api;
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

use crate::domains::issues::{GithubIssue};
use octocrate_infra::{ExpirableToken, GithubAPIClient, GithubError};

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
            "/repos/{}/{}/issues",
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
    use octocrate_infra::GithubResult;

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

You can pass query or body to test
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
        query {
          state "closed"
        }
        assert assert!(res.len() > 0)
      }
    }
  }
}
```

```rust
github_api! {
  GithubIssueAPI {
    create_issue_comment {
      // ...
      test {
        // ...
        body {
          body "Hello World"
        }
        assert assert!(res.body == "Hello World")
      }
    }
  }
}

```