use serde::{Deserialize, Serialize};

use crate::domains::{
    users::GithubUser,
    issues::{GithubLabel, GithubMilestone},
    repositories::GithubRepository,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubPullRequestRef {
    pub label: String,
    #[serde(rename = "ref")]
    pub ref_name: String,
    pub sha: String,
    pub user: GithubUser,
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
    pub user: GithubUser,
    pub body: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub closed_at: Option<String>,
    pub merged_at: Option<String>,
    pub merge_commit_sha: Option<String>,
    pub assignee: Option<GithubUser>,
    pub assignees: Vec<GithubUser>,
    pub requested_reviewers: Vec<GithubUser>,
    pub requested_teams: Vec<GithubUser>,
    pub labels: Vec<GithubLabel>,
    pub milestone: Option<GithubMilestone>,
    pub draft: bool,
    /// NOTE: Only present in pull request details
    pub commits: Option<u64>,
    /// NOTE: Only present in pull request details
    pub additions: Option<u64>,
    /// NOTE: Only present in pull request details
    pub deletions: Option<u64>,
    /// NOTE: Only present in pull request details
    pub changed_files: Option<u64>,
    pub base: GithubPullRequestRef,
    pub head: GithubPullRequestRef,
    pub author_association: String,
    pub active_lock_reason: Option<String>,
    pub merged: Option<bool>,
    pub mergeable: Option<bool>,
    pub rebaseable: Option<bool>,
    pub mergeable_state: Option<String>,
    pub merged_by: Option<GithubUser>,
    pub comments: Option<u64>,
    pub review_comments: Option<u64>,
    pub maintainer_can_modify: Option<bool>,
}
