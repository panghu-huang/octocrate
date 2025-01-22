#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use typed_builder::TypedBuilder;

#[allow(clippy::large_enum_variant)]
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
impl std::fmt::Display for Direction {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      Direction::Asc => write!(f, "asc"),
      Direction::Desc => write!(f, "desc"),
    }
  }
}

#[allow(clippy::large_enum_variant)]
#[cfg(any(feature = "full", feature = "dependabot"))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum DependabotAlertScope {
  #[serde(rename = "development")]
  Development,
  #[serde(rename = "runtime")]
  Runtime,
}

#[cfg(any(feature = "full", feature = "dependabot"))]
impl std::fmt::Display for DependabotAlertScope {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      DependabotAlertScope::Development => write!(f, "development"),
      DependabotAlertScope::Runtime => write!(f, "runtime"),
    }
  }
}

#[allow(clippy::large_enum_variant)]
#[cfg(any(feature = "full", feature = "dependabot"))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum DependabotAlertSort {
  #[serde(rename = "created")]
  Created,
  #[serde(rename = "updated")]
  Updated,
}

#[cfg(any(feature = "full", feature = "dependabot"))]
impl std::fmt::Display for DependabotAlertSort {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      DependabotAlertSort::Created => write!(f, "created"),
      DependabotAlertSort::Updated => write!(f, "updated"),
    }
  }
}

#[allow(clippy::large_enum_variant)]
#[cfg(any(feature = "full", feature = "secret_scanning"))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum SecretScanningAlertState {
  #[serde(rename = "open")]
  Open,
  #[serde(rename = "resolved")]
  Resolved,
}

#[cfg(any(feature = "full", feature = "secret_scanning"))]
impl std::fmt::Display for SecretScanningAlertState {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      SecretScanningAlertState::Open => write!(f, "open"),
      SecretScanningAlertState::Resolved => write!(f, "resolved"),
    }
  }
}

#[allow(clippy::large_enum_variant)]
#[cfg(any(feature = "full", feature = "secret_scanning"))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum SecretScanningAlertSort {
  #[serde(rename = "created")]
  Created,
  #[serde(rename = "updated")]
  Updated,
}

#[cfg(any(feature = "full", feature = "secret_scanning"))]
impl std::fmt::Display for SecretScanningAlertSort {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      SecretScanningAlertSort::Created => write!(f, "created"),
      SecretScanningAlertSort::Updated => write!(f, "updated"),
    }
  }
}

#[allow(clippy::large_enum_variant)]
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
impl std::fmt::Display for Sort {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      Sort::Created => write!(f, "created"),
      Sort::Updated => write!(f, "updated"),
    }
  }
}

#[allow(clippy::large_enum_variant)]
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
impl std::fmt::Display for PackageVisibility {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      PackageVisibility::Public => write!(f, "public"),
      PackageVisibility::Private => write!(f, "private"),
      PackageVisibility::Internal => write!(f, "internal"),
    }
  }
}

#[allow(clippy::large_enum_variant)]
#[cfg(any(feature = "full", feature = "orgs"))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum PersonalAccessTokenSort {
  #[serde(rename = "created_at")]
  CreatedAt,
}

#[cfg(any(feature = "full", feature = "orgs"))]
impl std::fmt::Display for PersonalAccessTokenSort {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      PersonalAccessTokenSort::CreatedAt => write!(f, "created_at"),
    }
  }
}

#[allow(clippy::large_enum_variant)]
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
impl std::fmt::Display for TimePeriod {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      TimePeriod::Hour => write!(f, "hour"),
      TimePeriod::Day => write!(f, "day"),
      TimePeriod::Week => write!(f, "week"),
      TimePeriod::Month => write!(f, "month"),
    }
  }
}

#[allow(clippy::large_enum_variant)]
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
impl std::fmt::Display for RuleSuiteResult {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      RuleSuiteResult::Pass => write!(f, "pass"),
      RuleSuiteResult::Fail => write!(f, "fail"),
      RuleSuiteResult::Bypass => write!(f, "bypass"),
      RuleSuiteResult::All => write!(f, "all"),
    }
  }
}

#[allow(clippy::large_enum_variant)]
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
impl std::fmt::Display for SecurityProduct {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      SecurityProduct::DependencyGraph => write!(f, "dependency_graph"),
      SecurityProduct::DependabotAlerts => write!(f, "dependabot_alerts"),
      SecurityProduct::DependabotSecurityUpdates => write!(f, "dependabot_security_updates"),
      SecurityProduct::AdvancedSecurity => write!(f, "advanced_security"),
      SecurityProduct::CodeScanningDefaultSetup => write!(f, "code_scanning_default_setup"),
      SecurityProduct::SecretScanning => write!(f, "secret_scanning"),
      SecurityProduct::SecretScanningPushProtection => write!(f, "secret_scanning_push_protection"),
    }
  }
}

#[allow(clippy::large_enum_variant)]
#[cfg(any(feature = "full", feature = "orgs"))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum OrgSecurityProductEnablement {
  #[serde(rename = "enable_all")]
  EnableAll,
  #[serde(rename = "disable_all")]
  DisableAll,
}

#[cfg(any(feature = "full", feature = "orgs"))]
impl std::fmt::Display for OrgSecurityProductEnablement {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      OrgSecurityProductEnablement::EnableAll => write!(f, "enable_all"),
      OrgSecurityProductEnablement::DisableAll => write!(f, "disable_all"),
    }
  }
}

#[allow(clippy::large_enum_variant)]
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
impl std::fmt::Display for ActionsCacheListSort {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      ActionsCacheListSort::CreatedAt => write!(f, "created_at"),
      ActionsCacheListSort::LastAccessedAt => write!(f, "last_accessed_at"),
      ActionsCacheListSort::SizeInBytes => write!(f, "size_in_bytes"),
    }
  }
}

#[allow(clippy::large_enum_variant)]
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
impl std::fmt::Display for WorkflowRunStatus {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      WorkflowRunStatus::Completed => write!(f, "completed"),
      WorkflowRunStatus::ActionRequired => write!(f, "action_required"),
      WorkflowRunStatus::Cancelled => write!(f, "cancelled"),
      WorkflowRunStatus::Failure => write!(f, "failure"),
      WorkflowRunStatus::Neutral => write!(f, "neutral"),
      WorkflowRunStatus::Skipped => write!(f, "skipped"),
      WorkflowRunStatus::Stale => write!(f, "stale"),
      WorkflowRunStatus::Success => write!(f, "success"),
      WorkflowRunStatus::TimedOut => write!(f, "timed_out"),
      WorkflowRunStatus::InProgress => write!(f, "in_progress"),
      WorkflowRunStatus::Queued => write!(f, "queued"),
      WorkflowRunStatus::Requested => write!(f, "requested"),
      WorkflowRunStatus::Waiting => write!(f, "waiting"),
      WorkflowRunStatus::Pending => write!(f, "pending"),
    }
  }
}

#[allow(clippy::large_enum_variant)]
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
impl std::fmt::Display for Status {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      Status::Queued => write!(f, "queued"),
      Status::InProgress => write!(f, "in_progress"),
      Status::Completed => write!(f, "completed"),
    }
  }
}

#[allow(clippy::large_enum_variant)]
#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum Per {
  #[serde(rename = "day")]
  Day,
  #[serde(rename = "week")]
  Week,
}

#[cfg(any(feature = "full", feature = "repos"))]
impl std::fmt::Display for Per {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      Per::Day => write!(f, "day"),
      Per::Week => write!(f, "week"),
    }
  }
}

#[allow(clippy::large_enum_variant)]
#[cfg(any(feature = "full", feature = "search"))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum Order {
  #[serde(rename = "desc")]
  Desc,
  #[serde(rename = "asc")]
  Asc,
}

#[cfg(any(feature = "full", feature = "search"))]
impl std::fmt::Display for Order {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      Order::Desc => write!(f, "desc"),
      Order::Asc => write!(f, "asc"),
    }
  }
}

#[allow(clippy::large_enum_variant)]
#[cfg(any(feature = "full", feature = "activity"))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum SortStarred {
  #[serde(rename = "created")]
  Created,
  #[serde(rename = "updated")]
  Updated,
}

#[cfg(any(feature = "full", feature = "activity"))]
impl std::fmt::Display for SortStarred {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      SortStarred::Created => write!(f, "created"),
      SortStarred::Updated => write!(f, "updated"),
    }
  }
}
