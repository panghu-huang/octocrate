use serde::{Deserialize, Serialize};
use crate::domains::users::GithubUser;

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
