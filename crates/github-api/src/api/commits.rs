use crate::domains::commits::GithubCommit;
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
  }
}
