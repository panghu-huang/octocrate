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
      test {
        params {
          envs.repo_owner
          envs.repo_name
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
      response GithubIssue
      test {
        params {
          envs.repo_owner
          envs.repo_name
          1 as u64
        }
        assert assert!(res.body.unwrap() == "Hello World")
      }
    }
  }
}