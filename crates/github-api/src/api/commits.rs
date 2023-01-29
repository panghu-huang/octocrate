use crate::domains::commits::{GithubCommit, GithubCommitCombinedStatuses, GithubCommitStatus};
use api_builder::github_api;

github_api! {
  GithubCommitAPI {
    get_commit {
      path "/repos/{}/{}/commits/{}"
      params {
        owner String
        repo String
        sha String
      }
      response GithubCommit
      test {
        params {
          envs.repo_owner
          envs.repo_name
          envs.commit_sha.clone()
        }
        assert assert_eq!(res.sha, envs.commit_sha)
      }
    }

    list_commits {
      path "/repos/{}/{}/commits"
      params {
        owner String
        repo String
      }
      response Vec<GithubCommit>
      test {
        params {
          envs.repo_owner
          envs.repo_name
        }
        assert assert!(res.len() > 1)
      }
    }

    get_commit_status {
      path "/repos/{}/{}/commits/{}/status"
      params {
        owner String
        repo String
        ref_name String
      }
      response GithubCommitCombinedStatuses
      test {
        params {
          envs.repo_owner
          envs.repo_name
          envs.commit_sha.clone()
        }
        assert println!("{:#?}", res)
      }
    }

    list_commit_statuses {
      path "/repos/{}/{}/commits/{}/statuses"
      params {
        owner String
        repo String
        ref_name String
      }
      response Vec<GithubCommitStatus>
      test {
        params {
          envs.repo_owner
          envs.repo_name
          envs.commit_sha.clone()
        }
        assert println!("{:#?}", res)
      }
    }

    create_commit_status {
      path "/repos/{}/{}/statuses/{}"
      method POST
      params {
        owner String
        repo String
        sha String
      }
      response GithubCommitStatus
      test {
        params {
          envs.repo_owner
          envs.repo_name
          envs.commit_sha.clone()
        }
        body {
          state "success"
          target_url "https://www.panghu.tech"
          description "test"
          context "coodev/test"
        }
        assert assert_eq!(res.state, "success")
      }
    }
  }
}
