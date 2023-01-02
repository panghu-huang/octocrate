use serde::{Deserialize, Serialize};

use crate::domains::{
    accounts::GithubAccount,
    issues::{GithubIssue, GithubIssueComment},
    repositories::GithubRepository,
    pulls::GithubPullRequest
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
    pub sender: GithubAccount,
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
    pub sender: GithubAccount,
    pub installation: GithubWebhookInstallation,
}

#[derive(Debug, Clone)]
pub enum GithubWebhookEvent {
    IssueComment(GithubWebhookIssueCommentEvent),
}