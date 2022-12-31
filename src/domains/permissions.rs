use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubPermissions {
  metadata: String,
  contents: String,
  issues: String,
  single_file: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubRepositoryPermissions {
  pub admin: bool,
  pub push: bool,
  pub pull: bool,
  pub triage: Option<bool>,
}