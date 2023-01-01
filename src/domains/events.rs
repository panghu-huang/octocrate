use serde::{Deserialize, Serialize};

use crate::domains::{
    accounts::GithubAccount,
    issues::{GithubIssue, GithubIssueComment},
    repositories::GithubRepository,
    pulls::GithubPullRequest
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Installation {
    pub id: i64,
    pub node_id: String,
}

// event name: issue_comment
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubWebhookIssueCommentEvent {
    /// created
    pub action: String,
    pub issue: GithubIssue,
    pub comment: GithubIssueComment,
    pub repository: GithubRepository,
    pub sender: GithubAccount,
    pub installation: Installation,
}

// event name: pull_request
#[derive(Serialize, Deserialize, Debug)]
pub struct GithubWebhookPullRequestOpenedEvent {
    /// opened
    pub action: String,
    pub number: i64,
    pub pull_request: GithubPullRequest,
    pub repository: GithubRepository,
    pub sender: GithubAccount,
    pub installation: Installation,
}