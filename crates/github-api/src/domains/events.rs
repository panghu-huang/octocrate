use serde::{Deserialize, Serialize};

use crate::domains::{
    users::GithubUser,
    issues::{GithubIssue, GithubIssueComment},
    pulls::GithubPullRequest,
    repositories::GithubRepository,
    commits::GithubCommitAuthor
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubWebhookInstallation {
    pub id: u64,
    pub node_id: String,
}

// event name: issue_comment
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubWebhookIssueCommentEvent {
    /// created / edited / deleted
    pub action: String,
    pub issue: GithubIssue,
    pub comment: GithubIssueComment,
    pub repository: GithubRepository,
    pub sender: GithubUser,
    pub installation: GithubWebhookInstallation,
}

// event name: pull_request
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubWebhookPullRequestEvent {
    /// opened
    pub action: String,
    pub number: u64,
    pub pull_request: GithubPullRequest,
    pub repository: GithubRepository,
    pub sender: GithubUser,
    pub installation: GithubWebhookInstallation,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubWebhookPushEventCommit {
    pub id: String,
    pub tree_id: String,
    pub distinct: bool,
    pub message: String,
    pub timestamp: String,
    pub url: String,
    pub author: GithubCommitAuthor,
    pub committer: GithubCommitAuthor,
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub modified: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubPusher {
    pub name: String,
    /// If pusher is a bot, email will be null
    pub email: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubWebhookPushEvent {
    #[serde(rename = "ref")]
    pub ref_name: String,
    /// If first commit, before will be 0000000000000000000000000000000000000000
    pub before: String,
    pub after: String,
    pub created: bool,
    pub deleted: bool,
    pub forced: bool,
    pub base_ref: Option<String>,
    pub compare: String,
    pub commits: Vec<GithubWebhookPushEventCommit>,
    pub head_commit: Option<GithubWebhookPushEventCommit>,
    pub repository: GithubRepository,
    pub pusher: GithubPusher,
    pub sender: GithubUser,
    pub installation: GithubWebhookInstallation,
}

#[derive(Debug, Clone)]
pub enum GithubWebhookEvent {
    IssueComment(GithubWebhookIssueCommentEvent),
    PullRequest(GithubWebhookPullRequestEvent),
    Push(GithubWebhookPushEvent),
    Unsupported(String),
}
