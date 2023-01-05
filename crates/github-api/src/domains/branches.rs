use crate::domains::accounts::GithubAccount;
use crate::domains::commits::GithubCommitContent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GithubBranchCommit {
    pub sha: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GithubBranchProtectionRequiredStatusChecks {
    pub enforcement_level: String,
    pub contexts: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GithubBranchProtection {
    pub enabled: Option<bool>,
    pub required_status_checks: Option<GithubBranchProtectionRequiredStatusChecks>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GithubBranch {
    pub name: String,
    pub commit: GithubBranchCommit,
    pub protected: bool,
    pub protection: Option<GithubBranchProtection>,
}
