use crate::domains::users::GithubUser;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubDeployment {
    pub url: String,
    pub id: u64,
    pub node_id: String,
    pub sha: String,
    #[serde(rename = "ref")]
    pub ref_name: String,
    pub task: String,
    // pub payload: Option<String>,
    pub environment: String,
    pub description: Option<String>,
    pub creator: Option<GithubUser>,
    pub created_at: String,
    pub updated_at: String,
    pub statuses_url: String,
    pub repository_url: String,
}

/// Can be one of: error, failure, inactive, in_progress, queued, pending, success
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum GithubDeploymentState {
    Error,
    Failure,
    Inactive,
    InProgress,
    Queued,
    Pending,
    Success,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubDeploymentStatus {
    pub url: String,
    pub id: u64,
    pub node_id: String,
    pub state: GithubDeploymentState,
    pub creator: Option<GithubUser>,
    pub description: Option<String>,
    pub target_url: Option<String>,
    pub log_url: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub deployment_url: String,
    pub repository_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubDeploymentEnvironment {
    pub url: String,
    pub id: u64,
    pub node_id: String,
    pub name: String,
}
