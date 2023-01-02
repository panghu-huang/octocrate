use serde::{Deserialize, Serialize};

use crate::domains::{
    accounts::GithubAccount,
    issues::{GithubLabel, GithubMilestone},
    repositories::GithubRepository,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubPullRequestRef {
    pub label: String,
    #[serde(rename = "ref")]
    pub ref_name: String,
    pub sha: String,
    pub user: GithubAccount,
    pub repo: GithubRepository,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubPullRequest {
    pub url: String,
    pub id: u64,
    pub node_id: String,
    pub html_url: String,
    pub diff_url: String,
    pub patch_url: String,
    pub issue_url: String,
    pub commits_url: String,
    pub review_comments_url: String,
    pub review_comment_url: String,
    pub comments_url: String,
    pub statuses_url: String,
    pub number: u64,
    pub state: String,
    pub locked: bool,
    pub title: String,
    pub user: GithubAccount,
    pub body: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub closed_at: Option<String>,
    pub merged_at: Option<String>,
    pub merge_commit_sha: Option<String>,
    pub assignee: Option<GithubAccount>,
    pub assignees: Vec<GithubAccount>,
    pub requested_reviewers: Vec<GithubAccount>,
    pub requested_teams: Vec<GithubAccount>,
    pub labels: Vec<GithubLabel>,
    pub milestone: Option<GithubMilestone>,
    pub draft: bool,
    pub commits: u64,
    pub additions: u64,
    pub deletions: u64,
    pub changed_files: u64,
    pub base: GithubPullRequestRef,
    pub head: GithubPullRequestRef,
    pub author_association: String,
    pub active_lock_reason: Option<String>,
    pub merged: bool,
    pub mergeable: Option<bool>,
    pub rebaseable: Option<bool>,
    pub mergeable_state: String,
    pub merged_by: Option<GithubAccount>,
    pub comments: u64,
    pub review_comments: u64,
    pub maintainer_can_modify: bool,
}
