use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubActionSecret {
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubActionSecrets {
    pub total_count: u64,
    pub secrets: Vec<GithubActionSecret>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubActionSecretPublicKey {
    pub key_id: String,
    pub key: String,
}