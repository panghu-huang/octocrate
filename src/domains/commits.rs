use serde::{Deserialize, Serialize};

use crate::domains::accounts::GithubAccount;

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
    pub comment_count: u32,
    pub verification: GithubCommitVerification,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubCommit {
    pub sha: String,
    pub node_id: String,
    pub commit: GithubCommitContent,
    pub url: String,
    pub html_url: String,
    pub comments_url: String,
    pub author: Option<GithubAccount>,
    pub committer: Option<GithubAccount>,
    pub parents: Vec<GithubCommitParent>,
}
