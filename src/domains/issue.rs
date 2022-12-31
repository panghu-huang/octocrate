use serde::{Deserialize, Serialize};
use crate::domains::repository::GithubRepository;
use crate::domains::account::GithubAccount;

#[derive(Serialize, Deserialize, Debug)]
pub struct GithubPullRequest {
    pub url: String,
    pub html_url: String,
    pub diff_url: String,
    pub patch_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct GithubLabel {
    pub id: u32,
    pub node_id: String,
    pub url: String,
    pub name: String,
    pub color: String,
    pub default: bool,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GithubIssue {
    pub id: String,
    pub number: String,
    pub title: String,
    pub body: String,
    pub state: String,
    pub created_at: String,
    pub updated_at: String,
    pub closed_at: String,
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
    pub pull_request: Option<GithubPullRequest>,
    pub closed_by: Option<GithubAccount>,
    pub repository: GithubRepository,
    pub user: GithubAccount,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GithubComment {
    pub id: u32,
    pub node_id: String,
    pub url: String,
    pub html_url: String,
    pub body: String,
    pub user: GithubAccount,
    pub created_at: String,
    pub updated_at: String,
}