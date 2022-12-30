use crate::domains::account::GithubAccount;
use crate::domains::permissions::GithubPermissions;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GithubInstallation {
  id: String,
  account: GithubAccount,
  access_tokens_url: String,
  repositories_url: String,
  html_url: String,
  app_id: String,
  target_id: String,
  target_type: String,
  permissions: GithubPermissions,
  events: Vec<String>,
  created_at: String,
  updated_at: String,
  single_file_name: Option<String>,
}