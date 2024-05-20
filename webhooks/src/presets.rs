use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum StringOrInteger {
  String(String),
  Number(i64),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum StringOrNumber {
  String(String),
  Number(f64),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ObjectOrString<T> {
  String(String),
  Object(T),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum StringOrBool {
  String(String),
  Bool(bool),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ReadWritePermission {
  #[serde(rename = "read")]
  Read,
  #[serde(rename = "write")]
  Write,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Permissions {
  #[serde(rename = "read")]
  Read,
  #[serde(rename = "write")]
  Write,
  #[serde(rename = "admin")]
  Admin,
  #[serde(rename = "none")]
  None,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ReadWriteAdminPermission {
  #[serde(rename = "read")]
  Read,
  #[serde(rename = "write")]
  Write,
  #[serde(rename = "admin")]
  Admin,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ReadPermission {
  #[serde(rename = "read")]
  Read,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum WritePermission {
  #[serde(rename = "write")]
  Write,
}

/// The default value for a merge commit message.
///
/// - `PR_TITLE` - default to the pull request's title.
/// - `PR_BODY` - default to the pull request's body.
/// - `BLANK` - default to a blank commit message.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum MergeCommitMessage {
  #[serde(rename = "PR_BODY")]
  PrBody,
  #[serde(rename = "PR_TITLE")]
  PrTitle,
  #[serde(rename = "BLANK")]
  Blank,
}

impl std::fmt::Display for MergeCommitMessage {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      MergeCommitMessage::PrBody => write!(f, "PR_BODY"),
      MergeCommitMessage::PrTitle => write!(f, "PR_TITLE"),
      MergeCommitMessage::Blank => write!(f, "BLANK"),
    }
  }
}

/// The default value for a squash merge commit message:
///
/// - `PR_BODY` - default to the pull request's body.
/// - `COMMIT_MESSAGES` - default to the branch's commit messages.
/// - `BLANK` - default to a blank commit message.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum SquashMergeCommitMessage {
  #[serde(rename = "PR_BODY")]
  PrBody,
  #[serde(rename = "COMMIT_MESSAGES")]
  CommitMessages,
  #[serde(rename = "BLANK")]
  Blank,
}

impl std::fmt::Display for SquashMergeCommitMessage {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      SquashMergeCommitMessage::PrBody => write!(f, "PR_BODY"),
      SquashMergeCommitMessage::CommitMessages => write!(f, "COMMIT_MESSAGES"),
      SquashMergeCommitMessage::Blank => write!(f, "BLANK"),
    }
  }
}

/// The default value for a squash merge commit title:
///
/// - `PR_TITLE` - default to the pull request's title.
/// - `COMMIT_OR_PR_TITLE` - default to the commit's title (if only one commit) or the pull request's title (when more than one commit).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum SquashMergeCommitTitle {
  #[serde(rename = "PR_TITLE")]
  PrTitle,
  #[serde(rename = "COMMIT_OR_PR_TITLE")]
  CommitOrPrTitle,
}

impl std::fmt::Display for SquashMergeCommitTitle {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      SquashMergeCommitTitle::PrTitle => write!(f, "PR_TITLE"),
      SquashMergeCommitTitle::CommitOrPrTitle => write!(f, "COMMIT_OR_PR_TITLE"),
    }
  }
}

/// The default value for a merge commit title.
///
///   - `PR_TITLE` - default to the pull request's title.
///   - `MERGE_MESSAGE` - default to the classic title for a merge message (e.g., Merge pull request #123 from branch-name).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum MergeCommitTitle {
  #[serde(rename = "PR_TITLE")]
  PrTitle,
  #[serde(rename = "MERGE_MESSAGE")]
  MergeMessage,
}

impl std::fmt::Display for MergeCommitTitle {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      MergeCommitTitle::PrTitle => write!(f, "PR_TITLE"),
      MergeCommitTitle::MergeMessage => write!(f, "MERGE_MESSAGE"),
    }
  }
}

/// The severity of the vulnerability / alert / advisory.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum Severity {
  #[serde(rename = "low")]
  Low,
  #[serde(rename = "medium")]
  Medium,
  #[serde(rename = "high")]
  High,
  #[serde(rename = "critical")]
  Critical,
}

impl std::fmt::Display for Severity {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      Severity::Low => write!(f, "low"),
      Severity::Medium => write!(f, "medium"),
      Severity::High => write!(f, "high"),
      Severity::Critical => write!(f, "critical"),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum Visibility {
  #[serde(rename = "all")]
  All,
  #[serde(rename = "private")]
  Private,
  #[serde(rename = "selected")]
  Selected,
}

impl std::fmt::Display for Visibility {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      Visibility::All => write!(f, "all"),
      Visibility::Private => write!(f, "private"),
      Visibility::Selected => write!(f, "selected"),
    }
  }
}

/// The operator to use for matching.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum Operator {
  #[serde(rename = "starts_with")]
  StartsWith,
  #[serde(rename = "ends_with")]
  EndsWith,
  #[serde(rename = "contains")]
  Contains,
  #[serde(rename = "regex")]
  Regex,
}

impl std::fmt::Display for Operator {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      Operator::StartsWith => write!(f, "starts_with"),
      Operator::EndsWith => write!(f, "ends_with"),
      Operator::Contains => write!(f, "contains"),
      Operator::Regex => write!(f, "regex"),
    }
  }
}

/// The type of identifier.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum IdentifiersType {
  #[serde(rename = "CVE")]
  Cve,
  #[serde(rename = "GHSA")]
  Ghsa,
}

impl std::fmt::Display for IdentifiersType {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      IdentifiersType::Cve => write!(f, "CVE"),
      IdentifiersType::Ghsa => write!(f, "GHSA"),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum RepositorySelection {
  #[serde(rename = "all")]
  All,
  #[serde(rename = "selected")]
  Selected,
}

impl std::fmt::Display for RepositorySelection {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      RepositorySelection::All => write!(f, "all"),
      RepositorySelection::Selected => write!(f, "selected"),
    }
  }
}

/// The state of the Dependabot alert.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum DependabotAlertState {
  #[serde(rename = "auto_dismissed")]
  AutoDismissed,
  #[serde(rename = "dismissed")]
  Dismissed,
  #[serde(rename = "fixed")]
  Fixed,
  #[serde(rename = "open")]
  Open,
}

impl std::fmt::Display for DependabotAlertState {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      DependabotAlertState::AutoDismissed => write!(f, "auto_dismissed"),
      DependabotAlertState::Dismissed => write!(f, "dismissed"),
      DependabotAlertState::Fixed => write!(f, "fixed"),
      DependabotAlertState::Open => write!(f, "open"),
    }
  }
}

/// The organization policy for allowing or disallowing organization members to use Copilot within their CLI / editor ..
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum CopilotOrganizationPolicy {
  #[serde(rename = "enabled")]
  Enabled,
  #[serde(rename = "disabled")]
  Disabled,
  #[serde(rename = "unconfigured")]
  Unconfigured,
}

impl std::fmt::Display for CopilotOrganizationPolicy {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      CopilotOrganizationPolicy::Enabled => write!(f, "enabled"),
      CopilotOrganizationPolicy::Disabled => write!(f, "disabled"),
      CopilotOrganizationPolicy::Unconfigured => write!(f, "unconfigured"),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum PackageType {
  #[serde(rename = "npm")]
  Npm,
  #[serde(rename = "maven")]
  Maven,
  #[serde(rename = "rubygems")]
  Rubygems,
  #[serde(rename = "docker")]
  Docker,
  #[serde(rename = "nuget")]
  Nuget,
  #[serde(rename = "container")]
  Container,
}

impl std::fmt::Display for PackageType {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      PackageType::Npm => write!(f, "npm"),
      PackageType::Maven => write!(f, "maven"),
      PackageType::Rubygems => write!(f, "rubygems"),
      PackageType::Docker => write!(f, "docker"),
      PackageType::Nuget => write!(f, "nuget"),
      PackageType::Container => write!(f, "container"),
    }
  }
}

/// Type of repository selection requested.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum RequestRepositorySelection {
  #[serde(rename = "none")]
  None,
  #[serde(rename = "all")]
  All,
  #[serde(rename = "subset")]
  Subset,
}

impl std::fmt::Display for RequestRepositorySelection {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      RequestRepositorySelection::None => write!(f, "none"),
      RequestRepositorySelection::All => write!(f, "all"),
      RequestRepositorySelection::Subset => write!(f, "subset"),
    }
  }
}

/// The reason that the alert was dismissed.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum DismissedReason {
  #[serde(rename = "fix_started")]
  FixStarted,
  #[serde(rename = "inaccurate")]
  Inaccurate,
  #[serde(rename = "no_bandwidth")]
  NoBandwidth,
  #[serde(rename = "not_used")]
  NotUsed,
  #[serde(rename = "tolerable_risk")]
  TolerableRisk,
}

impl std::fmt::Display for DismissedReason {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      DismissedReason::FixStarted => write!(f, "fix_started"),
      DismissedReason::Inaccurate => write!(f, "inaccurate"),
      DismissedReason::NoBandwidth => write!(f, "no_bandwidth"),
      DismissedReason::NotUsed => write!(f, "not_used"),
      DismissedReason::TolerableRisk => write!(f, "tolerable_risk"),
    }
  }
}

impl std::fmt::Display for StringOrNumber {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      StringOrNumber::String(x) => write!(f, "{}", x),
      StringOrNumber::Number(x) => write!(f, "{}", x),
    }
  }
}

impl std::fmt::Display for StringOrInteger {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      StringOrInteger::String(x) => write!(f, "{}", x),
      StringOrInteger::Number(x) => write!(f, "{}", x),
    }
  }
}

impl std::fmt::Display for StringOrBool {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      StringOrBool::String(x) => write!(f, "{}", x),
      StringOrBool::Bool(x) => write!(f, "{}", x),
    }
  }
}

impl std::fmt::Display for ReadWritePermission {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      ReadWritePermission::Read => write!(f, "read"),
      ReadWritePermission::Write => write!(f, "write"),
    }
  }
}

impl std::fmt::Display for ReadPermission {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      ReadPermission::Read => write!(f, "read"),
    }
  }
}

impl std::fmt::Display for WritePermission {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      WritePermission::Write => write!(f, "write"),
    }
  }
}

impl std::fmt::Display for Permissions {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      Permissions::Read => write!(f, "read"),
      Permissions::Write => write!(f, "write"),
      Permissions::Admin => write!(f, "admin"),
      Permissions::None => write!(f, "none"),
    }
  }
}

impl std::fmt::Display for ReadWriteAdminPermission {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      ReadWriteAdminPermission::Read => write!(f, "read"),
      ReadWriteAdminPermission::Write => write!(f, "write"),
      ReadWriteAdminPermission::Admin => write!(f, "admin"),
    }
  }
}
