use crate::domains::actions::{
    GithubActionSecret, GithubActionSecretPublicKey, GithubActionSecrets,
};
use octocrate_api_builder::github_api;

github_api! {
  /// see https://docs.github.com/en/rest/reference/actions#secrets
  GithubActionAPI {
    list_secrets {
      path "/repos/{}/{}/actions/secrets"
      params {
        owner String
        repo String
      }
      response GithubActionSecrets
      test {
        params {
          envs.repo_owner
          "personal-service-rs"
        }
        assert assert!(res.total_count > 0)
      }
    }

    get_secret {
      path "/repos/{}/{}/actions/secrets/{}"
      params {
        owner String
        repo String
        secret_name String
      }
      response GithubActionSecret
      test {
        params {
          envs.repo_owner
          "personal-service-rs"
          "HOST"
        }
        assert assert_eq!(res.name, "HOST")
      }
    }

    delete_secret {
      path "/repos/{}/{}/actions/secrets/{}"
      method DELETE
      params {
        owner String
        repo String
        secret_name String
      }
      response ()
    }

    get_public_key {
      path "/repos/{}/{}/actions/secrets/public-key"
      params {
        owner String
        repo String
      }
      response GithubActionSecretPublicKey
    }

    create_or_update_secret {
      path "/repos/{}/{}/actions/secrets/{}"
      method PUT
      params {
        owner String
        repo String
        secret_name String
      }
      response GithubActionSecret
    }
  }
}
