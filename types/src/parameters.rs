#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use typed_builder::TypedBuilder;

#[cfg(any(
  feature = "full",
  feature = "security_advisories",
  feature = "dependabot",
  feature = "secret_scanning",
  feature = "issues",
  feature = "code_scanning",
  feature = "orgs",
  feature = "teams",
  feature = "actions",
  feature = "repos",
  feature = "activity"
))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum Direction {
  #[serde(rename = "asc")]
  Asc,
  #[serde(rename = "desc")]
  Desc,
}

#[cfg(any(
  feature = "full",
  feature = "security_advisories",
  feature = "dependabot",
  feature = "secret_scanning",
  feature = "issues",
  feature = "code_scanning",
  feature = "orgs",
  feature = "teams",
  feature = "actions",
  feature = "repos",
  feature = "activity"
))]
impl ToString for Direction {
  fn to_string(&self) -> String {
    match self {
      Direction::Asc => "asc".to_string(),
      Direction::Desc => "desc".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "dependabot"))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum DependabotAlertScope {
  #[serde(rename = "development")]
  Development,
  #[serde(rename = "runtime")]
  Runtime,
}

#[cfg(any(feature = "full", feature = "dependabot"))]
impl ToString for DependabotAlertScope {
  fn to_string(&self) -> String {
    match self {
      DependabotAlertScope::Development => "development".to_string(),
      DependabotAlertScope::Runtime => "runtime".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "dependabot"))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum DependabotAlertSort {
  #[serde(rename = "created")]
  Created,
  #[serde(rename = "updated")]
  Updated,
}

#[cfg(any(feature = "full", feature = "dependabot"))]
impl ToString for DependabotAlertSort {
  fn to_string(&self) -> String {
    match self {
      DependabotAlertSort::Created => "created".to_string(),
      DependabotAlertSort::Updated => "updated".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "secret_scanning"))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum SecretScanningAlertState {
  #[serde(rename = "open")]
  Open,
  #[serde(rename = "resolved")]
  Resolved,
}

#[cfg(any(feature = "full", feature = "secret_scanning"))]
impl ToString for SecretScanningAlertState {
  fn to_string(&self) -> String {
    match self {
      SecretScanningAlertState::Open => "open".to_string(),
      SecretScanningAlertState::Resolved => "resolved".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "secret_scanning"))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum SecretScanningAlertSort {
  #[serde(rename = "created")]
  Created,
  #[serde(rename = "updated")]
  Updated,
}

#[cfg(any(feature = "full", feature = "secret_scanning"))]
impl ToString for SecretScanningAlertSort {
  fn to_string(&self) -> String {
    match self {
      SecretScanningAlertSort::Created => "created".to_string(),
      SecretScanningAlertSort::Updated => "updated".to_string(),
    }
  }
}

#[cfg(any(
  feature = "full",
  feature = "apps",
  feature = "issues",
  feature = "pulls"
))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum Sort {
  #[serde(rename = "created")]
  Created,
  #[serde(rename = "updated")]
  Updated,
}

#[cfg(any(
  feature = "full",
  feature = "apps",
  feature = "issues",
  feature = "pulls"
))]
impl ToString for Sort {
  fn to_string(&self) -> String {
    match self {
      Sort::Created => "created".to_string(),
      Sort::Updated => "updated".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "packages"))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum PackageVisibility {
  #[serde(rename = "public")]
  Public,
  #[serde(rename = "private")]
  Private,
  #[serde(rename = "internal")]
  Internal,
}

#[cfg(any(feature = "full", feature = "packages"))]
impl ToString for PackageVisibility {
  fn to_string(&self) -> String {
    match self {
      PackageVisibility::Public => "public".to_string(),
      PackageVisibility::Private => "private".to_string(),
      PackageVisibility::Internal => "internal".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "orgs"))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum PersonalAccessTokenSort {
  #[serde(rename = "created_at")]
  CreatedAt,
}

#[cfg(any(feature = "full", feature = "orgs"))]
impl ToString for PersonalAccessTokenSort {
  fn to_string(&self) -> String {
    match self {
      PersonalAccessTokenSort::CreatedAt => "created_at".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum TimePeriod {
  #[serde(rename = "hour")]
  Hour,
  #[serde(rename = "day")]
  Day,
  #[serde(rename = "week")]
  Week,
  #[serde(rename = "month")]
  Month,
}

#[cfg(any(feature = "full", feature = "repos"))]
impl ToString for TimePeriod {
  fn to_string(&self) -> String {
    match self {
      TimePeriod::Hour => "hour".to_string(),
      TimePeriod::Day => "day".to_string(),
      TimePeriod::Week => "week".to_string(),
      TimePeriod::Month => "month".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum RuleSuiteResult {
  #[serde(rename = "pass")]
  Pass,
  #[serde(rename = "fail")]
  Fail,
  #[serde(rename = "bypass")]
  Bypass,
  #[serde(rename = "all")]
  All,
}

#[cfg(any(feature = "full", feature = "repos"))]
impl ToString for RuleSuiteResult {
  fn to_string(&self) -> String {
    match self {
      RuleSuiteResult::Pass => "pass".to_string(),
      RuleSuiteResult::Fail => "fail".to_string(),
      RuleSuiteResult::Bypass => "bypass".to_string(),
      RuleSuiteResult::All => "all".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "orgs"))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum SecurityProduct {
  #[serde(rename = "dependency_graph")]
  DependencyGraph,
  #[serde(rename = "dependabot_alerts")]
  DependabotAlerts,
  #[serde(rename = "dependabot_security_updates")]
  DependabotSecurityUpdates,
  #[serde(rename = "advanced_security")]
  AdvancedSecurity,
  #[serde(rename = "code_scanning_default_setup")]
  CodeScanningDefaultSetup,
  #[serde(rename = "secret_scanning")]
  SecretScanning,
  #[serde(rename = "secret_scanning_push_protection")]
  SecretScanningPushProtection,
}

#[cfg(any(feature = "full", feature = "orgs"))]
impl ToString for SecurityProduct {
  fn to_string(&self) -> String {
    match self {
      SecurityProduct::DependencyGraph => "dependency_graph".to_string(),
      SecurityProduct::DependabotAlerts => "dependabot_alerts".to_string(),
      SecurityProduct::DependabotSecurityUpdates => "dependabot_security_updates".to_string(),
      SecurityProduct::AdvancedSecurity => "advanced_security".to_string(),
      SecurityProduct::CodeScanningDefaultSetup => "code_scanning_default_setup".to_string(),
      SecurityProduct::SecretScanning => "secret_scanning".to_string(),
      SecurityProduct::SecretScanningPushProtection => {
        "secret_scanning_push_protection".to_string()
      }
    }
  }
}

#[cfg(any(feature = "full", feature = "orgs"))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum OrgSecurityProductEnablement {
  #[serde(rename = "enable_all")]
  EnableAll,
  #[serde(rename = "disable_all")]
  DisableAll,
}

#[cfg(any(feature = "full", feature = "orgs"))]
impl ToString for OrgSecurityProductEnablement {
  fn to_string(&self) -> String {
    match self {
      OrgSecurityProductEnablement::EnableAll => "enable_all".to_string(),
      OrgSecurityProductEnablement::DisableAll => "disable_all".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "actions"))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum ActionsCacheListSort {
  #[serde(rename = "created_at")]
  CreatedAt,
  #[serde(rename = "last_accessed_at")]
  LastAccessedAt,
  #[serde(rename = "size_in_bytes")]
  SizeInBytes,
}

#[cfg(any(feature = "full", feature = "actions"))]
impl ToString for ActionsCacheListSort {
  fn to_string(&self) -> String {
    match self {
      ActionsCacheListSort::CreatedAt => "created_at".to_string(),
      ActionsCacheListSort::LastAccessedAt => "last_accessed_at".to_string(),
      ActionsCacheListSort::SizeInBytes => "size_in_bytes".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "actions"))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum WorkflowRunStatus {
  #[serde(rename = "completed")]
  Completed,
  #[serde(rename = "action_required")]
  ActionRequired,
  #[serde(rename = "cancelled")]
  Cancelled,
  #[serde(rename = "failure")]
  Failure,
  #[serde(rename = "neutral")]
  Neutral,
  #[serde(rename = "skipped")]
  Skipped,
  #[serde(rename = "stale")]
  Stale,
  #[serde(rename = "success")]
  Success,
  #[serde(rename = "timed_out")]
  TimedOut,
  #[serde(rename = "in_progress")]
  InProgress,
  #[serde(rename = "queued")]
  Queued,
  #[serde(rename = "requested")]
  Requested,
  #[serde(rename = "waiting")]
  Waiting,
  #[serde(rename = "pending")]
  Pending,
}

#[cfg(any(feature = "full", feature = "actions"))]
impl ToString for WorkflowRunStatus {
  fn to_string(&self) -> String {
    match self {
      WorkflowRunStatus::Completed => "completed".to_string(),
      WorkflowRunStatus::ActionRequired => "action_required".to_string(),
      WorkflowRunStatus::Cancelled => "cancelled".to_string(),
      WorkflowRunStatus::Failure => "failure".to_string(),
      WorkflowRunStatus::Neutral => "neutral".to_string(),
      WorkflowRunStatus::Skipped => "skipped".to_string(),
      WorkflowRunStatus::Stale => "stale".to_string(),
      WorkflowRunStatus::Success => "success".to_string(),
      WorkflowRunStatus::TimedOut => "timed_out".to_string(),
      WorkflowRunStatus::InProgress => "in_progress".to_string(),
      WorkflowRunStatus::Queued => "queued".to_string(),
      WorkflowRunStatus::Requested => "requested".to_string(),
      WorkflowRunStatus::Waiting => "waiting".to_string(),
      WorkflowRunStatus::Pending => "pending".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "checks"))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum Status {
  #[serde(rename = "queued")]
  Queued,
  #[serde(rename = "in_progress")]
  InProgress,
  #[serde(rename = "completed")]
  Completed,
}

#[cfg(any(feature = "full", feature = "checks"))]
impl ToString for Status {
  fn to_string(&self) -> String {
    match self {
      Status::Queued => "queued".to_string(),
      Status::InProgress => "in_progress".to_string(),
      Status::Completed => "completed".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum Per {
  #[serde(rename = "day")]
  Day,
  #[serde(rename = "week")]
  Week,
}

#[cfg(any(feature = "full", feature = "repos"))]
impl ToString for Per {
  fn to_string(&self) -> String {
    match self {
      Per::Day => "day".to_string(),
      Per::Week => "week".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "search"))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum Order {
  #[serde(rename = "desc")]
  Desc,
  #[serde(rename = "asc")]
  Asc,
}

#[cfg(any(feature = "full", feature = "search"))]
impl ToString for Order {
  fn to_string(&self) -> String {
    match self {
      Order::Desc => "desc".to_string(),
      Order::Asc => "asc".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "activity"))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum SortStarred {
  #[serde(rename = "created")]
  Created,
  #[serde(rename = "updated")]
  Updated,
}

#[cfg(any(feature = "full", feature = "activity"))]
impl ToString for SortStarred {
  fn to_string(&self) -> String {
    match self {
      SortStarred::Created => "created".to_string(),
      SortStarred::Updated => "updated".to_string(),
    }
  }
}
