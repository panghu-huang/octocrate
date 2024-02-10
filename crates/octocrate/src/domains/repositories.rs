use crate::domains::{
  commits::GithubCommitAuthor, permissions::GithubRepositoryPermissions, users::GithubUser,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubRepository {
  pub id: u64,
  pub node_id: String,
  pub name: String,
  pub full_name: String,
  pub owner: GithubUser,
  pub private: bool,
  pub html_url: String,
  pub description: Option<String>,
  pub fork: bool,
  pub url: String,
  pub archive_url: String,
  pub assignees_url: String,
  pub blobs_url: String,
  pub branches_url: String,
  pub collaborators_url: String,
  pub comments_url: String,
  pub commits_url: String,
  pub compare_url: String,
  pub contents_url: String,
  pub contributors_url: String,
  pub deployments_url: String,
  pub downloads_url: String,
  pub events_url: String,
  pub forks_url: String,
  pub git_commits_url: String,
  pub git_refs_url: String,
  pub git_tags_url: String,
  pub git_url: Option<String>,
  pub issue_comment_url: String,
  pub issue_events_url: String,
  pub issues_url: String,
  pub keys_url: String,
  pub labels_url: String,
  pub languages_url: String,
  pub merges_url: String,
  pub milestones_url: String,
  pub notifications_url: String,
  pub pulls_url: String,
  pub releases_url: String,
  pub ssh_url: Option<String>,
  pub stargazers_url: String,
  pub statuses_url: String,
  pub subscribers_url: String,
  pub subscription_url: String,
  pub tags_url: String,
  pub teams_url: String,
  pub trees_url: String,
  pub clone_url: Option<String>,
  pub mirror_url: Option<String>,
  pub hooks_url: String,
  pub svn_url: Option<String>,
  pub homepage: Option<String>,
  pub language: Option<String>,
  pub forks_count: Option<u64>,
  pub stargazers_count: Option<u64>,
  pub watchers_count: Option<u64>,
  pub size: Option<u64>,
  pub default_branch: Option<String>,
  pub open_issues_count: Option<u64>,
  pub is_template: Option<bool>,
  pub topics: Option<Vec<String>>,
  pub has_issues: Option<bool>,
  pub has_projects: Option<bool>,
  pub has_wiki: Option<bool>,
  pub has_pages: Option<bool>,
  pub has_downloads: Option<bool>,
  pub archived: Option<bool>,
  pub disabled: Option<bool>,
  pub visibility: Option<String>,
  // on push event, pushed_at's data type is integer
  // but on repository event, pushed_at's data type is string
  // pub pushed_at: String,
  // on push event, created_at's data type is integer
  // but on repository event, created_at's data type is string
  // pub created_at: String,
  pub updated_at: Option<String>,
  pub permissions: Option<GithubRepositoryPermissions>,
  pub temp_clone_token: Option<String>,
  pub allow_squash_merge: Option<bool>,
  pub allow_merge_commit: Option<bool>,
  pub allow_rebase_merge: Option<bool>,
  pub delete_branch_on_merge: Option<bool>,
  pub allow_auto_merge: Option<bool>,
}

// Repository contents
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubRepositoryContentsLinks {
  pub git: String,
  #[serde(rename = "self")]
  pub self_link: String,
  pub html: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum GithubRepositoryContentType {
  File,
  Dir,
  Submodule,
  Symlink,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubRepositoryContent {
  pub name: String,
  pub path: String,
  pub sha: String,
  pub size: u64,
  pub url: String,
  pub html_url: String,
  pub git_url: String,
  pub download_url: Option<String>,
  #[serde(rename = "type")]
  pub content_type: GithubRepositoryContentType,
  pub content: Option<String>,
  pub encoding: Option<String>,
  pub _links: GithubRepositoryContentsLinks,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum GithubRepositoryContentResponse {
  File(GithubRepositoryContent),
  Directory(Vec<GithubRepositoryContent>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubFileCommitCommit {
  pub sha: String,
  pub node_id: String,
  pub url: String,
  pub html_url: String,
  pub author: GithubCommitAuthor,
  pub committer: GithubCommitAuthor,
  pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubFileCommit {
  pub content: GithubRepositoryContent,
  pub commit: GithubFileCommitCommit,
}
