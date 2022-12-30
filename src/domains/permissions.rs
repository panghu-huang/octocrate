use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GithubPermissions {
  metadata: String,
  contents: String,
  issues: String,
  single_file: Option<String>,
}