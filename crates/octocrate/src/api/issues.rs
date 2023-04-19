use crate::{GithubIssue, GithubIssueComment};
use octocrate_api_builder::github_api;

github_api! {
  GithubIssueAPI {
    list_repository_issues {
      path "/repos/{}/{}/issues"
      params {
        owner String
        repo String
      }
      response Vec<GithubIssue>
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

    create_issue_comment {
      path "/repos/{}/{}/issues/{}/comments"
      method POST
      params {
        owner String
        repo String
        issue_number u64
      }
      response GithubIssueComment
      test {
        params {
          envs.repo_owner
          envs.repo_name
          envs.issue_number
        }
        body {
          body "Hello World"
        }
        assert assert!(res.body == "Hello World")
      }
    }
  }
}
