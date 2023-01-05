use serde::{Deserialize, Serialize};
use crate::domains::repositories::GithubRepository;
use crate::domains::accounts::GithubAccount;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubIssuePullRequest {
    pub url: String,
    pub html_url: String,
    pub diff_url: String,
    pub patch_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubMilestone {
    pub url: String,
    pub html_url: String,
    pub labels_url: String,
    pub id: u32,
    pub node_id: String,
    pub number: u32,
    pub state: String,
    pub title: String,
    pub description: String,
    pub creator: GithubAccount,
    pub open_issues: u32,
    pub closed_issues: u32,
    pub created_at: String,
    pub updated_at: String,
    pub closed_at: String,
    pub due_on: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubLabel {
    pub id: u32,
    pub node_id: String,
    pub url: String,
    pub name: String,
    pub color: String,
    pub default: bool,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubIssue {
    pub id: u64,
    pub number: u64,
    pub title: String,
    pub body: Option<String>,
    pub state: String,
    pub created_at: String,
    pub updated_at: String,
    pub closed_at: Option<String>,
    pub author_association: String,
    pub repository_url: String,
    pub labels_url: String,
    pub comments_url: String,
    pub events_url: String,
    pub html_url: String,
    pub node_id: String,
    pub url: String,
    pub labels: Vec<GithubLabel>,
    pub assignees: Vec<GithubAccount>,
    pub assignee: Option<GithubAccount>,
    pub milestone: Option<GithubMilestone>,
    pub locked: bool,
    pub active_lock_reason: Option<String>,
    pub comments: u32,
    pub pull_request: Option<GithubIssuePullRequest>,
    pub closed_by: Option<GithubAccount>,
    pub repository: Option<GithubRepository>,
    pub user: Option<GithubAccount>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubIssueComment {
    pub id: u32,
    pub node_id: String,
    pub url: String,
    pub html_url: String,
    pub body: String,
    pub user: GithubAccount,
    pub created_at: String,
    pub updated_at: String,
    pub author_association: String,
    pub issue_url: String,
}