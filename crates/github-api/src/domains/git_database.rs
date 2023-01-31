use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubReferenceObject {
    pub sha: String,
    #[serde(rename = "type")]
    pub type_name: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubReference {
    #[serde(rename = "ref")]
    pub ref_name: String,
    pub node_id: String,
    pub url: String,
    pub object: GithubReferenceObject,
}