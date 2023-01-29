use serde::{Deserialize, Serialize};
use crate::domains::repositories::GithubRepository;
use crate::domains::users::GithubUser;

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
    pub id: u64,
    pub node_id: String,
    pub number: u64,
    pub state: String,
    pub title: String,
    pub description: String,
    pub creator: GithubUser,
    pub open_issues: u64,
    pub closed_issues: u64,
    pub created_at: String,
    pub updated_at: String,
    pub closed_at: String,
    pub due_on: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubLabel {
    pub id: u64,
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
    pub assignees: Vec<GithubUser>,
    pub assignee: Option<GithubUser>,
    pub milestone: Option<GithubMilestone>,
    pub locked: bool,
    pub active_lock_reason: Option<String>,
    pub comments: u64,
    pub pull_request: Option<GithubIssuePullRequest>,
    pub closed_by: Option<GithubUser>,
    pub repository: Option<GithubRepository>,
    pub user: Option<GithubUser>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubIssueComment {
    pub id: u64,
    pub node_id: String,
    pub url: String,
    pub html_url: String,
    pub body: String,
    pub user: GithubUser,
    pub created_at: String,
    pub updated_at: String,
    pub author_association: String,
    pub issue_url: String,
}