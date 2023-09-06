use serde::{Deserialize, Serialize};

use crate::domains::repositories::GithubRepository;
use crate::domains::users::GithubUser;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubCommitParent {
  pub sha: String,
  pub url: String,
  pub html_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubCommitAuthor {
  pub name: String,
  pub email: String,
  pub date: Option<String>,
  pub username: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubCommitVerification {
  pub verified: bool,
  pub reason: String,
  pub signature: Option<String>,
  pub payload: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubCommitContent {
  pub author: GithubCommitAuthor,
  pub committer: GithubCommitAuthor,
  pub message: String,
  pub tree: GithubCommitParent,
  pub url: String,
  pub comment_count: u64,
  pub verification: GithubCommitVerification,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubCommitFile {
  pub sha: String,
  pub filename: String,
  pub status: String,
  pub additions: u64,
  pub deletions: u64,
  pub changes: u64,
  pub blob_url: String,
  pub raw_url: String,
  pub contents_url: String,
  pub patch: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubCommit {
  pub sha: String,
  pub url: String,
  pub node_id: Option<String>,
  pub commit: Option<GithubCommitContent>,
  pub html_url: Option<String>,
  pub comments_url: Option<String>,
  pub author: Option<GithubUser>,
  pub committer: Option<GithubUser>,
  pub parents: Option<Vec<GithubCommitParent>>,
  pub files: Option<Vec<GithubCommitFile>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubCommitStatus {
  pub url: String,
  pub id: u64,
  pub node_id: String,
  pub state: String,
  pub description: Option<String>,
  pub target_url: Option<String>,
  pub context: String,
  pub creator: Option<GithubUser>,
  pub created_at: String,
  pub updated_at: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubCommitCombinedStatuses {
  pub url: String,
  pub total_count: u64,
  pub statuses: Vec<GithubCommitStatus>,
  pub repository: GithubRepository,
  pub sha: String,
  pub state: String,
}
