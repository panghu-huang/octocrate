use crate::domains::repositories::GithubRepository;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubUser {
  pub id: u64,
  pub login: String,
  pub avatar_url: String,
  pub gravatar_id: String,
  pub url: String,
  pub html_url: String,
  pub followers_url: String,
  pub following_url: String,
  pub gists_url: String,
  pub starred_url: String,
  pub subscriptions_url: String,
  pub organizations_url: String,
  pub repos_url: String,
  pub events_url: String,
  pub received_events_url: String,
  pub site_admin: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubUserInstallation {
  pub id: u64,
  pub account: GithubUser,
  pub access_tokens_url: String,
  pub repositories_url: String,
  pub html_url: String,
  pub app_id: u64,
  pub target_id: u64,
  pub target_type: String,
  pub permissions: HashMap<String, String>,
  pub events: Vec<String>,
  pub single_file_name: Option<String>,
  pub repository_selection: String,
  pub has_multiple_single_files: Option<bool>,
  pub single_file_paths: Option<Vec<String>>,
  pub created_at: String,
  pub updated_at: String,
  pub suspended_at: Option<String>,
  pub suspender_by: Option<GithubUser>,
  pub app_slug: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubUserInstallations {
  pub total_count: u64,
  pub installations: Vec<GithubUserInstallation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubUserInstallationRepositories {
  pub total_count: u64,
  pub repositories: Vec<GithubRepository>,
}
