use serde::{Deserialize, Serialize};

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
    pub comment_count: u32,
    pub verification: GithubCommitVerification,
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
}
