use infrastructure::{GithubError, GithubResult};
use serde::{Deserialize, Serialize};

use crate::domains::{
    commits::GithubCommitAuthor,
    issues::{GithubIssue, GithubIssueComment},
    pulls::GithubPullRequest,
    repositories::GithubRepository,
    users::GithubUser,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubWebhookInstallation {
    pub id: u64,
    pub node_id: String,
}

// event name: issue_comment
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubWebhookIssueCommentEvent {
    /// created / edited / deleted
    pub action: String,
    pub issue: GithubIssue,
    pub comment: GithubIssueComment,
    pub repository: GithubRepository,
    pub sender: GithubUser,
    pub installation: GithubWebhookInstallation,
}

// event name: pull_request
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubWebhookPullRequestEvent {
    /// opened
    pub action: String,
    pub number: u64,
    pub pull_request: GithubPullRequest,
    pub repository: GithubRepository,
    pub sender: GithubUser,
    pub installation: GithubWebhookInstallation,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubWebhookPushEventCommit {
    pub id: String,
    pub tree_id: String,
    pub distinct: bool,
    pub message: String,
    pub timestamp: String,
    pub url: String,
    pub author: GithubCommitAuthor,
    pub committer: GithubCommitAuthor,
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub modified: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubPusher {
    pub name: String,
    /// If pusher is a bot, email will be null
    pub email: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubWebhookPushEvent {
    #[serde(rename = "ref")]
    pub ref_name: String,
    /// If first commit, before will be 0000000000000000000000000000000000000000
    pub before: String,
    pub after: String,
    pub created: bool,
    pub deleted: bool,
    pub forced: bool,
    pub base_ref: Option<String>,
    pub compare: String,
    pub commits: Vec<GithubWebhookPushEventCommit>,
    pub head_commit: Option<GithubWebhookPushEventCommit>,
    pub repository: GithubRepository,
    pub pusher: GithubPusher,
    pub sender: GithubUser,
    pub installation: GithubWebhookInstallation,
}

#[derive(Debug, Clone)]
pub enum GithubWebhookEvent {
    IssueComment(GithubWebhookIssueCommentEvent),
    PullRequest(GithubWebhookPullRequestEvent),
    Push(GithubWebhookPushEvent),
    Unsupported {
        payload: String,
        installation: GithubWebhookInstallation,
    },
}

impl GithubWebhookEvent {
    pub fn installation(&self) -> GithubWebhookInstallation {
        match self {
            GithubWebhookEvent::IssueComment(evt) => evt.installation.clone(),
            GithubWebhookEvent::PullRequest(evt) => evt.installation.clone(),
            GithubWebhookEvent::Push(evt) => evt.installation.clone(),
            GithubWebhookEvent::Unsupported {
                payload: _,
                installation,
            } => installation.clone(),
        }
    }

    pub fn try_parse(event_name: String, payload: String) -> GithubResult<Self> {
        match event_name.as_str() {
            "issue_comment" => {
                let evt = serde_json::from_str::<GithubWebhookIssueCommentEvent>(&payload)?;

                return Ok(GithubWebhookEvent::IssueComment(evt));
            }
            "pull_request" => {
                let evt = serde_json::from_str::<GithubWebhookPullRequestEvent>(&payload)?;

                return Ok(GithubWebhookEvent::PullRequest(evt));
            }
            "push" => {
                let evt = serde_json::from_str::<GithubWebhookPushEvent>(&payload)?;

                return Ok(GithubWebhookEvent::Push(evt));
            }
            _ => {
                let event = serde_json::from_str::<serde_json::Value>(&payload)?;
                let installation = event
                    .get("installation")
                    .ok_or(GithubError::new("No installation field on webhook event"))?;

                let installation =
                    serde_json::from_value::<GithubWebhookInstallation>(installation.clone())?;
                return Ok(GithubWebhookEvent::Unsupported {
                    payload,
                    installation,
                });
            }
        }
    }
}
