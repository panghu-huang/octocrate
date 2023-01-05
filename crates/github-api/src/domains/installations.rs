use crate::domains::accounts::GithubAccount;
use crate::domains::permissions::GithubPermissions;
use crate::domains::repositories::GithubRepository;
use infrastructure::ExpirableToken;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubInstallation {
  pub id: u64,
  pub account: GithubAccount,
  pub access_tokens_url: String,
  pub repositories_url: String,
  pub html_url: String,
  pub app_id: u64,
  pub app_slug: String,
  pub target_id: u64,
  pub target_type: String,
  pub permissions: GithubPermissions,
  pub events: Vec<String>,
  pub created_at: String,
  pub updated_at: String,
  pub single_file_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubInstallationAccessToken {
  pub token: String,
  pub expires_at: String,
  pub permissions: GithubPermissions,
  pub repository_selection: String,
  pub repositories: Option<Vec<GithubRepository>>,
}


impl GithubInstallationAccessToken {
  pub fn set_token(&mut self, token: String, expires_at: String) {
      self.token = token;
      self.expires_at = expires_at;
  }
}

impl ExpirableToken for GithubInstallationAccessToken {
  fn is_expired(&self) -> bool {
      let expires_at = DateTime::parse_from_rfc3339(&self.expires_at).unwrap();

      expires_at < Utc::now()
  }

  fn get_token(&self) -> Option<String> {
      if self.is_expired() {
          None
      } else {
          Some(self.token.clone())
      }
  }
}
