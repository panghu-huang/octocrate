#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use typed_builder::TypedBuilder;
#[cfg(any(feature = "full", feature = "repos"))]
pub type RuleSuites = Vec<RuleSuitesItem>;
#[cfg(any(feature = "full", feature = "repos"))]
pub type ContentDirectory = Vec<ContentDirectoryItem>;
#[cfg(any(feature = "full", feature = "dependency_graph"))]
pub type DependencyGraphDiff = Vec<DependencyGraphDiffItem>;
#[cfg(any(feature = "full", feature = "repos"))]
pub type CodeFrequencyStat = Vec<i64>;
#[cfg(any(feature = "full", feature = "search"))]
pub type SearchResultTextMatches = Vec<SearchResultTextMatchesItem>;

#[cfg(any(feature = "full", feature = "security_advisories"))]
/// The package's language or package management ecosystem.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum SecurityAdvisoryEcosystems {
  #[serde(rename = "rubygems")]
  Rubygems,
  #[serde(rename = "npm")]
  Npm,
  #[serde(rename = "pip")]
  Pip,
  #[serde(rename = "maven")]
  Maven,
  #[serde(rename = "nuget")]
  Nuget,
  #[serde(rename = "composer")]
  Composer,
  #[serde(rename = "go")]
  Go,
  #[serde(rename = "rust")]
  Rust,
  #[serde(rename = "erlang")]
  Erlang,
  #[serde(rename = "actions")]
  Actions,
  #[serde(rename = "pub")]
  Pub,
  #[serde(rename = "other")]
  Other,
  #[serde(rename = "swift")]
  Swift,
}

#[cfg(any(feature = "full", feature = "security_advisories"))]
impl ToString for SecurityAdvisoryEcosystems {
  fn to_string(&self) -> String {
    match self {
      SecurityAdvisoryEcosystems::Rubygems => "rubygems".to_string(),
      SecurityAdvisoryEcosystems::Npm => "npm".to_string(),
      SecurityAdvisoryEcosystems::Pip => "pip".to_string(),
      SecurityAdvisoryEcosystems::Maven => "maven".to_string(),
      SecurityAdvisoryEcosystems::Nuget => "nuget".to_string(),
      SecurityAdvisoryEcosystems::Composer => "composer".to_string(),
      SecurityAdvisoryEcosystems::Go => "go".to_string(),
      SecurityAdvisoryEcosystems::Rust => "rust".to_string(),
      SecurityAdvisoryEcosystems::Erlang => "erlang".to_string(),
      SecurityAdvisoryEcosystems::Actions => "actions".to_string(),
      SecurityAdvisoryEcosystems::Pub => "pub".to_string(),
      SecurityAdvisoryEcosystems::Other => "other".to_string(),
      SecurityAdvisoryEcosystems::Swift => "swift".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "security_advisories"))]
/// The type of credit the user is receiving.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum SecurityAdvisoryCreditTypes {
  #[serde(rename = "analyst")]
  Analyst,
  #[serde(rename = "finder")]
  Finder,
  #[serde(rename = "reporter")]
  Reporter,
  #[serde(rename = "coordinator")]
  Coordinator,
  #[serde(rename = "remediation_developer")]
  RemediationDeveloper,
  #[serde(rename = "remediation_reviewer")]
  RemediationReviewer,
  #[serde(rename = "remediation_verifier")]
  RemediationVerifier,
  #[serde(rename = "tool")]
  Tool,
  #[serde(rename = "sponsor")]
  Sponsor,
  #[serde(rename = "other")]
  Other,
}

#[cfg(any(feature = "full", feature = "security_advisories"))]
impl ToString for SecurityAdvisoryCreditTypes {
  fn to_string(&self) -> String {
    match self {
      SecurityAdvisoryCreditTypes::Analyst => "analyst".to_string(),
      SecurityAdvisoryCreditTypes::Finder => "finder".to_string(),
      SecurityAdvisoryCreditTypes::Reporter => "reporter".to_string(),
      SecurityAdvisoryCreditTypes::Coordinator => "coordinator".to_string(),
      SecurityAdvisoryCreditTypes::RemediationDeveloper => "remediation_developer".to_string(),
      SecurityAdvisoryCreditTypes::RemediationReviewer => "remediation_reviewer".to_string(),
      SecurityAdvisoryCreditTypes::RemediationVerifier => "remediation_verifier".to_string(),
      SecurityAdvisoryCreditTypes::Tool => "tool".to_string(),
      SecurityAdvisoryCreditTypes::Sponsor => "sponsor".to_string(),
      SecurityAdvisoryCreditTypes::Other => "other".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "security_advisories"))]
/// The severity of the advisory.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum GlobalAdvisorySeverity {
  #[serde(rename = "critical")]
  Critical,
  #[serde(rename = "high")]
  High,
  #[serde(rename = "medium")]
  Medium,
  #[serde(rename = "low")]
  Low,
  #[serde(rename = "unknown")]
  Unknown,
}

#[cfg(any(feature = "full", feature = "security_advisories"))]
impl ToString for GlobalAdvisorySeverity {
  fn to_string(&self) -> String {
    match self {
      GlobalAdvisorySeverity::Critical => "critical".to_string(),
      GlobalAdvisorySeverity::High => "high".to_string(),
      GlobalAdvisorySeverity::Medium => "medium".to_string(),
      GlobalAdvisorySeverity::Low => "low".to_string(),
      GlobalAdvisorySeverity::Unknown => "unknown".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "security_advisories"))]
/// The type of advisory.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum GlobalAdvisoryType {
  #[serde(rename = "reviewed")]
  Reviewed,
  #[serde(rename = "unreviewed")]
  Unreviewed,
  #[serde(rename = "malware")]
  Malware,
}

#[cfg(any(feature = "full", feature = "security_advisories"))]
impl ToString for GlobalAdvisoryType {
  fn to_string(&self) -> String {
    match self {
      GlobalAdvisoryType::Reviewed => "reviewed".to_string(),
      GlobalAdvisoryType::Unreviewed => "unreviewed".to_string(),
      GlobalAdvisoryType::Malware => "malware".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "apps"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IntegrationInstallationRequestAccount {
  /// A GitHub user.
  SimpleUser(SimpleUser),
  /// An enterprise on GitHub.
  Enterprise(Enterprise),
}

#[cfg(any(feature = "full", feature = "apps", feature = "orgs"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InstallationAccount {
  /// A GitHub user.
  SimpleUser(SimpleUser),
  /// An enterprise on GitHub.
  Enterprise(Enterprise),
}

#[cfg(any(feature = "full", feature = "classroom"))]
/// Whether it's a group assignment or individual assignment.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum ClassroomAssignmentType {
  #[serde(rename = "individual")]
  Individual,
  #[serde(rename = "group")]
  Group,
}

#[cfg(any(feature = "full", feature = "classroom"))]
impl ToString for ClassroomAssignmentType {
  fn to_string(&self) -> String {
    match self {
      ClassroomAssignmentType::Individual => "individual".to_string(),
      ClassroomAssignmentType::Group => "group".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "classroom"))]
/// Whether it's a Group Assignment or Individual Assignment.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum SimpleClassroomAssignmentType {
  #[serde(rename = "individual")]
  Individual,
  #[serde(rename = "group")]
  Group,
}

#[cfg(any(feature = "full", feature = "classroom"))]
impl ToString for SimpleClassroomAssignmentType {
  fn to_string(&self) -> String {
    match self {
      SimpleClassroomAssignmentType::Individual => "individual".to_string(),
      SimpleClassroomAssignmentType::Group => "group".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "dependabot"))]
/// The execution scope of the vulnerable dependency.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum DependabotAlertWithRepositoryDependencyScope {
  #[serde(rename = "development")]
  Development,
  #[serde(rename = "runtime")]
  Runtime,
}

#[cfg(any(feature = "full", feature = "dependabot"))]
impl ToString for DependabotAlertWithRepositoryDependencyScope {
  fn to_string(&self) -> String {
    match self {
      DependabotAlertWithRepositoryDependencyScope::Development => "development".to_string(),
      DependabotAlertWithRepositoryDependencyScope::Runtime => "runtime".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "secret_scanning"))]
/// **Required when the `state` is `resolved`.** The reason for resolving the alert.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum SecretScanningAlertResolution {
  #[serde(rename = "false_positive")]
  FalsePositive,
  #[serde(rename = "wont_fix")]
  WontFix,
  #[serde(rename = "revoked")]
  Revoked,
  #[serde(rename = "used_in_tests")]
  UsedInTests,
}

#[cfg(any(feature = "full", feature = "secret_scanning"))]
impl ToString for SecretScanningAlertResolution {
  fn to_string(&self) -> String {
    match self {
      SecretScanningAlertResolution::FalsePositive => "false_positive".to_string(),
      SecretScanningAlertResolution::WontFix => "wont_fix".to_string(),
      SecretScanningAlertResolution::Revoked => "revoked".to_string(),
      SecretScanningAlertResolution::UsedInTests => "used_in_tests".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "secret_scanning"))]
/// Sets the state of the secret scanning alert. You must provide `resolution` when you set the state to `resolved`.
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
/// The token status as of the latest validity check.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum OrganizationSecretScanningAlertValidity {
  #[serde(rename = "active")]
  Active,
  #[serde(rename = "inactive")]
  Inactive,
  #[serde(rename = "unknown")]
  Unknown,
}

#[cfg(any(feature = "full", feature = "secret_scanning"))]
impl ToString for OrganizationSecretScanningAlertValidity {
  fn to_string(&self) -> String {
    match self {
      OrganizationSecretScanningAlertValidity::Active => "active".to_string(),
      OrganizationSecretScanningAlertValidity::Inactive => "inactive".to_string(),
      OrganizationSecretScanningAlertValidity::Unknown => "unknown".to_string(),
    }
  }
}

#[cfg(any(
  feature = "full",
  feature = "activity",
  feature = "gists",
  feature = "issues",
  feature = "repos",
  feature = "pulls",
  feature = "search"
))]
/// How the author is associated with the repository.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum AuthorAssociation {
  #[serde(rename = "COLLABORATOR")]
  Collaborator,
  #[serde(rename = "CONTRIBUTOR")]
  Contributor,
  #[serde(rename = "FIRST_TIMER")]
  FirstTimer,
  #[serde(rename = "FIRST_TIME_CONTRIBUTOR")]
  FirstTimeContributor,
  #[serde(rename = "MANNEQUIN")]
  Mannequin,
  #[serde(rename = "MEMBER")]
  Member,
  #[serde(rename = "NONE")]
  None,
  #[serde(rename = "OWNER")]
  Owner,
}

#[cfg(any(
  feature = "full",
  feature = "activity",
  feature = "gists",
  feature = "issues",
  feature = "repos",
  feature = "pulls",
  feature = "search"
))]
impl ToString for AuthorAssociation {
  fn to_string(&self) -> String {
    match self {
      AuthorAssociation::Collaborator => "COLLABORATOR".to_string(),
      AuthorAssociation::Contributor => "CONTRIBUTOR".to_string(),
      AuthorAssociation::FirstTimer => "FIRST_TIMER".to_string(),
      AuthorAssociation::FirstTimeContributor => "FIRST_TIME_CONTRIBUTOR".to_string(),
      AuthorAssociation::Mannequin => "MANNEQUIN".to_string(),
      AuthorAssociation::Member => "MEMBER".to_string(),
      AuthorAssociation::None => "NONE".to_string(),
      AuthorAssociation::Owner => "OWNER".to_string(),
    }
  }
}

#[cfg(any(
  feature = "full",
  feature = "activity",
  feature = "issues",
  feature = "repos",
  feature = "pulls",
  feature = "search"
))]
/// The state of the milestone.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum MilestoneState {
  #[serde(rename = "open")]
  Open,
  #[serde(rename = "closed")]
  Closed,
}

#[cfg(any(
  feature = "full",
  feature = "activity",
  feature = "issues",
  feature = "repos",
  feature = "pulls",
  feature = "search"
))]
impl ToString for MilestoneState {
  fn to_string(&self) -> String {
    match self {
      MilestoneState::Open => "open".to_string(),
      MilestoneState::Closed => "closed".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "activity", feature = "issues"))]
/// The reason for the current state
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum IssueStateReason {
  #[serde(rename = "completed")]
  Completed,
  #[serde(rename = "reopened")]
  Reopened,
  #[serde(rename = "not_planned")]
  NotPlanned,
}

#[cfg(any(feature = "full", feature = "activity", feature = "issues"))]
impl ToString for IssueStateReason {
  fn to_string(&self) -> String {
    match self {
      IssueStateReason::Completed => "completed".to_string(),
      IssueStateReason::Reopened => "reopened".to_string(),
      IssueStateReason::NotPlanned => "not_planned".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "apps"))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum MarketplaceListingPlanPriceModel {
  #[serde(rename = "FREE")]
  Free,
  #[serde(rename = "FLAT_RATE")]
  FlatRate,
  #[serde(rename = "PER_UNIT")]
  PerUnit,
}

#[cfg(any(feature = "full", feature = "apps"))]
impl ToString for MarketplaceListingPlanPriceModel {
  fn to_string(&self) -> String {
    match self {
      MarketplaceListingPlanPriceModel::Free => "FREE".to_string(),
      MarketplaceListingPlanPriceModel::FlatRate => "FLAT_RATE".to_string(),
      MarketplaceListingPlanPriceModel::PerUnit => "PER_UNIT".to_string(),
    }
  }
}

#[cfg(any(
  feature = "full",
  feature = "activity",
  feature = "actions",
  feature = "codespaces",
  feature = "dependabot",
  feature = "packages",
  feature = "migrations",
  feature = "orgs",
  feature = "repos",
  feature = "teams",
  feature = "checks",
  feature = "security_advisories",
  feature = "search"
))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum SecurityAndAnalysisAdvancedSecurityStatus {
  #[serde(rename = "enabled")]
  Enabled,
  #[serde(rename = "disabled")]
  Disabled,
}

#[cfg(any(
  feature = "full",
  feature = "activity",
  feature = "actions",
  feature = "codespaces",
  feature = "dependabot",
  feature = "packages",
  feature = "migrations",
  feature = "orgs",
  feature = "repos",
  feature = "teams",
  feature = "checks",
  feature = "security_advisories",
  feature = "search"
))]
impl ToString for SecurityAndAnalysisAdvancedSecurityStatus {
  fn to_string(&self) -> String {
    match self {
      SecurityAndAnalysisAdvancedSecurityStatus::Enabled => "enabled".to_string(),
      SecurityAndAnalysisAdvancedSecurityStatus::Disabled => "disabled".to_string(),
    }
  }
}

#[cfg(any(
  feature = "full",
  feature = "activity",
  feature = "actions",
  feature = "codespaces",
  feature = "dependabot",
  feature = "packages",
  feature = "migrations",
  feature = "orgs",
  feature = "repos",
  feature = "teams",
  feature = "checks",
  feature = "security_advisories",
  feature = "search"
))]
/// The enablement status of Dependabot security updates for the repository.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum SecurityAndAnalysisDependabotSecurityUpdatesStatus {
  #[serde(rename = "enabled")]
  Enabled,
  #[serde(rename = "disabled")]
  Disabled,
}

#[cfg(any(
  feature = "full",
  feature = "activity",
  feature = "actions",
  feature = "codespaces",
  feature = "dependabot",
  feature = "packages",
  feature = "migrations",
  feature = "orgs",
  feature = "repos",
  feature = "teams",
  feature = "checks",
  feature = "security_advisories",
  feature = "search"
))]
impl ToString for SecurityAndAnalysisDependabotSecurityUpdatesStatus {
  fn to_string(&self) -> String {
    match self {
      SecurityAndAnalysisDependabotSecurityUpdatesStatus::Enabled => "enabled".to_string(),
      SecurityAndAnalysisDependabotSecurityUpdatesStatus::Disabled => "disabled".to_string(),
    }
  }
}

#[cfg(any(
  feature = "full",
  feature = "activity",
  feature = "actions",
  feature = "codespaces",
  feature = "dependabot",
  feature = "packages",
  feature = "migrations",
  feature = "orgs",
  feature = "repos",
  feature = "teams",
  feature = "checks",
  feature = "security_advisories",
  feature = "search"
))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum SecurityAndAnalysisSecretScanningStatus {
  #[serde(rename = "enabled")]
  Enabled,
  #[serde(rename = "disabled")]
  Disabled,
}

#[cfg(any(
  feature = "full",
  feature = "activity",
  feature = "actions",
  feature = "codespaces",
  feature = "dependabot",
  feature = "packages",
  feature = "migrations",
  feature = "orgs",
  feature = "repos",
  feature = "teams",
  feature = "checks",
  feature = "security_advisories",
  feature = "search"
))]
impl ToString for SecurityAndAnalysisSecretScanningStatus {
  fn to_string(&self) -> String {
    match self {
      SecurityAndAnalysisSecretScanningStatus::Enabled => "enabled".to_string(),
      SecurityAndAnalysisSecretScanningStatus::Disabled => "disabled".to_string(),
    }
  }
}

#[cfg(any(
  feature = "full",
  feature = "activity",
  feature = "actions",
  feature = "codespaces",
  feature = "dependabot",
  feature = "packages",
  feature = "migrations",
  feature = "orgs",
  feature = "repos",
  feature = "teams",
  feature = "checks",
  feature = "security_advisories",
  feature = "search"
))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum SecurityAndAnalysisSecretScanningPushProtectionStatus {
  #[serde(rename = "enabled")]
  Enabled,
  #[serde(rename = "disabled")]
  Disabled,
}

#[cfg(any(
  feature = "full",
  feature = "activity",
  feature = "actions",
  feature = "codespaces",
  feature = "dependabot",
  feature = "packages",
  feature = "migrations",
  feature = "orgs",
  feature = "repos",
  feature = "teams",
  feature = "checks",
  feature = "security_advisories",
  feature = "search"
))]
impl ToString for SecurityAndAnalysisSecretScanningPushProtectionStatus {
  fn to_string(&self) -> String {
    match self {
      SecurityAndAnalysisSecretScanningPushProtectionStatus::Enabled => "enabled".to_string(),
      SecurityAndAnalysisSecretScanningPushProtectionStatus::Disabled => "disabled".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "actions"))]
/// The permissions policy that controls the actions and reusable workflows that are allowed to run.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum AllowedActions {
  #[serde(rename = "all")]
  All,
  #[serde(rename = "local_only")]
  LocalOnly,
  #[serde(rename = "selected")]
  Selected,
}

#[cfg(any(feature = "full", feature = "actions"))]
impl ToString for AllowedActions {
  fn to_string(&self) -> String {
    match self {
      AllowedActions::All => "all".to_string(),
      AllowedActions::LocalOnly => "local_only".to_string(),
      AllowedActions::Selected => "selected".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "actions"))]
/// The policy that controls the repositories in the organization that are allowed to run GitHub Actions.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum EnabledRepositories {
  #[serde(rename = "all")]
  All,
  #[serde(rename = "none")]
  None,
  #[serde(rename = "selected")]
  Selected,
}

#[cfg(any(feature = "full", feature = "actions"))]
impl ToString for EnabledRepositories {
  fn to_string(&self) -> String {
    match self {
      EnabledRepositories::All => "all".to_string(),
      EnabledRepositories::None => "none".to_string(),
      EnabledRepositories::Selected => "selected".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "actions"))]
/// The type of label. Read-only labels are applied automatically when the runner is configured.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum RunnerLabelType {
  #[serde(rename = "read-only")]
  ReadOnly,
  #[serde(rename = "custom")]
  Custom,
}

#[cfg(any(feature = "full", feature = "actions"))]
impl ToString for RunnerLabelType {
  fn to_string(&self) -> String {
    match self {
      RunnerLabelType::ReadOnly => "read-only".to_string(),
      RunnerLabelType::Custom => "custom".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "code_scanning"))]
/// State of a code scanning alert.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum CodeScanningAlertStateQuery {
  #[serde(rename = "open")]
  Open,
  #[serde(rename = "closed")]
  Closed,
  #[serde(rename = "dismissed")]
  Dismissed,
  #[serde(rename = "fixed")]
  Fixed,
}

#[cfg(any(feature = "full", feature = "code_scanning"))]
impl ToString for CodeScanningAlertStateQuery {
  fn to_string(&self) -> String {
    match self {
      CodeScanningAlertStateQuery::Open => "open".to_string(),
      CodeScanningAlertStateQuery::Closed => "closed".to_string(),
      CodeScanningAlertStateQuery::Dismissed => "dismissed".to_string(),
      CodeScanningAlertStateQuery::Fixed => "fixed".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "code_scanning"))]
/// Severity of a code scanning alert.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum CodeScanningAlertSeverity {
  #[serde(rename = "critical")]
  Critical,
  #[serde(rename = "high")]
  High,
  #[serde(rename = "medium")]
  Medium,
  #[serde(rename = "low")]
  Low,
  #[serde(rename = "warning")]
  Warning,
  #[serde(rename = "note")]
  Note,
  #[serde(rename = "error")]
  Error,
}

#[cfg(any(feature = "full", feature = "code_scanning"))]
impl ToString for CodeScanningAlertSeverity {
  fn to_string(&self) -> String {
    match self {
      CodeScanningAlertSeverity::Critical => "critical".to_string(),
      CodeScanningAlertSeverity::High => "high".to_string(),
      CodeScanningAlertSeverity::Medium => "medium".to_string(),
      CodeScanningAlertSeverity::Low => "low".to_string(),
      CodeScanningAlertSeverity::Warning => "warning".to_string(),
      CodeScanningAlertSeverity::Note => "note".to_string(),
      CodeScanningAlertSeverity::Error => "error".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "code_scanning"))]
/// **Required when the state is dismissed.** The reason for dismissing or closing the alert.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum CodeScanningAlertDismissedReason {
  #[serde(rename = "false positive")]
  FalsePositive,
  #[serde(rename = "won't fix")]
  WonTFix,
  #[serde(rename = "used in tests")]
  UsedInTests,
}

#[cfg(any(feature = "full", feature = "code_scanning"))]
impl ToString for CodeScanningAlertDismissedReason {
  fn to_string(&self) -> String {
    match self {
      CodeScanningAlertDismissedReason::FalsePositive => "false positive".to_string(),
      CodeScanningAlertDismissedReason::WonTFix => "won't fix".to_string(),
      CodeScanningAlertDismissedReason::UsedInTests => "used in tests".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "code_scanning"))]
/// A classification of the file. For example to identify it as generated.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum CodeScanningAlertClassification {
  #[serde(rename = "source")]
  Source,
  #[serde(rename = "generated")]
  Generated,
  #[serde(rename = "test")]
  Test,
  #[serde(rename = "library")]
  Library,
}

#[cfg(any(feature = "full", feature = "code_scanning"))]
impl ToString for CodeScanningAlertClassification {
  fn to_string(&self) -> String {
    match self {
      CodeScanningAlertClassification::Source => "source".to_string(),
      CodeScanningAlertClassification::Generated => "generated".to_string(),
      CodeScanningAlertClassification::Test => "test".to_string(),
      CodeScanningAlertClassification::Library => "library".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "code_scanning"))]
/// State of a code scanning alert.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum CodeScanningAlertState {
  #[serde(rename = "open")]
  Open,
  #[serde(rename = "dismissed")]
  Dismissed,
  #[serde(rename = "fixed")]
  Fixed,
}

#[cfg(any(feature = "full", feature = "code_scanning"))]
impl ToString for CodeScanningAlertState {
  fn to_string(&self) -> String {
    match self {
      CodeScanningAlertState::Open => "open".to_string(),
      CodeScanningAlertState::Dismissed => "dismissed".to_string(),
      CodeScanningAlertState::Fixed => "fixed".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "code_scanning"))]
/// The severity of the alert.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum CodeScanningAlertRuleSummarySeverity {
  #[serde(rename = "none")]
  None,
  #[serde(rename = "note")]
  Note,
  #[serde(rename = "warning")]
  Warning,
  #[serde(rename = "error")]
  Error,
}

#[cfg(any(feature = "full", feature = "code_scanning"))]
impl ToString for CodeScanningAlertRuleSummarySeverity {
  fn to_string(&self) -> String {
    match self {
      CodeScanningAlertRuleSummarySeverity::None => "none".to_string(),
      CodeScanningAlertRuleSummarySeverity::Note => "note".to_string(),
      CodeScanningAlertRuleSummarySeverity::Warning => "warning".to_string(),
      CodeScanningAlertRuleSummarySeverity::Error => "error".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "codespaces"))]
/// The initally assigned location of a new codespace.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum CodespaceLocation {
  EastUs,
  SouthEastAsia,
  WestEurope,
  WestUs2,
}

#[cfg(any(feature = "full", feature = "codespaces"))]
impl ToString for CodespaceLocation {
  fn to_string(&self) -> String {
    match self {
      CodespaceLocation::EastUs => "EastUs".to_string(),
      CodespaceLocation::SouthEastAsia => "SouthEastAsia".to_string(),
      CodespaceLocation::WestEurope => "WestEurope".to_string(),
      CodespaceLocation::WestUs2 => "WestUs2".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "codespaces"))]
/// Whether a prebuild is currently available when creating a codespace for this machine and repository. If a branch was not specified as a ref, the default branch will be assumed. Value will be "null" if prebuilds are not supported or prebuild availability could not be determined. Value will be "none" if no prebuild is available. Latest values "ready" and "in_progress" indicate the prebuild availability status.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum CodespaceMachinePrebuildAvailability {
  #[serde(rename = "none")]
  None,
  #[serde(rename = "ready")]
  Ready,
  #[serde(rename = "in_progress")]
  InProgress,
}

#[cfg(any(feature = "full", feature = "codespaces"))]
impl ToString for CodespaceMachinePrebuildAvailability {
  fn to_string(&self) -> String {
    match self {
      CodespaceMachinePrebuildAvailability::None => "none".to_string(),
      CodespaceMachinePrebuildAvailability::Ready => "ready".to_string(),
      CodespaceMachinePrebuildAvailability::InProgress => "in_progress".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "codespaces"))]
/// State of this codespace.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum CodespaceState {
  Unknown,
  Created,
  Queued,
  Provisioning,
  Available,
  Awaiting,
  Unavailable,
  Deleted,
  Moved,
  Shutdown,
  Archived,
  Starting,
  ShuttingDown,
  Failed,
  Exporting,
  Updating,
  Rebuilding,
}

#[cfg(any(feature = "full", feature = "codespaces"))]
impl ToString for CodespaceState {
  fn to_string(&self) -> String {
    match self {
      CodespaceState::Unknown => "Unknown".to_string(),
      CodespaceState::Created => "Created".to_string(),
      CodespaceState::Queued => "Queued".to_string(),
      CodespaceState::Provisioning => "Provisioning".to_string(),
      CodespaceState::Available => "Available".to_string(),
      CodespaceState::Awaiting => "Awaiting".to_string(),
      CodespaceState::Unavailable => "Unavailable".to_string(),
      CodespaceState::Deleted => "Deleted".to_string(),
      CodespaceState::Moved => "Moved".to_string(),
      CodespaceState::Shutdown => "Shutdown".to_string(),
      CodespaceState::Archived => "Archived".to_string(),
      CodespaceState::Starting => "Starting".to_string(),
      CodespaceState::ShuttingDown => "ShuttingDown".to_string(),
      CodespaceState::Failed => "Failed".to_string(),
      CodespaceState::Exporting => "Exporting".to_string(),
      CodespaceState::Updating => "Updating".to_string(),
      CodespaceState::Rebuilding => "Rebuilding".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "copilot"))]
/// The organization policy for allowing or disallowing Copilot to make suggestions that match public code.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum CopilotOrganizationDetailsPublicCodeSuggestions {
  #[serde(rename = "allow")]
  Allow,
  #[serde(rename = "block")]
  Block,
  #[serde(rename = "unconfigured")]
  Unconfigured,
  #[serde(rename = "unknown")]
  Unknown,
}

#[cfg(any(feature = "full", feature = "copilot"))]
impl ToString for CopilotOrganizationDetailsPublicCodeSuggestions {
  fn to_string(&self) -> String {
    match self {
      CopilotOrganizationDetailsPublicCodeSuggestions::Allow => "allow".to_string(),
      CopilotOrganizationDetailsPublicCodeSuggestions::Block => "block".to_string(),
      CopilotOrganizationDetailsPublicCodeSuggestions::Unconfigured => "unconfigured".to_string(),
      CopilotOrganizationDetailsPublicCodeSuggestions::Unknown => "unknown".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "copilot"))]
/// The mode of assigning new seats.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum CopilotOrganizationDetailsSeatManagementSetting {
  #[serde(rename = "assign_all")]
  AssignAll,
  #[serde(rename = "assign_selected")]
  AssignSelected,
  #[serde(rename = "disabled")]
  Disabled,
  #[serde(rename = "unconfigured")]
  Unconfigured,
}

#[cfg(any(feature = "full", feature = "copilot"))]
impl ToString for CopilotOrganizationDetailsSeatManagementSetting {
  fn to_string(&self) -> String {
    match self {
      CopilotOrganizationDetailsSeatManagementSetting::AssignAll => "assign_all".to_string(),
      CopilotOrganizationDetailsSeatManagementSetting::AssignSelected => {
        "assign_selected".to_string()
      }
      CopilotOrganizationDetailsSeatManagementSetting::Disabled => "disabled".to_string(),
      CopilotOrganizationDetailsSeatManagementSetting::Unconfigured => "unconfigured".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "copilot"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CopilotSeatDetailsAssignee {
  /// A GitHub user.
  SimpleUser(SimpleUser),
  /// Groups of organization members that gives permissions on specified repositories.
  Team(Team),
  /// GitHub account for managing multiple users, teams, and repositories
  Organization(Organization),
}

#[cfg(any(feature = "full", feature = "packages"))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum PackageVisibility {
  #[serde(rename = "private")]
  Private,
  #[serde(rename = "public")]
  Public,
}

#[cfg(any(feature = "full", feature = "packages"))]
impl ToString for PackageVisibility {
  fn to_string(&self) -> String {
    match self {
      PackageVisibility::Private => "private".to_string(),
      PackageVisibility::Public => "public".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "interactions"))]
/// The type of GitHub user that can comment, open issues, or create pull requests while the interaction limit is in effect.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum InteractionGroup {
  #[serde(rename = "existing_users")]
  ExistingUsers,
  #[serde(rename = "contributors_only")]
  ContributorsOnly,
  #[serde(rename = "collaborators_only")]
  CollaboratorsOnly,
}

#[cfg(any(feature = "full", feature = "interactions"))]
impl ToString for InteractionGroup {
  fn to_string(&self) -> String {
    match self {
      InteractionGroup::ExistingUsers => "existing_users".to_string(),
      InteractionGroup::ContributorsOnly => "contributors_only".to_string(),
      InteractionGroup::CollaboratorsOnly => "collaborators_only".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "interactions"))]
/// The duration of the interaction restriction. Default: `one_day`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum InteractionExpiry {
  #[serde(rename = "one_day")]
  OneDay,
  #[serde(rename = "three_days")]
  ThreeDays,
  #[serde(rename = "one_week")]
  OneWeek,
  #[serde(rename = "one_month")]
  OneMonth,
  #[serde(rename = "six_months")]
  SixMonths,
}

#[cfg(any(feature = "full", feature = "interactions"))]
impl ToString for InteractionExpiry {
  fn to_string(&self) -> String {
    match self {
      InteractionExpiry::OneDay => "one_day".to_string(),
      InteractionExpiry::ThreeDays => "three_days".to_string(),
      InteractionExpiry::OneWeek => "one_week".to_string(),
      InteractionExpiry::OneMonth => "one_month".to_string(),
      InteractionExpiry::SixMonths => "six_months".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "orgs"))]
/// The user's membership type in the organization.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum OrgMembershipRole {
  #[serde(rename = "admin")]
  Admin,
  #[serde(rename = "member")]
  Member,
  #[serde(rename = "billing_manager")]
  BillingManager,
}

#[cfg(any(feature = "full", feature = "orgs"))]
impl ToString for OrgMembershipRole {
  fn to_string(&self) -> String {
    match self {
      OrgMembershipRole::Admin => "admin".to_string(),
      OrgMembershipRole::Member => "member".to_string(),
      OrgMembershipRole::BillingManager => "billing_manager".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "orgs"))]
/// The state of the member in the organization. The `pending` state indicates the user has not yet accepted an invitation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum OrgMembershipState {
  #[serde(rename = "active")]
  Active,
  #[serde(rename = "pending")]
  Pending,
}

#[cfg(any(feature = "full", feature = "orgs"))]
impl ToString for OrgMembershipState {
  fn to_string(&self) -> String {
    match self {
      OrgMembershipState::Active => "active".to_string(),
      OrgMembershipState::Pending => "pending".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "projects"))]
/// The baseline permission that all organization members have on this project. Only present if owner is an organization.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum ProjectOrganizationPermission {
  #[serde(rename = "read")]
  Read,
  #[serde(rename = "write")]
  Write,
  #[serde(rename = "admin")]
  Admin,
  #[serde(rename = "none")]
  None,
}

#[cfg(any(feature = "full", feature = "projects"))]
impl ToString for ProjectOrganizationPermission {
  fn to_string(&self) -> String {
    match self {
      ProjectOrganizationPermission::Read => "read".to_string(),
      ProjectOrganizationPermission::Write => "write".to_string(),
      ProjectOrganizationPermission::Admin => "admin".to_string(),
      ProjectOrganizationPermission::None => "none".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "orgs"))]
/// The type of the value for the property
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum OrgCustomPropertyValueType {
  #[serde(rename = "string")]
  String,
  #[serde(rename = "single_select")]
  SingleSelect,
}

#[cfg(any(feature = "full", feature = "orgs"))]
impl ToString for OrgCustomPropertyValueType {
  fn to_string(&self) -> String {
    match self {
      OrgCustomPropertyValueType::String => "string".to_string(),
      OrgCustomPropertyValueType::SingleSelect => "single_select".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "orgs"))]
/// Who can edit the values of the property
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum OrgCustomPropertyValuesEditableBy {
  #[serde(rename = "org_actors")]
  OrgActors,
  #[serde(rename = "org_and_repo_actors")]
  OrgAndRepoActors,
}

#[cfg(any(feature = "full", feature = "orgs"))]
impl ToString for OrgCustomPropertyValuesEditableBy {
  fn to_string(&self) -> String {
    match self {
      OrgCustomPropertyValuesEditableBy::OrgActors => "org_actors".to_string(),
      OrgCustomPropertyValuesEditableBy::OrgAndRepoActors => "org_and_repo_actors".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "repos"))]
/// The type of actor that can bypass a ruleset
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum RepositoryRulesetBypassActorActorType {
  RepositoryRole,
  Team,
  Integration,
  OrganizationAdmin,
}

#[cfg(any(feature = "full", feature = "repos"))]
impl ToString for RepositoryRulesetBypassActorActorType {
  fn to_string(&self) -> String {
    match self {
      RepositoryRulesetBypassActorActorType::RepositoryRole => "RepositoryRole".to_string(),
      RepositoryRulesetBypassActorActorType::Team => "Team".to_string(),
      RepositoryRulesetBypassActorActorType::Integration => "Integration".to_string(),
      RepositoryRulesetBypassActorActorType::OrganizationAdmin => "OrganizationAdmin".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "repos"))]
/// When the specified actor can bypass the ruleset. `pull_request` means that an actor can only bypass rules on pull requests.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum RepositoryRulesetBypassActorBypassMode {
  #[serde(rename = "always")]
  Always,
  #[serde(rename = "pull_request")]
  PullRequest,
}

#[cfg(any(feature = "full", feature = "repos"))]
impl ToString for RepositoryRulesetBypassActorBypassMode {
  fn to_string(&self) -> String {
    match self {
      RepositoryRulesetBypassActorBypassMode::Always => "always".to_string(),
      RepositoryRulesetBypassActorBypassMode::PullRequest => "pull_request".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OrgRulesetConditionsItem1 {
  /// Parameters for a repository ruleset ref name condition
  RepositoryRulesetConditions(RepositoryRulesetConditions),
  /// Parameters for a repository name condition
  RepositoryRulesetConditionsRepositoryNameTarget(RepositoryRulesetConditionsRepositoryNameTarget),
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OrgRulesetConditionsItem2 {
  /// Parameters for a repository ruleset ref name condition
  RepositoryRulesetConditions(RepositoryRulesetConditions),
  /// Parameters for a repository ID condition
  RepositoryRulesetConditionsRepositoryIdTarget(RepositoryRulesetConditionsRepositoryIdTarget),
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OrgRulesetConditionsItem3 {
  /// Parameters for a repository ruleset ref name condition
  RepositoryRulesetConditions(RepositoryRulesetConditions),
  /// Parameters for a repository property condition
  RepositoryRulesetConditionsRepositoryPropertyTarget(
    RepositoryRulesetConditionsRepositoryPropertyTarget,
  ),
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OrgRulesetConditions {
  OrgRulesetConditionsItem1(OrgRulesetConditionsItem1),
  OrgRulesetConditionsItem2(OrgRulesetConditionsItem2),
  OrgRulesetConditionsItem3(OrgRulesetConditionsItem3),
}

#[cfg(any(feature = "full", feature = "repos"))]
/// The bypass type of the user making the API request for this ruleset. This field is only returned when
/// querying the repository-level endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum RepositoryRulesetCurrentUserCanBypass {
  #[serde(rename = "always")]
  Always,
  #[serde(rename = "pull_requests_only")]
  PullRequestsOnly,
  #[serde(rename = "never")]
  Never,
}

#[cfg(any(feature = "full", feature = "repos"))]
impl ToString for RepositoryRulesetCurrentUserCanBypass {
  fn to_string(&self) -> String {
    match self {
      RepositoryRulesetCurrentUserCanBypass::Always => "always".to_string(),
      RepositoryRulesetCurrentUserCanBypass::PullRequestsOnly => "pull_requests_only".to_string(),
      RepositoryRulesetCurrentUserCanBypass::Never => "never".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "repos"))]
/// The enforcement level of the ruleset. `evaluate` allows admins to test rules before enforcing them. Admins can view insights on the Rule Insights page (`evaluate` is only available with GitHub Enterprise).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum RepositoryRuleEnforcement {
  #[serde(rename = "disabled")]
  Disabled,
  #[serde(rename = "active")]
  Active,
  #[serde(rename = "evaluate")]
  Evaluate,
}

#[cfg(any(feature = "full", feature = "repos"))]
impl ToString for RepositoryRuleEnforcement {
  fn to_string(&self) -> String {
    match self {
      RepositoryRuleEnforcement::Disabled => "disabled".to_string(),
      RepositoryRuleEnforcement::Active => "active".to_string(),
      RepositoryRuleEnforcement::Evaluate => "evaluate".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum RepositoryRuleCreationType {
  #[serde(rename = "creation")]
  Creation,
}

#[cfg(any(feature = "full", feature = "repos"))]
impl ToString for RepositoryRuleCreationType {
  fn to_string(&self) -> String {
    match self {
      RepositoryRuleCreationType::Creation => "creation".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum RepositoryRuleUpdateType {
  #[serde(rename = "update")]
  Update,
}

#[cfg(any(feature = "full", feature = "repos"))]
impl ToString for RepositoryRuleUpdateType {
  fn to_string(&self) -> String {
    match self {
      RepositoryRuleUpdateType::Update => "update".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum RepositoryRuleDeletionType {
  #[serde(rename = "deletion")]
  Deletion,
}

#[cfg(any(feature = "full", feature = "repos"))]
impl ToString for RepositoryRuleDeletionType {
  fn to_string(&self) -> String {
    match self {
      RepositoryRuleDeletionType::Deletion => "deletion".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum RepositoryRuleRequiredLinearHistoryType {
  #[serde(rename = "required_linear_history")]
  RequiredLinearHistory,
}

#[cfg(any(feature = "full", feature = "repos"))]
impl ToString for RepositoryRuleRequiredLinearHistoryType {
  fn to_string(&self) -> String {
    match self {
      RepositoryRuleRequiredLinearHistoryType::RequiredLinearHistory => {
        "required_linear_history".to_string()
      }
    }
  }
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum RepositoryRuleRequiredDeploymentsType {
  #[serde(rename = "required_deployments")]
  RequiredDeployments,
}

#[cfg(any(feature = "full", feature = "repos"))]
impl ToString for RepositoryRuleRequiredDeploymentsType {
  fn to_string(&self) -> String {
    match self {
      RepositoryRuleRequiredDeploymentsType::RequiredDeployments => {
        "required_deployments".to_string()
      }
    }
  }
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum RepositoryRuleRequiredSignaturesType {
  #[serde(rename = "required_signatures")]
  RequiredSignatures,
}

#[cfg(any(feature = "full", feature = "repos"))]
impl ToString for RepositoryRuleRequiredSignaturesType {
  fn to_string(&self) -> String {
    match self {
      RepositoryRuleRequiredSignaturesType::RequiredSignatures => "required_signatures".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum RepositoryRulePullRequestType {
  #[serde(rename = "pull_request")]
  PullRequest,
}

#[cfg(any(feature = "full", feature = "repos"))]
impl ToString for RepositoryRulePullRequestType {
  fn to_string(&self) -> String {
    match self {
      RepositoryRulePullRequestType::PullRequest => "pull_request".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum RepositoryRuleRequiredStatusChecksType {
  #[serde(rename = "required_status_checks")]
  RequiredStatusChecks,
}

#[cfg(any(feature = "full", feature = "repos"))]
impl ToString for RepositoryRuleRequiredStatusChecksType {
  fn to_string(&self) -> String {
    match self {
      RepositoryRuleRequiredStatusChecksType::RequiredStatusChecks => {
        "required_status_checks".to_string()
      }
    }
  }
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum RepositoryRuleNonFastForwardType {
  #[serde(rename = "non_fast_forward")]
  NonFastForward,
}

#[cfg(any(feature = "full", feature = "repos"))]
impl ToString for RepositoryRuleNonFastForwardType {
  fn to_string(&self) -> String {
    match self {
      RepositoryRuleNonFastForwardType::NonFastForward => "non_fast_forward".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum RepositoryRuleCommitMessagePatternType {
  #[serde(rename = "commit_message_pattern")]
  CommitMessagePattern,
}

#[cfg(any(feature = "full", feature = "repos"))]
impl ToString for RepositoryRuleCommitMessagePatternType {
  fn to_string(&self) -> String {
    match self {
      RepositoryRuleCommitMessagePatternType::CommitMessagePattern => {
        "commit_message_pattern".to_string()
      }
    }
  }
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum RepositoryRuleCommitAuthorEmailPatternType {
  #[serde(rename = "commit_author_email_pattern")]
  CommitAuthorEmailPattern,
}

#[cfg(any(feature = "full", feature = "repos"))]
impl ToString for RepositoryRuleCommitAuthorEmailPatternType {
  fn to_string(&self) -> String {
    match self {
      RepositoryRuleCommitAuthorEmailPatternType::CommitAuthorEmailPattern => {
        "commit_author_email_pattern".to_string()
      }
    }
  }
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum RepositoryRuleCommitterEmailPatternType {
  #[serde(rename = "committer_email_pattern")]
  CommitterEmailPattern,
}

#[cfg(any(feature = "full", feature = "repos"))]
impl ToString for RepositoryRuleCommitterEmailPatternType {
  fn to_string(&self) -> String {
    match self {
      RepositoryRuleCommitterEmailPatternType::CommitterEmailPattern => {
        "committer_email_pattern".to_string()
      }
    }
  }
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum RepositoryRuleBranchNamePatternType {
  #[serde(rename = "branch_name_pattern")]
  BranchNamePattern,
}

#[cfg(any(feature = "full", feature = "repos"))]
impl ToString for RepositoryRuleBranchNamePatternType {
  fn to_string(&self) -> String {
    match self {
      RepositoryRuleBranchNamePatternType::BranchNamePattern => "branch_name_pattern".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum RepositoryRuleTagNamePatternType {
  #[serde(rename = "tag_name_pattern")]
  TagNamePattern,
}

#[cfg(any(feature = "full", feature = "repos"))]
impl ToString for RepositoryRuleTagNamePatternType {
  fn to_string(&self) -> String {
    match self {
      RepositoryRuleTagNamePatternType::TagNamePattern => "tag_name_pattern".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum RepositoryRuleWorkflowsType {
  #[serde(rename = "workflows")]
  Workflows,
}

#[cfg(any(feature = "full", feature = "repos"))]
impl ToString for RepositoryRuleWorkflowsType {
  fn to_string(&self) -> String {
    match self {
      RepositoryRuleWorkflowsType::Workflows => "workflows".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RepositoryRule {
  /// Only allow users with bypass permission to create matching refs.
  RepositoryRuleCreation(RepositoryRuleCreation),
  /// Only allow users with bypass permission to update matching refs.
  RepositoryRuleUpdate(RepositoryRuleUpdate),
  /// Only allow users with bypass permissions to delete matching refs.
  RepositoryRuleDeletion(RepositoryRuleDeletion),
  /// Prevent merge commits from being pushed to matching refs.
  RepositoryRuleRequiredLinearHistory(RepositoryRuleRequiredLinearHistory),
  /// Choose which environments must be successfully deployed to before refs can be pushed into a ref that matches this rule.
  RepositoryRuleRequiredDeployments(RepositoryRuleRequiredDeployments),
  /// Commits pushed to matching refs must have verified signatures.
  RepositoryRuleRequiredSignatures(RepositoryRuleRequiredSignatures),
  /// Require all commits be made to a non-target branch and submitted via a pull request before they can be merged.
  RepositoryRulePullRequest(RepositoryRulePullRequest),
  /// Choose which status checks must pass before the ref is updated. When enabled, commits must first be pushed to another ref where the checks pass.
  RepositoryRuleRequiredStatusChecks(RepositoryRuleRequiredStatusChecks),
  /// Prevent users with push access from force pushing to refs.
  RepositoryRuleNonFastForward(RepositoryRuleNonFastForward),
  /// Parameters to be used for the commit_message_pattern rule
  RepositoryRuleCommitMessagePattern(RepositoryRuleCommitMessagePattern),
  /// Parameters to be used for the commit_author_email_pattern rule
  RepositoryRuleCommitAuthorEmailPattern(RepositoryRuleCommitAuthorEmailPattern),
  /// Parameters to be used for the committer_email_pattern rule
  RepositoryRuleCommitterEmailPattern(RepositoryRuleCommitterEmailPattern),
  /// Parameters to be used for the branch_name_pattern rule
  RepositoryRuleBranchNamePattern(RepositoryRuleBranchNamePattern),
  /// Parameters to be used for the tag_name_pattern rule
  RepositoryRuleTagNamePattern(RepositoryRuleTagNamePattern),
  /// Require all changes made to a targeted branch to pass the specified workflows before they can be merged.
  RepositoryRuleWorkflows(RepositoryRuleWorkflows),
}

#[cfg(any(feature = "full", feature = "repos"))]
/// The type of the source of the ruleset
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum RepositoryRulesetSourceType {
  Repository,
  Organization,
}

#[cfg(any(feature = "full", feature = "repos"))]
impl ToString for RepositoryRulesetSourceType {
  fn to_string(&self) -> String {
    match self {
      RepositoryRulesetSourceType::Repository => "Repository".to_string(),
      RepositoryRulesetSourceType::Organization => "Organization".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "repos"))]
/// The target of the ruleset
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum RepositoryRulesetTarget {
  #[serde(rename = "branch")]
  Branch,
  #[serde(rename = "tag")]
  Tag,
}

#[cfg(any(feature = "full", feature = "repos"))]
impl ToString for RepositoryRulesetTarget {
  fn to_string(&self) -> String {
    match self {
      RepositoryRulesetTarget::Branch => "branch".to_string(),
      RepositoryRulesetTarget::Tag => "tag".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "repos"))]
/// The result of the rule evaluations for rules with the `active` and `evaluate` enforcement statuses, demonstrating whether rules would pass or fail if all rules in the rule suite were `active`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum RuleSuitesItemEvaluationResult {
  #[serde(rename = "pass")]
  Pass,
  #[serde(rename = "fail")]
  Fail,
}

#[cfg(any(feature = "full", feature = "repos"))]
impl ToString for RuleSuitesItemEvaluationResult {
  fn to_string(&self) -> String {
    match self {
      RuleSuitesItemEvaluationResult::Pass => "pass".to_string(),
      RuleSuitesItemEvaluationResult::Fail => "fail".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "repos"))]
/// The result of the rule evaluations for rules with the `active` enforcement status.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum RuleSuitesItemResult {
  #[serde(rename = "pass")]
  Pass,
  #[serde(rename = "fail")]
  Fail,
  #[serde(rename = "bypass")]
  Bypass,
}

#[cfg(any(feature = "full", feature = "repos"))]
impl ToString for RuleSuitesItemResult {
  fn to_string(&self) -> String {
    match self {
      RuleSuitesItemResult::Pass => "pass".to_string(),
      RuleSuitesItemResult::Fail => "fail".to_string(),
      RuleSuitesItemResult::Bypass => "bypass".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "repos"))]
/// The result of the rule evaluations for rules with the `active` and `evaluate` enforcement statuses, demonstrating whether rules would pass or fail if all rules in the rule suite were `active`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum RuleSuiteEvaluationResult {
  #[serde(rename = "pass")]
  Pass,
  #[serde(rename = "fail")]
  Fail,
}

#[cfg(any(feature = "full", feature = "repos"))]
impl ToString for RuleSuiteEvaluationResult {
  fn to_string(&self) -> String {
    match self {
      RuleSuiteEvaluationResult::Pass => "pass".to_string(),
      RuleSuiteEvaluationResult::Fail => "fail".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "repos"))]
/// The result of the rule evaluations for rules with the `active` enforcement status.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum RuleSuiteResult {
  #[serde(rename = "pass")]
  Pass,
  #[serde(rename = "fail")]
  Fail,
  #[serde(rename = "bypass")]
  Bypass,
}

#[cfg(any(feature = "full", feature = "repos"))]
impl ToString for RuleSuiteResult {
  fn to_string(&self) -> String {
    match self {
      RuleSuiteResult::Pass => "pass".to_string(),
      RuleSuiteResult::Fail => "fail".to_string(),
      RuleSuiteResult::Bypass => "bypass".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "repos"))]
/// The enforcement level of this rule source.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum RuleSuiteRuleEvaluationsEnforcement {
  #[serde(rename = "active")]
  Active,
  #[serde(rename = "evaluate")]
  Evaluate,
  #[serde(rename = "deleted ruleset")]
  DeletedRuleset,
}

#[cfg(any(feature = "full", feature = "repos"))]
impl ToString for RuleSuiteRuleEvaluationsEnforcement {
  fn to_string(&self) -> String {
    match self {
      RuleSuiteRuleEvaluationsEnforcement::Active => "active".to_string(),
      RuleSuiteRuleEvaluationsEnforcement::Evaluate => "evaluate".to_string(),
      RuleSuiteRuleEvaluationsEnforcement::DeletedRuleset => "deleted ruleset".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "repos"))]
/// The result of the evaluation of the individual rule.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum RuleSuiteRuleEvaluationsResult {
  #[serde(rename = "pass")]
  Pass,
  #[serde(rename = "fail")]
  Fail,
}

#[cfg(any(feature = "full", feature = "repos"))]
impl ToString for RuleSuiteRuleEvaluationsResult {
  fn to_string(&self) -> String {
    match self {
      RuleSuiteRuleEvaluationsResult::Pass => "pass".to_string(),
      RuleSuiteRuleEvaluationsResult::Fail => "fail".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "security_advisories"))]
/// The state of the user's acceptance of the credit.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum RepositoryAdvisoryCreditState {
  #[serde(rename = "accepted")]
  Accepted,
  #[serde(rename = "declined")]
  Declined,
  #[serde(rename = "pending")]
  Pending,
}

#[cfg(any(feature = "full", feature = "security_advisories"))]
impl ToString for RepositoryAdvisoryCreditState {
  fn to_string(&self) -> String {
    match self {
      RepositoryAdvisoryCreditState::Accepted => "accepted".to_string(),
      RepositoryAdvisoryCreditState::Declined => "declined".to_string(),
      RepositoryAdvisoryCreditState::Pending => "pending".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "security_advisories"))]
/// The severity of the advisory.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum RepositoryAdvisorySeverity {
  #[serde(rename = "critical")]
  Critical,
  #[serde(rename = "high")]
  High,
  #[serde(rename = "medium")]
  Medium,
  #[serde(rename = "low")]
  Low,
}

#[cfg(any(feature = "full", feature = "security_advisories"))]
impl ToString for RepositoryAdvisorySeverity {
  fn to_string(&self) -> String {
    match self {
      RepositoryAdvisorySeverity::Critical => "critical".to_string(),
      RepositoryAdvisorySeverity::High => "high".to_string(),
      RepositoryAdvisorySeverity::Medium => "medium".to_string(),
      RepositoryAdvisorySeverity::Low => "low".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "security_advisories"))]
/// The state of the advisory.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum RepositoryAdvisoryState {
  #[serde(rename = "published")]
  Published,
  #[serde(rename = "closed")]
  Closed,
  #[serde(rename = "withdrawn")]
  Withdrawn,
  #[serde(rename = "draft")]
  Draft,
  #[serde(rename = "triage")]
  Triage,
}

#[cfg(any(feature = "full", feature = "security_advisories"))]
impl ToString for RepositoryAdvisoryState {
  fn to_string(&self) -> String {
    match self {
      RepositoryAdvisoryState::Published => "published".to_string(),
      RepositoryAdvisoryState::Closed => "closed".to_string(),
      RepositoryAdvisoryState::Withdrawn => "withdrawn".to_string(),
      RepositoryAdvisoryState::Draft => "draft".to_string(),
      RepositoryAdvisoryState::Triage => "triage".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "teams"))]
/// The notification setting the team has set
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum TeamFullNotificationSetting {
  #[serde(rename = "notifications_enabled")]
  NotificationsEnabled,
  #[serde(rename = "notifications_disabled")]
  NotificationsDisabled,
}

#[cfg(any(feature = "full", feature = "teams"))]
impl ToString for TeamFullNotificationSetting {
  fn to_string(&self) -> String {
    match self {
      TeamFullNotificationSetting::NotificationsEnabled => "notifications_enabled".to_string(),
      TeamFullNotificationSetting::NotificationsDisabled => "notifications_disabled".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "teams"))]
/// The level of privacy this team should have
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum TeamFullPrivacy {
  #[serde(rename = "closed")]
  Closed,
  #[serde(rename = "secret")]
  Secret,
}

#[cfg(any(feature = "full", feature = "teams"))]
impl ToString for TeamFullPrivacy {
  fn to_string(&self) -> String {
    match self {
      TeamFullPrivacy::Closed => "closed".to_string(),
      TeamFullPrivacy::Secret => "secret".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "reactions"))]
/// The reaction to use
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum ReactionContent {
  #[serde(rename = "+1")]
  PlusOne,
  #[serde(rename = "-1")]
  MinusOne,
  #[serde(rename = "laugh")]
  Laugh,
  #[serde(rename = "confused")]
  Confused,
  #[serde(rename = "heart")]
  Heart,
  #[serde(rename = "hooray")]
  Hooray,
  #[serde(rename = "rocket")]
  Rocket,
  #[serde(rename = "eyes")]
  Eyes,
}

#[cfg(any(feature = "full", feature = "reactions"))]
impl ToString for ReactionContent {
  fn to_string(&self) -> String {
    match self {
      ReactionContent::PlusOne => "+1".to_string(),
      ReactionContent::MinusOne => "-1".to_string(),
      ReactionContent::Laugh => "laugh".to_string(),
      ReactionContent::Confused => "confused".to_string(),
      ReactionContent::Heart => "heart".to_string(),
      ReactionContent::Hooray => "hooray".to_string(),
      ReactionContent::Rocket => "rocket".to_string(),
      ReactionContent::Eyes => "eyes".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "teams"))]
/// The role of the user in the team.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum TeamMembershipRole {
  #[serde(rename = "member")]
  Member,
  #[serde(rename = "maintainer")]
  Maintainer,
}

#[cfg(any(feature = "full", feature = "teams"))]
impl ToString for TeamMembershipRole {
  fn to_string(&self) -> String {
    match self {
      TeamMembershipRole::Member => "member".to_string(),
      TeamMembershipRole::Maintainer => "maintainer".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "teams"))]
/// The state of the user's membership in the team.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum TeamMembershipState {
  #[serde(rename = "active")]
  Active,
  #[serde(rename = "pending")]
  Pending,
}

#[cfg(any(feature = "full", feature = "teams"))]
impl ToString for TeamMembershipState {
  fn to_string(&self) -> String {
    match self {
      TeamMembershipState::Active => "active".to_string(),
      TeamMembershipState::Pending => "pending".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "actions"))]
/// The outcome of the job.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum JobConclusion {
  #[serde(rename = "success")]
  Success,
  #[serde(rename = "failure")]
  Failure,
  #[serde(rename = "neutral")]
  Neutral,
  #[serde(rename = "cancelled")]
  Cancelled,
  #[serde(rename = "skipped")]
  Skipped,
  #[serde(rename = "timed_out")]
  TimedOut,
  #[serde(rename = "action_required")]
  ActionRequired,
}

#[cfg(any(feature = "full", feature = "actions"))]
impl ToString for JobConclusion {
  fn to_string(&self) -> String {
    match self {
      JobConclusion::Success => "success".to_string(),
      JobConclusion::Failure => "failure".to_string(),
      JobConclusion::Neutral => "neutral".to_string(),
      JobConclusion::Cancelled => "cancelled".to_string(),
      JobConclusion::Skipped => "skipped".to_string(),
      JobConclusion::TimedOut => "timed_out".to_string(),
      JobConclusion::ActionRequired => "action_required".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "actions"))]
/// The phase of the lifecycle that the job is currently in.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum JobStatus {
  #[serde(rename = "queued")]
  Queued,
  #[serde(rename = "in_progress")]
  InProgress,
  #[serde(rename = "completed")]
  Completed,
  #[serde(rename = "waiting")]
  Waiting,
  #[serde(rename = "requested")]
  Requested,
  #[serde(rename = "pending")]
  Pending,
}

#[cfg(any(feature = "full", feature = "actions"))]
impl ToString for JobStatus {
  fn to_string(&self) -> String {
    match self {
      JobStatus::Queued => "queued".to_string(),
      JobStatus::InProgress => "in_progress".to_string(),
      JobStatus::Completed => "completed".to_string(),
      JobStatus::Waiting => "waiting".to_string(),
      JobStatus::Requested => "requested".to_string(),
      JobStatus::Pending => "pending".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "actions"))]
/// The phase of the lifecycle that the job is currently in.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum JobStepsStatus {
  #[serde(rename = "queued")]
  Queued,
  #[serde(rename = "in_progress")]
  InProgress,
  #[serde(rename = "completed")]
  Completed,
}

#[cfg(any(feature = "full", feature = "actions"))]
impl ToString for JobStepsStatus {
  fn to_string(&self) -> String {
    match self {
      JobStepsStatus::Queued => "queued".to_string(),
      JobStepsStatus::InProgress => "in_progress".to_string(),
      JobStepsStatus::Completed => "completed".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "actions"))]
/// Defines the level of access that workflows outside of the repository have to actions and reusable workflows within the
/// repository.
///
/// `none` means the access is only possible from workflows in this repository. `user` level access allows sharing across user owned private repositories only. `organization` level access allows sharing across the organization.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum ActionsWorkflowAccessToRepositoryAccessLevel {
  #[serde(rename = "none")]
  None,
  #[serde(rename = "user")]
  User,
  #[serde(rename = "organization")]
  Organization,
}

#[cfg(any(feature = "full", feature = "actions"))]
impl ToString for ActionsWorkflowAccessToRepositoryAccessLevel {
  fn to_string(&self) -> String {
    match self {
      ActionsWorkflowAccessToRepositoryAccessLevel::None => "none".to_string(),
      ActionsWorkflowAccessToRepositoryAccessLevel::User => "user".to_string(),
      ActionsWorkflowAccessToRepositoryAccessLevel::Organization => "organization".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "actions"))]
/// Whether deployment to the environment(s) was approved or rejected or pending (with comments)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum EnvironmentApprovalsState {
  #[serde(rename = "approved")]
  Approved,
  #[serde(rename = "rejected")]
  Rejected,
  #[serde(rename = "pending")]
  Pending,
}

#[cfg(any(feature = "full", feature = "actions"))]
impl ToString for EnvironmentApprovalsState {
  fn to_string(&self) -> String {
    match self {
      EnvironmentApprovalsState::Approved => "approved".to_string(),
      EnvironmentApprovalsState::Rejected => "rejected".to_string(),
      EnvironmentApprovalsState::Pending => "pending".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "actions"))]
/// Whether to approve or reject deployment to the specified environments.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum ReviewCustomGatesStateRequiredState {
  #[serde(rename = "approved")]
  Approved,
  #[serde(rename = "rejected")]
  Rejected,
}

#[cfg(any(feature = "full", feature = "actions"))]
impl ToString for ReviewCustomGatesStateRequiredState {
  fn to_string(&self) -> String {
    match self {
      ReviewCustomGatesStateRequiredState::Approved => "approved".to_string(),
      ReviewCustomGatesStateRequiredState::Rejected => "rejected".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "actions"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PendingDeploymentReviewersReviewer {
  /// A GitHub user.
  SimpleUser(SimpleUser),
  /// Groups of organization members that gives permissions on specified repositories.
  Team(Team),
}

#[cfg(any(feature = "full", feature = "actions", feature = "repos"))]
/// The type of reviewer.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum DeploymentReviewerType {
  User,
  Team,
}

#[cfg(any(feature = "full", feature = "actions", feature = "repos"))]
impl ToString for DeploymentReviewerType {
  fn to_string(&self) -> String {
    match self {
      DeploymentReviewerType::User => "User".to_string(),
      DeploymentReviewerType::Team => "Team".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "actions"))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum WorkflowState {
  #[serde(rename = "active")]
  Active,
  #[serde(rename = "deleted")]
  Deleted,
  #[serde(rename = "disabled_fork")]
  DisabledFork,
  #[serde(rename = "disabled_inactivity")]
  DisabledInactivity,
  #[serde(rename = "disabled_manually")]
  DisabledManually,
}

#[cfg(any(feature = "full", feature = "actions"))]
impl ToString for WorkflowState {
  fn to_string(&self) -> String {
    match self {
      WorkflowState::Active => "active".to_string(),
      WorkflowState::Deleted => "deleted".to_string(),
      WorkflowState::DisabledFork => "disabled_fork".to_string(),
      WorkflowState::DisabledInactivity => "disabled_inactivity".to_string(),
      WorkflowState::DisabledManually => "disabled_manually".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "repos"))]
/// The type of the activity that was performed.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum ActivityActivityType {
  #[serde(rename = "push")]
  Push,
  #[serde(rename = "force_push")]
  ForcePush,
  #[serde(rename = "branch_deletion")]
  BranchDeletion,
  #[serde(rename = "branch_creation")]
  BranchCreation,
  #[serde(rename = "pr_merge")]
  PrMerge,
  #[serde(rename = "merge_queue_merge")]
  MergeQueueMerge,
}

#[cfg(any(feature = "full", feature = "repos"))]
impl ToString for ActivityActivityType {
  fn to_string(&self) -> String {
    match self {
      ActivityActivityType::Push => "push".to_string(),
      ActivityActivityType::ForcePush => "force_push".to_string(),
      ActivityActivityType::BranchDeletion => "branch_deletion".to_string(),
      ActivityActivityType::BranchCreation => "branch_creation".to_string(),
      ActivityActivityType::PrMerge => "pr_merge".to_string(),
      ActivityActivityType::MergeQueueMerge => "merge_queue_merge".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "repos", feature = "pulls"))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum DiffEntryStatus {
  #[serde(rename = "added")]
  Added,
  #[serde(rename = "removed")]
  Removed,
  #[serde(rename = "modified")]
  Modified,
  #[serde(rename = "renamed")]
  Renamed,
  #[serde(rename = "copied")]
  Copied,
  #[serde(rename = "changed")]
  Changed,
  #[serde(rename = "unchanged")]
  Unchanged,
}

#[cfg(any(feature = "full", feature = "repos", feature = "pulls"))]
impl ToString for DiffEntryStatus {
  fn to_string(&self) -> String {
    match self {
      DiffEntryStatus::Added => "added".to_string(),
      DiffEntryStatus::Removed => "removed".to_string(),
      DiffEntryStatus::Modified => "modified".to_string(),
      DiffEntryStatus::Renamed => "renamed".to_string(),
      DiffEntryStatus::Copied => "copied".to_string(),
      DiffEntryStatus::Changed => "changed".to_string(),
      DiffEntryStatus::Unchanged => "unchanged".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "checks"))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum CheckRunConclusion {
  #[serde(rename = "success")]
  Success,
  #[serde(rename = "failure")]
  Failure,
  #[serde(rename = "neutral")]
  Neutral,
  #[serde(rename = "cancelled")]
  Cancelled,
  #[serde(rename = "skipped")]
  Skipped,
  #[serde(rename = "timed_out")]
  TimedOut,
  #[serde(rename = "action_required")]
  ActionRequired,
}

#[cfg(any(feature = "full", feature = "checks"))]
impl ToString for CheckRunConclusion {
  fn to_string(&self) -> String {
    match self {
      CheckRunConclusion::Success => "success".to_string(),
      CheckRunConclusion::Failure => "failure".to_string(),
      CheckRunConclusion::Neutral => "neutral".to_string(),
      CheckRunConclusion::Cancelled => "cancelled".to_string(),
      CheckRunConclusion::Skipped => "skipped".to_string(),
      CheckRunConclusion::TimedOut => "timed_out".to_string(),
      CheckRunConclusion::ActionRequired => "action_required".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "checks"))]
/// The phase of the lifecycle that the check is currently in. Statuses of waiting, requested, and pending are reserved for GitHub Actions check runs.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum CheckRunStatus {
  #[serde(rename = "queued")]
  Queued,
  #[serde(rename = "in_progress")]
  InProgress,
  #[serde(rename = "completed")]
  Completed,
  #[serde(rename = "waiting")]
  Waiting,
  #[serde(rename = "requested")]
  Requested,
  #[serde(rename = "pending")]
  Pending,
}

#[cfg(any(feature = "full", feature = "checks"))]
impl ToString for CheckRunStatus {
  fn to_string(&self) -> String {
    match self {
      CheckRunStatus::Queued => "queued".to_string(),
      CheckRunStatus::InProgress => "in_progress".to_string(),
      CheckRunStatus::Completed => "completed".to_string(),
      CheckRunStatus::Waiting => "waiting".to_string(),
      CheckRunStatus::Requested => "requested".to_string(),
      CheckRunStatus::Pending => "pending".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "checks"))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum CheckSuiteConclusion {
  #[serde(rename = "success")]
  Success,
  #[serde(rename = "failure")]
  Failure,
  #[serde(rename = "neutral")]
  Neutral,
  #[serde(rename = "cancelled")]
  Cancelled,
  #[serde(rename = "skipped")]
  Skipped,
  #[serde(rename = "timed_out")]
  TimedOut,
  #[serde(rename = "action_required")]
  ActionRequired,
  #[serde(rename = "startup_failure")]
  StartupFailure,
  #[serde(rename = "stale")]
  Stale,
}

#[cfg(any(feature = "full", feature = "checks"))]
impl ToString for CheckSuiteConclusion {
  fn to_string(&self) -> String {
    match self {
      CheckSuiteConclusion::Success => "success".to_string(),
      CheckSuiteConclusion::Failure => "failure".to_string(),
      CheckSuiteConclusion::Neutral => "neutral".to_string(),
      CheckSuiteConclusion::Cancelled => "cancelled".to_string(),
      CheckSuiteConclusion::Skipped => "skipped".to_string(),
      CheckSuiteConclusion::TimedOut => "timed_out".to_string(),
      CheckSuiteConclusion::ActionRequired => "action_required".to_string(),
      CheckSuiteConclusion::StartupFailure => "startup_failure".to_string(),
      CheckSuiteConclusion::Stale => "stale".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "checks"))]
/// The phase of the lifecycle that the check suite is currently in. Statuses of waiting, requested, and pending are reserved for GitHub Actions check suites.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum CheckSuiteStatus {
  #[serde(rename = "queued")]
  Queued,
  #[serde(rename = "in_progress")]
  InProgress,
  #[serde(rename = "completed")]
  Completed,
  #[serde(rename = "waiting")]
  Waiting,
  #[serde(rename = "requested")]
  Requested,
  #[serde(rename = "pending")]
  Pending,
}

#[cfg(any(feature = "full", feature = "checks"))]
impl ToString for CheckSuiteStatus {
  fn to_string(&self) -> String {
    match self {
      CheckSuiteStatus::Queued => "queued".to_string(),
      CheckSuiteStatus::InProgress => "in_progress".to_string(),
      CheckSuiteStatus::Completed => "completed".to_string(),
      CheckSuiteStatus::Waiting => "waiting".to_string(),
      CheckSuiteStatus::Requested => "requested".to_string(),
      CheckSuiteStatus::Pending => "pending".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "code_scanning"))]
/// The severity of the alert.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum CodeScanningAlertRuleSeverity {
  #[serde(rename = "none")]
  None,
  #[serde(rename = "note")]
  Note,
  #[serde(rename = "warning")]
  Warning,
  #[serde(rename = "error")]
  Error,
}

#[cfg(any(feature = "full", feature = "code_scanning"))]
impl ToString for CodeScanningAlertRuleSeverity {
  fn to_string(&self) -> String {
    match self {
      CodeScanningAlertRuleSeverity::None => "none".to_string(),
      CodeScanningAlertRuleSeverity::Note => "note".to_string(),
      CodeScanningAlertRuleSeverity::Warning => "warning".to_string(),
      CodeScanningAlertRuleSeverity::Error => "error".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "code_scanning"))]
/// Sets the state of the code scanning alert. You must provide `dismissed_reason` when you set the state to `dismissed`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum CodeScanningAlertSetState {
  #[serde(rename = "open")]
  Open,
  #[serde(rename = "dismissed")]
  Dismissed,
}

#[cfg(any(feature = "full", feature = "code_scanning"))]
impl ToString for CodeScanningAlertSetState {
  fn to_string(&self) -> String {
    match self {
      CodeScanningAlertSetState::Open => "open".to_string(),
      CodeScanningAlertSetState::Dismissed => "dismissed".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "code_scanning"))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum CodeScanningDefaultSetupLanguages {
  #[serde(rename = "c-cpp")]
  CCpp,
  #[serde(rename = "csharp")]
  Csharp,
  #[serde(rename = "go")]
  Go,
  #[serde(rename = "java-kotlin")]
  JavaKotlin,
  #[serde(rename = "javascript-typescript")]
  JavascriptTypescript,
  #[serde(rename = "javascript")]
  Javascript,
  #[serde(rename = "python")]
  Python,
  #[serde(rename = "ruby")]
  Ruby,
  #[serde(rename = "typescript")]
  Typescript,
  #[serde(rename = "swift")]
  Swift,
}

#[cfg(any(feature = "full", feature = "code_scanning"))]
impl ToString for CodeScanningDefaultSetupLanguages {
  fn to_string(&self) -> String {
    match self {
      CodeScanningDefaultSetupLanguages::CCpp => "c-cpp".to_string(),
      CodeScanningDefaultSetupLanguages::Csharp => "csharp".to_string(),
      CodeScanningDefaultSetupLanguages::Go => "go".to_string(),
      CodeScanningDefaultSetupLanguages::JavaKotlin => "java-kotlin".to_string(),
      CodeScanningDefaultSetupLanguages::JavascriptTypescript => {
        "javascript-typescript".to_string()
      }
      CodeScanningDefaultSetupLanguages::Javascript => "javascript".to_string(),
      CodeScanningDefaultSetupLanguages::Python => "python".to_string(),
      CodeScanningDefaultSetupLanguages::Ruby => "ruby".to_string(),
      CodeScanningDefaultSetupLanguages::Typescript => "typescript".to_string(),
      CodeScanningDefaultSetupLanguages::Swift => "swift".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "code_scanning"))]
/// CodeQL query suite to be used.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum CodeScanningDefaultSetupQuerySuite {
  #[serde(rename = "default")]
  Default,
  #[serde(rename = "extended")]
  Extended,
}

#[cfg(any(feature = "full", feature = "code_scanning"))]
impl ToString for CodeScanningDefaultSetupQuerySuite {
  fn to_string(&self) -> String {
    match self {
      CodeScanningDefaultSetupQuerySuite::Default => "default".to_string(),
      CodeScanningDefaultSetupQuerySuite::Extended => "extended".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "code_scanning"))]
/// The frequency of the periodic analysis.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum CodeScanningDefaultSetupSchedule {
  #[serde(rename = "weekly")]
  Weekly,
}

#[cfg(any(feature = "full", feature = "code_scanning"))]
impl ToString for CodeScanningDefaultSetupSchedule {
  fn to_string(&self) -> String {
    match self {
      CodeScanningDefaultSetupSchedule::Weekly => "weekly".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "code_scanning"))]
/// Code scanning default setup has been configured or not.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum CodeScanningDefaultSetupState {
  #[serde(rename = "configured")]
  Configured,
  #[serde(rename = "not-configured")]
  NotConfigured,
}

#[cfg(any(feature = "full", feature = "code_scanning"))]
impl ToString for CodeScanningDefaultSetupState {
  fn to_string(&self) -> String {
    match self {
      CodeScanningDefaultSetupState::Configured => "configured".to_string(),
      CodeScanningDefaultSetupState::NotConfigured => "not-configured".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "code_scanning"))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum CodeScanningDefaultSetupUpdateLanguages {
  #[serde(rename = "c-cpp")]
  CCpp,
  #[serde(rename = "csharp")]
  Csharp,
  #[serde(rename = "go")]
  Go,
  #[serde(rename = "java-kotlin")]
  JavaKotlin,
  #[serde(rename = "javascript-typescript")]
  JavascriptTypescript,
  #[serde(rename = "python")]
  Python,
  #[serde(rename = "ruby")]
  Ruby,
  #[serde(rename = "swift")]
  Swift,
}

#[cfg(any(feature = "full", feature = "code_scanning"))]
impl ToString for CodeScanningDefaultSetupUpdateLanguages {
  fn to_string(&self) -> String {
    match self {
      CodeScanningDefaultSetupUpdateLanguages::CCpp => "c-cpp".to_string(),
      CodeScanningDefaultSetupUpdateLanguages::Csharp => "csharp".to_string(),
      CodeScanningDefaultSetupUpdateLanguages::Go => "go".to_string(),
      CodeScanningDefaultSetupUpdateLanguages::JavaKotlin => "java-kotlin".to_string(),
      CodeScanningDefaultSetupUpdateLanguages::JavascriptTypescript => {
        "javascript-typescript".to_string()
      }
      CodeScanningDefaultSetupUpdateLanguages::Python => "python".to_string(),
      CodeScanningDefaultSetupUpdateLanguages::Ruby => "ruby".to_string(),
      CodeScanningDefaultSetupUpdateLanguages::Swift => "swift".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "code_scanning"))]
/// CodeQL query suite to be used.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum CodeScanningDefaultSetupUpdateQuerySuite {
  #[serde(rename = "default")]
  Default,
  #[serde(rename = "extended")]
  Extended,
}

#[cfg(any(feature = "full", feature = "code_scanning"))]
impl ToString for CodeScanningDefaultSetupUpdateQuerySuite {
  fn to_string(&self) -> String {
    match self {
      CodeScanningDefaultSetupUpdateQuerySuite::Default => "default".to_string(),
      CodeScanningDefaultSetupUpdateQuerySuite::Extended => "extended".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "code_scanning"))]
/// The desired state of code scanning default setup.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum CodeScanningDefaultSetupUpdateState {
  #[serde(rename = "configured")]
  Configured,
  #[serde(rename = "not-configured")]
  NotConfigured,
}

#[cfg(any(feature = "full", feature = "code_scanning"))]
impl ToString for CodeScanningDefaultSetupUpdateState {
  fn to_string(&self) -> String {
    match self {
      CodeScanningDefaultSetupUpdateState::Configured => "configured".to_string(),
      CodeScanningDefaultSetupUpdateState::NotConfigured => "not-configured".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "code_scanning"))]
/// `pending` files have not yet been processed, while `complete` means results from the SARIF have been stored. `failed` files have either not been processed at all, or could only be partially processed.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum CodeScanningSarifsStatusProcessingStatus {
  #[serde(rename = "pending")]
  Pending,
  #[serde(rename = "complete")]
  Complete,
  #[serde(rename = "failed")]
  Failed,
}

#[cfg(any(feature = "full", feature = "code_scanning"))]
impl ToString for CodeScanningSarifsStatusProcessingStatus {
  fn to_string(&self) -> String {
    match self {
      CodeScanningSarifsStatusProcessingStatus::Pending => "pending".to_string(),
      CodeScanningSarifsStatusProcessingStatus::Complete => "complete".to_string(),
      CodeScanningSarifsStatusProcessingStatus::Failed => "failed".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "repos"))]
/// The permission associated with the invitation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum RepositoryInvitationPermissions {
  #[serde(rename = "read")]
  Read,
  #[serde(rename = "write")]
  Write,
  #[serde(rename = "admin")]
  Admin,
  #[serde(rename = "triage")]
  Triage,
  #[serde(rename = "maintain")]
  Maintain,
}

#[cfg(any(feature = "full", feature = "repos"))]
impl ToString for RepositoryInvitationPermissions {
  fn to_string(&self) -> String {
    match self {
      RepositoryInvitationPermissions::Read => "read".to_string(),
      RepositoryInvitationPermissions::Write => "write".to_string(),
      RepositoryInvitationPermissions::Admin => "admin".to_string(),
      RepositoryInvitationPermissions::Triage => "triage".to_string(),
      RepositoryInvitationPermissions::Maintain => "maintain".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "repos", feature = "pulls"))]
/// The merge method to use.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum AutoMergeMergeMethod {
  #[serde(rename = "merge")]
  Merge,
  #[serde(rename = "squash")]
  Squash,
  #[serde(rename = "rebase")]
  Rebase,
}

#[cfg(any(feature = "full", feature = "repos", feature = "pulls"))]
impl ToString for AutoMergeMergeMethod {
  fn to_string(&self) -> String {
    match self {
      AutoMergeMergeMethod::Merge => "merge".to_string(),
      AutoMergeMergeMethod::Squash => "squash".to_string(),
      AutoMergeMergeMethod::Rebase => "rebase".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum CommitComparisonStatus {
  #[serde(rename = "diverged")]
  Diverged,
  #[serde(rename = "ahead")]
  Ahead,
  #[serde(rename = "behind")]
  Behind,
  #[serde(rename = "identical")]
  Identical,
}

#[cfg(any(feature = "full", feature = "repos"))]
impl ToString for CommitComparisonStatus {
  fn to_string(&self) -> String {
    match self {
      CommitComparisonStatus::Diverged => "diverged".to_string(),
      CommitComparisonStatus::Ahead => "ahead".to_string(),
      CommitComparisonStatus::Behind => "behind".to_string(),
      CommitComparisonStatus::Identical => "identical".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum ContentDirectoryItemType {
  #[serde(rename = "dir")]
  Dir,
  #[serde(rename = "file")]
  File,
  #[serde(rename = "submodule")]
  Submodule,
  #[serde(rename = "symlink")]
  Symlink,
}

#[cfg(any(feature = "full", feature = "repos"))]
impl ToString for ContentDirectoryItemType {
  fn to_string(&self) -> String {
    match self {
      ContentDirectoryItemType::Dir => "dir".to_string(),
      ContentDirectoryItemType::File => "file".to_string(),
      ContentDirectoryItemType::Submodule => "submodule".to_string(),
      ContentDirectoryItemType::Symlink => "symlink".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum ContentFileType {
  #[serde(rename = "file")]
  File,
}

#[cfg(any(feature = "full", feature = "repos"))]
impl ToString for ContentFileType {
  fn to_string(&self) -> String {
    match self {
      ContentFileType::File => "file".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum ContentSymlinkType {
  #[serde(rename = "symlink")]
  Symlink,
}

#[cfg(any(feature = "full", feature = "repos"))]
impl ToString for ContentSymlinkType {
  fn to_string(&self) -> String {
    match self {
      ContentSymlinkType::Symlink => "symlink".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum ContentSubmoduleType {
  #[serde(rename = "submodule")]
  Submodule,
}

#[cfg(any(feature = "full", feature = "repos"))]
impl ToString for ContentSubmoduleType {
  fn to_string(&self) -> String {
    match self {
      ContentSubmoduleType::Submodule => "submodule".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "dependabot"))]
/// The execution scope of the vulnerable dependency.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum DependabotAlertDependencyScope {
  #[serde(rename = "development")]
  Development,
  #[serde(rename = "runtime")]
  Runtime,
}

#[cfg(any(feature = "full", feature = "dependabot"))]
impl ToString for DependabotAlertDependencyScope {
  fn to_string(&self) -> String {
    match self {
      DependabotAlertDependencyScope::Development => "development".to_string(),
      DependabotAlertDependencyScope::Runtime => "runtime".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "dependency_graph"))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum DependencyGraphDiffItemChangeType {
  #[serde(rename = "added")]
  Added,
  #[serde(rename = "removed")]
  Removed,
}

#[cfg(any(feature = "full", feature = "dependency_graph"))]
impl ToString for DependencyGraphDiffItemChangeType {
  fn to_string(&self) -> String {
    match self {
      DependencyGraphDiffItemChangeType::Added => "added".to_string(),
      DependencyGraphDiffItemChangeType::Removed => "removed".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "dependency_graph"))]
/// Where the dependency is utilized. `development` means that the dependency is only utilized in the development environment. `runtime` means that the dependency is utilized at runtime and in the development environment.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum DependencyGraphDiffItemScope {
  #[serde(rename = "unknown")]
  Unknown,
  #[serde(rename = "runtime")]
  Runtime,
  #[serde(rename = "development")]
  Development,
}

#[cfg(any(feature = "full", feature = "dependency_graph"))]
impl ToString for DependencyGraphDiffItemScope {
  fn to_string(&self) -> String {
    match self {
      DependencyGraphDiffItemScope::Unknown => "unknown".to_string(),
      DependencyGraphDiffItemScope::Runtime => "runtime".to_string(),
      DependencyGraphDiffItemScope::Development => "development".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "repos"))]
/// The state of the status.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum DeploymentStatusState {
  #[serde(rename = "error")]
  Error,
  #[serde(rename = "failure")]
  Failure,
  #[serde(rename = "inactive")]
  Inactive,
  #[serde(rename = "pending")]
  Pending,
  #[serde(rename = "success")]
  Success,
  #[serde(rename = "queued")]
  Queued,
  #[serde(rename = "in_progress")]
  InProgress,
}

#[cfg(any(feature = "full", feature = "repos"))]
impl ToString for DeploymentStatusState {
  fn to_string(&self) -> String {
    match self {
      DeploymentStatusState::Error => "error".to_string(),
      DeploymentStatusState::Failure => "failure".to_string(),
      DeploymentStatusState::Inactive => "inactive".to_string(),
      DeploymentStatusState::Pending => "pending".to_string(),
      DeploymentStatusState::Success => "success".to_string(),
      DeploymentStatusState::Queued => "queued".to_string(),
      DeploymentStatusState::InProgress => "in_progress".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EnvironmentProtectionRulesItem2ReviewersReviewer {
  /// A GitHub user.
  SimpleUser(SimpleUser),
  /// Groups of organization members that gives permissions on specified repositories.
  Team(Team),
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EnvironmentProtectionRules {
  EnvironmentProtectionRulesItem1(EnvironmentProtectionRulesItem1),
  EnvironmentProtectionRulesItem2(EnvironmentProtectionRulesItem2),
  EnvironmentProtectionRulesItem3(EnvironmentProtectionRulesItem3),
}

#[cfg(any(feature = "full", feature = "repos"))]
/// Whether this rule targets a branch or tag.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum DeploymentBranchPolicyType {
  #[serde(rename = "branch")]
  Branch,
  #[serde(rename = "tag")]
  Tag,
}

#[cfg(any(feature = "full", feature = "repos"))]
impl ToString for DeploymentBranchPolicyType {
  fn to_string(&self) -> String {
    match self {
      DeploymentBranchPolicyType::Branch => "branch".to_string(),
      DeploymentBranchPolicyType::Tag => "tag".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "repos"))]
/// Whether this rule targets a branch or tag
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum DeploymentBranchPolicyNamePatternWithTypeType {
  #[serde(rename = "branch")]
  Branch,
  #[serde(rename = "tag")]
  Tag,
}

#[cfg(any(feature = "full", feature = "repos"))]
impl ToString for DeploymentBranchPolicyNamePatternWithTypeType {
  fn to_string(&self) -> String {
    match self {
      DeploymentBranchPolicyNamePatternWithTypeType::Branch => "branch".to_string(),
      DeploymentBranchPolicyNamePatternWithTypeType::Tag => "tag".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "migrations"))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum ImportStatus {
  #[serde(rename = "auth")]
  Auth,
  #[serde(rename = "error")]
  Error,
  #[serde(rename = "none")]
  None,
  #[serde(rename = "detecting")]
  Detecting,
  #[serde(rename = "choose")]
  Choose,
  #[serde(rename = "auth_failed")]
  AuthFailed,
  #[serde(rename = "importing")]
  Importing,
  #[serde(rename = "mapping")]
  Mapping,
  #[serde(rename = "waiting_to_push")]
  WaitingToPush,
  #[serde(rename = "pushing")]
  Pushing,
  #[serde(rename = "complete")]
  Complete,
  #[serde(rename = "setup")]
  Setup,
  #[serde(rename = "unknown")]
  Unknown,
  #[serde(rename = "detection_found_multiple")]
  DetectionFoundMultiple,
  #[serde(rename = "detection_found_nothing")]
  DetectionFoundNothing,
  #[serde(rename = "detection_needs_auth")]
  DetectionNeedsAuth,
}

#[cfg(any(feature = "full", feature = "migrations"))]
impl ToString for ImportStatus {
  fn to_string(&self) -> String {
    match self {
      ImportStatus::Auth => "auth".to_string(),
      ImportStatus::Error => "error".to_string(),
      ImportStatus::None => "none".to_string(),
      ImportStatus::Detecting => "detecting".to_string(),
      ImportStatus::Choose => "choose".to_string(),
      ImportStatus::AuthFailed => "auth_failed".to_string(),
      ImportStatus::Importing => "importing".to_string(),
      ImportStatus::Mapping => "mapping".to_string(),
      ImportStatus::WaitingToPush => "waiting_to_push".to_string(),
      ImportStatus::Pushing => "pushing".to_string(),
      ImportStatus::Complete => "complete".to_string(),
      ImportStatus::Setup => "setup".to_string(),
      ImportStatus::Unknown => "unknown".to_string(),
      ImportStatus::DetectionFoundMultiple => "detection_found_multiple".to_string(),
      ImportStatus::DetectionFoundNothing => "detection_found_nothing".to_string(),
      ImportStatus::DetectionNeedsAuth => "detection_needs_auth".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "issues"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IssueEventForIssue {
  /// Labeled Issue Event
  LabeledIssueEvent(LabeledIssueEvent),
  /// Unlabeled Issue Event
  UnlabeledIssueEvent(UnlabeledIssueEvent),
  /// Assigned Issue Event
  AssignedIssueEvent(AssignedIssueEvent),
  /// Unassigned Issue Event
  UnassignedIssueEvent(UnassignedIssueEvent),
  /// Milestoned Issue Event
  MilestonedIssueEvent(MilestonedIssueEvent),
  /// Demilestoned Issue Event
  DemilestonedIssueEvent(DemilestonedIssueEvent),
  /// Renamed Issue Event
  RenamedIssueEvent(RenamedIssueEvent),
  /// Review Requested Issue Event
  ReviewRequestedIssueEvent(ReviewRequestedIssueEvent),
  /// Review Request Removed Issue Event
  ReviewRequestRemovedIssueEvent(ReviewRequestRemovedIssueEvent),
  /// Review Dismissed Issue Event
  ReviewDismissedIssueEvent(ReviewDismissedIssueEvent),
  /// Locked Issue Event
  LockedIssueEvent(LockedIssueEvent),
  /// Added to Project Issue Event
  AddedToProjectIssueEvent(AddedToProjectIssueEvent),
  /// Moved Column in Project Issue Event
  MovedColumnInProjectIssueEvent(MovedColumnInProjectIssueEvent),
  /// Removed from Project Issue Event
  RemovedFromProjectIssueEvent(RemovedFromProjectIssueEvent),
  /// Converted Note to Issue Issue Event
  ConvertedNoteToIssueIssueEvent(ConvertedNoteToIssueIssueEvent),
}

#[cfg(any(feature = "full", feature = "issues", feature = "pulls"))]
/// The side of the diff to which the comment applies. The side of the last line of the range for a multi-line comment
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum PullRequestReviewCommentSide {
  #[serde(rename = "LEFT")]
  Left,
  #[serde(rename = "RIGHT")]
  Right,
}

#[cfg(any(feature = "full", feature = "issues", feature = "pulls"))]
impl ToString for PullRequestReviewCommentSide {
  fn to_string(&self) -> String {
    match self {
      PullRequestReviewCommentSide::Left => "LEFT".to_string(),
      PullRequestReviewCommentSide::Right => "RIGHT".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "issues", feature = "pulls"))]
/// The side of the first line of the range for a multi-line comment.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum PullRequestReviewCommentStartSide {
  #[serde(rename = "LEFT")]
  Left,
  #[serde(rename = "RIGHT")]
  Right,
}

#[cfg(any(feature = "full", feature = "issues", feature = "pulls"))]
impl ToString for PullRequestReviewCommentStartSide {
  fn to_string(&self) -> String {
    match self {
      PullRequestReviewCommentStartSide::Left => "LEFT".to_string(),
      PullRequestReviewCommentStartSide::Right => "RIGHT".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "issues", feature = "pulls"))]
/// The level at which the comment is targeted, can be a diff line or a file.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum PullRequestReviewCommentSubjectType {
  #[serde(rename = "line")]
  Line,
  #[serde(rename = "file")]
  File,
}

#[cfg(any(feature = "full", feature = "issues", feature = "pulls"))]
impl ToString for PullRequestReviewCommentSubjectType {
  fn to_string(&self) -> String {
    match self {
      PullRequestReviewCommentSubjectType::Line => "line".to_string(),
      PullRequestReviewCommentSubjectType::File => "file".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "issues"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TimelineIssueEvents {
  /// Labeled Issue Event
  LabeledIssueEvent(LabeledIssueEvent),
  /// Unlabeled Issue Event
  UnlabeledIssueEvent(UnlabeledIssueEvent),
  /// Milestoned Issue Event
  MilestonedIssueEvent(MilestonedIssueEvent),
  /// Demilestoned Issue Event
  DemilestonedIssueEvent(DemilestonedIssueEvent),
  /// Renamed Issue Event
  RenamedIssueEvent(RenamedIssueEvent),
  /// Review Requested Issue Event
  ReviewRequestedIssueEvent(ReviewRequestedIssueEvent),
  /// Review Request Removed Issue Event
  ReviewRequestRemovedIssueEvent(ReviewRequestRemovedIssueEvent),
  /// Review Dismissed Issue Event
  ReviewDismissedIssueEvent(ReviewDismissedIssueEvent),
  /// Locked Issue Event
  LockedIssueEvent(LockedIssueEvent),
  /// Added to Project Issue Event
  AddedToProjectIssueEvent(AddedToProjectIssueEvent),
  /// Moved Column in Project Issue Event
  MovedColumnInProjectIssueEvent(MovedColumnInProjectIssueEvent),
  /// Removed from Project Issue Event
  RemovedFromProjectIssueEvent(RemovedFromProjectIssueEvent),
  /// Converted Note to Issue Issue Event
  ConvertedNoteToIssueIssueEvent(ConvertedNoteToIssueIssueEvent),
  /// Timeline Comment Event
  TimelineCommentEvent(TimelineCommentEvent),
  /// Timeline Cross Referenced Event
  TimelineCrossReferencedEvent(TimelineCrossReferencedEvent),
  /// Timeline Committed Event
  TimelineCommittedEvent(TimelineCommittedEvent),
  /// Timeline Reviewed Event
  TimelineReviewedEvent(TimelineReviewedEvent),
  /// Timeline Line Commented Event
  TimelineLineCommentedEvent(TimelineLineCommentedEvent),
  /// Timeline Commit Commented Event
  TimelineCommitCommentedEvent(TimelineCommitCommentedEvent),
  /// Timeline Assigned Issue Event
  TimelineAssignedIssueEvent(TimelineAssignedIssueEvent),
  /// Timeline Unassigned Issue Event
  TimelineUnassignedIssueEvent(TimelineUnassignedIssueEvent),
  /// State Change Issue Event
  StateChangeIssueEvent(StateChangeIssueEvent),
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum MergedUpstreamMergeType {
  #[serde(rename = "merge")]
  Merge,
  #[serde(rename = "fast-forward")]
  FastForward,
  #[serde(rename = "none")]
  None,
}

#[cfg(any(feature = "full", feature = "repos"))]
impl ToString for MergedUpstreamMergeType {
  fn to_string(&self) -> String {
    match self {
      MergedUpstreamMergeType::Merge => "merge".to_string(),
      MergedUpstreamMergeType::FastForward => "fast-forward".to_string(),
      MergedUpstreamMergeType::None => "none".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "repos"))]
/// The process in which the Page will be built.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum PageBuildType {
  #[serde(rename = "legacy")]
  Legacy,
  #[serde(rename = "workflow")]
  Workflow,
}

#[cfg(any(feature = "full", feature = "repos"))]
impl ToString for PageBuildType {
  fn to_string(&self) -> String {
    match self {
      PageBuildType::Legacy => "legacy".to_string(),
      PageBuildType::Workflow => "workflow".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum PagesHttpsCertificateState {
  #[serde(rename = "new")]
  New,
  #[serde(rename = "authorization_created")]
  AuthorizationCreated,
  #[serde(rename = "authorization_pending")]
  AuthorizationPending,
  #[serde(rename = "authorized")]
  Authorized,
  #[serde(rename = "authorization_revoked")]
  AuthorizationRevoked,
  #[serde(rename = "issued")]
  Issued,
  #[serde(rename = "uploaded")]
  Uploaded,
  #[serde(rename = "approved")]
  Approved,
  #[serde(rename = "errored")]
  Errored,
  #[serde(rename = "bad_authz")]
  BadAuthz,
  #[serde(rename = "destroy_pending")]
  DestroyPending,
  #[serde(rename = "dns_changed")]
  DnsChanged,
}

#[cfg(any(feature = "full", feature = "repos"))]
impl ToString for PagesHttpsCertificateState {
  fn to_string(&self) -> String {
    match self {
      PagesHttpsCertificateState::New => "new".to_string(),
      PagesHttpsCertificateState::AuthorizationCreated => "authorization_created".to_string(),
      PagesHttpsCertificateState::AuthorizationPending => "authorization_pending".to_string(),
      PagesHttpsCertificateState::Authorized => "authorized".to_string(),
      PagesHttpsCertificateState::AuthorizationRevoked => "authorization_revoked".to_string(),
      PagesHttpsCertificateState::Issued => "issued".to_string(),
      PagesHttpsCertificateState::Uploaded => "uploaded".to_string(),
      PagesHttpsCertificateState::Approved => "approved".to_string(),
      PagesHttpsCertificateState::Errored => "errored".to_string(),
      PagesHttpsCertificateState::BadAuthz => "bad_authz".to_string(),
      PagesHttpsCertificateState::DestroyPending => "destroy_pending".to_string(),
      PagesHttpsCertificateState::DnsChanged => "dns_changed".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "repos"))]
/// The state if the domain is verified
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum PageProtectedDomainState {
  #[serde(rename = "pending")]
  Pending,
  #[serde(rename = "verified")]
  Verified,
  #[serde(rename = "unverified")]
  Unverified,
}

#[cfg(any(feature = "full", feature = "repos"))]
impl ToString for PageProtectedDomainState {
  fn to_string(&self) -> String {
    match self {
      PageProtectedDomainState::Pending => "pending".to_string(),
      PageProtectedDomainState::Verified => "verified".to_string(),
      PageProtectedDomainState::Unverified => "unverified".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "repos"))]
/// The status of the most recent build of the Page.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum PageStatus {
  #[serde(rename = "built")]
  Built,
  #[serde(rename = "building")]
  Building,
  #[serde(rename = "errored")]
  Errored,
}

#[cfg(any(feature = "full", feature = "repos"))]
impl ToString for PageStatus {
  fn to_string(&self) -> String {
    match self {
      PageStatus::Built => "built".to_string(),
      PageStatus::Building => "building".to_string(),
      PageStatus::Errored => "errored".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "repos"))]
/// The current status of the deployment.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum PagesDeploymentStatusStatus {
  #[serde(rename = "deployment_in_progress")]
  DeploymentInProgress,
  #[serde(rename = "syncing_files")]
  SyncingFiles,
  #[serde(rename = "finished_file_sync")]
  FinishedFileSync,
  #[serde(rename = "updating_pages")]
  UpdatingPages,
  #[serde(rename = "purging_cdn")]
  PurgingCdn,
  #[serde(rename = "deployment_cancelled")]
  DeploymentCancelled,
  #[serde(rename = "deployment_failed")]
  DeploymentFailed,
  #[serde(rename = "deployment_content_failed")]
  DeploymentContentFailed,
  #[serde(rename = "deployment_attempt_error")]
  DeploymentAttemptError,
  #[serde(rename = "deployment_lost")]
  DeploymentLost,
  #[serde(rename = "succeed")]
  Succeed,
}

#[cfg(any(feature = "full", feature = "repos"))]
impl ToString for PagesDeploymentStatusStatus {
  fn to_string(&self) -> String {
    match self {
      PagesDeploymentStatusStatus::DeploymentInProgress => "deployment_in_progress".to_string(),
      PagesDeploymentStatusStatus::SyncingFiles => "syncing_files".to_string(),
      PagesDeploymentStatusStatus::FinishedFileSync => "finished_file_sync".to_string(),
      PagesDeploymentStatusStatus::UpdatingPages => "updating_pages".to_string(),
      PagesDeploymentStatusStatus::PurgingCdn => "purging_cdn".to_string(),
      PagesDeploymentStatusStatus::DeploymentCancelled => "deployment_cancelled".to_string(),
      PagesDeploymentStatusStatus::DeploymentFailed => "deployment_failed".to_string(),
      PagesDeploymentStatusStatus::DeploymentContentFailed => {
        "deployment_content_failed".to_string()
      }
      PagesDeploymentStatusStatus::DeploymentAttemptError => "deployment_attempt_error".to_string(),
      PagesDeploymentStatusStatus::DeploymentLost => "deployment_lost".to_string(),
      PagesDeploymentStatusStatus::Succeed => "succeed".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "pulls"))]
/// State of this Pull Request. Either `open` or `closed`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum PullRequestState {
  #[serde(rename = "open")]
  Open,
  #[serde(rename = "closed")]
  Closed,
}

#[cfg(any(feature = "full", feature = "pulls"))]
impl ToString for PullRequestState {
  fn to_string(&self) -> String {
    match self {
      PullRequestState::Open => "open".to_string(),
      PullRequestState::Closed => "closed".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "pulls"))]
/// The side of the first line of the range for a multi-line comment.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum ReviewCommentSide {
  #[serde(rename = "LEFT")]
  Left,
  #[serde(rename = "RIGHT")]
  Right,
}

#[cfg(any(feature = "full", feature = "pulls"))]
impl ToString for ReviewCommentSide {
  fn to_string(&self) -> String {
    match self {
      ReviewCommentSide::Left => "LEFT".to_string(),
      ReviewCommentSide::Right => "RIGHT".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "pulls"))]
/// The side of the first line of the range for a multi-line comment.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum ReviewCommentStartSide {
  #[serde(rename = "LEFT")]
  Left,
  #[serde(rename = "RIGHT")]
  Right,
}

#[cfg(any(feature = "full", feature = "pulls"))]
impl ToString for ReviewCommentStartSide {
  fn to_string(&self) -> String {
    match self {
      ReviewCommentStartSide::Left => "LEFT".to_string(),
      ReviewCommentStartSide::Right => "RIGHT".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "repos"))]
/// State of the release asset.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum ReleaseAssetState {
  #[serde(rename = "uploaded")]
  Uploaded,
  #[serde(rename = "open")]
  Open,
}

#[cfg(any(feature = "full", feature = "repos"))]
impl ToString for ReleaseAssetState {
  fn to_string(&self) -> String {
    match self {
      ReleaseAssetState::Uploaded => "uploaded".to_string(),
      ReleaseAssetState::Open => "open".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "repos"))]
/// The type of source for the ruleset that includes this rule.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum RepositoryRuleRulesetInfoRulesetSourceType {
  Repository,
  Organization,
}

#[cfg(any(feature = "full", feature = "repos"))]
impl ToString for RepositoryRuleRulesetInfoRulesetSourceType {
  fn to_string(&self) -> String {
    match self {
      RepositoryRuleRulesetInfoRulesetSourceType::Repository => "Repository".to_string(),
      RepositoryRuleRulesetInfoRulesetSourceType::Organization => "Organization".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RepositoryRuleDetailedItem1 {
  /// Only allow users with bypass permission to create matching refs.
  RepositoryRuleCreation(RepositoryRuleCreation),
  /// User-defined metadata to store domain-specific information limited to 8 keys with scalar values.
  RepositoryRuleRulesetInfo(RepositoryRuleRulesetInfo),
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RepositoryRuleDetailedItem2 {
  /// Only allow users with bypass permission to update matching refs.
  RepositoryRuleUpdate(RepositoryRuleUpdate),
  /// User-defined metadata to store domain-specific information limited to 8 keys with scalar values.
  RepositoryRuleRulesetInfo(RepositoryRuleRulesetInfo),
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RepositoryRuleDetailedItem3 {
  /// Only allow users with bypass permissions to delete matching refs.
  RepositoryRuleDeletion(RepositoryRuleDeletion),
  /// User-defined metadata to store domain-specific information limited to 8 keys with scalar values.
  RepositoryRuleRulesetInfo(RepositoryRuleRulesetInfo),
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RepositoryRuleDetailedItem4 {
  /// Prevent merge commits from being pushed to matching refs.
  RepositoryRuleRequiredLinearHistory(RepositoryRuleRequiredLinearHistory),
  /// User-defined metadata to store domain-specific information limited to 8 keys with scalar values.
  RepositoryRuleRulesetInfo(RepositoryRuleRulesetInfo),
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RepositoryRuleDetailedItem5 {
  /// Choose which environments must be successfully deployed to before refs can be pushed into a ref that matches this rule.
  RepositoryRuleRequiredDeployments(RepositoryRuleRequiredDeployments),
  /// User-defined metadata to store domain-specific information limited to 8 keys with scalar values.
  RepositoryRuleRulesetInfo(RepositoryRuleRulesetInfo),
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RepositoryRuleDetailedItem6 {
  /// Commits pushed to matching refs must have verified signatures.
  RepositoryRuleRequiredSignatures(RepositoryRuleRequiredSignatures),
  /// User-defined metadata to store domain-specific information limited to 8 keys with scalar values.
  RepositoryRuleRulesetInfo(RepositoryRuleRulesetInfo),
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RepositoryRuleDetailedItem7 {
  /// Require all commits be made to a non-target branch and submitted via a pull request before they can be merged.
  RepositoryRulePullRequest(RepositoryRulePullRequest),
  /// User-defined metadata to store domain-specific information limited to 8 keys with scalar values.
  RepositoryRuleRulesetInfo(RepositoryRuleRulesetInfo),
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RepositoryRuleDetailedItem8 {
  /// Choose which status checks must pass before the ref is updated. When enabled, commits must first be pushed to another ref where the checks pass.
  RepositoryRuleRequiredStatusChecks(RepositoryRuleRequiredStatusChecks),
  /// User-defined metadata to store domain-specific information limited to 8 keys with scalar values.
  RepositoryRuleRulesetInfo(RepositoryRuleRulesetInfo),
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RepositoryRuleDetailedItem9 {
  /// Prevent users with push access from force pushing to refs.
  RepositoryRuleNonFastForward(RepositoryRuleNonFastForward),
  /// User-defined metadata to store domain-specific information limited to 8 keys with scalar values.
  RepositoryRuleRulesetInfo(RepositoryRuleRulesetInfo),
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RepositoryRuleDetailedItem10 {
  /// Parameters to be used for the commit_message_pattern rule
  RepositoryRuleCommitMessagePattern(RepositoryRuleCommitMessagePattern),
  /// User-defined metadata to store domain-specific information limited to 8 keys with scalar values.
  RepositoryRuleRulesetInfo(RepositoryRuleRulesetInfo),
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RepositoryRuleDetailedItem11 {
  /// Parameters to be used for the commit_author_email_pattern rule
  RepositoryRuleCommitAuthorEmailPattern(RepositoryRuleCommitAuthorEmailPattern),
  /// User-defined metadata to store domain-specific information limited to 8 keys with scalar values.
  RepositoryRuleRulesetInfo(RepositoryRuleRulesetInfo),
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RepositoryRuleDetailedItem12 {
  /// Parameters to be used for the committer_email_pattern rule
  RepositoryRuleCommitterEmailPattern(RepositoryRuleCommitterEmailPattern),
  /// User-defined metadata to store domain-specific information limited to 8 keys with scalar values.
  RepositoryRuleRulesetInfo(RepositoryRuleRulesetInfo),
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RepositoryRuleDetailedItem13 {
  /// Parameters to be used for the branch_name_pattern rule
  RepositoryRuleBranchNamePattern(RepositoryRuleBranchNamePattern),
  /// User-defined metadata to store domain-specific information limited to 8 keys with scalar values.
  RepositoryRuleRulesetInfo(RepositoryRuleRulesetInfo),
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RepositoryRuleDetailedItem14 {
  /// Parameters to be used for the tag_name_pattern rule
  RepositoryRuleTagNamePattern(RepositoryRuleTagNamePattern),
  /// User-defined metadata to store domain-specific information limited to 8 keys with scalar values.
  RepositoryRuleRulesetInfo(RepositoryRuleRulesetInfo),
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RepositoryRuleDetailedItem15 {
  /// Require all changes made to a targeted branch to pass the specified workflows before they can be merged.
  RepositoryRuleWorkflows(RepositoryRuleWorkflows),
  /// User-defined metadata to store domain-specific information limited to 8 keys with scalar values.
  RepositoryRuleRulesetInfo(RepositoryRuleRulesetInfo),
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RepositoryRuleDetailed {
  RepositoryRuleDetailedItem1(RepositoryRuleDetailedItem1),
  RepositoryRuleDetailedItem2(RepositoryRuleDetailedItem2),
  RepositoryRuleDetailedItem3(RepositoryRuleDetailedItem3),
  RepositoryRuleDetailedItem4(RepositoryRuleDetailedItem4),
  RepositoryRuleDetailedItem5(RepositoryRuleDetailedItem5),
  RepositoryRuleDetailedItem6(RepositoryRuleDetailedItem6),
  RepositoryRuleDetailedItem7(RepositoryRuleDetailedItem7),
  RepositoryRuleDetailedItem8(RepositoryRuleDetailedItem8),
  RepositoryRuleDetailedItem9(RepositoryRuleDetailedItem9),
  RepositoryRuleDetailedItem10(RepositoryRuleDetailedItem10),
  RepositoryRuleDetailedItem11(RepositoryRuleDetailedItem11),
  RepositoryRuleDetailedItem12(RepositoryRuleDetailedItem12),
  RepositoryRuleDetailedItem13(RepositoryRuleDetailedItem13),
  RepositoryRuleDetailedItem14(RepositoryRuleDetailedItem14),
  RepositoryRuleDetailedItem15(RepositoryRuleDetailedItem15),
}

#[cfg(any(feature = "full", feature = "secret_scanning"))]
/// The token status as of the latest validity check.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum SecretScanningAlertValidity {
  #[serde(rename = "active")]
  Active,
  #[serde(rename = "inactive")]
  Inactive,
  #[serde(rename = "unknown")]
  Unknown,
}

#[cfg(any(feature = "full", feature = "secret_scanning"))]
impl ToString for SecretScanningAlertValidity {
  fn to_string(&self) -> String {
    match self {
      SecretScanningAlertValidity::Active => "active".to_string(),
      SecretScanningAlertValidity::Inactive => "inactive".to_string(),
      SecretScanningAlertValidity::Unknown => "unknown".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "secret_scanning"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SecretScanningLocationDetails {
  /// Represents a 'commit' secret scanning location type. This location type shows that a secret was detected inside a commit to a repository.
  SecretScanningLocationCommit(SecretScanningLocationCommit),
  /// Represents a 'wiki_commit' secret scanning location type. This location type shows that a secret was detected inside a commit to a repository wiki.
  SecretScanningLocationWikiCommit(SecretScanningLocationWikiCommit),
  /// Represents an 'issue_title' secret scanning location type. This location type shows that a secret was detected in the title of an issue.
  SecretScanningLocationIssueTitle(SecretScanningLocationIssueTitle),
  /// Represents an 'issue_body' secret scanning location type. This location type shows that a secret was detected in the body of an issue.
  SecretScanningLocationIssueBody(SecretScanningLocationIssueBody),
  /// Represents an 'issue_comment' secret scanning location type. This location type shows that a secret was detected in a comment on an issue.
  SecretScanningLocationIssueComment(SecretScanningLocationIssueComment),
  /// Represents a 'discussion_title' secret scanning location type. This location type shows that a secret was detected in the title of a discussion.
  SecretScanningLocationDiscussionTitle(SecretScanningLocationDiscussionTitle),
  /// Represents a 'discussion_body' secret scanning location type. This location type shows that a secret was detected in the body of a discussion.
  SecretScanningLocationDiscussionBody(SecretScanningLocationDiscussionBody),
  /// Represents a 'discussion_comment' secret scanning location type. This location type shows that a secret was detected in a comment on a discussion.
  SecretScanningLocationDiscussionComment(SecretScanningLocationDiscussionComment),
  /// Represents a 'pull_request_title' secret scanning location type. This location type shows that a secret was detected in the title of a pull request.
  SecretScanningLocationPullRequestTitle(SecretScanningLocationPullRequestTitle),
  /// Represents a 'pull_request_body' secret scanning location type. This location type shows that a secret was detected in the body of a pull request.
  SecretScanningLocationPullRequestBody(SecretScanningLocationPullRequestBody),
  /// Represents a 'pull_request_comment' secret scanning location type. This location type shows that a secret was detected in a comment on a pull request.
  SecretScanningLocationPullRequestComment(SecretScanningLocationPullRequestComment),
  /// Represents a 'pull_request_review' secret scanning location type. This location type shows that a secret was detected in a review on a pull request.
  SecretScanningLocationPullRequestReview(SecretScanningLocationPullRequestReview),
  /// Represents a 'pull_request_review_comment' secret scanning location type. This location type shows that a secret was detected in a review comment on a pull request.
  SecretScanningLocationPullRequestReviewComment(SecretScanningLocationPullRequestReviewComment),
}

#[cfg(any(feature = "full", feature = "secret_scanning"))]
/// The location type. Because secrets may be found in different types of resources (ie. code, comments, issues, pull requests, discussions), this field identifies the type of resource where the secret was found.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum SecretScanningLocationType {
  #[serde(rename = "commit")]
  Commit,
  #[serde(rename = "wiki_commit")]
  WikiCommit,
  #[serde(rename = "issue_title")]
  IssueTitle,
  #[serde(rename = "issue_body")]
  IssueBody,
  #[serde(rename = "issue_comment")]
  IssueComment,
  #[serde(rename = "discussion_title")]
  DiscussionTitle,
  #[serde(rename = "discussion_body")]
  DiscussionBody,
  #[serde(rename = "discussion_comment")]
  DiscussionComment,
  #[serde(rename = "pull_request_title")]
  PullRequestTitle,
  #[serde(rename = "pull_request_body")]
  PullRequestBody,
  #[serde(rename = "pull_request_comment")]
  PullRequestComment,
  #[serde(rename = "pull_request_review")]
  PullRequestReview,
  #[serde(rename = "pull_request_review_comment")]
  PullRequestReviewComment,
}

#[cfg(any(feature = "full", feature = "secret_scanning"))]
impl ToString for SecretScanningLocationType {
  fn to_string(&self) -> String {
    match self {
      SecretScanningLocationType::Commit => "commit".to_string(),
      SecretScanningLocationType::WikiCommit => "wiki_commit".to_string(),
      SecretScanningLocationType::IssueTitle => "issue_title".to_string(),
      SecretScanningLocationType::IssueBody => "issue_body".to_string(),
      SecretScanningLocationType::IssueComment => "issue_comment".to_string(),
      SecretScanningLocationType::DiscussionTitle => "discussion_title".to_string(),
      SecretScanningLocationType::DiscussionBody => "discussion_body".to_string(),
      SecretScanningLocationType::DiscussionComment => "discussion_comment".to_string(),
      SecretScanningLocationType::PullRequestTitle => "pull_request_title".to_string(),
      SecretScanningLocationType::PullRequestBody => "pull_request_body".to_string(),
      SecretScanningLocationType::PullRequestComment => "pull_request_comment".to_string(),
      SecretScanningLocationType::PullRequestReview => "pull_request_review".to_string(),
      SecretScanningLocationType::PullRequestReviewComment => {
        "pull_request_review_comment".to_string()
      }
    }
  }
}

#[cfg(any(feature = "full", feature = "security_advisories"))]
/// The severity of the advisory. You must choose between setting this field or `cvss_vector_string`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum RepositoryAdvisoryCreateSeverity {
  #[serde(rename = "critical")]
  Critical,
  #[serde(rename = "high")]
  High,
  #[serde(rename = "medium")]
  Medium,
  #[serde(rename = "low")]
  Low,
}

#[cfg(any(feature = "full", feature = "security_advisories"))]
impl ToString for RepositoryAdvisoryCreateSeverity {
  fn to_string(&self) -> String {
    match self {
      RepositoryAdvisoryCreateSeverity::Critical => "critical".to_string(),
      RepositoryAdvisoryCreateSeverity::High => "high".to_string(),
      RepositoryAdvisoryCreateSeverity::Medium => "medium".to_string(),
      RepositoryAdvisoryCreateSeverity::Low => "low".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "security_advisories"))]
/// The severity of the advisory. You must choose between setting this field or `cvss_vector_string`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum PrivateVulnerabilityReportCreateSeverity {
  #[serde(rename = "critical")]
  Critical,
  #[serde(rename = "high")]
  High,
  #[serde(rename = "medium")]
  Medium,
  #[serde(rename = "low")]
  Low,
}

#[cfg(any(feature = "full", feature = "security_advisories"))]
impl ToString for PrivateVulnerabilityReportCreateSeverity {
  fn to_string(&self) -> String {
    match self {
      PrivateVulnerabilityReportCreateSeverity::Critical => "critical".to_string(),
      PrivateVulnerabilityReportCreateSeverity::High => "high".to_string(),
      PrivateVulnerabilityReportCreateSeverity::Medium => "medium".to_string(),
      PrivateVulnerabilityReportCreateSeverity::Low => "low".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "security_advisories"))]
/// The severity of the advisory. You must choose between setting this field or `cvss_vector_string`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum RepositoryAdvisoryUpdateSeverity {
  #[serde(rename = "critical")]
  Critical,
  #[serde(rename = "high")]
  High,
  #[serde(rename = "medium")]
  Medium,
  #[serde(rename = "low")]
  Low,
}

#[cfg(any(feature = "full", feature = "security_advisories"))]
impl ToString for RepositoryAdvisoryUpdateSeverity {
  fn to_string(&self) -> String {
    match self {
      RepositoryAdvisoryUpdateSeverity::Critical => "critical".to_string(),
      RepositoryAdvisoryUpdateSeverity::High => "high".to_string(),
      RepositoryAdvisoryUpdateSeverity::Medium => "medium".to_string(),
      RepositoryAdvisoryUpdateSeverity::Low => "low".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "security_advisories"))]
/// The state of the advisory.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum RepositoryAdvisoryUpdateState {
  #[serde(rename = "published")]
  Published,
  #[serde(rename = "closed")]
  Closed,
  #[serde(rename = "draft")]
  Draft,
}

#[cfg(any(feature = "full", feature = "security_advisories"))]
impl ToString for RepositoryAdvisoryUpdateState {
  fn to_string(&self) -> String {
    match self {
      RepositoryAdvisoryUpdateState::Published => "published".to_string(),
      RepositoryAdvisoryUpdateState::Closed => "closed".to_string(),
      RepositoryAdvisoryUpdateState::Draft => "draft".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "codespaces"))]
/// The initally assigned location of a new codespace.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum CodespaceWithFullRepositoryLocation {
  EastUs,
  SouthEastAsia,
  WestEurope,
  WestUs2,
}

#[cfg(any(feature = "full", feature = "codespaces"))]
impl ToString for CodespaceWithFullRepositoryLocation {
  fn to_string(&self) -> String {
    match self {
      CodespaceWithFullRepositoryLocation::EastUs => "EastUs".to_string(),
      CodespaceWithFullRepositoryLocation::SouthEastAsia => "SouthEastAsia".to_string(),
      CodespaceWithFullRepositoryLocation::WestEurope => "WestEurope".to_string(),
      CodespaceWithFullRepositoryLocation::WestUs2 => "WestUs2".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "codespaces"))]
/// State of this codespace.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum CodespaceWithFullRepositoryState {
  Unknown,
  Created,
  Queued,
  Provisioning,
  Available,
  Awaiting,
  Unavailable,
  Deleted,
  Moved,
  Shutdown,
  Archived,
  Starting,
  ShuttingDown,
  Failed,
  Exporting,
  Updating,
  Rebuilding,
}

#[cfg(any(feature = "full", feature = "codespaces"))]
impl ToString for CodespaceWithFullRepositoryState {
  fn to_string(&self) -> String {
    match self {
      CodespaceWithFullRepositoryState::Unknown => "Unknown".to_string(),
      CodespaceWithFullRepositoryState::Created => "Created".to_string(),
      CodespaceWithFullRepositoryState::Queued => "Queued".to_string(),
      CodespaceWithFullRepositoryState::Provisioning => "Provisioning".to_string(),
      CodespaceWithFullRepositoryState::Available => "Available".to_string(),
      CodespaceWithFullRepositoryState::Awaiting => "Awaiting".to_string(),
      CodespaceWithFullRepositoryState::Unavailable => "Unavailable".to_string(),
      CodespaceWithFullRepositoryState::Deleted => "Deleted".to_string(),
      CodespaceWithFullRepositoryState::Moved => "Moved".to_string(),
      CodespaceWithFullRepositoryState::Shutdown => "Shutdown".to_string(),
      CodespaceWithFullRepositoryState::Archived => "Archived".to_string(),
      CodespaceWithFullRepositoryState::Starting => "Starting".to_string(),
      CodespaceWithFullRepositoryState::ShuttingDown => "ShuttingDown".to_string(),
      CodespaceWithFullRepositoryState::Failed => "Failed".to_string(),
      CodespaceWithFullRepositoryState::Exporting => "Exporting".to_string(),
      CodespaceWithFullRepositoryState::Updating => "Updating".to_string(),
      CodespaceWithFullRepositoryState::Rebuilding => "Rebuilding".to_string(),
    }
  }
}

#[cfg(any(feature = "full", feature = "meta"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct Root {
  pub authorizations_url: String,
  pub code_search_url: String,
  pub commit_search_url: String,
  pub current_user_authorizations_html_url: String,
  pub current_user_repositories_url: String,
  pub current_user_url: String,
  pub emails_url: String,
  pub emojis_url: String,
  pub events_url: String,
  pub feeds_url: String,
  pub followers_url: String,
  pub following_url: String,
  pub gists_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub hub_url: Option<String>,
  pub issue_search_url: String,
  pub issues_url: String,
  pub keys_url: String,
  pub label_search_url: String,
  pub notifications_url: String,
  pub organization_repositories_url: String,
  pub organization_teams_url: String,
  pub organization_url: String,
  pub public_gists_url: String,
  pub rate_limit_url: String,
  pub repository_search_url: String,
  pub repository_url: String,
  pub starred_gists_url: String,
  pub starred_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub topic_search_url: Option<String>,
  pub user_organizations_url: String,
  pub user_repositories_url: String,
  pub user_search_url: String,
  pub user_url: String,
}

#[cfg(any(
  feature = "full",
  feature = "security_advisories",
  feature = "apps",
  feature = "dependabot",
  feature = "secret_scanning",
  feature = "activity",
  feature = "gists",
  feature = "issues",
  feature = "actions",
  feature = "orgs",
  feature = "code_scanning",
  feature = "codespaces",
  feature = "copilot",
  feature = "packages",
  feature = "migrations",
  feature = "projects",
  feature = "repos",
  feature = "teams",
  feature = "reactions",
  feature = "checks",
  feature = "pulls",
  feature = "search",
  feature = "users"
))]
/// A GitHub user.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct SimpleUser {
  pub avatar_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub email: Option<String>,
  pub events_url: String,
  pub followers_url: String,
  pub following_url: String,
  pub gists_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub gravatar_id: Option<String>,
  pub html_url: String,
  pub id: i64,
  pub login: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub name: Option<String>,
  pub node_id: String,
  pub organizations_url: String,
  pub received_events_url: String,
  pub repos_url: String,
  pub site_admin: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub starred_at: Option<String>,
  pub starred_url: String,
  pub subscriptions_url: String,
  #[serde(rename = "type")]
  pub type_: String,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "security_advisories"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct GlobalAdvisoryCredits {
  #[serde(rename = "type")]
  pub type_: SecurityAdvisoryCreditTypes,
  pub user: SimpleUser,
}

#[cfg(any(feature = "full", feature = "security_advisories"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct GlobalAdvisoryCvss {
  /// The CVSS score.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub score: Option<f64>,
  /// The CVSS vector.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub vector_string: Option<String>,
}

#[cfg(any(feature = "full", feature = "security_advisories"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct GlobalAdvisoryCwes {
  /// The Common Weakness Enumeration (CWE) identifier.
  pub cwe_id: String,
  /// The name of the CWE.
  pub name: String,
}

#[cfg(any(feature = "full", feature = "security_advisories"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct GlobalAdvisoryIdentifiers {
  /// The type of identifier.
  #[serde(rename = "type")]
  pub type_: IdentifiersType,
  /// The identifier value.
  pub value: String,
}

#[cfg(any(feature = "full", feature = "security_advisories"))]
/// The name of the package affected by the vulnerability.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct GlobalAdvisoryVulnerabilitiesPackage {
  pub ecosystem: SecurityAdvisoryEcosystems,
  /// The unique package name within its ecosystem.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub name: Option<String>,
}

#[cfg(any(feature = "full", feature = "security_advisories"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct GlobalAdvisoryVulnerabilities {
  /// The package version that resolve the vulnerability.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub first_patched_version: Option<String>,
  /// The name of the package affected by the vulnerability.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub package: Option<GlobalAdvisoryVulnerabilitiesPackage>,
  /// The functions in the package that are affected by the vulnerability.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub vulnerable_functions: Option<Vec<String>>,
  /// The range of the package versions affected by the vulnerability.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub vulnerable_version_range: Option<String>,
}

#[cfg(any(feature = "full", feature = "security_advisories"))]
/// A GitHub Security Advisory.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct GlobalAdvisory {
  /// The users who contributed to the advisory.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub credits: Option<Vec<GlobalAdvisoryCredits>>,
  /// The Common Vulnerabilities and Exposures (CVE) ID.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub cve_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub cvss: Option<GlobalAdvisoryCvss>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub cwes: Option<Vec<GlobalAdvisoryCwes>>,
  /// A detailed description of what the advisory entails.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub description: Option<String>,
  /// The GitHub Security Advisory ID.
  pub ghsa_id: String,
  /// The date and time of when the advisory was reviewed by GitHub, in ISO 8601 format.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub github_reviewed_at: Option<String>,
  /// The URL for the advisory.
  pub html_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub identifiers: Option<Vec<GlobalAdvisoryIdentifiers>>,
  /// The date and time when the advisory was published in the National Vulnerability Database, in ISO 8601 format.
  /// This field is only populated when the advisory is imported from the National Vulnerability Database.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub nvd_published_at: Option<String>,
  /// The date and time of when the advisory was published, in ISO 8601 format.
  pub published_at: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub references: Option<Vec<String>>,
  /// The API URL for the repository advisory.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub repository_advisory_url: Option<String>,
  /// The severity of the advisory.
  pub severity: GlobalAdvisorySeverity,
  /// The URL of the advisory's source code.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub source_code_location: Option<String>,
  /// A short summary of the advisory.
  pub summary: String,
  /// The type of advisory.
  #[serde(rename = "type")]
  pub type_: GlobalAdvisoryType,
  /// The date and time of when the advisory was last updated, in ISO 8601 format.
  pub updated_at: String,
  /// The API URL for the advisory.
  pub url: String,
  /// The products and respective version ranges affected by the advisory.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub vulnerabilities: Option<Vec<GlobalAdvisoryVulnerabilities>>,
  /// The date and time of when the advisory was withdrawn, in ISO 8601 format.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub withdrawn_at: Option<String>,
}

#[cfg(any(
  feature = "full",
  feature = "apps",
  feature = "activity",
  feature = "issues",
  feature = "actions",
  feature = "repos",
  feature = "checks",
  feature = "search"
))]
/// The set of permissions for the GitHub app
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct IntegrationPermissions {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub checks: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub contents: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub deployments: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub issues: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub metadata: Option<String>,
}

#[cfg(any(
  feature = "full",
  feature = "apps",
  feature = "activity",
  feature = "issues",
  feature = "actions",
  feature = "repos",
  feature = "checks",
  feature = "search"
))]
/// GitHub apps are a new way to extend GitHub. They can be installed directly on organizations and user accounts and granted access to specific repositories. They come with granular permissions and built-in webhooks. GitHub apps are first class actors within GitHub.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct Integration {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub client_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub client_secret: Option<String>,
  pub created_at: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub description: Option<String>,
  /// The list of events for the GitHub app
  pub events: Vec<String>,
  pub external_url: String,
  pub html_url: String,
  /// Unique identifier of the GitHub app
  pub id: i64,
  /// The number of installations associated with the GitHub app
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub installations_count: Option<i64>,
  /// The name of the GitHub app
  pub name: String,
  pub node_id: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub owner: Option<SimpleUser>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub pem: Option<String>,
  /// The set of permissions for the GitHub app
  pub permissions: IntegrationPermissions,
  /// The slug name of the GitHub app
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub slug: Option<String>,
  pub updated_at: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub webhook_secret: Option<String>,
}

#[cfg(any(
  feature = "full",
  feature = "apps",
  feature = "orgs",
  feature = "repos"
))]
/// Configuration object of the webhook
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct WebhookConfig {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub content_type: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub insecure_ssl: Option<StringOrNumber>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub secret: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub url: Option<String>,
}

#[cfg(any(
  feature = "full",
  feature = "apps",
  feature = "orgs",
  feature = "repos"
))]
/// Delivery made by a webhook, without request and response information.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct HookDeliveryItem {
  /// The type of activity for the event that triggered the delivery.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub action: Option<String>,
  /// Time when the webhook delivery occurred.
  pub delivered_at: String,
  /// Time spent delivering.
  pub duration: f64,
  /// The event that triggered the delivery.
  pub event: String,
  /// Unique identifier for the event (shared with all deliveries for all webhooks that subscribe to this event).
  pub guid: String,
  /// Unique identifier of the webhook delivery.
  pub id: i64,
  /// The id of the GitHub App installation associated with this event.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub installation_id: Option<i64>,
  /// Whether the webhook delivery is a redelivery.
  pub redelivery: bool,
  /// The id of the repository associated with this event.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub repository_id: Option<i64>,
  /// Describes the response returned after attempting the delivery.
  pub status: String,
  /// Status code received when delivery was made.
  pub status_code: i64,
}

#[cfg(any(
  feature = "full",
  feature = "apps",
  feature = "orgs",
  feature = "repos"
))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct HookDeliveryRequest {
  /// The request headers sent with the webhook delivery.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub headers: Option<serde_json::Value>,
  /// The webhook payload.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub payload: Option<serde_json::Value>,
}

#[cfg(any(
  feature = "full",
  feature = "apps",
  feature = "orgs",
  feature = "repos"
))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct HookDeliveryResponse {
  /// The response headers received when the delivery was made.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub headers: Option<serde_json::Value>,
  /// The response payload received.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub payload: Option<String>,
}

#[cfg(any(
  feature = "full",
  feature = "apps",
  feature = "orgs",
  feature = "repos"
))]
/// Delivery made by a webhook.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct HookDelivery {
  /// The type of activity for the event that triggered the delivery.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub action: Option<String>,
  /// Time when the delivery was delivered.
  pub delivered_at: String,
  /// Time spent delivering.
  pub duration: f64,
  /// The event that triggered the delivery.
  pub event: String,
  /// Unique identifier for the event (shared with all deliveries for all webhooks that subscribe to this event).
  pub guid: String,
  /// Unique identifier of the delivery.
  pub id: i64,
  /// The id of the GitHub App installation associated with this event.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub installation_id: Option<i64>,
  /// Whether the delivery is a redelivery.
  pub redelivery: bool,
  /// The id of the repository associated with this event.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub repository_id: Option<i64>,
  pub request: HookDeliveryRequest,
  pub response: HookDeliveryResponse,
  /// Description of the status of the attempted delivery
  pub status: String,
  /// Status code received when delivery was made.
  pub status_code: i64,
  /// The URL target of the delivery.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub url: Option<String>,
}

#[cfg(any(feature = "full", feature = "apps", feature = "orgs"))]
/// An enterprise on GitHub.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct Enterprise {
  pub avatar_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub created_at: Option<String>,
  /// A short description of the enterprise.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub description: Option<String>,
  pub html_url: String,
  /// Unique identifier of the enterprise
  pub id: i64,
  /// The name of the enterprise.
  pub name: String,
  pub node_id: String,
  /// The slug url identifier for the enterprise.
  pub slug: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub updated_at: Option<String>,
  /// The enterprise's website URL.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub website_url: Option<String>,
}

#[cfg(any(feature = "full", feature = "apps"))]
/// Request to install an integration on a target
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct IntegrationInstallationRequest {
  pub account: IntegrationInstallationRequestAccount,
  pub created_at: String,
  /// Unique identifier of the request installation.
  pub id: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub node_id: Option<String>,
  pub requester: SimpleUser,
}

#[cfg(any(feature = "full", feature = "apps", feature = "orgs"))]
/// The permissions granted to the user access token.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct AppPermissions {
  /// The level of permission to grant the access token for GitHub Actions workflows, workflow runs, and artifacts.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub actions: Option<ReadWritePermission>,
  /// The level of permission to grant the access token for repository creation, deletion, settings, teams, and collaborators creation.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub administration: Option<ReadWritePermission>,
  /// The level of permission to grant the access token for checks on code.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub checks: Option<ReadWritePermission>,
  /// The level of permission to grant the access token to create, edit, delete, and list Codespaces.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub codespaces: Option<ReadWritePermission>,
  /// The level of permission to grant the access token for repository contents, commits, branches, downloads, releases, and merges.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub contents: Option<ReadWritePermission>,
  /// The leve of permission to grant the access token to manage Dependabot secrets.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub dependabot_secrets: Option<ReadWritePermission>,
  /// The level of permission to grant the access token for deployments and deployment statuses.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub deployments: Option<ReadWritePermission>,
  /// The level of permission to grant the access token to manage the email addresses belonging to a user.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub email_addresses: Option<ReadWritePermission>,
  /// The level of permission to grant the access token for managing repository environments.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub environments: Option<ReadWritePermission>,
  /// The level of permission to grant the access token to manage the followers belonging to a user.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub followers: Option<ReadWritePermission>,
  /// The level of permission to grant the access token to manage git SSH keys.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub git_ssh_keys: Option<ReadWritePermission>,
  /// The level of permission to grant the access token to view and manage GPG keys belonging to a user.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub gpg_keys: Option<ReadWritePermission>,
  /// The level of permission to grant the access token to view and manage interaction limits on a repository.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub interaction_limits: Option<ReadWritePermission>,
  /// The level of permission to grant the access token for issues and related comments, assignees, labels, and milestones.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub issues: Option<ReadWritePermission>,
  /// The level of permission to grant the access token for organization teams and members.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub members: Option<ReadWritePermission>,
  /// The level of permission to grant the access token to search repositories, list collaborators, and access repository metadata.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub metadata: Option<ReadWritePermission>,
  /// The level of permission to grant the access token to manage access to an organization.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub organization_administration: Option<ReadWritePermission>,
  /// The level of permission to grant the access token to view and manage announcement banners for an organization.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub organization_announcement_banners: Option<ReadWritePermission>,
  /// The level of permission to grant the access token for managing access to GitHub Copilot for members of an organization with a Copilot Business subscription. This property is in beta and is subject to change.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub organization_copilot_seat_management: Option<WritePermission>,
  /// The level of permission to grant the access token for custom organization roles management.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub organization_custom_org_roles: Option<ReadWritePermission>,
  /// The level of permission to grant the access token for custom property management.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub organization_custom_properties: Option<ReadWriteAdminPermission>,
  /// The level of permission to grant the access token for custom repository roles management.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub organization_custom_roles: Option<ReadWritePermission>,
  /// The level of permission to grant the access token to view events triggered by an activity in an organization.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub organization_events: Option<ReadPermission>,
  /// The level of permission to grant the access token to manage the post-receive hooks for an organization.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub organization_hooks: Option<ReadWritePermission>,
  /// The level of permission to grant the access token for organization packages published to GitHub Packages.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub organization_packages: Option<ReadWritePermission>,
  /// The level of permission to grant the access token for viewing and managing fine-grained personal access tokens that have been approved by an organization.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub organization_personal_access_token_requests: Option<ReadWritePermission>,
  /// The level of permission to grant the access token for viewing and managing fine-grained personal access token requests to an organization.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub organization_personal_access_tokens: Option<ReadWritePermission>,
  /// The level of permission to grant the access token for viewing an organization's plan.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub organization_plan: Option<ReadPermission>,
  /// The level of permission to grant the access token to manage organization projects and projects beta (where available).
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub organization_projects: Option<ReadWriteAdminPermission>,
  /// The level of permission to grant the access token to manage organization secrets.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub organization_secrets: Option<ReadWritePermission>,
  /// The level of permission to grant the access token to view and manage GitHub Actions self-hosted runners available to an organization.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub organization_self_hosted_runners: Option<ReadWritePermission>,
  /// The level of permission to grant the access token to view and manage users blocked by the organization.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub organization_user_blocking: Option<ReadWritePermission>,
  /// The level of permission to grant the access token for packages published to GitHub Packages.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub packages: Option<ReadWritePermission>,
  /// The level of permission to grant the access token to retrieve Pages statuses, configuration, and builds, as well as create new builds.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub pages: Option<ReadWritePermission>,
  /// The level of permission to grant the access token to manage the profile settings belonging to a user.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub profile: Option<WritePermission>,
  /// The level of permission to grant the access token for pull requests and related comments, assignees, labels, milestones, and merges.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub pull_requests: Option<ReadWritePermission>,
  /// The level of permission to grant the access token to view and edit custom properties for a repository, when allowed by the property.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub repository_custom_properties: Option<ReadWritePermission>,
  /// The level of permission to grant the access token to manage the post-receive hooks for a repository.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub repository_hooks: Option<ReadWritePermission>,
  /// The level of permission to grant the access token to manage repository projects, columns, and cards.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub repository_projects: Option<ReadWriteAdminPermission>,
  /// The level of permission to grant the access token to view and manage secret scanning alerts.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub secret_scanning_alerts: Option<ReadWritePermission>,
  /// The level of permission to grant the access token to manage repository secrets.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub secrets: Option<ReadWritePermission>,
  /// The level of permission to grant the access token to view and manage security events like code scanning alerts.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub security_events: Option<ReadWritePermission>,
  /// The level of permission to grant the access token to manage just a single file.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub single_file: Option<ReadWritePermission>,
  /// The level of permission to grant the access token to list and manage repositories a user is starring.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub starring: Option<ReadWritePermission>,
  /// The level of permission to grant the access token for commit statuses.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub statuses: Option<ReadWritePermission>,
  /// The level of permission to grant the access token to manage team discussions and related comments.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub team_discussions: Option<ReadWritePermission>,
  /// The level of permission to grant the access token to manage Dependabot alerts.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub vulnerability_alerts: Option<ReadWritePermission>,
  /// The level of permission to grant the access token to update GitHub Actions workflow files.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub workflows: Option<WritePermission>,
}

#[cfg(any(feature = "full", feature = "apps", feature = "orgs"))]
/// Installation
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct Installation {
  pub access_tokens_url: String,
  pub account: InstallationAccount,
  pub app_id: i64,
  pub app_slug: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub contact_email: Option<String>,
  pub created_at: String,
  pub events: Vec<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub has_multiple_single_files: Option<bool>,
  pub html_url: String,
  /// The ID of the installation.
  pub id: i64,
  pub permissions: AppPermissions,
  pub repositories_url: String,
  /// Describe whether all repositories have been selected or there's a selection involved
  pub repository_selection: RepositorySelection,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub single_file_name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub single_file_paths: Option<Vec<String>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub suspended_at: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub suspended_by: Option<SimpleUser>,
  /// The ID of the user or organization this token is being scoped to.
  pub target_id: i64,
  pub target_type: String,
  pub updated_at: String,
}

#[cfg(any(
  feature = "full",
  feature = "apps",
  feature = "activity",
  feature = "issues",
  feature = "licenses",
  feature = "actions",
  feature = "migrations",
  feature = "repos",
  feature = "teams",
  feature = "pulls",
  feature = "security_advisories",
  feature = "search",
  feature = "codespaces"
))]
/// License Simple
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct LicenseSimple {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub html_url: Option<String>,
  pub key: String,
  pub name: String,
  pub node_id: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub spdx_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub url: Option<String>,
}

#[cfg(any(
  feature = "full",
  feature = "apps",
  feature = "activity",
  feature = "issues",
  feature = "actions",
  feature = "migrations",
  feature = "repos",
  feature = "pulls",
  feature = "security_advisories",
  feature = "search",
  feature = "codespaces"
))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RepositoryPermissions {
  pub admin: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub maintain: Option<bool>,
  pub pull: bool,
  pub push: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub triage: Option<bool>,
}

#[cfg(any(
  feature = "full",
  feature = "apps",
  feature = "activity",
  feature = "issues",
  feature = "actions",
  feature = "migrations",
  feature = "repos",
  feature = "pulls",
  feature = "security_advisories",
  feature = "search",
  feature = "codespaces"
))]
/// A repository on GitHub.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct Repository {
  /// Whether to allow Auto-merge to be used on pull requests.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub allow_auto_merge: Option<bool>,
  /// Whether to allow forking this repo
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub allow_forking: Option<bool>,
  /// Whether to allow merge commits for pull requests.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub allow_merge_commit: Option<bool>,
  /// Whether to allow rebase merges for pull requests.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub allow_rebase_merge: Option<bool>,
  /// Whether to allow squash merges for pull requests.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub allow_squash_merge: Option<bool>,
  /// Whether or not a pull request head branch that is behind its base branch can always be updated even if it is not required to be up to date before merging.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub allow_update_branch: Option<bool>,
  /// Whether anonymous git access is enabled for this repository
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub anonymous_access_enabled: Option<bool>,
  pub archive_url: String,
  /// Whether the repository is archived.
  pub archived: bool,
  pub assignees_url: String,
  pub blobs_url: String,
  pub branches_url: String,
  pub clone_url: String,
  pub collaborators_url: String,
  pub comments_url: String,
  pub commits_url: String,
  pub compare_url: String,
  pub contents_url: String,
  pub contributors_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub created_at: Option<String>,
  /// The default branch of the repository.
  pub default_branch: String,
  /// Whether to delete head branches when pull requests are merged
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub delete_branch_on_merge: Option<bool>,
  pub deployments_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub description: Option<String>,
  /// Returns whether or not this repository disabled.
  pub disabled: bool,
  pub downloads_url: String,
  pub events_url: String,
  pub fork: bool,
  pub forks: i64,
  pub forks_count: i64,
  pub forks_url: String,
  pub full_name: String,
  pub git_commits_url: String,
  pub git_refs_url: String,
  pub git_tags_url: String,
  pub git_url: String,
  /// Whether discussions are enabled.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub has_discussions: Option<bool>,
  /// Whether downloads are enabled.
  pub has_downloads: bool,
  /// Whether issues are enabled.
  pub has_issues: bool,
  pub has_pages: bool,
  /// Whether projects are enabled.
  pub has_projects: bool,
  /// Whether the wiki is enabled.
  pub has_wiki: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub homepage: Option<String>,
  pub hooks_url: String,
  pub html_url: String,
  /// Unique identifier of the repository
  pub id: i64,
  /// Whether this repository acts as a template that can be used to generate new repositories.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub is_template: Option<bool>,
  pub issue_comment_url: String,
  pub issue_events_url: String,
  pub issues_url: String,
  pub keys_url: String,
  pub labels_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub language: Option<String>,
  pub languages_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub license: Option<LicenseSimple>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub master_branch: Option<String>,
  /// The default value for a merge commit message.
  ///
  /// - `PR_TITLE` - default to the pull request's title.
  /// - `PR_BODY` - default to the pull request's body.
  /// - `BLANK` - default to a blank commit message.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub merge_commit_message: Option<MergeCommitMessage>,
  /// The default value for a merge commit title.
  ///
  /// - `PR_TITLE` - default to the pull request's title.
  /// - `MERGE_MESSAGE` - default to the classic title for a merge message (e.g., Merge pull request #123 from branch-name).
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub merge_commit_title: Option<MergeCommitTitle>,
  pub merges_url: String,
  pub milestones_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub mirror_url: Option<String>,
  /// The name of the repository.
  pub name: String,
  pub node_id: String,
  pub notifications_url: String,
  pub open_issues: i64,
  pub open_issues_count: i64,
  pub owner: SimpleUser,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub permissions: Option<RepositoryPermissions>,
  /// Whether the repository is private or public.
  pub private: bool,
  pub pulls_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub pushed_at: Option<String>,
  pub releases_url: String,
  /// The size of the repository, in kilobytes. Size is calculated hourly. When a repository is initially created, the size is 0.
  pub size: i64,
  /// The default value for a squash merge commit message:
  ///
  /// - `PR_BODY` - default to the pull request's body.
  /// - `COMMIT_MESSAGES` - default to the branch's commit messages.
  /// - `BLANK` - default to a blank commit message.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub squash_merge_commit_message: Option<SquashMergeCommitMessage>,
  /// The default value for a squash merge commit title:
  ///
  /// - `PR_TITLE` - default to the pull request's title.
  /// - `COMMIT_OR_PR_TITLE` - default to the commit's title (if only one commit) or the pull request's title (when more than one commit).
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub squash_merge_commit_title: Option<SquashMergeCommitTitle>,
  pub ssh_url: String,
  pub stargazers_count: i64,
  pub stargazers_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub starred_at: Option<String>,
  pub statuses_url: String,
  pub subscribers_url: String,
  pub subscription_url: String,
  pub svn_url: String,
  pub tags_url: String,
  pub teams_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub temp_clone_token: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub topics: Option<Vec<String>>,
  pub trees_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub updated_at: Option<String>,
  pub url: String,
  /// Whether a squash merge commit can use the pull request title as default. **This property has been deprecated. Please use `squash_merge_commit_title` instead.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub use_squash_pr_title_as_default: Option<bool>,
  /// The repository visibility: public, private, or internal.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub visibility: Option<String>,
  pub watchers: i64,
  pub watchers_count: i64,
  /// Whether to require contributors to sign off on web-based commits
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub web_commit_signoff_required: Option<bool>,
}

#[cfg(any(feature = "full", feature = "apps"))]
/// Authentication token for a GitHub App installed on a user or org.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct InstallationToken {
  pub expires_at: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub has_multiple_single_files: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub permissions: Option<AppPermissions>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub repositories: Option<Vec<Repository>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub repository_selection: Option<RepositorySelection>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub single_file: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub single_file_paths: Option<Vec<String>>,
  pub token: String,
}

#[cfg(any(feature = "full", feature = "apps"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct AuthorizationApp {
  pub client_id: String,
  pub name: String,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "apps"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ScopedInstallation {
  pub account: SimpleUser,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub has_multiple_single_files: Option<bool>,
  pub permissions: AppPermissions,
  pub repositories_url: String,
  /// Describe whether all repositories have been selected or there's a selection involved
  pub repository_selection: RepositorySelection,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub single_file_name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub single_file_paths: Option<Vec<String>>,
}

#[cfg(any(feature = "full", feature = "apps"))]
/// The authorization for an OAuth app, GitHub App, or a Personal Access Token.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct Authorization {
  pub app: AuthorizationApp,
  pub created_at: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub expires_at: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub fingerprint: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub hashed_token: Option<String>,
  pub id: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub installation: Option<ScopedInstallation>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub note: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub note_url: Option<String>,
  /// A list of scopes that this authorization is in.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub scopes: Option<Vec<String>>,
  pub token: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub token_last_eight: Option<String>,
  pub updated_at: String,
  pub url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub user: Option<SimpleUser>,
}

#[cfg(any(feature = "full", feature = "classroom"))]
/// A GitHub organization.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct SimpleClassroomOrganization {
  pub avatar_url: String,
  pub html_url: String,
  pub id: i64,
  pub login: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub name: Option<String>,
  pub node_id: String,
}

#[cfg(any(feature = "full", feature = "classroom"))]
/// A GitHub Classroom classroom
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct Classroom {
  /// Whether classroom is archived.
  pub archived: bool,
  /// Unique identifier of the classroom.
  pub id: i64,
  /// The name of the classroom.
  pub name: String,
  pub organization: SimpleClassroomOrganization,
  /// The URL of the classroom on GitHub Classroom.
  pub url: String,
}

#[cfg(any(feature = "full", feature = "classroom"))]
/// A GitHub repository view for Classroom
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct SimpleClassroomRepository {
  /// The default branch for the repository.
  pub default_branch: String,
  /// The full, globally unique name of the repository.
  pub full_name: String,
  /// The URL to view the repository on GitHub.com.
  pub html_url: String,
  /// A unique identifier of the repository.
  pub id: i64,
  /// The GraphQL identifier of the repository.
  pub node_id: String,
  /// Whether the repository is private.
  pub private: bool,
}

#[cfg(any(feature = "full", feature = "classroom"))]
/// A GitHub Classroom assignment
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ClassroomAssignment {
  /// The number of students that have accepted the assignment.
  pub accepted: i64,
  pub classroom: Classroom,
  /// The time at which the assignment is due.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub deadline: Option<String>,
  /// The selected editor for the assignment.
  pub editor: String,
  /// Whether feedback pull request will be created when a student accepts the assignment.
  pub feedback_pull_requests_enabled: bool,
  /// Unique identifier of the repository.
  pub id: i64,
  /// Whether the invitation link is enabled. Visiting an enabled invitation link will accept the assignment.
  pub invitations_enabled: bool,
  /// The link that a student can use to accept the assignment.
  pub invite_link: String,
  /// The programming language used in the assignment.
  pub language: String,
  /// The maximum allowable members per team.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub max_members: Option<i64>,
  /// The maximum allowable teams for the assignment.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub max_teams: Option<i64>,
  /// The number of students that have passed the assignment.
  pub passing: i64,
  /// Whether an accepted assignment creates a public repository.
  pub public_repo: bool,
  /// Sluggified name of the assignment.
  pub slug: String,
  pub starter_code_repository: SimpleClassroomRepository,
  /// Whether students are admins on created repository when a student accepts the assignment.
  pub students_are_repo_admins: bool,
  /// The number of students that have submitted the assignment.
  pub submitted: i64,
  /// Assignment title.
  pub title: String,
  /// Whether it's a group assignment or individual assignment.
  #[serde(rename = "type")]
  pub type_: ClassroomAssignmentType,
}

#[cfg(any(feature = "full", feature = "classroom"))]
/// A GitHub Classroom classroom
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct SimpleClassroom {
  /// Returns whether classroom is archived or not.
  pub archived: bool,
  /// Unique identifier of the classroom.
  pub id: i64,
  /// The name of the classroom.
  pub name: String,
  /// The url of the classroom on GitHub Classroom.
  pub url: String,
}

#[cfg(any(feature = "full", feature = "classroom"))]
/// A GitHub Classroom assignment
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct SimpleClassroomAssignment {
  /// The number of students that have accepted the assignment.
  pub accepted: i64,
  pub classroom: SimpleClassroom,
  /// The time at which the assignment is due.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub deadline: Option<String>,
  /// The selected editor for the assignment.
  pub editor: String,
  /// Whether feedback pull request will be created on assignment acceptance.
  pub feedback_pull_requests_enabled: bool,
  /// Unique identifier of the repository.
  pub id: i64,
  /// Whether the invitation link is enabled. Visiting an enabled invitation link will accept the assignment.
  pub invitations_enabled: bool,
  /// The link that a student can use to accept the assignment.
  pub invite_link: String,
  /// The programming language used in the assignment.
  pub language: String,
  /// The maximum allowable members per team.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub max_members: Option<i64>,
  /// The maximum allowable teams for the assignment.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub max_teams: Option<i64>,
  /// The number of students that have passed the assignment.
  pub passing: i64,
  /// Whether an accepted assignment creates a public repository.
  pub public_repo: bool,
  /// Sluggified name of the assignment.
  pub slug: String,
  /// Whether students are admins on created repository on accepted assignment.
  pub students_are_repo_admins: bool,
  /// The number of students that have submitted the assignment.
  pub submitted: i64,
  /// Assignment title.
  pub title: String,
  /// Whether it's a Group Assignment or Individual Assignment.
  #[serde(rename = "type")]
  pub type_: SimpleClassroomAssignmentType,
}

#[cfg(any(feature = "full", feature = "classroom"))]
/// A GitHub user simplified for Classroom.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct SimpleClassroomUser {
  pub avatar_url: String,
  pub html_url: String,
  pub id: i64,
  pub login: String,
}

#[cfg(any(feature = "full", feature = "classroom"))]
/// A GitHub Classroom accepted assignment
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ClassroomAcceptedAssignment {
  pub assignment: SimpleClassroomAssignment,
  /// Count of student commits.
  pub commit_count: i64,
  /// Most recent grade.
  pub grade: String,
  /// Unique identifier of the repository.
  pub id: i64,
  /// Whether a submission passed.
  pub passing: bool,
  pub repository: SimpleClassroomRepository,
  pub students: Vec<SimpleClassroomUser>,
  /// Whether an accepted assignment has been submitted.
  pub submitted: bool,
}

#[cfg(any(feature = "full", feature = "classroom"))]
/// Grade for a student or groups GitHub Classroom assignment
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ClassroomAssignmentGrade {
  /// Name of the assignment
  pub assignment_name: String,
  /// URL of the assignment
  pub assignment_url: String,
  /// GitHub username of the student
  pub github_username: String,
  /// If a group assignment, name of the group the student is in
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub group_name: Option<String>,
  /// Number of points available for the assignment
  pub points_available: i64,
  /// Number of points awarded to the student
  pub points_awarded: i64,
  /// Roster identifier of the student
  pub roster_identifier: String,
  /// URL of the starter code for the assignment
  pub starter_code_url: String,
  /// Name of the student's assignment repository
  pub student_repository_name: String,
  /// URL of the student's assignment repository
  pub student_repository_url: String,
  /// Timestamp of the student's assignment submission
  pub submission_timestamp: String,
}

#[cfg(any(
  feature = "full",
  feature = "codes_of_conduct",
  feature = "activity",
  feature = "actions",
  feature = "codespaces",
  feature = "dependabot",
  feature = "packages",
  feature = "migrations",
  feature = "orgs",
  feature = "repos",
  feature = "teams",
  feature = "checks",
  feature = "search"
))]
/// Code Of Conduct
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CodeOfConduct {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub body: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub html_url: Option<String>,
  pub key: String,
  pub name: String,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "dependabot"))]
/// Details for the vulnerable package.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct DependabotAlertPackage {
  /// The package's language or package management ecosystem.
  pub ecosystem: String,
  /// The unique package name within its ecosystem.
  pub name: String,
}

#[cfg(any(feature = "full", feature = "dependabot"))]
/// Details for the vulnerable dependency.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct DependabotAlertWithRepositoryDependency {
  /// The full path to the dependency manifest file, relative to the root of the repository.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub manifest_path: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub package: Option<DependabotAlertPackage>,
  /// The execution scope of the vulnerable dependency.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub scope: Option<DependabotAlertWithRepositoryDependencyScope>,
}

#[cfg(any(
  feature = "full",
  feature = "dependabot",
  feature = "secret_scanning",
  feature = "code_scanning",
  feature = "security_advisories"
))]
/// A GitHub repository.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct SimpleRepository {
  /// A template for the API URL to download the repository as an archive.
  pub archive_url: String,
  /// A template for the API URL to list the available assignees for issues in the repository.
  pub assignees_url: String,
  /// A template for the API URL to create or retrieve a raw Git blob in the repository.
  pub blobs_url: String,
  /// A template for the API URL to get information about branches in the repository.
  pub branches_url: String,
  /// A template for the API URL to get information about collaborators of the repository.
  pub collaborators_url: String,
  /// A template for the API URL to get information about comments on the repository.
  pub comments_url: String,
  /// A template for the API URL to get information about commits on the repository.
  pub commits_url: String,
  /// A template for the API URL to compare two commits or refs.
  pub compare_url: String,
  /// A template for the API URL to get the contents of the repository.
  pub contents_url: String,
  /// A template for the API URL to list the contributors to the repository.
  pub contributors_url: String,
  /// The API URL to list the deployments of the repository.
  pub deployments_url: String,
  /// The repository description.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub description: Option<String>,
  /// The API URL to list the downloads on the repository.
  pub downloads_url: String,
  /// The API URL to list the events of the repository.
  pub events_url: String,
  /// Whether the repository is a fork.
  pub fork: bool,
  /// The API URL to list the forks of the repository.
  pub forks_url: String,
  /// The full, globally unique, name of the repository.
  pub full_name: String,
  /// A template for the API URL to get information about Git commits of the repository.
  pub git_commits_url: String,
  /// A template for the API URL to get information about Git refs of the repository.
  pub git_refs_url: String,
  /// A template for the API URL to get information about Git tags of the repository.
  pub git_tags_url: String,
  /// The API URL to list the hooks on the repository.
  pub hooks_url: String,
  /// The URL to view the repository on GitHub.com.
  pub html_url: String,
  /// A unique identifier of the repository.
  pub id: i64,
  /// A template for the API URL to get information about issue comments on the repository.
  pub issue_comment_url: String,
  /// A template for the API URL to get information about issue events on the repository.
  pub issue_events_url: String,
  /// A template for the API URL to get information about issues on the repository.
  pub issues_url: String,
  /// A template for the API URL to get information about deploy keys on the repository.
  pub keys_url: String,
  /// A template for the API URL to get information about labels of the repository.
  pub labels_url: String,
  /// The API URL to get information about the languages of the repository.
  pub languages_url: String,
  /// The API URL to merge branches in the repository.
  pub merges_url: String,
  /// A template for the API URL to get information about milestones of the repository.
  pub milestones_url: String,
  /// The name of the repository.
  pub name: String,
  /// The GraphQL identifier of the repository.
  pub node_id: String,
  /// A template for the API URL to get information about notifications on the repository.
  pub notifications_url: String,
  pub owner: SimpleUser,
  /// Whether the repository is private.
  pub private: bool,
  /// A template for the API URL to get information about pull requests on the repository.
  pub pulls_url: String,
  /// A template for the API URL to get information about releases on the repository.
  pub releases_url: String,
  /// The API URL to list the stargazers on the repository.
  pub stargazers_url: String,
  /// A template for the API URL to get information about statuses of a commit.
  pub statuses_url: String,
  /// The API URL to list the subscribers on the repository.
  pub subscribers_url: String,
  /// The API URL to subscribe to notifications for this repository.
  pub subscription_url: String,
  /// The API URL to get information about tags on the repository.
  pub tags_url: String,
  /// The API URL to list the teams on the repository.
  pub teams_url: String,
  /// A template for the API URL to create or retrieve a raw Git tree of the repository.
  pub trees_url: String,
  /// The URL to get more information about the repository from the GitHub API.
  pub url: String,
}

#[cfg(any(feature = "full", feature = "dependabot"))]
/// Details for the advisory pertaining to the Common Vulnerability Scoring System.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct DependabotAlertSecurityAdvisoryCvss {
  /// The overall CVSS score of the advisory.
  pub score: f64,
  /// The full CVSS vector string for the advisory.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub vector_string: Option<String>,
}

#[cfg(any(feature = "full", feature = "dependabot"))]
/// A CWE weakness assigned to the advisory.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct DependabotAlertSecurityAdvisoryCwes {
  /// The unique CWE ID.
  pub cwe_id: String,
  /// The short, plain text name of the CWE.
  pub name: String,
}

#[cfg(any(feature = "full", feature = "dependabot"))]
/// An advisory identifier.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct DependabotAlertSecurityAdvisoryIdentifiers {
  /// The type of advisory identifier.
  #[serde(rename = "type")]
  pub type_: IdentifiersType,
  /// The value of the advisory identifer.
  pub value: String,
}

#[cfg(any(feature = "full", feature = "dependabot"))]
/// A link to additional advisory information.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct DependabotAlertSecurityAdvisoryReferences {
  /// The URL of the reference.
  pub url: String,
}

#[cfg(any(feature = "full", feature = "dependabot"))]
/// Details pertaining to the package version that patches this vulnerability.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct DependabotAlertSecurityVulnerabilityFirstPatchedVersion {
  /// The package version that patches this vulnerability.
  pub identifier: String,
}

#[cfg(any(feature = "full", feature = "dependabot"))]
/// Details pertaining to one vulnerable version range for the advisory.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct DependabotAlertSecurityVulnerability {
  /// Details pertaining to the package version that patches this vulnerability.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub first_patched_version: Option<DependabotAlertSecurityVulnerabilityFirstPatchedVersion>,
  pub package: DependabotAlertPackage,
  /// The severity of the vulnerability.
  pub severity: Severity,
  /// Conditions that identify vulnerable versions of this vulnerability's package.
  pub vulnerable_version_range: String,
}

#[cfg(any(feature = "full", feature = "dependabot"))]
/// Details for the GitHub Security Advisory.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct DependabotAlertSecurityAdvisory {
  /// The unique CVE ID assigned to the advisory.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub cve_id: Option<String>,
  /// Details for the advisory pertaining to the Common Vulnerability Scoring System.
  pub cvss: DependabotAlertSecurityAdvisoryCvss,
  /// Details for the advisory pertaining to Common Weakness Enumeration.
  pub cwes: Vec<DependabotAlertSecurityAdvisoryCwes>,
  /// A long-form Markdown-supported description of the advisory.
  pub description: String,
  /// The unique GitHub Security Advisory ID assigned to the advisory.
  pub ghsa_id: String,
  /// Values that identify this advisory among security information sources.
  pub identifiers: Vec<DependabotAlertSecurityAdvisoryIdentifiers>,
  /// The time that the advisory was published in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`.
  pub published_at: String,
  /// Links to additional advisory information.
  pub references: Vec<DependabotAlertSecurityAdvisoryReferences>,
  /// The severity of the advisory.
  pub severity: Severity,
  /// A short, plain text summary of the advisory.
  pub summary: String,
  /// The time that the advisory was last modified in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`.
  pub updated_at: String,
  /// Vulnerable version range information for the advisory.
  pub vulnerabilities: Vec<DependabotAlertSecurityVulnerability>,
  /// The time that the advisory was withdrawn in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub withdrawn_at: Option<String>,
}

#[cfg(any(feature = "full", feature = "dependabot"))]
/// A Dependabot alert.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct DependabotAlertWithRepository {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub auto_dismissed_at: Option<String>,
  pub created_at: String,
  /// Details for the vulnerable dependency.
  pub dependency: DependabotAlertWithRepositoryDependency,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub dismissed_at: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub dismissed_by: Option<SimpleUser>,
  /// An optional comment associated with the alert's dismissal.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub dismissed_comment: Option<String>,
  /// The reason that the alert was dismissed.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub dismissed_reason: Option<DismissedReason>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub fixed_at: Option<String>,
  pub html_url: String,
  pub number: i64,
  pub repository: SimpleRepository,
  pub security_advisory: DependabotAlertSecurityAdvisory,
  pub security_vulnerability: DependabotAlertSecurityVulnerability,
  /// The state of the Dependabot alert.
  pub state: DependabotAlertState,
  pub updated_at: String,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "secret_scanning"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct OrganizationSecretScanningAlert {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub created_at: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub html_url: Option<String>,
  /// The REST API URL of the code locations for this alert.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub locations_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub number: Option<i64>,
  /// Whether push protection was bypassed for the detected secret.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub push_protection_bypassed: Option<bool>,
  /// The time that push protection was bypassed in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub push_protection_bypassed_at: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub push_protection_bypassed_by: Option<SimpleUser>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub repository: Option<SimpleRepository>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub resolution: Option<SecretScanningAlertResolution>,
  /// The comment that was optionally added when this alert was closed
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub resolution_comment: Option<String>,
  /// The time that the alert was resolved in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub resolved_at: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub resolved_by: Option<SimpleUser>,
  /// The secret that was detected.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub secret: Option<String>,
  /// The type of secret that secret scanning detected.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub secret_type: Option<String>,
  /// User-friendly name for the detected secret, matching the `secret_type`.
  /// For a list of built-in patterns, see "[Secret scanning patterns](https://docs.github.com/code-security/secret-scanning/secret-scanning-patterns#supported-secrets-for-advanced-security)."
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub secret_type_display_name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub state: Option<SecretScanningAlertState>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub updated_at: Option<serde_json::Value>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub url: Option<String>,
  /// The token status as of the latest validity check.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub validity: Option<OrganizationSecretScanningAlertValidity>,
}

#[cfg(any(feature = "full", feature = "activity"))]
/// Actor
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct Actor {
  pub avatar_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub display_login: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub gravatar_id: Option<String>,
  pub id: i64,
  pub login: String,
  pub url: String,
}

#[cfg(any(
  feature = "full",
  feature = "activity",
  feature = "issues",
  feature = "teams",
  feature = "repos",
  feature = "pulls",
  feature = "search"
))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ReactionRollup {
  #[serde(rename = "+1")]
  pub plus_one: i64,
  #[serde(rename = "-1")]
  pub minus_one: i64,
  pub confused: i64,
  pub eyes: i64,
  pub heart: i64,
  pub hooray: i64,
  pub laugh: i64,
  pub rocket: i64,
  pub total_count: i64,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "activity", feature = "issues"))]
/// Comments provide a way for people to collaborate on an issue.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct IssueComment {
  pub author_association: AuthorAssociation,
  /// Contents of the issue comment
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub body: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub body_html: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub body_text: Option<String>,
  pub created_at: String,
  pub html_url: String,
  /// Unique identifier of the issue comment
  pub id: i64,
  pub issue_url: String,
  pub node_id: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub performed_via_github_app: Option<Integration>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub reactions: Option<ReactionRollup>,
  pub updated_at: String,
  /// URL for the issue comment
  pub url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub user: Option<SimpleUser>,
}

#[cfg(any(feature = "full", feature = "activity", feature = "issues"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct IssueLabelsItem2 {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub color: Option<String>,
  #[serde(rename = "default")]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub default_: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub description: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub id: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub node_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub url: Option<String>,
}

#[cfg(any(
  feature = "full",
  feature = "activity",
  feature = "issues",
  feature = "repos",
  feature = "pulls",
  feature = "search"
))]
/// A collection of related issues and pull requests.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct Milestone {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub closed_at: Option<String>,
  pub closed_issues: i64,
  pub created_at: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub creator: Option<SimpleUser>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub description: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub due_on: Option<String>,
  pub html_url: String,
  pub id: i64,
  pub labels_url: String,
  pub node_id: String,
  /// The number of the milestone.
  pub number: i64,
  pub open_issues: i64,
  /// The state of the milestone.
  pub state: MilestoneState,
  /// The title of the milestone.
  pub title: String,
  pub updated_at: String,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "activity", feature = "issues"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct IssuePullRequest {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub diff_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub html_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub merged_at: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub patch_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub url: Option<String>,
}

#[cfg(any(feature = "full", feature = "activity", feature = "issues"))]
/// Issues are a great way to keep track of tasks, enhancements, and bugs for your projects.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct Issue {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub active_lock_reason: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub assignee: Option<SimpleUser>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub assignees: Option<Vec<SimpleUser>>,
  pub author_association: AuthorAssociation,
  /// Contents of the issue
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub body: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub body_html: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub body_text: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub closed_at: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub closed_by: Option<SimpleUser>,
  pub comments: i64,
  pub comments_url: String,
  pub created_at: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub draft: Option<bool>,
  pub events_url: String,
  pub html_url: String,
  pub id: i64,
  /// Labels to associate with this issue; pass one or more label names to replace the set of labels on this issue; send an empty array to clear all labels from the issue; note that the labels are silently dropped for users without push access to the repository
  pub labels: Vec<ObjectOrString<IssueLabelsItem2>>,
  pub labels_url: String,
  pub locked: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub milestone: Option<Milestone>,
  pub node_id: String,
  /// Number uniquely identifying the issue within its repository
  pub number: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub performed_via_github_app: Option<Integration>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub pull_request: Option<IssuePullRequest>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub reactions: Option<ReactionRollup>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub repository: Option<Repository>,
  pub repository_url: String,
  /// State of the issue; either 'open' or 'closed'
  pub state: String,
  /// The reason for the current state
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub state_reason: Option<IssueStateReason>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub timeline_url: Option<String>,
  /// Title of the issue
  pub title: String,
  pub updated_at: String,
  /// URL for the issue
  pub url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub user: Option<SimpleUser>,
}

#[cfg(any(feature = "full", feature = "activity"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct EventPayloadPages {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub action: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub html_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub page_name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub sha: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub summary: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub title: Option<String>,
}

#[cfg(any(feature = "full", feature = "activity"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct EventPayload {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub action: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub comment: Option<IssueComment>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub issue: Option<Issue>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub pages: Option<Vec<EventPayloadPages>>,
}

#[cfg(any(feature = "full", feature = "activity"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct EventRepo {
  pub id: i64,
  pub name: String,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "activity"))]
/// Event
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct Event {
  pub actor: Actor,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub created_at: Option<String>,
  pub id: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub org: Option<Actor>,
  pub payload: EventPayload,
  pub public: bool,
  pub repo: EventRepo,
  #[serde(rename = "type")]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub type_: Option<String>,
}

#[cfg(any(feature = "full", feature = "activity"))]
/// Hypermedia Link with Type
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct LinkWithType {
  pub href: String,
  #[serde(rename = "type")]
  pub type_: String,
}

#[cfg(any(feature = "full", feature = "activity"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct FeedLinks {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub current_user: Option<LinkWithType>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub current_user_actor: Option<LinkWithType>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub current_user_organization: Option<LinkWithType>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub current_user_organizations: Option<Vec<LinkWithType>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub current_user_public: Option<LinkWithType>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub repository_discussions: Option<LinkWithType>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub repository_discussions_category: Option<LinkWithType>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub security_advisories: Option<LinkWithType>,
  pub timeline: LinkWithType,
  pub user: LinkWithType,
}

#[cfg(any(feature = "full", feature = "activity"))]
/// Feed
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct Feed {
  #[serde(rename = "_links")]
  pub links: FeedLinks,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub current_user_actor_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub current_user_organization_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub current_user_organization_urls: Option<Vec<String>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub current_user_public_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub current_user_url: Option<String>,
  /// A feed of discussions for a given repository and category.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub repository_discussions_category_url: Option<String>,
  /// A feed of discussions for a given repository.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub repository_discussions_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub security_advisories_url: Option<String>,
  pub timeline_url: String,
  pub user_url: String,
}

#[cfg(any(feature = "full", feature = "gists"))]
/// Base Gist
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct BaseGist {
  pub comments: i64,
  pub comments_url: String,
  pub commits_url: String,
  pub created_at: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub description: Option<String>,
  pub files: serde_json::Value,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub forks: Option<Vec<serde_json::Value>>,
  pub forks_url: String,
  pub git_pull_url: String,
  pub git_push_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub history: Option<Vec<serde_json::Value>>,
  pub html_url: String,
  pub id: String,
  pub node_id: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub owner: Option<SimpleUser>,
  pub public: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub truncated: Option<bool>,
  pub updated_at: String,
  pub url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub user: Option<SimpleUser>,
}

#[cfg(any(feature = "full", feature = "gists"))]
/// Gist
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct Gist {
  pub comments: i64,
  pub comments_url: String,
  pub commits_url: String,
  pub created_at: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub description: Option<String>,
  pub files: serde_json::Value,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub forks: Option<Vec<serde_json::Value>>,
  pub forks_url: String,
  pub git_pull_url: String,
  pub git_push_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub history: Option<Vec<serde_json::Value>>,
  pub html_url: String,
  pub id: String,
  pub node_id: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub owner: Option<SimpleUser>,
  pub public: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub truncated: Option<bool>,
  pub updated_at: String,
  pub url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub user: Option<SimpleUser>,
}

#[cfg(any(feature = "full", feature = "gists", feature = "users"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct PublicUserPlan {
  pub collaborators: i64,
  pub name: String,
  pub private_repos: i64,
  pub space: i64,
}

#[cfg(any(feature = "full", feature = "gists", feature = "users"))]
/// Public User
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct PublicUser {
  pub avatar_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub bio: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub blog: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub collaborators: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub company: Option<String>,
  pub created_at: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub disk_usage: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub email: Option<String>,
  pub events_url: String,
  pub followers: i64,
  pub followers_url: String,
  pub following: i64,
  pub following_url: String,
  pub gists_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub gravatar_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub hireable: Option<bool>,
  pub html_url: String,
  pub id: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub location: Option<String>,
  pub login: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub name: Option<String>,
  pub node_id: String,
  pub organizations_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub owned_private_repos: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub plan: Option<PublicUserPlan>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub private_gists: Option<i64>,
  pub public_gists: i64,
  pub public_repos: i64,
  pub received_events_url: String,
  pub repos_url: String,
  pub site_admin: bool,
  pub starred_url: String,
  pub subscriptions_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub suspended_at: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub total_private_repos: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub twitter_username: Option<String>,
  #[serde(rename = "type")]
  pub type_: String,
  pub updated_at: String,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "gists"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct GistSimpleForks {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub created_at: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub updated_at: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub user: Option<PublicUser>,
}

#[cfg(any(feature = "full", feature = "gists"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct GistHistoryChangeStatus {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub additions: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub deletions: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub total: Option<i64>,
}

#[cfg(any(feature = "full", feature = "gists"))]
/// Gist History
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct GistHistory {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub change_status: Option<GistHistoryChangeStatus>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub committed_at: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub user: Option<SimpleUser>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub version: Option<String>,
}

#[cfg(any(feature = "full", feature = "gists"))]
/// Gist Simple
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct GistSimple {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub comments: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub comments_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub commits_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub created_at: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub description: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub files: Option<serde_json::Value>,
  /// Gist
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub fork_of: Option<Gist>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub forks: Option<Vec<GistSimpleForks>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub forks_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub git_pull_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub git_push_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub history: Option<Vec<GistHistory>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub html_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub node_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub owner: Option<SimpleUser>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub public: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub truncated: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub updated_at: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub user: Option<String>,
}

#[cfg(any(feature = "full", feature = "gists"))]
/// A comment made to a gist.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct GistComment {
  pub author_association: AuthorAssociation,
  /// The comment text.
  pub body: String,
  pub created_at: String,
  pub id: i64,
  pub node_id: String,
  pub updated_at: String,
  pub url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub user: Option<SimpleUser>,
}

#[cfg(any(feature = "full", feature = "gists"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct GistCommitChangeStatus {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub additions: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub deletions: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub total: Option<i64>,
}

#[cfg(any(feature = "full", feature = "gists"))]
/// Gist Commit
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct GistCommit {
  pub change_status: GistCommitChangeStatus,
  pub committed_at: String,
  pub url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub user: Option<SimpleUser>,
  pub version: String,
}

#[cfg(any(feature = "full", feature = "gitignore"))]
/// Gitignore Template
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct GitignoreTemplate {
  pub name: String,
  pub source: String,
}

#[cfg(any(feature = "full", feature = "licenses"))]
/// License
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct License {
  pub body: String,
  pub conditions: Vec<String>,
  pub description: String,
  pub featured: bool,
  pub html_url: String,
  pub implementation: String,
  pub key: String,
  pub limitations: Vec<String>,
  pub name: String,
  pub node_id: String,
  pub permissions: Vec<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub spdx_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub url: Option<String>,
}

#[cfg(any(feature = "full", feature = "apps"))]
/// Marketplace Listing Plan
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct MarketplaceListingPlan {
  pub accounts_url: String,
  pub bullets: Vec<String>,
  pub description: String,
  pub has_free_trial: bool,
  pub id: i64,
  pub monthly_price_in_cents: i64,
  pub name: String,
  pub number: i64,
  pub price_model: MarketplaceListingPlanPriceModel,
  pub state: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub unit_name: Option<String>,
  pub url: String,
  pub yearly_price_in_cents: i64,
}

#[cfg(any(feature = "full", feature = "apps"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct MarketplacePurchaseMarketplacePendingChange {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub effective_date: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub id: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub is_installed: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub plan: Option<MarketplaceListingPlan>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub unit_count: Option<i64>,
}

#[cfg(any(feature = "full", feature = "apps"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct MarketplacePurchaseMarketplacePurchase {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub billing_cycle: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub free_trial_ends_on: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub is_installed: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub next_billing_date: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub on_free_trial: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub plan: Option<MarketplaceListingPlan>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub unit_count: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub updated_at: Option<String>,
}

#[cfg(any(feature = "full", feature = "apps"))]
/// Marketplace Purchase
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct MarketplacePurchase {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub email: Option<String>,
  pub id: i64,
  pub login: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub marketplace_pending_change: Option<MarketplacePurchaseMarketplacePendingChange>,
  pub marketplace_purchase: MarketplacePurchaseMarketplacePurchase,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub organization_billing_email: Option<String>,
  #[serde(rename = "type")]
  pub type_: String,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "meta"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ApiOverviewDomains {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub actions: Option<Vec<String>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub codespaces: Option<Vec<String>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub copilot: Option<Vec<String>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub packages: Option<Vec<String>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub website: Option<Vec<String>>,
}

#[cfg(any(feature = "full", feature = "meta"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ApiOverviewSshKeyFingerprints {
  #[serde(rename = "SHA256_DSA")]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub sha256_dsa: Option<String>,
  #[serde(rename = "SHA256_ECDSA")]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub sha256_ecdsa: Option<String>,
  #[serde(rename = "SHA256_ED25519")]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub sha256_ed25519: Option<String>,
  #[serde(rename = "SHA256_RSA")]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub sha256_rsa: Option<String>,
}

#[cfg(any(feature = "full", feature = "meta"))]
/// Api Overview
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ApiOverview {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub actions: Option<Vec<String>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub api: Option<Vec<String>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub dependabot: Option<Vec<String>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub domains: Option<ApiOverviewDomains>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub git: Option<Vec<String>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub github_enterprise_importer: Option<Vec<String>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub hooks: Option<Vec<String>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub importer: Option<Vec<String>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub packages: Option<Vec<String>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub pages: Option<Vec<String>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub ssh_key_fingerprints: Option<ApiOverviewSshKeyFingerprints>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub ssh_keys: Option<Vec<String>>,
  pub verifiable_password_authentication: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub web: Option<Vec<String>>,
}

#[cfg(any(
  feature = "full",
  feature = "activity",
  feature = "actions",
  feature = "codespaces",
  feature = "dependabot",
  feature = "packages",
  feature = "migrations",
  feature = "orgs",
  feature = "repos",
  feature = "teams",
  feature = "checks",
  feature = "search"
))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct MinimalRepositoryLicense {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub key: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub node_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub spdx_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub url: Option<String>,
}

#[cfg(any(
  feature = "full",
  feature = "activity",
  feature = "actions",
  feature = "codespaces",
  feature = "dependabot",
  feature = "packages",
  feature = "migrations",
  feature = "orgs",
  feature = "repos",
  feature = "teams",
  feature = "checks",
  feature = "search"
))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct MinimalRepositoryPermissions {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub admin: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub maintain: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub pull: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub push: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub triage: Option<bool>,
}

#[cfg(any(
  feature = "full",
  feature = "activity",
  feature = "actions",
  feature = "codespaces",
  feature = "dependabot",
  feature = "packages",
  feature = "migrations",
  feature = "orgs",
  feature = "repos",
  feature = "teams",
  feature = "checks",
  feature = "security_advisories",
  feature = "search"
))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct SecurityAndAnalysisAdvancedSecurity {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub status: Option<SecurityAndAnalysisAdvancedSecurityStatus>,
}

#[cfg(any(
  feature = "full",
  feature = "activity",
  feature = "actions",
  feature = "codespaces",
  feature = "dependabot",
  feature = "packages",
  feature = "migrations",
  feature = "orgs",
  feature = "repos",
  feature = "teams",
  feature = "checks",
  feature = "security_advisories",
  feature = "search"
))]
/// Enable or disable Dependabot security updates for the repository.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct SecurityAndAnalysisDependabotSecurityUpdates {
  /// The enablement status of Dependabot security updates for the repository.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub status: Option<SecurityAndAnalysisDependabotSecurityUpdatesStatus>,
}

#[cfg(any(
  feature = "full",
  feature = "activity",
  feature = "actions",
  feature = "codespaces",
  feature = "dependabot",
  feature = "packages",
  feature = "migrations",
  feature = "orgs",
  feature = "repos",
  feature = "teams",
  feature = "checks",
  feature = "security_advisories",
  feature = "search"
))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct SecurityAndAnalysisSecretScanning {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub status: Option<SecurityAndAnalysisSecretScanningStatus>,
}

#[cfg(any(
  feature = "full",
  feature = "activity",
  feature = "actions",
  feature = "codespaces",
  feature = "dependabot",
  feature = "packages",
  feature = "migrations",
  feature = "orgs",
  feature = "repos",
  feature = "teams",
  feature = "checks",
  feature = "security_advisories",
  feature = "search"
))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct SecurityAndAnalysisSecretScanningPushProtection {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub status: Option<SecurityAndAnalysisSecretScanningPushProtectionStatus>,
}

#[cfg(any(
  feature = "full",
  feature = "activity",
  feature = "actions",
  feature = "codespaces",
  feature = "dependabot",
  feature = "packages",
  feature = "migrations",
  feature = "orgs",
  feature = "repos",
  feature = "teams",
  feature = "checks",
  feature = "security_advisories",
  feature = "search"
))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct SecurityAndAnalysis {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub advanced_security: Option<SecurityAndAnalysisAdvancedSecurity>,
  /// Enable or disable Dependabot security updates for the repository.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub dependabot_security_updates: Option<SecurityAndAnalysisDependabotSecurityUpdates>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub secret_scanning: Option<SecurityAndAnalysisSecretScanning>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub secret_scanning_push_protection: Option<SecurityAndAnalysisSecretScanningPushProtection>,
}

#[cfg(any(
  feature = "full",
  feature = "activity",
  feature = "actions",
  feature = "codespaces",
  feature = "dependabot",
  feature = "packages",
  feature = "migrations",
  feature = "orgs",
  feature = "repos",
  feature = "teams",
  feature = "checks",
  feature = "search"
))]
/// Minimal Repository
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct MinimalRepository {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub allow_forking: Option<bool>,
  pub archive_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub archived: Option<bool>,
  pub assignees_url: String,
  pub blobs_url: String,
  pub branches_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub clone_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub code_of_conduct: Option<CodeOfConduct>,
  pub collaborators_url: String,
  pub comments_url: String,
  pub commits_url: String,
  pub compare_url: String,
  pub contents_url: String,
  pub contributors_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub created_at: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub default_branch: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub delete_branch_on_merge: Option<bool>,
  pub deployments_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub description: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub disabled: Option<bool>,
  pub downloads_url: String,
  pub events_url: String,
  pub fork: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub forks: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub forks_count: Option<i64>,
  pub forks_url: String,
  pub full_name: String,
  pub git_commits_url: String,
  pub git_refs_url: String,
  pub git_tags_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub git_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub has_discussions: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub has_downloads: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub has_issues: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub has_pages: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub has_projects: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub has_wiki: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub homepage: Option<String>,
  pub hooks_url: String,
  pub html_url: String,
  pub id: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub is_template: Option<bool>,
  pub issue_comment_url: String,
  pub issue_events_url: String,
  pub issues_url: String,
  pub keys_url: String,
  pub labels_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub language: Option<String>,
  pub languages_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub license: Option<MinimalRepositoryLicense>,
  pub merges_url: String,
  pub milestones_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub mirror_url: Option<String>,
  pub name: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub network_count: Option<i64>,
  pub node_id: String,
  pub notifications_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub open_issues: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub open_issues_count: Option<i64>,
  pub owner: SimpleUser,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub permissions: Option<MinimalRepositoryPermissions>,
  pub private: bool,
  pub pulls_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub pushed_at: Option<String>,
  pub releases_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub role_name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub security_and_analysis: Option<SecurityAndAnalysis>,
  /// The size of the repository, in kilobytes. Size is calculated hourly. When a repository is initially created, the size is 0.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub size: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub ssh_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub stargazers_count: Option<i64>,
  pub stargazers_url: String,
  pub statuses_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub subscribers_count: Option<i64>,
  pub subscribers_url: String,
  pub subscription_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub svn_url: Option<String>,
  pub tags_url: String,
  pub teams_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub temp_clone_token: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub topics: Option<Vec<String>>,
  pub trees_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub updated_at: Option<String>,
  pub url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub visibility: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub watchers: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub watchers_count: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub web_commit_signoff_required: Option<bool>,
}

#[cfg(any(feature = "full", feature = "activity"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ThreadSubject {
  pub latest_comment_url: String,
  pub title: String,
  #[serde(rename = "type")]
  pub type_: String,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "activity"))]
/// Thread
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct Thread {
  pub id: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub last_read_at: Option<String>,
  pub reason: String,
  pub repository: MinimalRepository,
  pub subject: ThreadSubject,
  pub subscription_url: String,
  pub unread: bool,
  pub updated_at: String,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "activity"))]
/// Thread Subscription
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ThreadSubscription {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub created_at: Option<String>,
  pub ignored: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub reason: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub repository_url: Option<String>,
  pub subscribed: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub thread_url: Option<String>,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "orgs"))]
/// A GitHub organization.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct OrganizationSimple {
  pub avatar_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub description: Option<String>,
  pub events_url: String,
  pub hooks_url: String,
  pub id: i64,
  pub issues_url: String,
  pub login: String,
  pub members_url: String,
  pub node_id: String,
  pub public_members_url: String,
  pub repos_url: String,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "orgs"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct OrganizationFullPlan {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub filled_seats: Option<i64>,
  pub name: String,
  pub private_repos: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub seats: Option<i64>,
  pub space: i64,
}

#[cfg(any(feature = "full", feature = "orgs"))]
/// Organization Full
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct OrganizationFull {
  /// Whether GitHub Advanced Security is enabled for new repositories and repositories transferred to this organization.
  ///
  /// This field is only visible to organization owners or members of a team with the security manager role.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub advanced_security_enabled_for_new_repositories: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub archived_at: Option<String>,
  pub avatar_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub billing_email: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub blog: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub collaborators: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub company: Option<String>,
  pub created_at: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub default_repository_permission: Option<String>,
  /// Whether GitHub Advanced Security is automatically enabled for new repositories and repositories transferred to
  /// this organization.
  ///
  /// This field is only visible to organization owners or members of a team with the security manager role.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub dependabot_alerts_enabled_for_new_repositories: Option<bool>,
  /// Whether dependabot security updates are automatically enabled for new repositories and repositories transferred
  /// to this organization.
  ///
  /// This field is only visible to organization owners or members of a team with the security manager role.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub dependabot_security_updates_enabled_for_new_repositories: Option<bool>,
  /// Whether dependency graph is automatically enabled for new repositories and repositories transferred to this
  /// organization.
  ///
  /// This field is only visible to organization owners or members of a team with the security manager role.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub dependency_graph_enabled_for_new_repositories: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub description: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub disk_usage: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub email: Option<String>,
  pub events_url: String,
  pub followers: i64,
  pub following: i64,
  pub has_organization_projects: bool,
  pub has_repository_projects: bool,
  pub hooks_url: String,
  pub html_url: String,
  pub id: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub is_verified: Option<bool>,
  pub issues_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub location: Option<String>,
  pub login: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub members_allowed_repository_creation_type: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub members_can_create_internal_repositories: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub members_can_create_pages: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub members_can_create_private_pages: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub members_can_create_private_repositories: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub members_can_create_public_pages: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub members_can_create_public_repositories: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub members_can_create_repositories: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub members_can_fork_private_repositories: Option<bool>,
  pub members_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub name: Option<String>,
  pub node_id: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub owned_private_repos: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub plan: Option<OrganizationFullPlan>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub private_gists: Option<i64>,
  pub public_gists: i64,
  pub public_members_url: String,
  pub public_repos: i64,
  pub repos_url: String,
  /// Whether secret scanning is automatically enabled for new repositories and repositories transferred to this
  /// organization.
  ///
  /// This field is only visible to organization owners or members of a team with the security manager role.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub secret_scanning_enabled_for_new_repositories: Option<bool>,
  /// An optional URL string to display to contributors who are blocked from pushing a secret.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub secret_scanning_push_protection_custom_link: Option<String>,
  /// Whether a custom link is shown to contributors who are blocked from pushing a secret by push protection.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub secret_scanning_push_protection_custom_link_enabled: Option<bool>,
  /// Whether secret scanning push protection is automatically enabled for new repositories and repositories
  /// transferred to this organization.
  ///
  /// This field is only visible to organization owners or members of a team with the security manager role.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub secret_scanning_push_protection_enabled_for_new_repositories: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub total_private_repos: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub twitter_username: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub two_factor_requirement_enabled: Option<bool>,
  #[serde(rename = "type")]
  pub type_: String,
  pub updated_at: String,
  pub url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub web_commit_signoff_required: Option<bool>,
}

#[cfg(any(feature = "full", feature = "actions"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ActionsCacheUsageOrgEnterprise {
  /// The count of active caches across all repositories of an enterprise or an organization.
  pub total_active_caches_count: i64,
  /// The total size in bytes of all active cache items across all repositories of an enterprise or an organization.
  pub total_active_caches_size_in_bytes: i64,
}

#[cfg(any(feature = "full", feature = "actions"))]
/// GitHub Actions Cache Usage by repository.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ActionsCacheUsageByRepository {
  /// The number of active caches in the repository.
  pub active_caches_count: i64,
  /// The sum of the size in bytes of all the active cache items in the repository.
  pub active_caches_size_in_bytes: i64,
  /// The repository owner and name for the cache usage being shown.
  pub full_name: String,
}

#[cfg(any(feature = "full", feature = "oidc"))]
/// Actions OIDC Subject customization
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct OidcCustomSub {
  /// Array of unique strings. Each claim key can only contain alphanumeric characters and underscores.
  pub include_claim_keys: Vec<String>,
}

#[cfg(any(
  feature = "full",
  feature = "oidc",
  feature = "actions",
  feature = "codespaces",
  feature = "dependabot",
  feature = "checks",
  feature = "code_scanning",
  feature = "repos"
))]
/// An object without any properties.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct EmptyObject {}

#[cfg(any(feature = "full", feature = "actions"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ActionsOrganizationPermissions {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub allowed_actions: Option<AllowedActions>,
  pub enabled_repositories: EnabledRepositories,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub selected_actions_url: Option<String>,
  /// The API URL to use to get or set the selected repositories that are allowed to run GitHub Actions, when `enabled_repositories` is set to `selected`.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub selected_repositories_url: Option<String>,
}

#[cfg(any(feature = "full", feature = "actions"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct SelectedActions {
  /// Whether GitHub-owned actions are allowed. For example, this includes the actions in the `actions` organization.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub github_owned_allowed: Option<bool>,
  /// Specifies a list of string-matching patterns to allow specific action(s) and reusable workflow(s). Wildcards, tags, and SHAs are allowed. For example, `monalisa/octocat@*`, `monalisa/octocat@v2`, `monalisa/*`.
  ///
  /// **Note**: The `patterns_allowed` setting only applies to public repositories.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub patterns_allowed: Option<Vec<String>>,
  /// Whether actions from GitHub Marketplace verified creators are allowed. Set to `true` to allow all actions by GitHub Marketplace verified creators.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub verified_allowed: Option<bool>,
}

#[cfg(any(feature = "full", feature = "actions"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ActionsGetDefaultWorkflowPermissions {
  pub can_approve_pull_request_reviews: bool,
  pub default_workflow_permissions: ReadWritePermission,
}

#[cfg(any(feature = "full", feature = "actions"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ActionsSetDefaultWorkflowPermissions {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub can_approve_pull_request_reviews: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub default_workflow_permissions: Option<ReadWritePermission>,
}

#[cfg(any(feature = "full", feature = "actions"))]
/// A label for a self hosted runner
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RunnerLabel {
  /// Unique identifier of the label.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub id: Option<i64>,
  /// Name of the label.
  pub name: String,
  /// The type of label. Read-only labels are applied automatically when the runner is configured.
  #[serde(rename = "type")]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub type_: Option<RunnerLabelType>,
}

#[cfg(any(feature = "full", feature = "actions"))]
/// A self hosted runner
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct Runner {
  pub busy: bool,
  /// The id of the runner.
  pub id: i64,
  pub labels: Vec<RunnerLabel>,
  /// The name of the runner.
  pub name: String,
  /// The Operating System of the runner.
  pub os: String,
  /// The id of the runner group.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub runner_group_id: Option<i64>,
  /// The status of the runner.
  pub status: String,
}

#[cfg(any(feature = "full", feature = "actions"))]
/// Runner Application
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RunnerApplication {
  pub architecture: String,
  pub download_url: String,
  pub filename: String,
  pub os: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub sha256_checksum: Option<String>,
  /// A short lived bearer token used to download the runner, if needed.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub temp_download_token: Option<String>,
}

#[cfg(any(feature = "full", feature = "actions"))]
/// Authentication Token
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct AuthenticationToken {
  /// The time this token expires
  pub expires_at: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub permissions: Option<serde_json::Value>,
  /// The repositories this token has access to
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub repositories: Option<Vec<Repository>>,
  /// Describe whether all repositories have been selected or there's a selection involved
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub repository_selection: Option<RepositorySelection>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub single_file: Option<String>,
  /// The token used for authentication
  pub token: String,
}

#[cfg(any(feature = "full", feature = "actions"))]
/// Secrets for GitHub Actions for an organization.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct OrganizationActionsSecret {
  pub created_at: String,
  /// The name of the secret.
  pub name: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub selected_repositories_url: Option<String>,
  pub updated_at: String,
  /// Visibility of a secret
  pub visibility: Visibility,
}

#[cfg(any(feature = "full", feature = "actions"))]
/// The public key used for setting Actions Secrets.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ActionsPublicKey {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub created_at: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub id: Option<i64>,
  /// The Base64 encoded public key.
  pub key: String,
  /// The identifier for the key.
  pub key_id: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub title: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub url: Option<String>,
}

#[cfg(any(feature = "full", feature = "actions"))]
/// Organization variable for GitHub Actions.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct OrganizationActionsVariable {
  /// The date and time at which the variable was created, in ISO 8601 format':' YYYY-MM-DDTHH:MM:SSZ.
  pub created_at: String,
  /// The name of the variable.
  pub name: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub selected_repositories_url: Option<String>,
  /// The date and time at which the variable was last updated, in ISO 8601 format':' YYYY-MM-DDTHH:MM:SSZ.
  pub updated_at: String,
  /// The value of the variable.
  pub value: String,
  /// Visibility of a variable
  pub visibility: Visibility,
}

#[cfg(any(feature = "full", feature = "code_scanning"))]
/// Describe a region within a file for the alert.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CodeScanningAlertLocation {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub end_column: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub end_line: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub path: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub start_column: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub start_line: Option<i64>,
}

#[cfg(any(feature = "full", feature = "code_scanning"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CodeScanningAlertInstanceMessage {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub text: Option<String>,
}

#[cfg(any(feature = "full", feature = "code_scanning"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CodeScanningAlertInstance {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub analysis_key: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub category: Option<String>,
  /// Classifications that have been applied to the file that triggered the alert.
  /// For example identifying it as documentation, or a generated file.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub classifications: Option<Vec<Option<CodeScanningAlertClassification>>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub commit_sha: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub environment: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub html_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub location: Option<CodeScanningAlertLocation>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub message: Option<CodeScanningAlertInstanceMessage>,
  #[serde(rename = "ref")]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub ref_: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub state: Option<CodeScanningAlertState>,
}

#[cfg(any(feature = "full", feature = "code_scanning"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CodeScanningAlertRuleSummary {
  /// A short description of the rule used to detect the alert.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub description: Option<String>,
  /// A unique identifier for the rule used to detect the alert.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub id: Option<String>,
  /// The name of the rule used to detect the alert.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub name: Option<String>,
  /// The security severity of the alert.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub security_severity_level: Option<Severity>,
  /// The severity of the alert.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub severity: Option<CodeScanningAlertRuleSummarySeverity>,
  /// A set of tags applicable for the rule.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub tags: Option<Vec<String>>,
}

#[cfg(any(feature = "full", feature = "code_scanning"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CodeScanningAnalysisTool {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub guid: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub version: Option<String>,
}

#[cfg(any(feature = "full", feature = "code_scanning"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CodeScanningOrganizationAlertItems {
  pub created_at: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub dismissed_at: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub dismissed_by: Option<SimpleUser>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub dismissed_comment: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub dismissed_reason: Option<CodeScanningAlertDismissedReason>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub fixed_at: Option<String>,
  pub html_url: String,
  pub instances_url: String,
  pub most_recent_instance: CodeScanningAlertInstance,
  pub number: i64,
  pub repository: SimpleRepository,
  pub rule: CodeScanningAlertRuleSummary,
  pub state: CodeScanningAlertState,
  pub tool: CodeScanningAnalysisTool,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub updated_at: Option<String>,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "codespaces"))]
/// Details about the codespace's git repository.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CodespaceGitStatus {
  /// The number of commits the local repository is ahead of the remote.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub ahead: Option<i64>,
  /// The number of commits the local repository is behind the remote.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub behind: Option<i64>,
  /// Whether the local repository has uncommitted changes.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub has_uncommitted_changes: Option<bool>,
  /// Whether the local repository has unpushed changes.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub has_unpushed_changes: Option<bool>,
  /// The current branch (or SHA if in detached HEAD state) of the local repository.
  #[serde(rename = "ref")]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub ref_: Option<String>,
}

#[cfg(any(feature = "full", feature = "codespaces"))]
/// A description of the machine powering a codespace.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CodespaceMachine {
  /// How many cores are available to the codespace.
  pub cpus: i64,
  /// The display name of the machine includes cores, memory, and storage.
  pub display_name: String,
  /// How much memory is available to the codespace.
  pub memory_in_bytes: i64,
  /// The name of the machine.
  pub name: String,
  /// The operating system of the machine.
  pub operating_system: String,
  /// Whether a prebuild is currently available when creating a codespace for this machine and repository. If a branch was not specified as a ref, the default branch will be assumed. Value will be "null" if prebuilds are not supported or prebuild availability could not be determined. Value will be "none" if no prebuild is available. Latest values "ready" and "in_progress" indicate the prebuild availability status.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub prebuild_availability: Option<CodespaceMachinePrebuildAvailability>,
  /// How much storage is available to the codespace.
  pub storage_in_bytes: i64,
}

#[cfg(any(feature = "full", feature = "codespaces"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CodespaceRuntimeConstraints {
  /// The privacy settings a user can select from when forwarding a port.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub allowed_port_privacy_settings: Option<Vec<String>>,
}

#[cfg(any(feature = "full", feature = "codespaces"))]
/// A codespace.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct Codespace {
  pub billable_owner: SimpleUser,
  pub created_at: String,
  /// Path to devcontainer.json from repo root used to create Codespace.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub devcontainer_path: Option<String>,
  /// Display name for this codespace.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub display_name: Option<String>,
  /// UUID identifying this codespace's environment.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub environment_id: Option<String>,
  /// Details about the codespace's git repository.
  pub git_status: CodespaceGitStatus,
  pub id: i64,
  /// The number of minutes of inactivity after which this codespace will be automatically stopped.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub idle_timeout_minutes: Option<i64>,
  /// Text to show user when codespace idle timeout minutes has been overriden by an organization policy
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub idle_timeout_notice: Option<String>,
  /// The text to display to a user when a codespace has been stopped for a potentially actionable reason.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub last_known_stop_notice: Option<String>,
  /// Last known time this codespace was started.
  pub last_used_at: String,
  /// The initally assigned location of a new codespace.
  pub location: CodespaceLocation,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub machine: Option<CodespaceMachine>,
  /// API URL to access available alternate machine types for this codespace.
  pub machines_url: String,
  /// Automatically generated name of this codespace.
  pub name: String,
  pub owner: SimpleUser,
  /// Whether or not a codespace has a pending async operation. This would mean that the codespace is temporarily unavailable. The only thing that you can do with a codespace in this state is delete it.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub pending_operation: Option<bool>,
  /// Text to show user when codespace is disabled by a pending operation
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub pending_operation_disabled_reason: Option<String>,
  /// Whether the codespace was created from a prebuild.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub prebuild: Option<bool>,
  /// API URL to publish this codespace to a new repository.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub publish_url: Option<String>,
  /// API URL for the Pull Request associated with this codespace, if any.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub pulls_url: Option<String>,
  pub recent_folders: Vec<String>,
  pub repository: MinimalRepository,
  /// When a codespace will be auto-deleted based on the "retention_period_minutes" and "last_used_at"
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub retention_expires_at: Option<String>,
  /// Duration in minutes after codespace has gone idle in which it will be deleted. Must be integer minutes between 0 and 43200 (30 days).
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub retention_period_minutes: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub runtime_constraints: Option<CodespaceRuntimeConstraints>,
  /// API URL to start this codespace.
  pub start_url: String,
  /// State of this codespace.
  pub state: CodespaceState,
  /// API URL to stop this codespace.
  pub stop_url: String,
  pub updated_at: String,
  /// API URL for this codespace.
  pub url: String,
  /// URL to access this codespace on the web.
  pub web_url: String,
}

#[cfg(any(feature = "full", feature = "codespaces"))]
/// Secrets for a GitHub Codespace.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CodespacesOrgSecret {
  /// The date and time at which the secret was created, in ISO 8601 format':' YYYY-MM-DDTHH:MM:SSZ.
  pub created_at: String,
  /// The name of the secret
  pub name: String,
  /// The API URL at which the list of repositories this secret is visible to can be retrieved
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub selected_repositories_url: Option<String>,
  /// The date and time at which the secret was created, in ISO 8601 format':' YYYY-MM-DDTHH:MM:SSZ.
  pub updated_at: String,
  /// The type of repositories in the organization that the secret is visible to
  pub visibility: Visibility,
}

#[cfg(any(feature = "full", feature = "codespaces"))]
/// The public key used for setting Codespaces secrets.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CodespacesPublicKey {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub created_at: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub id: Option<i64>,
  /// The Base64 encoded public key.
  pub key: String,
  /// The identifier for the key.
  pub key_id: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub title: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub url: Option<String>,
}

#[cfg(any(feature = "full", feature = "copilot"))]
/// The breakdown of Copilot Business seats for the organization.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CopilotSeatBreakdown {
  /// The number of seats that have used Copilot during the current billing cycle.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub active_this_cycle: Option<i64>,
  /// Seats added during the current billing cycle.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub added_this_cycle: Option<i64>,
  /// The number of seats that have not used Copilot during the current billing cycle.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub inactive_this_cycle: Option<i64>,
  /// The number of seats that are pending cancellation at the end of the current billing cycle.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub pending_cancellation: Option<i64>,
  /// The number of seats that have been assigned to users that have not yet accepted an invitation to this organization.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub pending_invitation: Option<i64>,
  /// The total number of seats being billed for the organization as of the current billing cycle.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub total: Option<i64>,
}

#[cfg(any(feature = "full", feature = "copilot"))]
/// Information about the seat breakdown and policies set for an organization with a Copilot Business subscription.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CopilotOrganizationDetails {
  /// The organization policy for allowing or disallowing organization members to use Copilot within their CLI.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub cli: Option<CopilotOrganizationPolicy>,
  /// The organization policy for allowing or disallowing organization members to use Copilot Chat within their editor.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub ide_chat: Option<CopilotOrganizationPolicy>,
  /// The organization policy for allowing or disallowing organization members to use Copilot features within github.com.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub platform_chat: Option<CopilotOrganizationPolicy>,
  /// The organization policy for allowing or disallowing Copilot to make suggestions that match public code.
  pub public_code_suggestions: CopilotOrganizationDetailsPublicCodeSuggestions,
  pub seat_breakdown: CopilotSeatBreakdown,
  /// The mode of assigning new seats.
  pub seat_management_setting: CopilotOrganizationDetailsSeatManagementSetting,
}

#[cfg(any(
  feature = "full",
  feature = "copilot",
  feature = "orgs",
  feature = "security_advisories",
  feature = "teams",
  feature = "actions",
  feature = "repos",
  feature = "issues",
  feature = "pulls"
))]
/// Groups of organization members that gives permissions on specified repositories.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct TeamSimple {
  /// Description of the team
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub description: Option<String>,
  pub html_url: String,
  /// Unique identifier of the team
  pub id: i64,
  /// Distinguished Name (DN) that team maps to within LDAP environment
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub ldap_dn: Option<String>,
  pub members_url: String,
  /// Name of the team
  pub name: String,
  pub node_id: String,
  /// The notification setting the team has set
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub notification_setting: Option<String>,
  /// Permission that the team will have for its repositories
  pub permission: String,
  /// The level of privacy this team should have
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub privacy: Option<String>,
  pub repositories_url: String,
  pub slug: String,
  /// URL for the team
  pub url: String,
}

#[cfg(any(
  feature = "full",
  feature = "copilot",
  feature = "orgs",
  feature = "security_advisories",
  feature = "teams",
  feature = "actions",
  feature = "repos",
  feature = "issues",
  feature = "pulls"
))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct TeamPermissions {
  pub admin: bool,
  pub maintain: bool,
  pub pull: bool,
  pub push: bool,
  pub triage: bool,
}

#[cfg(any(
  feature = "full",
  feature = "copilot",
  feature = "orgs",
  feature = "security_advisories",
  feature = "teams",
  feature = "actions",
  feature = "repos",
  feature = "issues",
  feature = "pulls"
))]
/// Groups of organization members that gives permissions on specified repositories.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct Team {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub description: Option<String>,
  pub html_url: String,
  pub id: i64,
  pub members_url: String,
  pub name: String,
  pub node_id: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub notification_setting: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub parent: Option<TeamSimple>,
  pub permission: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub permissions: Option<TeamPermissions>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub privacy: Option<String>,
  pub repositories_url: String,
  pub slug: String,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "copilot"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct OrganizationPlan {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub filled_seats: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub private_repos: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub seats: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub space: Option<i64>,
}

#[cfg(any(feature = "full", feature = "copilot"))]
/// GitHub account for managing multiple users, teams, and repositories
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct Organization {
  pub avatar_url: String,
  /// Display blog url for the organization
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub blog: Option<String>,
  /// Display company name for the organization
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub company: Option<String>,
  pub created_at: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub description: Option<String>,
  /// Display email for the organization
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub email: Option<String>,
  pub events_url: String,
  pub followers: i64,
  pub following: i64,
  /// Specifies if organization projects are enabled for this org
  pub has_organization_projects: bool,
  /// Specifies if repository projects are enabled for repositories that belong to this org
  pub has_repository_projects: bool,
  pub hooks_url: String,
  pub html_url: String,
  pub id: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub is_verified: Option<bool>,
  pub issues_url: String,
  /// Display location for the organization
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub location: Option<String>,
  /// Unique login name of the organization
  pub login: String,
  pub members_url: String,
  /// Display name for the organization
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub name: Option<String>,
  pub node_id: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub plan: Option<OrganizationPlan>,
  pub public_gists: i64,
  pub public_members_url: String,
  pub public_repos: i64,
  pub repos_url: String,
  #[serde(rename = "type")]
  pub type_: String,
  pub updated_at: String,
  /// URL for the organization
  pub url: String,
}

#[cfg(any(feature = "full", feature = "copilot"))]
/// Information about a Copilot Business seat assignment for a user, team, or organization.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CopilotSeatDetails {
  /// The assignee that has been granted access to GitHub Copilot.
  pub assignee: CopilotSeatDetailsAssignee,
  /// The team that granted access to GitHub Copilot to the assignee. This will be null if the user was assigned a seat individually.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub assigning_team: Option<Team>,
  /// Timestamp of when the assignee was last granted access to GitHub Copilot, in ISO 8601 format.
  pub created_at: String,
  /// Timestamp of user's last GitHub Copilot activity, in ISO 8601 format.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub last_activity_at: Option<String>,
  /// Last editor that was used by the user for a GitHub Copilot completion.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub last_activity_editor: Option<String>,
  /// The pending cancellation date for the seat, in `YYYY-MM-DD` format. This will be null unless the assignee's Copilot access has been canceled during the current billing cycle. If the seat has been cancelled, this corresponds to the start of the organization's next billing cycle.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub pending_cancellation_date: Option<String>,
  /// Timestamp of when the assignee's GitHub Copilot access was last updated, in ISO 8601 format.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub updated_at: Option<String>,
}

#[cfg(any(feature = "full", feature = "dependabot"))]
/// Secrets for GitHub Dependabot for an organization.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct OrganizationDependabotSecret {
  pub created_at: String,
  /// The name of the secret.
  pub name: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub selected_repositories_url: Option<String>,
  pub updated_at: String,
  /// Visibility of a secret
  pub visibility: Visibility,
}

#[cfg(any(feature = "full", feature = "dependabot"))]
/// The public key used for setting Dependabot Secrets.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct DependabotPublicKey {
  /// The Base64 encoded public key.
  pub key: String,
  /// The identifier for the key.
  pub key_id: String,
}

#[cfg(any(feature = "full", feature = "packages"))]
/// A software package
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct Package {
  pub created_at: String,
  pub html_url: String,
  /// Unique identifier of the package.
  pub id: i64,
  /// The name of the package.
  pub name: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub owner: Option<SimpleUser>,
  pub package_type: PackageType,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub repository: Option<MinimalRepository>,
  pub updated_at: String,
  pub url: String,
  /// The number of versions of the package.
  pub version_count: i64,
  pub visibility: PackageVisibility,
}

#[cfg(any(feature = "full", feature = "orgs", feature = "teams"))]
/// Organization Invitation
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct OrganizationInvitation {
  pub created_at: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub email: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub failed_at: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub failed_reason: Option<String>,
  pub id: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub invitation_source: Option<String>,
  pub invitation_teams_url: String,
  pub inviter: SimpleUser,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub login: Option<String>,
  pub node_id: String,
  pub role: String,
  pub team_count: i64,
}

#[cfg(any(feature = "full", feature = "orgs"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct OrgHookConfig {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub content_type: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub insecure_ssl: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub secret: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub url: Option<String>,
}

#[cfg(any(feature = "full", feature = "orgs"))]
/// Org Hook
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct OrgHook {
  pub active: bool,
  pub config: OrgHookConfig,
  pub created_at: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub deliveries_url: Option<String>,
  pub events: Vec<String>,
  pub id: i64,
  pub name: String,
  pub ping_url: String,
  #[serde(rename = "type")]
  pub type_: String,
  pub updated_at: String,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "interactions"))]
/// Interaction limit settings.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct InteractionLimitResponse {
  pub expires_at: String,
  pub limit: InteractionGroup,
  pub origin: String,
}

#[cfg(any(feature = "full", feature = "interactions"))]
/// Limit interactions to a specific type of user for a specified duration
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct InteractionLimit {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub expiry: Option<InteractionExpiry>,
  pub limit: InteractionGroup,
}

#[cfg(any(feature = "full", feature = "orgs"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct OrgMembershipPermissions {
  pub can_create_repository: bool,
}

#[cfg(any(feature = "full", feature = "orgs"))]
/// Org Membership
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct OrgMembership {
  pub organization: OrganizationSimple,
  pub organization_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub permissions: Option<OrgMembershipPermissions>,
  /// The user's membership type in the organization.
  pub role: OrgMembershipRole,
  /// The state of the member in the organization. The `pending` state indicates the user has not yet accepted an invitation.
  pub state: OrgMembershipState,
  pub url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub user: Option<SimpleUser>,
}

#[cfg(any(feature = "full", feature = "migrations"))]
/// A migration.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct Migration {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub archive_url: Option<String>,
  pub created_at: String,
  /// Exclude related items from being returned in the response in order to improve performance of the request. The array can include any of: `"repositories"`.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub exclude: Option<Vec<String>>,
  pub exclude_attachments: bool,
  pub exclude_git_data: bool,
  pub exclude_metadata: bool,
  pub exclude_owner_projects: bool,
  pub exclude_releases: bool,
  pub guid: String,
  pub id: i64,
  pub lock_repositories: bool,
  pub node_id: String,
  pub org_metadata_only: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub owner: Option<SimpleUser>,
  /// The repositories included in the migration. Only returned for export migrations.
  pub repositories: Vec<Repository>,
  pub state: String,
  pub updated_at: String,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "orgs"))]
/// A fine-grained permission that protects organization resources.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct OrganizationFineGrainedPermission {
  pub description: String,
  pub name: String,
}

#[cfg(any(feature = "full", feature = "orgs"))]
/// Organization roles
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct OrganizationRole {
  /// The date and time the role was created.
  pub created_at: String,
  /// A short description about who this role is for or what permissions it grants.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub description: Option<String>,
  /// The unique identifier of the role.
  pub id: i64,
  /// The name of the role.
  pub name: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub organization: Option<SimpleUser>,
  /// A list of permissions included in this role.
  pub permissions: Vec<String>,
  /// The date and time the role was last updated.
  pub updated_at: String,
}

#[cfg(any(feature = "full", feature = "packages"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ContainerMetadata {
  pub tags: Vec<String>,
}

#[cfg(any(feature = "full", feature = "packages"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct DockerMetadata {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub tag: Option<Vec<String>>,
}

#[cfg(any(feature = "full", feature = "packages"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct PackageVersionMetadata {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub container: Option<ContainerMetadata>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub docker: Option<DockerMetadata>,
  pub package_type: PackageType,
}

#[cfg(any(feature = "full", feature = "packages"))]
/// A version of a software package
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct PackageVersion {
  pub created_at: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub deleted_at: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub description: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub html_url: Option<String>,
  /// Unique identifier of the package version.
  pub id: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub license: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub metadata: Option<PackageVersionMetadata>,
  /// The name of the package version.
  pub name: String,
  pub package_html_url: String,
  pub updated_at: String,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "orgs"))]
/// Permissions requested, categorized by type of permission.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct OrganizationProgrammaticAccessGrantRequestPermissions {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub organization: Option<serde_json::Value>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub other: Option<serde_json::Value>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub repository: Option<serde_json::Value>,
}

#[cfg(any(feature = "full", feature = "orgs"))]
/// Minimal representation of an organization programmatic access grant request for enumerations
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct OrganizationProgrammaticAccessGrantRequest {
  /// Date and time when the request for access was created.
  pub created_at: String,
  /// Unique identifier of the request for access via fine-grained personal access token. The `pat_request_id` used to review PAT requests.
  pub id: i64,
  pub owner: SimpleUser,
  /// Permissions requested, categorized by type of permission.
  pub permissions: OrganizationProgrammaticAccessGrantRequestPermissions,
  /// Reason for requesting access.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub reason: Option<String>,
  /// URL to the list of repositories requested to be accessed via fine-grained personal access token. Should only be followed when `repository_selection` is `subset`.
  pub repositories_url: String,
  /// Type of repository selection requested.
  pub repository_selection: RequestRepositorySelection,
  /// Whether the associated fine-grained personal access token has expired.
  pub token_expired: bool,
  /// Date and time when the associated fine-grained personal access token expires.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub token_expires_at: Option<String>,
  /// Date and time when the associated fine-grained personal access token was last used for authentication.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub token_last_used_at: Option<String>,
}

#[cfg(any(feature = "full", feature = "orgs"))]
/// Permissions requested, categorized by type of permission.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct OrganizationProgrammaticAccessGrantPermissions {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub organization: Option<serde_json::Value>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub other: Option<serde_json::Value>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub repository: Option<serde_json::Value>,
}

#[cfg(any(feature = "full", feature = "orgs"))]
/// Minimal representation of an organization programmatic access grant for enumerations
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct OrganizationProgrammaticAccessGrant {
  /// Date and time when the fine-grained personal access token was approved to access the organization.
  pub access_granted_at: String,
  /// Unique identifier of the fine-grained personal access token. The `pat_id` used to get details about an approved fine-grained personal access token.
  pub id: i64,
  pub owner: SimpleUser,
  /// Permissions requested, categorized by type of permission.
  pub permissions: OrganizationProgrammaticAccessGrantPermissions,
  /// URL to the list of repositories the fine-grained personal access token can access. Only follow when `repository_selection` is `subset`.
  pub repositories_url: String,
  /// Type of repository selection requested.
  pub repository_selection: RequestRepositorySelection,
  /// Whether the associated fine-grained personal access token has expired.
  pub token_expired: bool,
  /// Date and time when the associated fine-grained personal access token expires.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub token_expires_at: Option<String>,
  /// Date and time when the associated fine-grained personal access token was last used for authentication.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub token_last_used_at: Option<String>,
}

#[cfg(any(feature = "full", feature = "projects"))]
/// Projects are a way to organize columns and cards of work.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct Project {
  /// Body of the project
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub body: Option<String>,
  pub columns_url: String,
  pub created_at: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub creator: Option<SimpleUser>,
  pub html_url: String,
  pub id: i64,
  /// Name of the project
  pub name: String,
  pub node_id: String,
  pub number: i64,
  /// The baseline permission that all organization members have on this project. Only present if owner is an organization.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub organization_permission: Option<ProjectOrganizationPermission>,
  pub owner_url: String,
  /// Whether or not this project can be seen by everyone. Only present if owner is an organization.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub private: Option<bool>,
  /// State of the project; either 'open' or 'closed'
  pub state: String,
  pub updated_at: String,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "orgs"))]
/// Custom property defined on an organization
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct OrgCustomProperty {
  /// An ordered list of the allowed values of the property.
  /// The property can have up to 200 allowed values.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub allowed_values: Option<Vec<String>>,
  /// Default value of the property
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub default_value: Option<Vec<String>>,
  /// Short description of the property
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub description: Option<String>,
  /// The name of the property
  pub property_name: String,
  /// Whether the property is required.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub required: Option<bool>,
  /// The type of the value for the property
  pub value_type: OrgCustomPropertyValueType,
  /// Who can edit the values of the property
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub values_editable_by: Option<OrgCustomPropertyValuesEditableBy>,
}

#[cfg(any(feature = "full", feature = "orgs", feature = "repos"))]
/// Custom property name and associated value
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CustomPropertyValue {
  /// The name of the property
  pub property_name: String,
  /// The value assigned to the property
  pub value: Vec<String>,
}

#[cfg(any(feature = "full", feature = "orgs"))]
/// List of custom property values for a repository
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct OrgRepoCustomPropertyValues {
  /// List of custom property names and associated values
  pub properties: Vec<CustomPropertyValue>,
  pub repository_full_name: String,
  pub repository_id: i64,
  pub repository_name: String,
}

#[cfg(any(
  feature = "full",
  feature = "repos",
  feature = "security_advisories",
  feature = "codespaces"
))]
/// Code of Conduct Simple
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CodeOfConductSimple {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub html_url: Option<String>,
  pub key: String,
  pub name: String,
  pub url: String,
}

#[cfg(any(
  feature = "full",
  feature = "repos",
  feature = "security_advisories",
  feature = "codespaces"
))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct FullRepositoryPermissions {
  pub admin: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub maintain: Option<bool>,
  pub pull: bool,
  pub push: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub triage: Option<bool>,
}

#[cfg(any(
  feature = "full",
  feature = "repos",
  feature = "security_advisories",
  feature = "codespaces"
))]
/// Full Repository
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct FullRepository {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub allow_auto_merge: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub allow_forking: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub allow_merge_commit: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub allow_rebase_merge: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub allow_squash_merge: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub allow_update_branch: Option<bool>,
  /// Whether anonymous git access is allowed.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub anonymous_access_enabled: Option<bool>,
  pub archive_url: String,
  pub archived: bool,
  pub assignees_url: String,
  pub blobs_url: String,
  pub branches_url: String,
  pub clone_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub code_of_conduct: Option<CodeOfConductSimple>,
  pub collaborators_url: String,
  pub comments_url: String,
  pub commits_url: String,
  pub compare_url: String,
  pub contents_url: String,
  pub contributors_url: String,
  pub created_at: String,
  /// The custom properties that were defined for the repository. The keys are the custom property names, and the values are the corresponding custom property values.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub custom_properties: Option<serde_json::Value>,
  pub default_branch: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub delete_branch_on_merge: Option<bool>,
  pub deployments_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub description: Option<String>,
  /// Returns whether or not this repository disabled.
  pub disabled: bool,
  pub downloads_url: String,
  pub events_url: String,
  pub fork: bool,
  pub forks: i64,
  pub forks_count: i64,
  pub forks_url: String,
  pub full_name: String,
  pub git_commits_url: String,
  pub git_refs_url: String,
  pub git_tags_url: String,
  pub git_url: String,
  pub has_discussions: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub has_downloads: Option<bool>,
  pub has_issues: bool,
  pub has_pages: bool,
  pub has_projects: bool,
  pub has_wiki: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub homepage: Option<String>,
  pub hooks_url: String,
  pub html_url: String,
  pub id: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub is_template: Option<bool>,
  pub issue_comment_url: String,
  pub issue_events_url: String,
  pub issues_url: String,
  pub keys_url: String,
  pub labels_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub language: Option<String>,
  pub languages_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub license: Option<LicenseSimple>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub master_branch: Option<String>,
  /// The default value for a merge commit message.
  ///
  /// - `PR_TITLE` - default to the pull request's title.
  /// - `PR_BODY` - default to the pull request's body.
  /// - `BLANK` - default to a blank commit message.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub merge_commit_message: Option<MergeCommitMessage>,
  /// The default value for a merge commit title.
  ///
  ///   - `PR_TITLE` - default to the pull request's title.
  ///   - `MERGE_MESSAGE` - default to the classic title for a merge message (e.g., Merge pull request #123 from branch-name).
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub merge_commit_title: Option<MergeCommitTitle>,
  pub merges_url: String,
  pub milestones_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub mirror_url: Option<String>,
  pub name: String,
  pub network_count: i64,
  pub node_id: String,
  pub notifications_url: String,
  pub open_issues: i64,
  pub open_issues_count: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub organization: Option<SimpleUser>,
  pub owner: SimpleUser,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub parent: Option<Repository>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub permissions: Option<FullRepositoryPermissions>,
  pub private: bool,
  pub pulls_url: String,
  pub pushed_at: String,
  pub releases_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub security_and_analysis: Option<SecurityAndAnalysis>,
  /// The size of the repository, in kilobytes. Size is calculated hourly. When a repository is initially created, the size is 0.
  pub size: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub source: Option<Repository>,
  /// The default value for a squash merge commit message:
  ///
  /// - `PR_BODY` - default to the pull request's body.
  /// - `COMMIT_MESSAGES` - default to the branch's commit messages.
  /// - `BLANK` - default to a blank commit message.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub squash_merge_commit_message: Option<SquashMergeCommitMessage>,
  /// The default value for a squash merge commit title:
  ///
  /// - `PR_TITLE` - default to the pull request's title.
  /// - `COMMIT_OR_PR_TITLE` - default to the commit's title (if only one commit) or the pull request's title (when more than one commit).
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub squash_merge_commit_title: Option<SquashMergeCommitTitle>,
  pub ssh_url: String,
  pub stargazers_count: i64,
  pub stargazers_url: String,
  pub statuses_url: String,
  pub subscribers_count: i64,
  pub subscribers_url: String,
  pub subscription_url: String,
  pub svn_url: String,
  pub tags_url: String,
  pub teams_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub temp_clone_token: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub template_repository: Option<Repository>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub topics: Option<Vec<String>>,
  pub trees_url: String,
  pub updated_at: String,
  pub url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub use_squash_pr_title_as_default: Option<bool>,
  /// The repository visibility: public, private, or internal.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub visibility: Option<String>,
  pub watchers: i64,
  pub watchers_count: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub web_commit_signoff_required: Option<bool>,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RepositoryRulesetLinksHtml {
  /// The html URL of the ruleset
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub href: Option<String>,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RepositoryRulesetLinksSelf {
  /// The URL of the ruleset
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub href: Option<String>,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RepositoryRulesetLinks {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub html: Option<RepositoryRulesetLinksHtml>,
  #[serde(rename = "self")]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub self_: Option<RepositoryRulesetLinksSelf>,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// An actor that can bypass rules in a ruleset
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RepositoryRulesetBypassActor {
  /// The ID of the actor that can bypass a ruleset. If `actor_type` is `OrganizationAdmin`, this should be `1`.
  pub actor_id: i64,
  /// The type of actor that can bypass a ruleset
  pub actor_type: RepositoryRulesetBypassActorActorType,
  /// When the specified actor can bypass the ruleset. `pull_request` means that an actor can only bypass rules on pull requests.
  pub bypass_mode: RepositoryRulesetBypassActorBypassMode,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RepositoryRulesetConditionsRefName {
  /// Array of ref names or patterns to exclude. The condition will not pass if any of these patterns match.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub exclude: Option<Vec<String>>,
  /// Array of ref names or patterns to include. One of these patterns must match for the condition to pass. Also accepts `~DEFAULT_BRANCH` to include the default branch or `~ALL` to include all branches.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub include: Option<Vec<String>>,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// Parameters for a repository ruleset ref name condition
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RepositoryRulesetConditions {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub ref_name: Option<RepositoryRulesetConditionsRefName>,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RepositoryRulesetConditionsRepositoryNameTargetRepositoryName {
  /// Array of repository names or patterns to exclude. The condition will not pass if any of these patterns match.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub exclude: Option<Vec<String>>,
  /// Array of repository names or patterns to include. One of these patterns must match for the condition to pass. Also accepts `~ALL` to include all repositories.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub include: Option<Vec<String>>,
  /// Whether renaming of target repositories is prevented.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub protected: Option<bool>,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// Parameters for a repository name condition
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RepositoryRulesetConditionsRepositoryNameTarget {
  pub repository_name: RepositoryRulesetConditionsRepositoryNameTargetRepositoryName,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RepositoryRulesetConditionsRepositoryIdTargetRepositoryId {
  /// The repository IDs that the ruleset applies to. One of these IDs must match for the condition to pass.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub repository_ids: Option<Vec<i64>>,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// Parameters for a repository ID condition
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RepositoryRulesetConditionsRepositoryIdTarget {
  pub repository_id: RepositoryRulesetConditionsRepositoryIdTargetRepositoryId,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// Parameters for a targeting a repository property
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RepositoryRulesetConditionsRepositoryPropertySpec {
  /// The name of the repository property to target
  pub name: String,
  /// The values to match for the repository property
  pub property_values: Vec<String>,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RepositoryRulesetConditionsRepositoryPropertyTargetRepositoryProperty {
  /// The repository properties and values to exclude. The condition will not pass if any of these properties match.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub exclude: Option<Vec<RepositoryRulesetConditionsRepositoryPropertySpec>>,
  /// The repository properties and values to include. All of these properties must match for the condition to pass.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub include: Option<Vec<RepositoryRulesetConditionsRepositoryPropertySpec>>,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// Parameters for a repository property condition
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RepositoryRulesetConditionsRepositoryPropertyTarget {
  pub repository_property: RepositoryRulesetConditionsRepositoryPropertyTargetRepositoryProperty,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// Only allow users with bypass permission to create matching refs.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RepositoryRuleCreation {
  #[serde(rename = "type")]
  pub type_: RepositoryRuleCreationType,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RepositoryRuleUpdateParameters {
  /// Branch can pull changes from its upstream repository
  pub update_allows_fetch_and_merge: bool,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// Only allow users with bypass permission to update matching refs.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RepositoryRuleUpdate {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub parameters: Option<RepositoryRuleUpdateParameters>,
  #[serde(rename = "type")]
  pub type_: RepositoryRuleUpdateType,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// Only allow users with bypass permissions to delete matching refs.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RepositoryRuleDeletion {
  #[serde(rename = "type")]
  pub type_: RepositoryRuleDeletionType,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// Prevent merge commits from being pushed to matching refs.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RepositoryRuleRequiredLinearHistory {
  #[serde(rename = "type")]
  pub type_: RepositoryRuleRequiredLinearHistoryType,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RepositoryRuleRequiredDeploymentsParameters {
  /// The environments that must be successfully deployed to before branches can be merged.
  pub required_deployment_environments: Vec<String>,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// Choose which environments must be successfully deployed to before refs can be pushed into a ref that matches this rule.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RepositoryRuleRequiredDeployments {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub parameters: Option<RepositoryRuleRequiredDeploymentsParameters>,
  #[serde(rename = "type")]
  pub type_: RepositoryRuleRequiredDeploymentsType,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// Commits pushed to matching refs must have verified signatures.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RepositoryRuleRequiredSignatures {
  #[serde(rename = "type")]
  pub type_: RepositoryRuleRequiredSignaturesType,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RepositoryRulePullRequestParameters {
  /// New, reviewable commits pushed will dismiss previous pull request review approvals.
  pub dismiss_stale_reviews_on_push: bool,
  /// Require an approving review in pull requests that modify files that have a designated code owner.
  pub require_code_owner_review: bool,
  /// Whether the most recent reviewable push must be approved by someone other than the person who pushed it.
  pub require_last_push_approval: bool,
  /// The number of approving reviews that are required before a pull request can be merged.
  pub required_approving_review_count: i64,
  /// All conversations on code must be resolved before a pull request can be merged.
  pub required_review_thread_resolution: bool,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// Require all commits be made to a non-target branch and submitted via a pull request before they can be merged.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RepositoryRulePullRequest {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub parameters: Option<RepositoryRulePullRequestParameters>,
  #[serde(rename = "type")]
  pub type_: RepositoryRulePullRequestType,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// Required status check
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RepositoryRuleParamsStatusCheckConfiguration {
  /// The status check context name that must be present on the commit.
  pub context: String,
  /// The optional integration ID that this status check must originate from.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub integration_id: Option<i64>,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RepositoryRuleRequiredStatusChecksParameters {
  /// Status checks that are required.
  pub required_status_checks: Vec<RepositoryRuleParamsStatusCheckConfiguration>,
  /// Whether pull requests targeting a matching branch must be tested with the latest code. This setting will not take effect unless at least one status check is enabled.
  pub strict_required_status_checks_policy: bool,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// Choose which status checks must pass before the ref is updated. When enabled, commits must first be pushed to another ref where the checks pass.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RepositoryRuleRequiredStatusChecks {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub parameters: Option<RepositoryRuleRequiredStatusChecksParameters>,
  #[serde(rename = "type")]
  pub type_: RepositoryRuleRequiredStatusChecksType,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// Prevent users with push access from force pushing to refs.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RepositoryRuleNonFastForward {
  #[serde(rename = "type")]
  pub type_: RepositoryRuleNonFastForwardType,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RepositoryRuleCommitMessagePatternParameters {
  /// How this rule will appear to users.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub name: Option<String>,
  /// If true, the rule will fail if the pattern matches.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub negate: Option<bool>,
  /// The operator to use for matching.
  pub operator: Operator,
  /// The pattern to match with.
  pub pattern: String,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// Parameters to be used for the commit_message_pattern rule
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RepositoryRuleCommitMessagePattern {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub parameters: Option<RepositoryRuleCommitMessagePatternParameters>,
  #[serde(rename = "type")]
  pub type_: RepositoryRuleCommitMessagePatternType,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RepositoryRuleCommitAuthorEmailPatternParameters {
  /// How this rule will appear to users.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub name: Option<String>,
  /// If true, the rule will fail if the pattern matches.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub negate: Option<bool>,
  /// The operator to use for matching.
  pub operator: Operator,
  /// The pattern to match with.
  pub pattern: String,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// Parameters to be used for the commit_author_email_pattern rule
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RepositoryRuleCommitAuthorEmailPattern {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub parameters: Option<RepositoryRuleCommitAuthorEmailPatternParameters>,
  #[serde(rename = "type")]
  pub type_: RepositoryRuleCommitAuthorEmailPatternType,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RepositoryRuleCommitterEmailPatternParameters {
  /// How this rule will appear to users.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub name: Option<String>,
  /// If true, the rule will fail if the pattern matches.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub negate: Option<bool>,
  /// The operator to use for matching.
  pub operator: Operator,
  /// The pattern to match with.
  pub pattern: String,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// Parameters to be used for the committer_email_pattern rule
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RepositoryRuleCommitterEmailPattern {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub parameters: Option<RepositoryRuleCommitterEmailPatternParameters>,
  #[serde(rename = "type")]
  pub type_: RepositoryRuleCommitterEmailPatternType,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RepositoryRuleBranchNamePatternParameters {
  /// How this rule will appear to users.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub name: Option<String>,
  /// If true, the rule will fail if the pattern matches.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub negate: Option<bool>,
  /// The operator to use for matching.
  pub operator: Operator,
  /// The pattern to match with.
  pub pattern: String,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// Parameters to be used for the branch_name_pattern rule
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RepositoryRuleBranchNamePattern {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub parameters: Option<RepositoryRuleBranchNamePatternParameters>,
  #[serde(rename = "type")]
  pub type_: RepositoryRuleBranchNamePatternType,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RepositoryRuleTagNamePatternParameters {
  /// How this rule will appear to users.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub name: Option<String>,
  /// If true, the rule will fail if the pattern matches.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub negate: Option<bool>,
  /// The operator to use for matching.
  pub operator: Operator,
  /// The pattern to match with.
  pub pattern: String,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// Parameters to be used for the tag_name_pattern rule
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RepositoryRuleTagNamePattern {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub parameters: Option<RepositoryRuleTagNamePatternParameters>,
  #[serde(rename = "type")]
  pub type_: RepositoryRuleTagNamePatternType,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// A workflow that must run for this rule to pass
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RepositoryRuleParamsWorkflowFileReference {
  /// The path to the workflow file
  pub path: String,
  /// The ref (branch or tag) of the workflow file to use
  #[serde(rename = "ref")]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub ref_: Option<String>,
  /// The ID of the repository where the workflow is defined
  pub repository_id: i64,
  /// The commit SHA of the workflow file to use
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub sha: Option<String>,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RepositoryRuleWorkflowsParameters {
  /// Workflows that must pass for this rule to pass.
  pub workflows: Vec<RepositoryRuleParamsWorkflowFileReference>,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// Require all changes made to a targeted branch to pass the specified workflows before they can be merged.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RepositoryRuleWorkflows {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub parameters: Option<RepositoryRuleWorkflowsParameters>,
  #[serde(rename = "type")]
  pub type_: RepositoryRuleWorkflowsType,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// A set of rules to apply when specified conditions are met.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RepositoryRuleset {
  #[serde(rename = "_links")]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub links: Option<RepositoryRulesetLinks>,
  /// The actors that can bypass the rules in this ruleset
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub bypass_actors: Option<Vec<RepositoryRulesetBypassActor>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub conditions: Option<RepositoryRulesetConditions>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub created_at: Option<String>,
  /// The bypass type of the user making the API request for this ruleset. This field is only returned when
  /// querying the repository-level endpoint.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub current_user_can_bypass: Option<RepositoryRulesetCurrentUserCanBypass>,
  pub enforcement: RepositoryRuleEnforcement,
  /// The ID of the ruleset
  pub id: i64,
  /// The name of the ruleset
  pub name: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub node_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub rules: Option<Vec<RepositoryRule>>,
  /// The name of the source
  pub source: String,
  /// The type of the source of the ruleset
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub source_type: Option<RepositoryRulesetSourceType>,
  /// The target of the ruleset
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub target: Option<RepositoryRulesetTarget>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub updated_at: Option<String>,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RuleSuitesItem {
  /// The number that identifies the user.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub actor_id: Option<i64>,
  /// The handle for the GitHub user account.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub actor_name: Option<String>,
  /// The last commit sha in the push evaluation.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub after_sha: Option<String>,
  /// The first commit sha before the push evaluation.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub before_sha: Option<String>,
  /// The result of the rule evaluations for rules with the `active` and `evaluate` enforcement statuses, demonstrating whether rules would pass or fail if all rules in the rule suite were `active`.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub evaluation_result: Option<RuleSuitesItemEvaluationResult>,
  /// The unique identifier of the rule insight.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub id: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub pushed_at: Option<String>,
  /// The ref name that the evaluation ran on.
  #[serde(rename = "ref")]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub ref_: Option<String>,
  /// The ID of the repository associated with the rule evaluation.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub repository_id: Option<i64>,
  /// The name of the repository without the `.git` extension.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub repository_name: Option<String>,
  /// The result of the rule evaluations for rules with the `active` enforcement status.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub result: Option<RuleSuitesItemResult>,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RuleSuiteRuleEvaluationsRuleSource {
  /// The ID of the rule source.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub id: Option<i64>,
  /// The name of the rule source.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub name: Option<String>,
  /// The type of rule source.
  #[serde(rename = "type")]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub type_: Option<String>,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RuleSuiteRuleEvaluations {
  /// Any associated details with the rule evaluation.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub details: Option<String>,
  /// The enforcement level of this rule source.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub enforcement: Option<RuleSuiteRuleEvaluationsEnforcement>,
  /// The result of the evaluation of the individual rule.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub result: Option<RuleSuiteRuleEvaluationsResult>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub rule_source: Option<RuleSuiteRuleEvaluationsRuleSource>,
  /// The type of rule.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub rule_type: Option<String>,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// Response
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RuleSuite {
  /// The number that identifies the user.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub actor_id: Option<i64>,
  /// The handle for the GitHub user account.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub actor_name: Option<String>,
  /// The last commit sha in the push evaluation.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub after_sha: Option<String>,
  /// The first commit sha before the push evaluation.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub before_sha: Option<String>,
  /// The result of the rule evaluations for rules with the `active` and `evaluate` enforcement statuses, demonstrating whether rules would pass or fail if all rules in the rule suite were `active`.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub evaluation_result: Option<RuleSuiteEvaluationResult>,
  /// The unique identifier of the rule insight.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub id: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub pushed_at: Option<String>,
  /// The ref name that the evaluation ran on.
  #[serde(rename = "ref")]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub ref_: Option<String>,
  /// The ID of the repository associated with the rule evaluation.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub repository_id: Option<i64>,
  /// The name of the repository without the `.git` extension.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub repository_name: Option<String>,
  /// The result of the rule evaluations for rules with the `active` enforcement status.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub result: Option<RuleSuiteResult>,
  /// Details on the evaluated rules.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub rule_evaluations: Option<Vec<RuleSuiteRuleEvaluations>>,
}

#[cfg(any(feature = "full", feature = "security_advisories"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RepositoryAdvisoryCredits {
  /// The username of the user credited.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub login: Option<String>,
  #[serde(rename = "type")]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub type_: Option<SecurityAdvisoryCreditTypes>,
}

#[cfg(any(feature = "full", feature = "security_advisories"))]
/// A credit given to a user for a repository security advisory.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RepositoryAdvisoryCredit {
  /// The state of the user's acceptance of the credit.
  pub state: RepositoryAdvisoryCreditState,
  #[serde(rename = "type")]
  pub type_: SecurityAdvisoryCreditTypes,
  pub user: SimpleUser,
}

#[cfg(any(feature = "full", feature = "security_advisories"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RepositoryAdvisoryCvss {
  /// The CVSS score.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub score: Option<f64>,
  /// The CVSS vector.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub vector_string: Option<String>,
}

#[cfg(any(feature = "full", feature = "security_advisories"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RepositoryAdvisoryCwes {
  /// The Common Weakness Enumeration (CWE) identifier.
  pub cwe_id: String,
  /// The name of the CWE.
  pub name: String,
}

#[cfg(any(feature = "full", feature = "security_advisories"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RepositoryAdvisoryIdentifiers {
  /// The type of identifier.
  #[serde(rename = "type")]
  pub type_: IdentifiersType,
  /// The identifier value.
  pub value: String,
}

#[cfg(any(feature = "full", feature = "security_advisories"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RepositoryAdvisorySubmission {
  /// Whether a private vulnerability report was accepted by the repository's administrators.
  pub accepted: bool,
}

#[cfg(any(feature = "full", feature = "security_advisories"))]
/// The name of the package affected by the vulnerability.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RepositoryAdvisoryVulnerabilityPackage {
  pub ecosystem: SecurityAdvisoryEcosystems,
  /// The unique package name within its ecosystem.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub name: Option<String>,
}

#[cfg(any(feature = "full", feature = "security_advisories"))]
/// A product affected by the vulnerability detailed in a repository security advisory.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RepositoryAdvisoryVulnerability {
  /// The name of the package affected by the vulnerability.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub package: Option<RepositoryAdvisoryVulnerabilityPackage>,
  /// The package version(s) that resolve the vulnerability.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub patched_versions: Option<String>,
  /// The functions in the package that are affected.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub vulnerable_functions: Option<Vec<String>>,
  /// The range of the package versions affected by the vulnerability.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub vulnerable_version_range: Option<String>,
}

#[cfg(any(feature = "full", feature = "security_advisories"))]
/// A repository security advisory.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RepositoryAdvisory {
  /// The author of the advisory.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub author: Option<SimpleUser>,
  /// The date and time of when the advisory was closed, in ISO 8601 format.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub closed_at: Option<String>,
  /// A list of teams that collaborate on the advisory.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub collaborating_teams: Option<Vec<Team>>,
  /// A list of users that collaborate on the advisory.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub collaborating_users: Option<Vec<SimpleUser>>,
  /// The date and time of when the advisory was created, in ISO 8601 format.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub created_at: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub credits: Option<Vec<RepositoryAdvisoryCredits>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub credits_detailed: Option<Vec<RepositoryAdvisoryCredit>>,
  /// The Common Vulnerabilities and Exposures (CVE) ID.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub cve_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub cvss: Option<RepositoryAdvisoryCvss>,
  /// A list of only the CWE IDs.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub cwe_ids: Option<Vec<String>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub cwes: Option<Vec<RepositoryAdvisoryCwes>>,
  /// A detailed description of what the advisory entails.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub description: Option<String>,
  /// The GitHub Security Advisory ID.
  pub ghsa_id: String,
  /// The URL for the advisory.
  pub html_url: String,
  pub identifiers: Vec<RepositoryAdvisoryIdentifiers>,
  /// A temporary private fork of the advisory's repository for collaborating on a fix.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub private_fork: Option<SimpleRepository>,
  /// The date and time of when the advisory was published, in ISO 8601 format.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub published_at: Option<String>,
  /// The publisher of the advisory.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub publisher: Option<SimpleUser>,
  /// The severity of the advisory.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub severity: Option<RepositoryAdvisorySeverity>,
  /// The state of the advisory.
  pub state: RepositoryAdvisoryState,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub submission: Option<RepositoryAdvisorySubmission>,
  /// A short summary of the advisory.
  pub summary: String,
  /// The date and time of when the advisory was last updated, in ISO 8601 format.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub updated_at: Option<String>,
  /// The API URL for the advisory.
  pub url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub vulnerabilities: Option<Vec<RepositoryAdvisoryVulnerability>>,
  /// The date and time of when the advisory was withdrawn, in ISO 8601 format.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub withdrawn_at: Option<String>,
}

#[cfg(any(feature = "full", feature = "billing"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ActionsBillingUsageMinutesUsedBreakdown {
  /// Total minutes used on macOS runner machines.
  #[serde(rename = "MACOS")]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub macos: Option<i64>,
  /// Total minutes used on Ubuntu runner machines.
  #[serde(rename = "UBUNTU")]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub ubuntu: Option<i64>,
  /// Total minutes used on Windows runner machines.
  #[serde(rename = "WINDOWS")]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub windows: Option<i64>,
  /// Total minutes used on macOS 12 core runner machines.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub macos_12_core: Option<i64>,
  /// Total minutes used on all runner machines.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub total: Option<i64>,
  /// Total minutes used on Ubuntu 16 core runner machines.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub ubuntu_16_core: Option<i64>,
  /// Total minutes used on Ubuntu 32 core runner machines.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub ubuntu_32_core: Option<i64>,
  /// Total minutes used on Ubuntu 4 core runner machines.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub ubuntu_4_core: Option<i64>,
  /// Total minutes used on Ubuntu 64 core runner machines.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub ubuntu_64_core: Option<i64>,
  /// Total minutes used on Ubuntu 8 core runner machines.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub ubuntu_8_core: Option<i64>,
  /// Total minutes used on Windows 16 core runner machines.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub windows_16_core: Option<i64>,
  /// Total minutes used on Windows 32 core runner machines.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub windows_32_core: Option<i64>,
  /// Total minutes used on Windows 4 core runner machines.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub windows_4_core: Option<i64>,
  /// Total minutes used on Windows 64 core runner machines.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub windows_64_core: Option<i64>,
  /// Total minutes used on Windows 8 core runner machines.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub windows_8_core: Option<i64>,
}

#[cfg(any(feature = "full", feature = "billing"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ActionsBillingUsage {
  /// The amount of free GitHub Actions minutes available.
  pub included_minutes: i64,
  pub minutes_used_breakdown: ActionsBillingUsageMinutesUsedBreakdown,
  /// The sum of the free and paid GitHub Actions minutes used.
  pub total_minutes_used: i64,
  /// The total paid GitHub Actions minutes used.
  pub total_paid_minutes_used: i64,
}

#[cfg(any(feature = "full", feature = "billing"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct PackagesBillingUsage {
  /// Free storage space (GB) for GitHub Packages.
  pub included_gigabytes_bandwidth: i64,
  /// Sum of the free and paid storage space (GB) for GitHuub Packages.
  pub total_gigabytes_bandwidth_used: i64,
  /// Total paid storage space (GB) for GitHuub Packages.
  pub total_paid_gigabytes_bandwidth_used: i64,
}

#[cfg(any(feature = "full", feature = "billing"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CombinedBillingUsage {
  /// Numbers of days left in billing cycle.
  pub days_left_in_billing_cycle: i64,
  /// Estimated storage space (GB) used in billing cycle.
  pub estimated_paid_storage_for_month: i64,
  /// Estimated sum of free and paid storage space (GB) used in billing cycle.
  pub estimated_storage_for_month: i64,
}

#[cfg(any(feature = "full", feature = "teams"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct TeamOrganizationPlan {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub filled_seats: Option<i64>,
  pub name: String,
  pub private_repos: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub seats: Option<i64>,
  pub space: i64,
}

#[cfg(any(feature = "full", feature = "teams"))]
/// Team Organization
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct TeamOrganization {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub archived_at: Option<String>,
  pub avatar_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub billing_email: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub blog: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub collaborators: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub company: Option<String>,
  pub created_at: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub default_repository_permission: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub description: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub disk_usage: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub email: Option<String>,
  pub events_url: String,
  pub followers: i64,
  pub following: i64,
  pub has_organization_projects: bool,
  pub has_repository_projects: bool,
  pub hooks_url: String,
  pub html_url: String,
  pub id: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub is_verified: Option<bool>,
  pub issues_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub location: Option<String>,
  pub login: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub members_allowed_repository_creation_type: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub members_can_create_internal_repositories: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub members_can_create_pages: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub members_can_create_private_pages: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub members_can_create_private_repositories: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub members_can_create_public_pages: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub members_can_create_public_repositories: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub members_can_create_repositories: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub members_can_fork_private_repositories: Option<bool>,
  pub members_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub name: Option<String>,
  pub node_id: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub owned_private_repos: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub plan: Option<TeamOrganizationPlan>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub private_gists: Option<i64>,
  pub public_gists: i64,
  pub public_members_url: String,
  pub public_repos: i64,
  pub repos_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub total_private_repos: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub twitter_username: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub two_factor_requirement_enabled: Option<bool>,
  #[serde(rename = "type")]
  pub type_: String,
  pub updated_at: String,
  pub url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub web_commit_signoff_required: Option<bool>,
}

#[cfg(any(feature = "full", feature = "teams"))]
/// Groups of organization members that gives permissions on specified repositories.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct TeamFull {
  pub created_at: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub description: Option<String>,
  pub html_url: String,
  /// Unique identifier of the team
  pub id: i64,
  /// Distinguished Name (DN) that team maps to within LDAP environment
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub ldap_dn: Option<String>,
  pub members_count: i64,
  pub members_url: String,
  /// Name of the team
  pub name: String,
  pub node_id: String,
  /// The notification setting the team has set
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub notification_setting: Option<TeamFullNotificationSetting>,
  pub organization: TeamOrganization,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub parent: Option<TeamSimple>,
  /// Permission that the team will have for its repositories
  pub permission: String,
  /// The level of privacy this team should have
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub privacy: Option<TeamFullPrivacy>,
  pub repos_count: i64,
  pub repositories_url: String,
  pub slug: String,
  pub updated_at: String,
  /// URL for the team
  pub url: String,
}

#[cfg(any(feature = "full", feature = "teams"))]
/// A team discussion is a persistent record of a free-form conversation within a team.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct TeamDiscussion {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub author: Option<SimpleUser>,
  /// The main text of the discussion.
  pub body: String,
  pub body_html: String,
  /// The current version of the body content. If provided, this update operation will be rejected if the given version does not match the latest version on the server.
  pub body_version: String,
  pub comments_count: i64,
  pub comments_url: String,
  pub created_at: String,
  pub html_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub last_edited_at: Option<String>,
  pub node_id: String,
  /// The unique sequence number of a team discussion.
  pub number: i64,
  /// Whether or not this discussion should be pinned for easy retrieval.
  pub pinned: bool,
  /// Whether or not this discussion should be restricted to team members and organization owners.
  pub private: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub reactions: Option<ReactionRollup>,
  pub team_url: String,
  /// The title of the discussion.
  pub title: String,
  pub updated_at: String,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "teams"))]
/// A reply to a discussion within a team.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct TeamDiscussionComment {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub author: Option<SimpleUser>,
  /// The main text of the comment.
  pub body: String,
  pub body_html: String,
  /// The current version of the body content. If provided, this update operation will be rejected if the given version does not match the latest version on the server.
  pub body_version: String,
  pub created_at: String,
  pub discussion_url: String,
  pub html_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub last_edited_at: Option<String>,
  pub node_id: String,
  /// The unique sequence number of a team discussion comment.
  pub number: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub reactions: Option<ReactionRollup>,
  pub updated_at: String,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "reactions"))]
/// Reactions to conversations provide a way to help people express their feelings more simply and effectively.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct Reaction {
  /// The reaction to use
  pub content: ReactionContent,
  pub created_at: String,
  pub id: i64,
  pub node_id: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub user: Option<SimpleUser>,
}

#[cfg(any(feature = "full", feature = "teams"))]
/// Team Membership
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct TeamMembership {
  /// The role of the user in the team.
  pub role: TeamMembershipRole,
  /// The state of the user's membership in the team.
  pub state: TeamMembershipState,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "teams"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct TeamProjectPermissions {
  pub admin: bool,
  pub read: bool,
  pub write: bool,
}

#[cfg(any(feature = "full", feature = "teams"))]
/// A team's access to a project.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct TeamProject {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub body: Option<String>,
  pub columns_url: String,
  pub created_at: String,
  pub creator: SimpleUser,
  pub html_url: String,
  pub id: i64,
  pub name: String,
  pub node_id: String,
  pub number: i64,
  /// The organization permission for this project. Only present when owner is an organization.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub organization_permission: Option<String>,
  pub owner_url: String,
  pub permissions: TeamProjectPermissions,
  /// Whether the project is private or not. Only present when owner is an organization.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub private: Option<bool>,
  pub state: String,
  pub updated_at: String,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "teams"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct TeamRepositoryPermissions {
  pub admin: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub maintain: Option<bool>,
  pub pull: bool,
  pub push: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub triage: Option<bool>,
}

#[cfg(any(feature = "full", feature = "teams"))]
/// A team's access to a repository.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct TeamRepository {
  /// Whether to allow Auto-merge to be used on pull requests.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub allow_auto_merge: Option<bool>,
  /// Whether to allow forking this repo
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub allow_forking: Option<bool>,
  /// Whether to allow merge commits for pull requests.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub allow_merge_commit: Option<bool>,
  /// Whether to allow rebase merges for pull requests.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub allow_rebase_merge: Option<bool>,
  /// Whether to allow squash merges for pull requests.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub allow_squash_merge: Option<bool>,
  pub archive_url: String,
  /// Whether the repository is archived.
  pub archived: bool,
  pub assignees_url: String,
  pub blobs_url: String,
  pub branches_url: String,
  pub clone_url: String,
  pub collaborators_url: String,
  pub comments_url: String,
  pub commits_url: String,
  pub compare_url: String,
  pub contents_url: String,
  pub contributors_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub created_at: Option<String>,
  /// The default branch of the repository.
  pub default_branch: String,
  /// Whether to delete head branches when pull requests are merged
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub delete_branch_on_merge: Option<bool>,
  pub deployments_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub description: Option<String>,
  /// Returns whether or not this repository disabled.
  pub disabled: bool,
  pub downloads_url: String,
  pub events_url: String,
  pub fork: bool,
  pub forks: i64,
  pub forks_count: i64,
  pub forks_url: String,
  pub full_name: String,
  pub git_commits_url: String,
  pub git_refs_url: String,
  pub git_tags_url: String,
  pub git_url: String,
  /// Whether downloads are enabled.
  pub has_downloads: bool,
  /// Whether issues are enabled.
  pub has_issues: bool,
  pub has_pages: bool,
  /// Whether projects are enabled.
  pub has_projects: bool,
  /// Whether the wiki is enabled.
  pub has_wiki: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub homepage: Option<String>,
  pub hooks_url: String,
  pub html_url: String,
  /// Unique identifier of the repository
  pub id: i64,
  /// Whether this repository acts as a template that can be used to generate new repositories.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub is_template: Option<bool>,
  pub issue_comment_url: String,
  pub issue_events_url: String,
  pub issues_url: String,
  pub keys_url: String,
  pub labels_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub language: Option<String>,
  pub languages_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub license: Option<LicenseSimple>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub master_branch: Option<String>,
  pub merges_url: String,
  pub milestones_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub mirror_url: Option<String>,
  /// The name of the repository.
  pub name: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub network_count: Option<i64>,
  pub node_id: String,
  pub notifications_url: String,
  pub open_issues: i64,
  pub open_issues_count: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub owner: Option<SimpleUser>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub permissions: Option<TeamRepositoryPermissions>,
  /// Whether the repository is private or public.
  pub private: bool,
  pub pulls_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub pushed_at: Option<String>,
  pub releases_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub role_name: Option<String>,
  pub size: i64,
  pub ssh_url: String,
  pub stargazers_count: i64,
  pub stargazers_url: String,
  pub statuses_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub subscribers_count: Option<i64>,
  pub subscribers_url: String,
  pub subscription_url: String,
  pub svn_url: String,
  pub tags_url: String,
  pub teams_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub temp_clone_token: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub topics: Option<Vec<String>>,
  pub trees_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub updated_at: Option<String>,
  pub url: String,
  /// The repository visibility: public, private, or internal.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub visibility: Option<String>,
  pub watchers: i64,
  pub watchers_count: i64,
  /// Whether to require contributors to sign off on web-based commits
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub web_commit_signoff_required: Option<bool>,
}

#[cfg(any(feature = "full", feature = "projects"))]
/// Project cards represent a scope of work.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ProjectCard {
  /// Whether or not the card is archived
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub archived: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub column_name: Option<String>,
  pub column_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub content_url: Option<String>,
  pub created_at: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub creator: Option<SimpleUser>,
  /// The project card's ID
  pub id: i64,
  pub node_id: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub note: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub project_id: Option<String>,
  pub project_url: String,
  pub updated_at: String,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "projects"))]
/// Project columns contain cards of work.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ProjectColumn {
  pub cards_url: String,
  pub created_at: String,
  /// The unique identifier of the project column
  pub id: i64,
  /// Name of the project column
  pub name: String,
  pub node_id: String,
  pub project_url: String,
  pub updated_at: String,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "projects"))]
/// Project Collaborator Permission
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ProjectCollaboratorPermission {
  pub permission: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub user: Option<SimpleUser>,
}

#[cfg(any(feature = "full", feature = "rate_limit"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RateLimit {
  pub limit: i64,
  pub remaining: i64,
  pub reset: i64,
  pub used: i64,
}

#[cfg(any(feature = "full", feature = "rate_limit"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RateLimitOverviewResources {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub actions_runner_registration: Option<RateLimit>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub code_scanning_upload: Option<RateLimit>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub code_search: Option<RateLimit>,
  pub core: RateLimit,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub dependency_snapshots: Option<RateLimit>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub graphql: Option<RateLimit>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub integration_manifest: Option<RateLimit>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub scim: Option<RateLimit>,
  pub search: RateLimit,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub source_import: Option<RateLimit>,
}

#[cfg(any(feature = "full", feature = "rate_limit"))]
/// Rate Limit Overview
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RateLimitOverview {
  pub rate: RateLimit,
  pub resources: RateLimitOverviewResources,
}

#[cfg(any(feature = "full", feature = "actions"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ArtifactWorkflowRun {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub head_branch: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub head_repository_id: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub head_sha: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub id: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub repository_id: Option<i64>,
}

#[cfg(any(feature = "full", feature = "actions"))]
/// An artifact
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct Artifact {
  pub archive_download_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub created_at: Option<String>,
  /// Whether or not the artifact has expired.
  pub expired: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub expires_at: Option<String>,
  pub id: i64,
  /// The name of the artifact.
  pub name: String,
  pub node_id: String,
  /// The size in bytes of the artifact.
  pub size_in_bytes: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub updated_at: Option<String>,
  pub url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub workflow_run: Option<ArtifactWorkflowRun>,
}

#[cfg(any(feature = "full", feature = "actions"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ActionsCacheListActionsCaches {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub created_at: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub id: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub key: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub last_accessed_at: Option<String>,
  #[serde(rename = "ref")]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub ref_: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub size_in_bytes: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub version: Option<String>,
}

#[cfg(any(feature = "full", feature = "actions"))]
/// Repository actions caches
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ActionsCacheList {
  /// Array of caches
  pub actions_caches: Vec<ActionsCacheListActionsCaches>,
  /// Total number of caches
  pub total_count: i64,
}

#[cfg(any(feature = "full", feature = "actions"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct JobSteps {
  /// The time that the job finished, in ISO 8601 format.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub completed_at: Option<String>,
  /// The outcome of the job.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub conclusion: Option<String>,
  /// The name of the job.
  pub name: String,
  pub number: i64,
  /// The time that the step started, in ISO 8601 format.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub started_at: Option<String>,
  /// The phase of the lifecycle that the job is currently in.
  pub status: JobStepsStatus,
}

#[cfg(any(feature = "full", feature = "actions"))]
/// Information of a job execution in a workflow run
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct Job {
  pub check_run_url: String,
  /// The time that the job finished, in ISO 8601 format.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub completed_at: Option<String>,
  /// The outcome of the job.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub conclusion: Option<JobConclusion>,
  /// The time that the job created, in ISO 8601 format.
  pub created_at: String,
  /// The name of the current branch.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub head_branch: Option<String>,
  /// The SHA of the commit that is being run.
  pub head_sha: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub html_url: Option<String>,
  /// The id of the job.
  pub id: i64,
  /// Labels for the workflow job. Specified by the "runs_on" attribute in the action's workflow file.
  pub labels: Vec<String>,
  /// The name of the job.
  pub name: String,
  pub node_id: String,
  /// Attempt number of the associated workflow run, 1 for first attempt and higher if the workflow was re-run.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub run_attempt: Option<i64>,
  /// The id of the associated workflow run.
  pub run_id: i64,
  pub run_url: String,
  /// The ID of the runner group to which this job has been assigned. (If a runner hasn't yet been assigned, this will be null.)
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub runner_group_id: Option<i64>,
  /// The name of the runner group to which this job has been assigned. (If a runner hasn't yet been assigned, this will be null.)
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub runner_group_name: Option<String>,
  /// The ID of the runner to which this job has been assigned. (If a runner hasn't yet been assigned, this will be null.)
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub runner_id: Option<i64>,
  /// The name of the runner to which this job has been assigned. (If a runner hasn't yet been assigned, this will be null.)
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub runner_name: Option<String>,
  /// The time that the job started, in ISO 8601 format.
  pub started_at: String,
  /// The phase of the lifecycle that the job is currently in.
  pub status: JobStatus,
  /// Steps in this job.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub steps: Option<Vec<JobSteps>>,
  pub url: String,
  /// The name of the workflow.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub workflow_name: Option<String>,
}

#[cfg(any(feature = "full", feature = "actions"))]
/// Actions OIDC subject customization for a repository
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct OidcCustomSubRepo {
  /// Array of unique strings. Each claim key can only contain alphanumeric characters and underscores.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub include_claim_keys: Option<Vec<String>>,
  /// Whether to use the default template or not. If `true`, the `include_claim_keys` field is ignored.
  pub use_default: bool,
}

#[cfg(any(feature = "full", feature = "actions"))]
/// Set secrets for GitHub Actions.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ActionsSecret {
  pub created_at: String,
  /// The name of the secret.
  pub name: String,
  pub updated_at: String,
}

#[cfg(any(feature = "full", feature = "actions"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ActionsVariable {
  /// The date and time at which the variable was created, in ISO 8601 format':' YYYY-MM-DDTHH:MM:SSZ.
  pub created_at: String,
  /// The name of the variable.
  pub name: String,
  /// The date and time at which the variable was last updated, in ISO 8601 format':' YYYY-MM-DDTHH:MM:SSZ.
  pub updated_at: String,
  /// The value of the variable.
  pub value: String,
}

#[cfg(any(feature = "full", feature = "actions"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ActionsRepositoryPermissions {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub allowed_actions: Option<AllowedActions>,
  pub enabled: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub selected_actions_url: Option<String>,
}

#[cfg(any(feature = "full", feature = "actions"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ActionsWorkflowAccessToRepository {
  /// Defines the level of access that workflows outside of the repository have to actions and reusable workflows within the
  /// repository.
  ///
  /// `none` means the access is only possible from workflows in this repository. `user` level access allows sharing across user owned private repositories only. `organization` level access allows sharing across the organization.
  pub access_level: ActionsWorkflowAccessToRepositoryAccessLevel,
}

#[cfg(any(feature = "full", feature = "actions", feature = "checks"))]
/// Information about the Git author
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct SimpleCommitAuthor {
  /// Git email address of the commit's author
  pub email: String,
  /// Name of the commit's author
  pub name: String,
}

#[cfg(any(feature = "full", feature = "actions", feature = "checks"))]
/// Information about the Git committer
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct SimpleCommitCommitter {
  /// Git email address of the commit's committer
  pub email: String,
  /// Name of the commit's committer
  pub name: String,
}

#[cfg(any(feature = "full", feature = "actions", feature = "checks"))]
/// A commit.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct SimpleCommit {
  /// Information about the Git author
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub author: Option<SimpleCommitAuthor>,
  /// Information about the Git committer
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub committer: Option<SimpleCommitCommitter>,
  /// SHA for the commit
  pub id: String,
  /// Message describing the purpose of the commit
  pub message: String,
  /// Timestamp of the commit
  pub timestamp: String,
  /// SHA for the commit's tree
  pub tree_id: String,
}

#[cfg(any(feature = "full", feature = "actions", feature = "checks"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct PullRequestMinimalBaseRepo {
  pub id: i64,
  pub name: String,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "actions", feature = "checks"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct PullRequestMinimalBase {
  #[serde(rename = "ref")]
  pub ref_: String,
  pub repo: PullRequestMinimalBaseRepo,
  pub sha: String,
}

#[cfg(any(feature = "full", feature = "actions", feature = "checks"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct PullRequestMinimalHeadRepo {
  pub id: i64,
  pub name: String,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "actions", feature = "checks"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct PullRequestMinimalHead {
  #[serde(rename = "ref")]
  pub ref_: String,
  pub repo: PullRequestMinimalHeadRepo,
  pub sha: String,
}

#[cfg(any(feature = "full", feature = "actions", feature = "checks"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct PullRequestMinimal {
  pub base: PullRequestMinimalBase,
  pub head: PullRequestMinimalHead,
  pub id: i64,
  pub number: i64,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "actions"))]
/// A workflow referenced/reused by the initial caller workflow
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ReferencedWorkflow {
  pub path: String,
  #[serde(rename = "ref")]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub ref_: Option<String>,
  pub sha: String,
}

#[cfg(any(feature = "full", feature = "actions"))]
/// An invocation of a workflow
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct WorkflowRun {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub actor: Option<SimpleUser>,
  /// The URL to the artifacts for the workflow run.
  pub artifacts_url: String,
  /// The URL to cancel the workflow run.
  pub cancel_url: String,
  /// The ID of the associated check suite.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub check_suite_id: Option<i64>,
  /// The node ID of the associated check suite.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub check_suite_node_id: Option<String>,
  /// The URL to the associated check suite.
  pub check_suite_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub conclusion: Option<String>,
  pub created_at: String,
  /// The event-specific title associated with the run or the run-name if set, or the value of `run-name` if it is set in the workflow.
  pub display_title: String,
  pub event: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub head_branch: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub head_commit: Option<SimpleCommit>,
  pub head_repository: MinimalRepository,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub head_repository_id: Option<i64>,
  /// The SHA of the head commit that points to the version of the workflow being run.
  pub head_sha: String,
  pub html_url: String,
  /// The ID of the workflow run.
  pub id: i64,
  /// The URL to the jobs for the workflow run.
  pub jobs_url: String,
  /// The URL to download the logs for the workflow run.
  pub logs_url: String,
  /// The name of the workflow run.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub name: Option<String>,
  pub node_id: String,
  /// The full path of the workflow
  pub path: String,
  /// The URL to the previous attempted run of this workflow, if one exists.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub previous_attempt_url: Option<String>,
  /// Pull requests that are open with a `head_sha` or `head_branch` that matches the workflow run. The returned pull requests do not necessarily indicate pull requests that triggered the run.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub pull_requests: Option<Vec<PullRequestMinimal>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub referenced_workflows: Option<Vec<ReferencedWorkflow>>,
  pub repository: MinimalRepository,
  /// The URL to rerun the workflow run.
  pub rerun_url: String,
  /// Attempt number of the run, 1 for first attempt and higher if the workflow was re-run.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub run_attempt: Option<i64>,
  /// The auto incrementing run number for the workflow run.
  pub run_number: i64,
  /// The start time of the latest run. Resets on re-run.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub run_started_at: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub status: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub triggering_actor: Option<SimpleUser>,
  pub updated_at: String,
  /// The URL to the workflow run.
  pub url: String,
  /// The ID of the parent workflow.
  pub workflow_id: i64,
  /// The URL to the workflow.
  pub workflow_url: String,
}

#[cfg(any(feature = "full", feature = "actions"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct EnvironmentApprovalsEnvironments {
  /// The time that the environment was created, in ISO 8601 format.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub created_at: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub html_url: Option<String>,
  /// The id of the environment.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub id: Option<i64>,
  /// The name of the environment.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub node_id: Option<String>,
  /// The time that the environment was last updated, in ISO 8601 format.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub updated_at: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub url: Option<String>,
}

#[cfg(any(feature = "full", feature = "actions"))]
/// An entry in the reviews log for environment deployments
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct EnvironmentApprovals {
  /// The comment submitted with the deployment review
  pub comment: String,
  /// The list of environments that were approved or rejected
  pub environments: Vec<EnvironmentApprovalsEnvironments>,
  /// Whether deployment to the environment(s) was approved or rejected or pending (with comments)
  pub state: EnvironmentApprovalsState,
  pub user: SimpleUser,
}

#[cfg(any(feature = "full", feature = "actions"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ReviewCustomGatesCommentRequired {
  /// Comment associated with the pending deployment protection rule. **Required when state is not provided.**
  pub comment: String,
  /// The name of the environment to approve or reject.
  pub environment_name: String,
}

#[cfg(any(feature = "full", feature = "actions"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ReviewCustomGatesStateRequired {
  /// Optional comment to include with the review.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub comment: Option<String>,
  /// The name of the environment to approve or reject.
  pub environment_name: String,
  /// Whether to approve or reject deployment to the specified environments.
  pub state: ReviewCustomGatesStateRequiredState,
}

#[cfg(any(feature = "full", feature = "actions"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct PendingDeploymentEnvironment {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub html_url: Option<String>,
  /// The id of the environment.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub id: Option<i64>,
  /// The name of the environment.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub node_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub url: Option<String>,
}

#[cfg(any(feature = "full", feature = "actions"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct PendingDeploymentReviewers {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub reviewer: Option<PendingDeploymentReviewersReviewer>,
  #[serde(rename = "type")]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub type_: Option<DeploymentReviewerType>,
}

#[cfg(any(feature = "full", feature = "actions"))]
/// Details of a deployment that is waiting for protection rules to pass
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct PendingDeployment {
  /// Whether the currently authenticated user can approve the deployment
  pub current_user_can_approve: bool,
  pub environment: PendingDeploymentEnvironment,
  /// The people or teams that may approve jobs that reference the environment. You can list up to six users or teams as reviewers. The reviewers must have at least read access to the repository. Only one of the required reviewers needs to approve the job for it to proceed.
  pub reviewers: Vec<PendingDeploymentReviewers>,
  /// The set duration of the wait timer
  pub wait_timer: i64,
  /// The time that the wait timer began.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub wait_timer_started_at: Option<String>,
}

#[cfg(any(feature = "full", feature = "actions", feature = "repos"))]
/// A request for a specific ref(branch,sha,tag) to be deployed
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct Deployment {
  pub created_at: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub creator: Option<SimpleUser>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub description: Option<String>,
  /// Name for the target deployment environment.
  pub environment: String,
  /// Unique identifier of the deployment
  pub id: i64,
  pub node_id: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub original_environment: Option<String>,
  pub payload: serde_json::Value,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub performed_via_github_app: Option<Integration>,
  /// Specifies if the given environment is one that end-users directly interact with. Default: false.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub production_environment: Option<bool>,
  /// The ref to deploy. This can be a branch, tag, or sha.
  #[serde(rename = "ref")]
  pub ref_: String,
  pub repository_url: String,
  pub sha: String,
  pub statuses_url: String,
  /// Parameter to specify a task to execute
  pub task: String,
  /// Specifies if the given environment is will no longer exist at some point in the future. Default: false.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub transient_environment: Option<bool>,
  pub updated_at: String,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "actions"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct WorkflowRunUsageBillableMACOSJobRuns {
  pub duration_ms: i64,
  pub job_id: i64,
}

#[cfg(any(feature = "full", feature = "actions"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct WorkflowRunUsageBillableMACOS {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub job_runs: Option<Vec<WorkflowRunUsageBillableMACOSJobRuns>>,
  pub jobs: i64,
  pub total_ms: i64,
}

#[cfg(any(feature = "full", feature = "actions"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct WorkflowRunUsageBillableUBUNTUJobRuns {
  pub duration_ms: i64,
  pub job_id: i64,
}

#[cfg(any(feature = "full", feature = "actions"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct WorkflowRunUsageBillableUBUNTU {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub job_runs: Option<Vec<WorkflowRunUsageBillableUBUNTUJobRuns>>,
  pub jobs: i64,
  pub total_ms: i64,
}

#[cfg(any(feature = "full", feature = "actions"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct WorkflowRunUsageBillableWINDOWSJobRuns {
  pub duration_ms: i64,
  pub job_id: i64,
}

#[cfg(any(feature = "full", feature = "actions"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct WorkflowRunUsageBillableWINDOWS {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub job_runs: Option<Vec<WorkflowRunUsageBillableWINDOWSJobRuns>>,
  pub jobs: i64,
  pub total_ms: i64,
}

#[cfg(any(feature = "full", feature = "actions"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct WorkflowRunUsageBillable {
  #[serde(rename = "MACOS")]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub macos: Option<WorkflowRunUsageBillableMACOS>,
  #[serde(rename = "UBUNTU")]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub ubuntu: Option<WorkflowRunUsageBillableUBUNTU>,
  #[serde(rename = "WINDOWS")]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub windows: Option<WorkflowRunUsageBillableWINDOWS>,
}

#[cfg(any(feature = "full", feature = "actions"))]
/// Workflow Run Usage
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct WorkflowRunUsage {
  pub billable: WorkflowRunUsageBillable,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub run_duration_ms: Option<i64>,
}

#[cfg(any(feature = "full", feature = "actions"))]
/// A GitHub Actions workflow
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct Workflow {
  pub badge_url: String,
  pub created_at: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub deleted_at: Option<String>,
  pub html_url: String,
  pub id: i64,
  pub name: String,
  pub node_id: String,
  pub path: String,
  pub state: WorkflowState,
  pub updated_at: String,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "actions"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct WorkflowUsageBillableMACOS {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub total_ms: Option<i64>,
}

#[cfg(any(feature = "full", feature = "actions"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct WorkflowUsageBillableUBUNTU {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub total_ms: Option<i64>,
}

#[cfg(any(feature = "full", feature = "actions"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct WorkflowUsageBillableWINDOWS {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub total_ms: Option<i64>,
}

#[cfg(any(feature = "full", feature = "actions"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct WorkflowUsageBillable {
  #[serde(rename = "MACOS")]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub macos: Option<WorkflowUsageBillableMACOS>,
  #[serde(rename = "UBUNTU")]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub ubuntu: Option<WorkflowUsageBillableUBUNTU>,
  #[serde(rename = "WINDOWS")]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub windows: Option<WorkflowUsageBillableWINDOWS>,
}

#[cfg(any(feature = "full", feature = "actions"))]
/// Workflow Usage
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct WorkflowUsage {
  pub billable: WorkflowUsageBillable,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// Activity
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct Activity {
  /// The type of the activity that was performed.
  pub activity_type: ActivityActivityType,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub actor: Option<SimpleUser>,
  /// The SHA of the commit after the activity.
  pub after: String,
  /// The SHA of the commit before the activity.
  pub before: String,
  pub id: i64,
  pub node_id: String,
  /// The full Git reference, formatted as `refs/heads/<branch name>`.
  #[serde(rename = "ref")]
  pub ref_: String,
  /// The time when the activity occurred.
  pub timestamp: String,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// An autolink reference.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct Autolink {
  pub id: i64,
  /// Whether this autolink reference matches alphanumeric characters. If false, this autolink reference only matches numeric characters.
  pub is_alphanumeric: bool,
  /// The prefix of a key that is linkified.
  pub key_prefix: String,
  /// A template for the target URL that is generated if a key was found.
  pub url_template: String,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// Check Automated Security Fixes
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CheckAutomatedSecurityFixes {
  /// Whether automated security fixes are enabled for the repository.
  pub enabled: bool,
  /// Whether automated security fixes are paused for the repository.
  pub paused: bool,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ShortBranchCommit {
  pub sha: String,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct BranchProtectionAllowDeletions {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub enabled: Option<bool>,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct BranchProtectionAllowForcePushes {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub enabled: Option<bool>,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// Whether users can pull changes from upstream when the branch is locked. Set to `true` to allow fork syncing. Set to `false` to prevent fork syncing.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct BranchProtectionAllowForkSyncing {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub enabled: Option<bool>,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct BranchProtectionBlockCreations {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub enabled: Option<bool>,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// Protected Branch Admin Enforced
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ProtectedBranchAdminEnforced {
  pub enabled: bool,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// Whether to set the branch as read-only. If this is true, users will not be able to push to the branch.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct BranchProtectionLockBranch {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub enabled: Option<bool>,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct BranchProtectionRequiredConversationResolution {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub enabled: Option<bool>,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct BranchProtectionRequiredLinearHistory {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub enabled: Option<bool>,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// Allow specific users, teams, or apps to bypass pull request requirements.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ProtectedBranchPullRequestReviewBypassPullRequestAllowances {
  /// The list of apps allowed to bypass pull request requirements.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub apps: Option<Vec<Integration>>,
  /// The list of teams allowed to bypass pull request requirements.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub teams: Option<Vec<Team>>,
  /// The list of users allowed to bypass pull request requirements.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub users: Option<Vec<SimpleUser>>,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ProtectedBranchPullRequestReviewDismissalRestrictions {
  /// The list of apps with review dismissal access.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub apps: Option<Vec<Integration>>,
  /// The list of teams with review dismissal access.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub teams: Option<Vec<Team>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub teams_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub url: Option<String>,
  /// The list of users with review dismissal access.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub users: Option<Vec<SimpleUser>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub users_url: Option<String>,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// Protected Branch Pull Request Review
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ProtectedBranchPullRequestReview {
  /// Allow specific users, teams, or apps to bypass pull request requirements.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub bypass_pull_request_allowances:
    Option<ProtectedBranchPullRequestReviewBypassPullRequestAllowances>,
  pub dismiss_stale_reviews: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub dismissal_restrictions: Option<ProtectedBranchPullRequestReviewDismissalRestrictions>,
  pub require_code_owner_reviews: bool,
  /// Whether the most recent push must be approved by someone other than the person who pushed it.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub require_last_push_approval: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub required_approving_review_count: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub url: Option<String>,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct BranchProtectionRequiredSignatures {
  pub enabled: bool,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ProtectedBranchRequiredStatusCheckChecks {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub app_id: Option<i64>,
  pub context: String,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// Protected Branch Required Status Check
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ProtectedBranchRequiredStatusCheck {
  pub checks: Vec<ProtectedBranchRequiredStatusCheckChecks>,
  pub contexts: Vec<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub contexts_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub enforcement_level: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub strict: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub url: Option<String>,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct BranchRestrictionPolicyAppsOwner {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub avatar_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub description: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub events_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub followers_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub following_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub gists_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub gravatar_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub hooks_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub html_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub id: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub issues_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub login: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub members_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub node_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub organizations_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub public_members_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub received_events_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub repos_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub site_admin: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub starred_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub subscriptions_url: Option<String>,
  #[serde(rename = "type")]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub type_: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub url: Option<String>,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct BranchRestrictionPolicyAppsPermissions {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub contents: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub issues: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub metadata: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub single_file: Option<String>,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct BranchRestrictionPolicyApps {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub created_at: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub description: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub events: Option<Vec<String>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub external_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub html_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub id: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub node_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub owner: Option<BranchRestrictionPolicyAppsOwner>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub permissions: Option<BranchRestrictionPolicyAppsPermissions>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub slug: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub updated_at: Option<String>,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct BranchRestrictionPolicyTeams {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub description: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub html_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub id: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub members_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub node_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub notification_setting: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub parent: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub permission: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub privacy: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub repositories_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub slug: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub url: Option<String>,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct BranchRestrictionPolicyUsers {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub avatar_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub events_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub followers_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub following_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub gists_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub gravatar_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub html_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub id: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub login: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub node_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub organizations_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub received_events_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub repos_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub site_admin: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub starred_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub subscriptions_url: Option<String>,
  #[serde(rename = "type")]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub type_: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub url: Option<String>,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// Branch Restriction Policy
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct BranchRestrictionPolicy {
  pub apps: Vec<BranchRestrictionPolicyApps>,
  pub apps_url: String,
  pub teams: Vec<BranchRestrictionPolicyTeams>,
  pub teams_url: String,
  pub url: String,
  pub users: Vec<BranchRestrictionPolicyUsers>,
  pub users_url: String,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// Branch Protection
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct BranchProtection {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub allow_deletions: Option<BranchProtectionAllowDeletions>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub allow_force_pushes: Option<BranchProtectionAllowForcePushes>,
  /// Whether users can pull changes from upstream when the branch is locked. Set to `true` to allow fork syncing. Set to `false` to prevent fork syncing.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub allow_fork_syncing: Option<BranchProtectionAllowForkSyncing>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub block_creations: Option<BranchProtectionBlockCreations>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub enabled: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub enforce_admins: Option<ProtectedBranchAdminEnforced>,
  /// Whether to set the branch as read-only. If this is true, users will not be able to push to the branch.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub lock_branch: Option<BranchProtectionLockBranch>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub protection_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub required_conversation_resolution: Option<BranchProtectionRequiredConversationResolution>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub required_linear_history: Option<BranchProtectionRequiredLinearHistory>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub required_pull_request_reviews: Option<ProtectedBranchPullRequestReview>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub required_signatures: Option<BranchProtectionRequiredSignatures>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub required_status_checks: Option<ProtectedBranchRequiredStatusCheck>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub restrictions: Option<BranchRestrictionPolicy>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub url: Option<String>,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// Short Branch
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ShortBranch {
  pub commit: ShortBranchCommit,
  pub name: String,
  pub protected: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub protection: Option<BranchProtection>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub protection_url: Option<String>,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct BranchWithProtectionLinks {
  pub html: String,
  #[serde(rename = "self")]
  pub self_: String,
}

#[cfg(any(
  feature = "full",
  feature = "repos",
  feature = "pulls",
  feature = "search"
))]
/// Metaproperties for Git author/committer information.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct GitUser {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub date: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub email: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub name: Option<String>,
}

#[cfg(any(feature = "full", feature = "repos", feature = "pulls"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CommitCommitTree {
  pub sha: String,
  pub url: String,
}

#[cfg(any(
  feature = "full",
  feature = "repos",
  feature = "git",
  feature = "pulls",
  feature = "search"
))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct Verification {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub payload: Option<String>,
  pub reason: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub signature: Option<String>,
  pub verified: bool,
}

#[cfg(any(feature = "full", feature = "repos", feature = "pulls"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CommitCommit {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub author: Option<GitUser>,
  pub comment_count: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub committer: Option<GitUser>,
  pub message: String,
  pub tree: CommitCommitTree,
  pub url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub verification: Option<Verification>,
}

#[cfg(any(feature = "full", feature = "repos", feature = "pulls"))]
/// Diff Entry
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct DiffEntry {
  pub additions: i64,
  pub blob_url: String,
  pub changes: i64,
  pub contents_url: String,
  pub deletions: i64,
  pub filename: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub patch: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub previous_filename: Option<String>,
  pub raw_url: String,
  pub sha: String,
  pub status: DiffEntryStatus,
}

#[cfg(any(feature = "full", feature = "repos", feature = "pulls"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CommitParents {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub html_url: Option<String>,
  pub sha: String,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "repos", feature = "pulls"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CommitStats {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub additions: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub deletions: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub total: Option<i64>,
}

#[cfg(any(feature = "full", feature = "repos", feature = "pulls"))]
/// Commit
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct Commit {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub author: Option<SimpleUser>,
  pub comments_url: String,
  pub commit: CommitCommit,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub committer: Option<SimpleUser>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub files: Option<Vec<DiffEntry>>,
  pub html_url: String,
  pub node_id: String,
  pub parents: Vec<CommitParents>,
  pub sha: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub stats: Option<CommitStats>,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// Branch With Protection
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct BranchWithProtection {
  #[serde(rename = "_links")]
  pub links: BranchWithProtectionLinks,
  pub commit: Commit,
  pub name: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub pattern: Option<String>,
  pub protected: bool,
  pub protection: BranchProtection,
  pub protection_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub required_approving_review_count: Option<i64>,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ProtectedBranchAllowDeletions {
  pub enabled: bool,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ProtectedBranchAllowForcePushes {
  pub enabled: bool,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// Whether users can pull changes from upstream when the branch is locked. Set to `true` to allow fork syncing. Set to `false` to prevent fork syncing.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ProtectedBranchAllowForkSyncing {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub enabled: Option<bool>,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ProtectedBranchBlockCreations {
  pub enabled: bool,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ProtectedBranchEnforceAdmins {
  pub enabled: bool,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// Whether to set the branch as read-only. If this is true, users will not be able to push to the branch.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ProtectedBranchLockBranch {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub enabled: Option<bool>,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ProtectedBranchRequiredConversationResolution {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub enabled: Option<bool>,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ProtectedBranchRequiredLinearHistory {
  pub enabled: bool,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ProtectedBranchRequiredPullRequestReviewsBypassPullRequestAllowances {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub apps: Option<Vec<Integration>>,
  pub teams: Vec<Team>,
  pub users: Vec<SimpleUser>,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ProtectedBranchRequiredPullRequestReviewsDismissalRestrictions {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub apps: Option<Vec<Integration>>,
  pub teams: Vec<Team>,
  pub teams_url: String,
  pub url: String,
  pub users: Vec<SimpleUser>,
  pub users_url: String,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ProtectedBranchRequiredPullRequestReviews {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub bypass_pull_request_allowances:
    Option<ProtectedBranchRequiredPullRequestReviewsBypassPullRequestAllowances>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub dismiss_stale_reviews: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub dismissal_restrictions:
    Option<ProtectedBranchRequiredPullRequestReviewsDismissalRestrictions>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub require_code_owner_reviews: Option<bool>,
  /// Whether the most recent push must be approved by someone other than the person who pushed it.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub require_last_push_approval: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub required_approving_review_count: Option<i64>,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ProtectedBranchRequiredSignatures {
  pub enabled: bool,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct StatusCheckPolicyChecks {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub app_id: Option<i64>,
  pub context: String,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// Status Check Policy
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct StatusCheckPolicy {
  pub checks: Vec<StatusCheckPolicyChecks>,
  pub contexts: Vec<String>,
  pub contexts_url: String,
  pub strict: bool,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// Branch protections protect branches
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ProtectedBranch {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub allow_deletions: Option<ProtectedBranchAllowDeletions>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub allow_force_pushes: Option<ProtectedBranchAllowForcePushes>,
  /// Whether users can pull changes from upstream when the branch is locked. Set to `true` to allow fork syncing. Set to `false` to prevent fork syncing.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub allow_fork_syncing: Option<ProtectedBranchAllowForkSyncing>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub block_creations: Option<ProtectedBranchBlockCreations>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub enforce_admins: Option<ProtectedBranchEnforceAdmins>,
  /// Whether to set the branch as read-only. If this is true, users will not be able to push to the branch.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub lock_branch: Option<ProtectedBranchLockBranch>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub required_conversation_resolution: Option<ProtectedBranchRequiredConversationResolution>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub required_linear_history: Option<ProtectedBranchRequiredLinearHistory>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub required_pull_request_reviews: Option<ProtectedBranchRequiredPullRequestReviews>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub required_signatures: Option<ProtectedBranchRequiredSignatures>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub required_status_checks: Option<StatusCheckPolicy>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub restrictions: Option<BranchRestrictionPolicy>,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "checks"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CheckRunCheckSuite {
  pub id: i64,
}

#[cfg(any(feature = "full", feature = "checks"))]
/// A deployment created as the result of an Actions check run from a workflow that references an environment
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct DeploymentSimple {
  pub created_at: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub description: Option<String>,
  /// Name for the target deployment environment.
  pub environment: String,
  /// Unique identifier of the deployment
  pub id: i64,
  pub node_id: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub original_environment: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub performed_via_github_app: Option<Integration>,
  /// Specifies if the given environment is one that end-users directly interact with. Default: false.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub production_environment: Option<bool>,
  pub repository_url: String,
  pub statuses_url: String,
  /// Parameter to specify a task to execute
  pub task: String,
  /// Specifies if the given environment is will no longer exist at some point in the future. Default: false.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub transient_environment: Option<bool>,
  pub updated_at: String,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "checks"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CheckRunOutput {
  pub annotations_count: i64,
  pub annotations_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub summary: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub text: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub title: Option<String>,
}

#[cfg(any(feature = "full", feature = "checks"))]
/// A check performed on the code of a given code change
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CheckRun {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub app: Option<Integration>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub check_suite: Option<CheckRunCheckSuite>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub completed_at: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub conclusion: Option<CheckRunConclusion>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub deployment: Option<DeploymentSimple>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub details_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub external_id: Option<String>,
  /// The SHA of the commit that is being checked.
  pub head_sha: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub html_url: Option<String>,
  /// The id of the check.
  pub id: i64,
  /// The name of the check.
  pub name: String,
  pub node_id: String,
  pub output: CheckRunOutput,
  /// Pull requests that are open with a `head_sha` or `head_branch` that matches the check. The returned pull requests do not necessarily indicate pull requests that triggered the check.
  pub pull_requests: Vec<PullRequestMinimal>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub started_at: Option<String>,
  /// The phase of the lifecycle that the check is currently in. Statuses of waiting, requested, and pending are reserved for GitHub Actions check runs.
  pub status: CheckRunStatus,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "checks"))]
/// Check Annotation
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CheckAnnotation {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub annotation_level: Option<String>,
  pub blob_href: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub end_column: Option<i64>,
  pub end_line: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub message: Option<String>,
  pub path: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub raw_details: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub start_column: Option<i64>,
  pub start_line: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub title: Option<String>,
}

#[cfg(any(feature = "full", feature = "checks"))]
/// A suite of checks performed on the code of a given code change
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CheckSuite {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub after: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub app: Option<Integration>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub before: Option<String>,
  pub check_runs_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub conclusion: Option<CheckSuiteConclusion>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub created_at: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub head_branch: Option<String>,
  pub head_commit: SimpleCommit,
  /// The SHA of the head commit that is being checked.
  pub head_sha: String,
  pub id: i64,
  pub latest_check_runs_count: i64,
  pub node_id: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub pull_requests: Option<Vec<PullRequestMinimal>>,
  pub repository: MinimalRepository,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub rerequestable: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub runs_rerequestable: Option<bool>,
  /// The phase of the lifecycle that the check suite is currently in. Statuses of waiting, requested, and pending are reserved for GitHub Actions check suites.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub status: Option<CheckSuiteStatus>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub updated_at: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub url: Option<String>,
}

#[cfg(any(feature = "full", feature = "checks"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CheckSuitePreferencePreferencesAutoTriggerChecks {
  pub app_id: i64,
  pub setting: bool,
}

#[cfg(any(feature = "full", feature = "checks"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CheckSuitePreferencePreferences {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub auto_trigger_checks: Option<Vec<CheckSuitePreferencePreferencesAutoTriggerChecks>>,
}

#[cfg(any(feature = "full", feature = "checks"))]
/// Check suite configuration preferences for a repository.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CheckSuitePreference {
  pub preferences: CheckSuitePreferencePreferences,
  pub repository: MinimalRepository,
}

#[cfg(any(feature = "full", feature = "code_scanning"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CodeScanningAlertItems {
  pub created_at: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub dismissed_at: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub dismissed_by: Option<SimpleUser>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub dismissed_comment: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub dismissed_reason: Option<CodeScanningAlertDismissedReason>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub fixed_at: Option<String>,
  pub html_url: String,
  pub instances_url: String,
  pub most_recent_instance: CodeScanningAlertInstance,
  pub number: i64,
  pub rule: CodeScanningAlertRuleSummary,
  pub state: CodeScanningAlertState,
  pub tool: CodeScanningAnalysisTool,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub updated_at: Option<String>,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "code_scanning"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CodeScanningAlertRule {
  /// A short description of the rule used to detect the alert.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub description: Option<String>,
  /// description of the rule used to detect the alert.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub full_description: Option<String>,
  /// Detailed documentation for the rule as GitHub Flavored Markdown.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub help: Option<String>,
  /// A link to the documentation for the rule used to detect the alert.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub help_uri: Option<String>,
  /// A unique identifier for the rule used to detect the alert.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub id: Option<String>,
  /// The name of the rule used to detect the alert.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub name: Option<String>,
  /// The security severity of the alert.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub security_severity_level: Option<Severity>,
  /// The severity of the alert.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub severity: Option<CodeScanningAlertRuleSeverity>,
  /// A set of tags applicable for the rule.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub tags: Option<Vec<String>>,
}

#[cfg(any(feature = "full", feature = "code_scanning"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CodeScanningAlert {
  pub created_at: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub dismissed_at: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub dismissed_by: Option<SimpleUser>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub dismissed_comment: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub dismissed_reason: Option<CodeScanningAlertDismissedReason>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub fixed_at: Option<String>,
  pub html_url: String,
  pub instances_url: String,
  pub most_recent_instance: CodeScanningAlertInstance,
  pub number: i64,
  pub rule: CodeScanningAlertRule,
  pub state: CodeScanningAlertState,
  pub tool: CodeScanningAnalysisTool,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub updated_at: Option<String>,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "code_scanning"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CodeScanningAnalysis {
  pub analysis_key: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub category: Option<String>,
  pub commit_sha: String,
  pub created_at: String,
  pub deletable: bool,
  pub environment: String,
  pub error: String,
  /// Unique identifier for this analysis.
  pub id: i64,
  #[serde(rename = "ref")]
  pub ref_: String,
  /// The total number of results in the analysis.
  pub results_count: i64,
  /// The total number of rules used in the analysis.
  pub rules_count: i64,
  pub sarif_id: String,
  pub tool: CodeScanningAnalysisTool,
  pub url: String,
  /// Warning generated when processing the analysis
  pub warning: String,
}

#[cfg(any(feature = "full", feature = "code_scanning"))]
/// Successful deletion of a code scanning analysis
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CodeScanningAnalysisDeletion {
  /// Next deletable analysis in chain, with last analysis deletion confirmation
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub confirm_delete_url: Option<String>,
  /// Next deletable analysis in chain, without last analysis deletion confirmation
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub next_analysis_url: Option<String>,
}

#[cfg(any(feature = "full", feature = "code_scanning"))]
/// A CodeQL database.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CodeScanningCodeqlDatabase {
  /// The commit SHA of the repository at the time the CodeQL database was created.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub commit_oid: Option<String>,
  /// The MIME type of the CodeQL database file.
  pub content_type: String,
  /// The date and time at which the CodeQL database was created, in ISO 8601 format':' YYYY-MM-DDTHH:MM:SSZ.
  pub created_at: String,
  /// The ID of the CodeQL database.
  pub id: i64,
  /// The language of the CodeQL database.
  pub language: String,
  /// The name of the CodeQL database.
  pub name: String,
  /// The size of the CodeQL database file in bytes.
  pub size: i64,
  /// The date and time at which the CodeQL database was last updated, in ISO 8601 format':' YYYY-MM-DDTHH:MM:SSZ.
  pub updated_at: String,
  pub uploader: SimpleUser,
  /// The URL at which to download the CodeQL database. The `Accept` header must be set to the value of the `content_type` property.
  pub url: String,
}

#[cfg(any(feature = "full", feature = "code_scanning"))]
/// Configuration for code scanning default setup.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CodeScanningDefaultSetup {
  /// Languages to be analyzed.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub languages: Option<Vec<CodeScanningDefaultSetupLanguages>>,
  /// CodeQL query suite to be used.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub query_suite: Option<CodeScanningDefaultSetupQuerySuite>,
  /// The frequency of the periodic analysis.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub schedule: Option<CodeScanningDefaultSetupSchedule>,
  /// Code scanning default setup has been configured or not.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub state: Option<CodeScanningDefaultSetupState>,
  /// Timestamp of latest configuration update.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub updated_at: Option<String>,
}

#[cfg(any(feature = "full", feature = "code_scanning"))]
/// Configuration for code scanning default setup.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CodeScanningDefaultSetupUpdate {
  /// CodeQL languages to be analyzed.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub languages: Option<Vec<CodeScanningDefaultSetupUpdateLanguages>>,
  /// CodeQL query suite to be used.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub query_suite: Option<CodeScanningDefaultSetupUpdateQuerySuite>,
  /// The desired state of code scanning default setup.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub state: Option<CodeScanningDefaultSetupUpdateState>,
}

#[cfg(any(feature = "full", feature = "code_scanning"))]
/// You can use `run_url` to track the status of the run. This includes a property status and conclusion.
/// You should not rely on this always being an actions workflow run object.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CodeScanningDefaultSetupUpdateResponse {
  /// ID of the corresponding run.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub run_id: Option<i64>,
  /// URL of the corresponding run.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub run_url: Option<String>,
}

#[cfg(any(feature = "full", feature = "code_scanning"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CodeScanningSarifsReceipt {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub id: Option<String>,
  /// The REST API URL for checking the status of the upload.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub url: Option<String>,
}

#[cfg(any(feature = "full", feature = "code_scanning"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CodeScanningSarifsStatus {
  /// The REST API URL for getting the analyses associated with the upload.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub analyses_url: Option<String>,
  /// Any errors that ocurred during processing of the delivery.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub errors: Option<Vec<String>>,
  /// `pending` files have not yet been processed, while `complete` means results from the SARIF have been stored. `failed` files have either not been processed at all, or could only be partially processed.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub processing_status: Option<CodeScanningSarifsStatusProcessingStatus>,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CodeownersErrorsErrors {
  /// The column number where this errors occurs.
  pub column: i64,
  /// The type of error.
  pub kind: String,
  /// The line number where this errors occurs.
  pub line: i64,
  /// A human-readable description of the error, combining information from multiple fields, laid out for display in a monospaced typeface (for example, a command-line setting).
  pub message: String,
  /// The path of the file where the error occured.
  pub path: String,
  /// The contents of the line where the error occurs.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub source: Option<String>,
  /// Suggested action to fix the error. This will usually be `null`, but is provided for some common errors.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub suggestion: Option<String>,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// A list of errors found in a repo's CODEOWNERS file
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CodeownersErrors {
  pub errors: Vec<CodeownersErrorsErrors>,
}

#[cfg(any(feature = "full", feature = "codespaces"))]
/// Permission check result for a given devcontainer config.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CodespacesPermissionsCheckForDevcontainer {
  /// Whether the user has accepted the permissions defined by the devcontainer config
  pub accepted: bool,
}

#[cfg(any(feature = "full", feature = "codespaces"))]
/// Set repository secrets for GitHub Codespaces.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RepoCodespacesSecret {
  pub created_at: String,
  /// The name of the secret.
  pub name: String,
  pub updated_at: String,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CollaboratorPermissions {
  pub admin: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub maintain: Option<bool>,
  pub pull: bool,
  pub push: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub triage: Option<bool>,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// Collaborator
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct Collaborator {
  pub avatar_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub email: Option<String>,
  pub events_url: String,
  pub followers_url: String,
  pub following_url: String,
  pub gists_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub gravatar_id: Option<String>,
  pub html_url: String,
  pub id: i64,
  pub login: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub name: Option<String>,
  pub node_id: String,
  pub organizations_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub permissions: Option<CollaboratorPermissions>,
  pub received_events_url: String,
  pub repos_url: String,
  pub role_name: String,
  pub site_admin: bool,
  pub starred_url: String,
  pub subscriptions_url: String,
  #[serde(rename = "type")]
  pub type_: String,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// Repository invitations let you manage who you collaborate with.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RepositoryInvitation {
  pub created_at: String,
  /// Whether or not the invitation has expired
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub expired: Option<bool>,
  pub html_url: String,
  /// Unique identifier of the repository invitation.
  pub id: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub invitee: Option<SimpleUser>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub inviter: Option<SimpleUser>,
  pub node_id: String,
  /// The permission associated with the invitation.
  pub permissions: RepositoryInvitationPermissions,
  pub repository: MinimalRepository,
  /// URL for the repository invitation
  pub url: String,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// Repository Collaborator Permission
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RepositoryCollaboratorPermission {
  pub permission: String,
  pub role_name: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub user: Option<Collaborator>,
}

#[cfg(any(feature = "full", feature = "repos", feature = "issues"))]
/// Commit Comment
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CommitComment {
  pub author_association: AuthorAssociation,
  pub body: String,
  pub commit_id: String,
  pub created_at: String,
  pub html_url: String,
  pub id: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub line: Option<i64>,
  pub node_id: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub path: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub position: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub reactions: Option<ReactionRollup>,
  pub updated_at: String,
  pub url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub user: Option<SimpleUser>,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct BranchShortCommit {
  pub sha: String,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// Branch Short
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct BranchShort {
  pub commit: BranchShortCommit,
  pub name: String,
  pub protected: bool,
}

#[cfg(any(feature = "full", feature = "repos", feature = "pulls"))]
/// Hypermedia Link
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct Link {
  pub href: String,
}

#[cfg(any(feature = "full", feature = "repos", feature = "pulls"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct PullRequestSimpleLinks {
  pub comments: Link,
  pub commits: Link,
  pub html: Link,
  pub issue: Link,
  pub review_comment: Link,
  pub review_comments: Link,
  #[serde(rename = "self")]
  pub self_: Link,
  pub statuses: Link,
}

#[cfg(any(feature = "full", feature = "repos", feature = "pulls"))]
/// The status of auto merging a pull request.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct AutoMerge {
  /// Commit message for the merge commit.
  pub commit_message: String,
  /// Title for the merge commit message.
  pub commit_title: String,
  pub enabled_by: SimpleUser,
  /// The merge method to use.
  pub merge_method: AutoMergeMergeMethod,
}

#[cfg(any(feature = "full", feature = "repos", feature = "pulls"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct PullRequestSimpleBase {
  pub label: String,
  #[serde(rename = "ref")]
  pub ref_: String,
  pub repo: Repository,
  pub sha: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub user: Option<SimpleUser>,
}

#[cfg(any(feature = "full", feature = "repos", feature = "pulls"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct PullRequestSimpleHead {
  pub label: String,
  #[serde(rename = "ref")]
  pub ref_: String,
  pub repo: Repository,
  pub sha: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub user: Option<SimpleUser>,
}

#[cfg(any(feature = "full", feature = "repos", feature = "pulls"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct PullRequestSimpleLabels {
  pub color: String,
  #[serde(rename = "default")]
  pub default_: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub description: Option<String>,
  pub id: i64,
  pub name: String,
  pub node_id: String,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "repos", feature = "pulls"))]
/// Pull Request Simple
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct PullRequestSimple {
  #[serde(rename = "_links")]
  pub links: PullRequestSimpleLinks,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub active_lock_reason: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub assignee: Option<SimpleUser>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub assignees: Option<Vec<SimpleUser>>,
  pub author_association: AuthorAssociation,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub auto_merge: Option<AutoMerge>,
  pub base: PullRequestSimpleBase,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub body: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub closed_at: Option<String>,
  pub comments_url: String,
  pub commits_url: String,
  pub created_at: String,
  pub diff_url: String,
  /// Indicates whether or not the pull request is a draft.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub draft: Option<bool>,
  pub head: PullRequestSimpleHead,
  pub html_url: String,
  pub id: i64,
  pub issue_url: String,
  pub labels: Vec<PullRequestSimpleLabels>,
  pub locked: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub merge_commit_sha: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub merged_at: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub milestone: Option<Milestone>,
  pub node_id: String,
  pub number: i64,
  pub patch_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub requested_reviewers: Option<Vec<SimpleUser>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub requested_teams: Option<Vec<Team>>,
  pub review_comment_url: String,
  pub review_comments_url: String,
  pub state: String,
  pub statuses_url: String,
  pub title: String,
  pub updated_at: String,
  pub url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub user: Option<SimpleUser>,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct SimpleCommitStatus {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub avatar_url: Option<String>,
  pub context: String,
  pub created_at: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub description: Option<String>,
  pub id: i64,
  pub node_id: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub required: Option<bool>,
  pub state: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub target_url: Option<String>,
  pub updated_at: String,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// Combined Commit Status
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CombinedCommitStatus {
  pub commit_url: String,
  pub repository: MinimalRepository,
  pub sha: String,
  pub state: String,
  pub statuses: Vec<SimpleCommitStatus>,
  pub total_count: i64,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// The status of a commit.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct Status {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub avatar_url: Option<String>,
  pub context: String,
  pub created_at: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub creator: Option<SimpleUser>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub description: Option<String>,
  pub id: i64,
  pub node_id: String,
  pub state: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub target_url: Option<String>,
  pub updated_at: String,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CommunityHealthFile {
  pub html_url: String,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CommunityProfileFiles {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub code_of_conduct: Option<CodeOfConductSimple>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub code_of_conduct_file: Option<CommunityHealthFile>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub contributing: Option<CommunityHealthFile>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub issue_template: Option<CommunityHealthFile>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub license: Option<LicenseSimple>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub pull_request_template: Option<CommunityHealthFile>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub readme: Option<CommunityHealthFile>,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// Community Profile
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CommunityProfile {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub content_reports_enabled: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub description: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub documentation: Option<String>,
  pub files: CommunityProfileFiles,
  pub health_percentage: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub updated_at: Option<String>,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// Commit Comparison
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CommitComparison {
  pub ahead_by: i64,
  pub base_commit: Commit,
  pub behind_by: i64,
  pub commits: Vec<Commit>,
  pub diff_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub files: Option<Vec<DiffEntry>>,
  pub html_url: String,
  pub merge_base_commit: Commit,
  pub patch_url: String,
  pub permalink_url: String,
  pub status: CommitComparisonStatus,
  pub total_commits: i64,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ContentDirectoryItemLinks {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub git: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub html: Option<String>,
  #[serde(rename = "self")]
  pub self_: String,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ContentDirectoryItem {
  #[serde(rename = "_links")]
  pub links: ContentDirectoryItemLinks,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub content: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub download_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub git_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub html_url: Option<String>,
  pub name: String,
  pub path: String,
  pub sha: String,
  pub size: i64,
  #[serde(rename = "type")]
  pub type_: ContentDirectoryItemType,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ContentFileLinks {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub git: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub html: Option<String>,
  #[serde(rename = "self")]
  pub self_: String,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// Content File
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ContentFile {
  #[serde(rename = "_links")]
  pub links: ContentFileLinks,
  pub content: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub download_url: Option<String>,
  pub encoding: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub git_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub html_url: Option<String>,
  pub name: String,
  pub path: String,
  pub sha: String,
  pub size: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub submodule_git_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub target: Option<String>,
  #[serde(rename = "type")]
  pub type_: ContentFileType,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ContentSymlinkLinks {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub git: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub html: Option<String>,
  #[serde(rename = "self")]
  pub self_: String,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// An object describing a symlink
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ContentSymlink {
  #[serde(rename = "_links")]
  pub links: ContentSymlinkLinks,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub download_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub git_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub html_url: Option<String>,
  pub name: String,
  pub path: String,
  pub sha: String,
  pub size: i64,
  pub target: String,
  #[serde(rename = "type")]
  pub type_: ContentSymlinkType,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ContentSubmoduleLinks {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub git: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub html: Option<String>,
  #[serde(rename = "self")]
  pub self_: String,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// An object describing a submodule
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ContentSubmodule {
  #[serde(rename = "_links")]
  pub links: ContentSubmoduleLinks,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub download_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub git_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub html_url: Option<String>,
  pub name: String,
  pub path: String,
  pub sha: String,
  pub size: i64,
  pub submodule_git_url: String,
  #[serde(rename = "type")]
  pub type_: ContentSubmoduleType,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct FileCommitCommitAuthor {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub date: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub email: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub name: Option<String>,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct FileCommitCommitCommitter {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub date: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub email: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub name: Option<String>,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct FileCommitCommitParents {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub html_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub sha: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub url: Option<String>,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct FileCommitCommitTree {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub sha: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub url: Option<String>,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct FileCommitCommitVerification {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub payload: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub reason: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub signature: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub verified: Option<bool>,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct FileCommitCommit {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub author: Option<FileCommitCommitAuthor>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub committer: Option<FileCommitCommitCommitter>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub html_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub message: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub node_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub parents: Option<Vec<FileCommitCommitParents>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub sha: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub tree: Option<FileCommitCommitTree>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub verification: Option<FileCommitCommitVerification>,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct FileCommitContentLinks {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub git: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub html: Option<String>,
  #[serde(rename = "self")]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub self_: Option<String>,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct FileCommitContent {
  #[serde(rename = "_links")]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub links: Option<FileCommitContentLinks>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub download_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub git_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub html_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub path: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub sha: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub size: Option<i64>,
  #[serde(rename = "type")]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub type_: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub url: Option<String>,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// File Commit
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct FileCommit {
  pub commit: FileCommitCommit,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub content: Option<FileCommitContent>,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// Contributor
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct Contributor {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub avatar_url: Option<String>,
  pub contributions: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub email: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub events_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub followers_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub following_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub gists_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub gravatar_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub html_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub id: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub login: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub node_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub organizations_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub received_events_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub repos_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub site_admin: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub starred_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub subscriptions_url: Option<String>,
  #[serde(rename = "type")]
  pub type_: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub url: Option<String>,
}

#[cfg(any(feature = "full", feature = "dependabot"))]
/// Details for the vulnerable dependency.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct DependabotAlertDependency {
  /// The full path to the dependency manifest file, relative to the root of the repository.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub manifest_path: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub package: Option<DependabotAlertPackage>,
  /// The execution scope of the vulnerable dependency.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub scope: Option<DependabotAlertDependencyScope>,
}

#[cfg(any(feature = "full", feature = "dependabot"))]
/// A Dependabot alert.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct DependabotAlert {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub auto_dismissed_at: Option<String>,
  pub created_at: String,
  /// Details for the vulnerable dependency.
  pub dependency: DependabotAlertDependency,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub dismissed_at: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub dismissed_by: Option<SimpleUser>,
  /// An optional comment associated with the alert's dismissal.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub dismissed_comment: Option<String>,
  /// The reason that the alert was dismissed.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub dismissed_reason: Option<DismissedReason>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub fixed_at: Option<String>,
  pub html_url: String,
  pub number: i64,
  pub security_advisory: DependabotAlertSecurityAdvisory,
  pub security_vulnerability: DependabotAlertSecurityVulnerability,
  /// The state of the Dependabot alert.
  pub state: DependabotAlertState,
  pub updated_at: String,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "dependabot"))]
/// Set secrets for Dependabot.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct DependabotSecret {
  pub created_at: String,
  /// The name of the secret.
  pub name: String,
  pub updated_at: String,
}

#[cfg(any(feature = "full", feature = "dependency_graph"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct DependencyGraphDiffItemVulnerabilities {
  pub advisory_ghsa_id: String,
  pub advisory_summary: String,
  pub advisory_url: String,
  pub severity: String,
}

#[cfg(any(feature = "full", feature = "dependency_graph"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct DependencyGraphDiffItem {
  pub change_type: DependencyGraphDiffItemChangeType,
  pub ecosystem: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub license: Option<String>,
  pub manifest: String,
  pub name: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub package_url: Option<String>,
  /// Where the dependency is utilized. `development` means that the dependency is only utilized in the development environment. `runtime` means that the dependency is utilized at runtime and in the development environment.
  pub scope: DependencyGraphDiffItemScope,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub source_repository_url: Option<String>,
  pub version: String,
  pub vulnerabilities: Vec<DependencyGraphDiffItemVulnerabilities>,
}

#[cfg(any(feature = "full", feature = "dependency_graph"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct DependencyGraphSpdxSbomSbomCreationInfo {
  /// The date and time the SPDX document was created.
  pub created: String,
  /// The tools that were used to generate the SPDX document.
  pub creators: Vec<String>,
}

#[cfg(any(feature = "full", feature = "dependency_graph"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct DependencyGraphSpdxSbomSbomPackagesExternalRefs {
  /// The category of reference to an external resource this reference refers to.
  #[serde(rename = "referenceCategory")]
  pub referencecategory: String,
  /// A locator for the particular external resource this reference refers to.
  #[serde(rename = "referenceLocator")]
  pub referencelocator: String,
  /// The category of reference to an external resource this reference refers to.
  #[serde(rename = "referenceType")]
  pub referencetype: String,
}

#[cfg(any(feature = "full", feature = "dependency_graph"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct DependencyGraphSpdxSbomSbomPackages {
  /// A unique SPDX identifier for the package.
  #[serde(rename = "SPDXID")]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub spdxid: Option<String>,
  /// The location where the package can be downloaded,
  /// or NOASSERTION if this has not been determined.
  #[serde(rename = "downloadLocation")]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub downloadlocation: Option<String>,
  #[serde(rename = "externalRefs")]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub externalrefs: Option<Vec<DependencyGraphSpdxSbomSbomPackagesExternalRefs>>,
  /// Whether the package's file content has been subjected to
  /// analysis during the creation of the SPDX document.
  #[serde(rename = "filesAnalyzed")]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub filesanalyzed: Option<bool>,
  /// The license of the package as determined while creating the SPDX document.
  #[serde(rename = "licenseConcluded")]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub licenseconcluded: Option<String>,
  /// The license of the package as declared by its author, or NOASSERTION if this information
  /// was not available when the SPDX document was created.
  #[serde(rename = "licenseDeclared")]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub licensedeclared: Option<String>,
  /// The name of the package.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub name: Option<String>,
  /// The distribution source of this package, or NOASSERTION if this was not determined.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub supplier: Option<String>,
  /// The version of the package. If the package does not have an exact version specified,
  /// a version range is given.
  #[serde(rename = "versionInfo")]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub versioninfo: Option<String>,
}

#[cfg(any(feature = "full", feature = "dependency_graph"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct DependencyGraphSpdxSbomSbom {
  /// The SPDX identifier for the SPDX document.
  #[serde(rename = "SPDXID")]
  pub spdxid: String,
  #[serde(rename = "creationInfo")]
  pub creationinfo: DependencyGraphSpdxSbomSbomCreationInfo,
  /// The license under which the SPDX document is licensed.
  #[serde(rename = "dataLicense")]
  pub datalicense: String,
  /// The name of the repository that the SPDX document describes.
  #[serde(rename = "documentDescribes")]
  pub documentdescribes: Vec<String>,
  /// The namespace for the SPDX document.
  #[serde(rename = "documentNamespace")]
  pub documentnamespace: String,
  /// The name of the SPDX document.
  pub name: String,
  pub packages: Vec<DependencyGraphSpdxSbomSbomPackages>,
  /// The version of the SPDX specification that this document conforms to.
  #[serde(rename = "spdxVersion")]
  pub spdxversion: String,
}

#[cfg(any(feature = "full", feature = "dependency_graph"))]
/// A schema for the SPDX JSON format returned by the Dependency Graph.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct DependencyGraphSpdxSbom {
  pub sbom: DependencyGraphSpdxSbomSbom,
}

#[cfg(any(feature = "full", feature = "dependency_graph"))]
/// A description of the detector used.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct SnapshotDetector {
  /// The name of the detector used.
  pub name: String,
  /// The url of the detector used.
  pub url: String,
  /// The version of the detector used.
  pub version: String,
}

#[cfg(any(feature = "full", feature = "dependency_graph"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct SnapshotJob {
  /// Correlator provides a key that is used to group snapshots submitted over time. Only the "latest" submitted snapshot for a given combination of `job.correlator` and `detector.name` will be considered when calculating a repository's current dependencies. Correlator should be as unique as it takes to distinguish all detection runs for a given "wave" of CI workflow you run. If you're using GitHub Actions, a good default value for this could be the environment variables GITHUB_WORKFLOW and GITHUB_JOB concatenated together. If you're using a build matrix, then you'll also need to add additional key(s) to distinguish between each submission inside a matrix variation.
  pub correlator: String,
  /// The url for the job.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub html_url: Option<String>,
  /// The external ID of the job.
  pub id: String,
}

#[cfg(any(feature = "full", feature = "dependency_graph"))]
/// Create a new snapshot of a repository's dependencies.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct Snapshot {
  /// A description of the detector used.
  pub detector: SnapshotDetector,
  pub job: SnapshotJob,
  /// A collection of package manifests, which are a collection of related dependencies declared in a file or representing a logical group of dependencies.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub manifests: Option<serde_json::Value>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub metadata: Option<serde_json::Value>,
  /// The repository branch that triggered this snapshot.
  #[serde(rename = "ref")]
  pub ref_: String,
  /// The time at which the snapshot was scanned.
  pub scanned: String,
  /// The commit SHA associated with this dependency snapshot. Maximum length: 40 characters.
  pub sha: String,
  /// The version of the repository snapshot submission.
  pub version: i64,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// The status of a deployment.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct DeploymentStatus {
  pub created_at: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub creator: Option<SimpleUser>,
  pub deployment_url: String,
  /// A short description of the status.
  pub description: String,
  /// The environment of the deployment that the status is for.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub environment: Option<String>,
  /// The URL for accessing your environment.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub environment_url: Option<String>,
  pub id: i64,
  /// The URL to associate with this status.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub log_url: Option<String>,
  pub node_id: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub performed_via_github_app: Option<Integration>,
  pub repository_url: String,
  /// The state of the status.
  pub state: DeploymentStatusState,
  /// Deprecated: the URL to associate with this status.
  pub target_url: String,
  pub updated_at: String,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// The type of deployment branch policy for this environment. To allow all branches to deploy, set to `null`.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct DeploymentBranchPolicySettings {
  /// Whether only branches that match the specified name patterns can deploy to this environment.  If `custom_branch_policies` is `true`, `protected_branches` must be `false`; if `custom_branch_policies` is `false`, `protected_branches` must be `true`.
  pub custom_branch_policies: bool,
  /// Whether only branches with branch protection rules can deploy to this environment. If `protected_branches` is `true`, `custom_branch_policies` must be `false`; if `protected_branches` is `false`, `custom_branch_policies` must be `true`.
  pub protected_branches: bool,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct EnvironmentProtectionRulesItem1 {
  pub id: i64,
  pub node_id: String,
  #[serde(rename = "type")]
  pub type_: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub wait_timer: Option<i64>,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct EnvironmentProtectionRulesItem2Reviewers {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub reviewer: Option<EnvironmentProtectionRulesItem2ReviewersReviewer>,
  #[serde(rename = "type")]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub type_: Option<DeploymentReviewerType>,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct EnvironmentProtectionRulesItem2 {
  pub id: i64,
  pub node_id: String,
  /// Whether deployments to this environment can be approved by the user who created the deployment.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub prevent_self_review: Option<bool>,
  /// The people or teams that may approve jobs that reference the environment. You can list up to six users or teams as reviewers. The reviewers must have at least read access to the repository. Only one of the required reviewers needs to approve the job for it to proceed.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub reviewers: Option<Vec<EnvironmentProtectionRulesItem2Reviewers>>,
  #[serde(rename = "type")]
  pub type_: String,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct EnvironmentProtectionRulesItem3 {
  pub id: i64,
  pub node_id: String,
  #[serde(rename = "type")]
  pub type_: String,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// Details of a deployment environment
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct Environment {
  /// The time that the environment was created, in ISO 8601 format.
  pub created_at: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub deployment_branch_policy: Option<DeploymentBranchPolicySettings>,
  pub html_url: String,
  /// The id of the environment.
  pub id: i64,
  /// The name of the environment.
  pub name: String,
  pub node_id: String,
  /// Built-in deployment protection rules for the environment.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub protection_rules: Option<Vec<EnvironmentProtectionRules>>,
  /// The time that the environment was last updated, in ISO 8601 format.
  pub updated_at: String,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// Details of a deployment branch or tag policy.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct DeploymentBranchPolicy {
  /// The unique identifier of the branch or tag policy.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub id: Option<i64>,
  /// The name pattern that branches or tags must match in order to deploy to the environment.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub node_id: Option<String>,
  /// Whether this rule targets a branch or tag.
  #[serde(rename = "type")]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub type_: Option<DeploymentBranchPolicyType>,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct DeploymentBranchPolicyNamePatternWithType {
  /// The name pattern that branches or tags must match in order to deploy to the environment.
  ///
  /// Wildcard characters will not match `/`. For example, to match branches that begin with `release/` and contain an additional single slash, use `release/*/*`.
  /// For more information about pattern matching syntax, see the [Ruby File.fnmatch documentation](https://ruby-doc.org/core-2.5.1/File.html#method-c-fnmatch).
  pub name: String,
  /// Whether this rule targets a branch or tag
  #[serde(rename = "type")]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub type_: Option<DeploymentBranchPolicyNamePatternWithTypeType>,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct DeploymentBranchPolicyNamePattern {
  /// The name pattern that branches must match in order to deploy to the environment.
  ///
  /// Wildcard characters will not match `/`. For example, to match branches that begin with `release/` and contain an additional single slash, use `release/*/*`.
  /// For more information about pattern matching syntax, see the [Ruby File.fnmatch documentation](https://ruby-doc.org/core-2.5.1/File.html#method-c-fnmatch).
  pub name: String,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// A GitHub App that is providing a custom deployment protection rule.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CustomDeploymentRuleApp {
  /// The unique identifier of the deployment protection rule integration.
  pub id: i64,
  /// The URL for the endpoint to get details about the app.
  pub integration_url: String,
  /// The node ID for the deployment protection rule integration.
  pub node_id: String,
  /// The slugified name of the deployment protection rule integration.
  pub slug: String,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// Deployment protection rule
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct DeploymentProtectionRule {
  pub app: CustomDeploymentRuleApp,
  /// Whether the deployment protection rule is enabled for the environment.
  pub enabled: bool,
  /// The unique identifier for the deployment protection rule.
  pub id: i64,
  /// The node ID for the deployment protection rule.
  pub node_id: String,
}

#[cfg(any(feature = "full", feature = "git"))]
/// Short Blob
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ShortBlob {
  pub sha: String,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "git"))]
/// Blob
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct Blob {
  pub content: String,
  pub encoding: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub highlighted_content: Option<String>,
  pub node_id: String,
  pub sha: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub size: Option<i64>,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "git"))]
/// Identifying information for the git-user
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct GitCommitAuthor {
  /// Timestamp of the commit
  pub date: String,
  /// Git email address of the user
  pub email: String,
  /// Name of the git user
  pub name: String,
}

#[cfg(any(feature = "full", feature = "git"))]
/// Identifying information for the git-user
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct GitCommitCommitter {
  /// Timestamp of the commit
  pub date: String,
  /// Git email address of the user
  pub email: String,
  /// Name of the git user
  pub name: String,
}

#[cfg(any(feature = "full", feature = "git"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct GitCommitParents {
  pub html_url: String,
  /// SHA for the commit
  pub sha: String,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "git"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct GitCommitTree {
  /// SHA for the commit
  pub sha: String,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "git"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct GitCommitVerification {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub payload: Option<String>,
  pub reason: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub signature: Option<String>,
  pub verified: bool,
}

#[cfg(any(feature = "full", feature = "git"))]
/// Low-level Git commit operations within a repository
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct GitCommit {
  /// Identifying information for the git-user
  pub author: GitCommitAuthor,
  /// Identifying information for the git-user
  pub committer: GitCommitCommitter,
  pub html_url: String,
  /// Message describing the purpose of the commit
  pub message: String,
  pub node_id: String,
  pub parents: Vec<GitCommitParents>,
  /// SHA for the commit
  pub sha: String,
  pub tree: GitCommitTree,
  pub url: String,
  pub verification: GitCommitVerification,
}

#[cfg(any(feature = "full", feature = "git"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct GitRefObject {
  /// SHA for the reference
  pub sha: String,
  #[serde(rename = "type")]
  pub type_: String,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "git"))]
/// Git references within a repository
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct GitRef {
  pub node_id: String,
  pub object: GitRefObject,
  #[serde(rename = "ref")]
  pub ref_: String,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "git"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct GitTagObject {
  pub sha: String,
  #[serde(rename = "type")]
  pub type_: String,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "git"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct GitTagTagger {
  pub date: String,
  pub email: String,
  pub name: String,
}

#[cfg(any(feature = "full", feature = "git"))]
/// Metadata for a Git tag
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct GitTag {
  /// Message describing the purpose of the tag
  pub message: String,
  pub node_id: String,
  pub object: GitTagObject,
  pub sha: String,
  /// Name of the tag
  pub tag: String,
  pub tagger: GitTagTagger,
  /// URL for the tag
  pub url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub verification: Option<Verification>,
}

#[cfg(any(feature = "full", feature = "git"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct GitTreeTree {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub mode: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub path: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub sha: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub size: Option<i64>,
  #[serde(rename = "type")]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub type_: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub url: Option<String>,
}

#[cfg(any(feature = "full", feature = "git"))]
/// The hierarchy between files in a Git repository.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct GitTree {
  pub sha: String,
  /// Objects specifying a tree structure
  pub tree: Vec<GitTreeTree>,
  pub truncated: bool,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct HookResponse {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub code: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub message: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub status: Option<String>,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// Webhooks for repositories.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct Hook {
  /// Determines whether the hook is actually triggered on pushes.
  pub active: bool,
  pub config: WebhookConfig,
  pub created_at: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub deliveries_url: Option<String>,
  /// Determines what events the hook is triggered for. Default: ['push'].
  pub events: Vec<String>,
  /// Unique identifier of the webhook.
  pub id: i64,
  pub last_response: HookResponse,
  /// The name of a valid service, use 'web' for a webhook.
  pub name: String,
  pub ping_url: String,
  pub test_url: String,
  #[serde(rename = "type")]
  pub type_: String,
  pub updated_at: String,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "migrations"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ImportProjectChoices {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub human_name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub tfvc_project: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub vcs: Option<String>,
}

#[cfg(any(feature = "full", feature = "migrations"))]
/// A repository import from an external source.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct Import {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub authors_count: Option<i64>,
  pub authors_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub commit_count: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub error_message: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub failed_step: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub has_large_files: Option<bool>,
  pub html_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub import_percent: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub large_files_count: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub large_files_size: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub message: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub project_choices: Option<Vec<ImportProjectChoices>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub push_percent: Option<i64>,
  pub repository_url: String,
  pub status: ImportStatus,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub status_text: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub svc_root: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub svn_root: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub tfvc_project: Option<String>,
  pub url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub use_lfs: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub vcs: Option<String>,
  /// The URL of the originating repository.
  pub vcs_url: String,
}

#[cfg(any(feature = "full", feature = "migrations"))]
/// Porter Author
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct PorterAuthor {
  pub email: String,
  pub id: i64,
  pub import_url: String,
  pub name: String,
  pub remote_id: String,
  pub remote_name: String,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "migrations"))]
/// Porter Large File
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct PorterLargeFile {
  pub oid: String,
  pub path: String,
  pub ref_name: String,
  pub size: i64,
}

#[cfg(any(feature = "full", feature = "issues"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct IssueEventDismissedReview {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub dismissal_commit_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub dismissal_message: Option<String>,
  pub review_id: i64,
  pub state: String,
}

#[cfg(any(feature = "full", feature = "issues"))]
/// Issue Event Label
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct IssueEventLabel {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub color: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub name: Option<String>,
}

#[cfg(any(feature = "full", feature = "issues"))]
/// Issue Event Milestone
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct IssueEventMilestone {
  pub title: String,
}

#[cfg(any(feature = "full", feature = "issues"))]
/// Issue Event Project Card
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct IssueEventProjectCard {
  pub column_name: String,
  pub id: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub previous_column_name: Option<String>,
  pub project_id: i64,
  pub project_url: String,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "issues"))]
/// Issue Event Rename
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct IssueEventRename {
  pub from: String,
  pub to: String,
}

#[cfg(any(feature = "full", feature = "issues"))]
/// Issue Event
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct IssueEvent {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub actor: Option<SimpleUser>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub assignee: Option<SimpleUser>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub assigner: Option<SimpleUser>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub author_association: Option<AuthorAssociation>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub commit_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub commit_url: Option<String>,
  pub created_at: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub dismissed_review: Option<IssueEventDismissedReview>,
  pub event: String,
  pub id: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub issue: Option<Issue>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub label: Option<IssueEventLabel>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub lock_reason: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub milestone: Option<IssueEventMilestone>,
  pub node_id: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub performed_via_github_app: Option<Integration>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub project_card: Option<IssueEventProjectCard>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub rename: Option<IssueEventRename>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub requested_reviewer: Option<SimpleUser>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub requested_team: Option<Team>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub review_requester: Option<SimpleUser>,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "issues"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct LabeledIssueEventLabel {
  pub color: String,
  pub name: String,
}

#[cfg(any(feature = "full", feature = "issues"))]
/// Labeled Issue Event
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct LabeledIssueEvent {
  pub actor: SimpleUser,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub commit_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub commit_url: Option<String>,
  pub created_at: String,
  pub event: String,
  pub id: i64,
  pub label: LabeledIssueEventLabel,
  pub node_id: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub performed_via_github_app: Option<Integration>,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "issues"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct UnlabeledIssueEventLabel {
  pub color: String,
  pub name: String,
}

#[cfg(any(feature = "full", feature = "issues"))]
/// Unlabeled Issue Event
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct UnlabeledIssueEvent {
  pub actor: SimpleUser,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub commit_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub commit_url: Option<String>,
  pub created_at: String,
  pub event: String,
  pub id: i64,
  pub label: UnlabeledIssueEventLabel,
  pub node_id: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub performed_via_github_app: Option<Integration>,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "issues"))]
/// Assigned Issue Event
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct AssignedIssueEvent {
  pub actor: SimpleUser,
  pub assignee: SimpleUser,
  pub assigner: SimpleUser,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub commit_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub commit_url: Option<String>,
  pub created_at: String,
  pub event: String,
  pub id: i64,
  pub node_id: String,
  pub performed_via_github_app: Integration,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "issues"))]
/// Unassigned Issue Event
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct UnassignedIssueEvent {
  pub actor: SimpleUser,
  pub assignee: SimpleUser,
  pub assigner: SimpleUser,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub commit_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub commit_url: Option<String>,
  pub created_at: String,
  pub event: String,
  pub id: i64,
  pub node_id: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub performed_via_github_app: Option<Integration>,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "issues"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct MilestonedIssueEventMilestone {
  pub title: String,
}

#[cfg(any(feature = "full", feature = "issues"))]
/// Milestoned Issue Event
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct MilestonedIssueEvent {
  pub actor: SimpleUser,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub commit_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub commit_url: Option<String>,
  pub created_at: String,
  pub event: String,
  pub id: i64,
  pub milestone: MilestonedIssueEventMilestone,
  pub node_id: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub performed_via_github_app: Option<Integration>,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "issues"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct DemilestonedIssueEventMilestone {
  pub title: String,
}

#[cfg(any(feature = "full", feature = "issues"))]
/// Demilestoned Issue Event
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct DemilestonedIssueEvent {
  pub actor: SimpleUser,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub commit_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub commit_url: Option<String>,
  pub created_at: String,
  pub event: String,
  pub id: i64,
  pub milestone: DemilestonedIssueEventMilestone,
  pub node_id: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub performed_via_github_app: Option<Integration>,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "issues"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RenamedIssueEventRename {
  pub from: String,
  pub to: String,
}

#[cfg(any(feature = "full", feature = "issues"))]
/// Renamed Issue Event
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RenamedIssueEvent {
  pub actor: SimpleUser,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub commit_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub commit_url: Option<String>,
  pub created_at: String,
  pub event: String,
  pub id: i64,
  pub node_id: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub performed_via_github_app: Option<Integration>,
  pub rename: RenamedIssueEventRename,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "issues"))]
/// Review Requested Issue Event
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ReviewRequestedIssueEvent {
  pub actor: SimpleUser,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub commit_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub commit_url: Option<String>,
  pub created_at: String,
  pub event: String,
  pub id: i64,
  pub node_id: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub performed_via_github_app: Option<Integration>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub requested_reviewer: Option<SimpleUser>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub requested_team: Option<Team>,
  pub review_requester: SimpleUser,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "issues"))]
/// Review Request Removed Issue Event
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ReviewRequestRemovedIssueEvent {
  pub actor: SimpleUser,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub commit_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub commit_url: Option<String>,
  pub created_at: String,
  pub event: String,
  pub id: i64,
  pub node_id: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub performed_via_github_app: Option<Integration>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub requested_reviewer: Option<SimpleUser>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub requested_team: Option<Team>,
  pub review_requester: SimpleUser,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "issues"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ReviewDismissedIssueEventDismissedReview {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub dismissal_commit_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub dismissal_message: Option<String>,
  pub review_id: i64,
  pub state: String,
}

#[cfg(any(feature = "full", feature = "issues"))]
/// Review Dismissed Issue Event
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ReviewDismissedIssueEvent {
  pub actor: SimpleUser,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub commit_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub commit_url: Option<String>,
  pub created_at: String,
  pub dismissed_review: ReviewDismissedIssueEventDismissedReview,
  pub event: String,
  pub id: i64,
  pub node_id: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub performed_via_github_app: Option<Integration>,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "issues"))]
/// Locked Issue Event
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct LockedIssueEvent {
  pub actor: SimpleUser,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub commit_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub commit_url: Option<String>,
  pub created_at: String,
  pub event: String,
  pub id: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub lock_reason: Option<String>,
  pub node_id: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub performed_via_github_app: Option<Integration>,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "issues"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct AddedToProjectIssueEventProjectCard {
  pub column_name: String,
  pub id: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub previous_column_name: Option<String>,
  pub project_id: i64,
  pub project_url: String,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "issues"))]
/// Added to Project Issue Event
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct AddedToProjectIssueEvent {
  pub actor: SimpleUser,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub commit_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub commit_url: Option<String>,
  pub created_at: String,
  pub event: String,
  pub id: i64,
  pub node_id: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub performed_via_github_app: Option<Integration>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub project_card: Option<AddedToProjectIssueEventProjectCard>,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "issues"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct MovedColumnInProjectIssueEventProjectCard {
  pub column_name: String,
  pub id: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub previous_column_name: Option<String>,
  pub project_id: i64,
  pub project_url: String,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "issues"))]
/// Moved Column in Project Issue Event
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct MovedColumnInProjectIssueEvent {
  pub actor: SimpleUser,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub commit_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub commit_url: Option<String>,
  pub created_at: String,
  pub event: String,
  pub id: i64,
  pub node_id: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub performed_via_github_app: Option<Integration>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub project_card: Option<MovedColumnInProjectIssueEventProjectCard>,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "issues"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RemovedFromProjectIssueEventProjectCard {
  pub column_name: String,
  pub id: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub previous_column_name: Option<String>,
  pub project_id: i64,
  pub project_url: String,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "issues"))]
/// Removed from Project Issue Event
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RemovedFromProjectIssueEvent {
  pub actor: SimpleUser,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub commit_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub commit_url: Option<String>,
  pub created_at: String,
  pub event: String,
  pub id: i64,
  pub node_id: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub performed_via_github_app: Option<Integration>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub project_card: Option<RemovedFromProjectIssueEventProjectCard>,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "issues"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ConvertedNoteToIssueIssueEventProjectCard {
  pub column_name: String,
  pub id: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub previous_column_name: Option<String>,
  pub project_id: i64,
  pub project_url: String,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "issues"))]
/// Converted Note to Issue Issue Event
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ConvertedNoteToIssueIssueEvent {
  pub actor: SimpleUser,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub commit_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub commit_url: Option<String>,
  pub created_at: String,
  pub event: String,
  pub id: i64,
  pub node_id: String,
  pub performed_via_github_app: Integration,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub project_card: Option<ConvertedNoteToIssueIssueEventProjectCard>,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "issues"))]
/// Color-coded labels help you categorize and filter your issues (just like labels in Gmail).
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct Label {
  /// 6-character hex code, without the leading #, identifying the color
  pub color: String,
  #[serde(rename = "default")]
  pub default_: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub description: Option<String>,
  pub id: i64,
  /// The name of the label.
  pub name: String,
  pub node_id: String,
  /// URL for the label
  pub url: String,
}

#[cfg(any(feature = "full", feature = "issues"))]
/// Timeline Comment Event
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct TimelineCommentEvent {
  pub actor: SimpleUser,
  pub author_association: AuthorAssociation,
  /// Contents of the issue comment
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub body: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub body_html: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub body_text: Option<String>,
  pub created_at: String,
  pub event: String,
  pub html_url: String,
  /// Unique identifier of the issue comment
  pub id: i64,
  pub issue_url: String,
  pub node_id: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub performed_via_github_app: Option<Integration>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub reactions: Option<ReactionRollup>,
  pub updated_at: String,
  /// URL for the issue comment
  pub url: String,
  pub user: SimpleUser,
}

#[cfg(any(feature = "full", feature = "issues"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct TimelineCrossReferencedEventSource {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub issue: Option<Issue>,
  #[serde(rename = "type")]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub type_: Option<String>,
}

#[cfg(any(feature = "full", feature = "issues"))]
/// Timeline Cross Referenced Event
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct TimelineCrossReferencedEvent {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub actor: Option<SimpleUser>,
  pub created_at: String,
  pub event: String,
  pub source: TimelineCrossReferencedEventSource,
  pub updated_at: String,
}

#[cfg(any(feature = "full", feature = "issues"))]
/// Identifying information for the git-user
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct TimelineCommittedEventAuthor {
  /// Timestamp of the commit
  pub date: String,
  /// Git email address of the user
  pub email: String,
  /// Name of the git user
  pub name: String,
}

#[cfg(any(feature = "full", feature = "issues"))]
/// Identifying information for the git-user
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct TimelineCommittedEventCommitter {
  /// Timestamp of the commit
  pub date: String,
  /// Git email address of the user
  pub email: String,
  /// Name of the git user
  pub name: String,
}

#[cfg(any(feature = "full", feature = "issues"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct TimelineCommittedEventParents {
  pub html_url: String,
  /// SHA for the commit
  pub sha: String,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "issues"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct TimelineCommittedEventTree {
  /// SHA for the commit
  pub sha: String,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "issues"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct TimelineCommittedEventVerification {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub payload: Option<String>,
  pub reason: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub signature: Option<String>,
  pub verified: bool,
}

#[cfg(any(feature = "full", feature = "issues"))]
/// Timeline Committed Event
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct TimelineCommittedEvent {
  /// Identifying information for the git-user
  pub author: TimelineCommittedEventAuthor,
  /// Identifying information for the git-user
  pub committer: TimelineCommittedEventCommitter,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub event: Option<String>,
  pub html_url: String,
  /// Message describing the purpose of the commit
  pub message: String,
  pub node_id: String,
  pub parents: Vec<TimelineCommittedEventParents>,
  /// SHA for the commit
  pub sha: String,
  pub tree: TimelineCommittedEventTree,
  pub url: String,
  pub verification: TimelineCommittedEventVerification,
}

#[cfg(any(feature = "full", feature = "issues"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct TimelineReviewedEventLinksHtml {
  pub href: String,
}

#[cfg(any(feature = "full", feature = "issues"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct TimelineReviewedEventLinksPullRequest {
  pub href: String,
}

#[cfg(any(feature = "full", feature = "issues"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct TimelineReviewedEventLinks {
  pub html: TimelineReviewedEventLinksHtml,
  pub pull_request: TimelineReviewedEventLinksPullRequest,
}

#[cfg(any(feature = "full", feature = "issues"))]
/// Timeline Reviewed Event
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct TimelineReviewedEvent {
  #[serde(rename = "_links")]
  pub links: TimelineReviewedEventLinks,
  pub author_association: AuthorAssociation,
  /// The text of the review.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub body: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub body_html: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub body_text: Option<String>,
  /// A commit SHA for the review.
  pub commit_id: String,
  pub event: String,
  pub html_url: String,
  /// Unique identifier of the review
  pub id: i64,
  pub node_id: String,
  pub pull_request_url: String,
  pub state: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub submitted_at: Option<String>,
  pub user: SimpleUser,
}

#[cfg(any(feature = "full", feature = "issues", feature = "pulls"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct PullRequestReviewCommentLinksHtml {
  pub href: String,
}

#[cfg(any(feature = "full", feature = "issues", feature = "pulls"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct PullRequestReviewCommentLinksPullRequest {
  pub href: String,
}

#[cfg(any(feature = "full", feature = "issues", feature = "pulls"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct PullRequestReviewCommentLinksSelf {
  pub href: String,
}

#[cfg(any(feature = "full", feature = "issues", feature = "pulls"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct PullRequestReviewCommentLinks {
  pub html: PullRequestReviewCommentLinksHtml,
  pub pull_request: PullRequestReviewCommentLinksPullRequest,
  #[serde(rename = "self")]
  pub self_: PullRequestReviewCommentLinksSelf,
}

#[cfg(any(feature = "full", feature = "issues", feature = "pulls"))]
/// Pull Request Review Comments are comments on a portion of the Pull Request's diff.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct PullRequestReviewComment {
  #[serde(rename = "_links")]
  pub links: PullRequestReviewCommentLinks,
  pub author_association: AuthorAssociation,
  /// The text of the comment.
  pub body: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub body_html: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub body_text: Option<String>,
  /// The SHA of the commit to which the comment applies.
  pub commit_id: String,
  pub created_at: String,
  /// The diff of the line that the comment refers to.
  pub diff_hunk: String,
  /// HTML URL for the pull request review comment.
  pub html_url: String,
  /// The ID of the pull request review comment.
  pub id: i64,
  /// The comment ID to reply to.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub in_reply_to_id: Option<i64>,
  /// The line of the blob to which the comment applies. The last line of the range for a multi-line comment
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub line: Option<i64>,
  /// The node ID of the pull request review comment.
  pub node_id: String,
  /// The SHA of the original commit to which the comment applies.
  pub original_commit_id: String,
  /// The line of the blob to which the comment applies. The last line of the range for a multi-line comment
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub original_line: Option<i64>,
  /// The index of the original line in the diff to which the comment applies. This field is deprecated; use `original_line` instead.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub original_position: Option<i64>,
  /// The first line of the range for a multi-line comment.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub original_start_line: Option<i64>,
  /// The relative path of the file to which the comment applies.
  pub path: String,
  /// The line index in the diff to which the comment applies. This field is deprecated; use `line` instead.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub position: Option<i64>,
  /// The ID of the pull request review to which the comment belongs.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub pull_request_review_id: Option<i64>,
  /// URL for the pull request that the review comment belongs to.
  pub pull_request_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub reactions: Option<ReactionRollup>,
  /// The side of the diff to which the comment applies. The side of the last line of the range for a multi-line comment
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub side: Option<PullRequestReviewCommentSide>,
  /// The first line of the range for a multi-line comment.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub start_line: Option<i64>,
  /// The side of the first line of the range for a multi-line comment.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub start_side: Option<PullRequestReviewCommentStartSide>,
  /// The level at which the comment is targeted, can be a diff line or a file.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub subject_type: Option<PullRequestReviewCommentSubjectType>,
  pub updated_at: String,
  /// URL for the pull request review comment
  pub url: String,
  pub user: SimpleUser,
}

#[cfg(any(feature = "full", feature = "issues"))]
/// Timeline Line Commented Event
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct TimelineLineCommentedEvent {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub comments: Option<Vec<PullRequestReviewComment>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub event: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub node_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "issues"))]
/// Timeline Commit Commented Event
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct TimelineCommitCommentedEvent {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub comments: Option<Vec<CommitComment>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub commit_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub event: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub node_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "issues"))]
/// Timeline Assigned Issue Event
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct TimelineAssignedIssueEvent {
  pub actor: SimpleUser,
  pub assignee: SimpleUser,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub commit_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub commit_url: Option<String>,
  pub created_at: String,
  pub event: String,
  pub id: i64,
  pub node_id: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub performed_via_github_app: Option<Integration>,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "issues"))]
/// Timeline Unassigned Issue Event
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct TimelineUnassignedIssueEvent {
  pub actor: SimpleUser,
  pub assignee: SimpleUser,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub commit_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub commit_url: Option<String>,
  pub created_at: String,
  pub event: String,
  pub id: i64,
  pub node_id: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub performed_via_github_app: Option<Integration>,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "issues"))]
/// State Change Issue Event
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct StateChangeIssueEvent {
  pub actor: SimpleUser,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub commit_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub commit_url: Option<String>,
  pub created_at: String,
  pub event: String,
  pub id: i64,
  pub node_id: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub performed_via_github_app: Option<Integration>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub state_reason: Option<String>,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// An SSH key granting access to a single repository.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct DeployKey {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub added_by: Option<String>,
  pub created_at: String,
  pub id: i64,
  pub key: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub last_used: Option<String>,
  pub read_only: bool,
  pub title: String,
  pub url: String,
  pub verified: bool,
}

#[cfg(any(feature = "full", feature = "licenses"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct LicenseContentLinks {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub git: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub html: Option<String>,
  #[serde(rename = "self")]
  pub self_: String,
}

#[cfg(any(feature = "full", feature = "licenses"))]
/// License Content
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct LicenseContent {
  #[serde(rename = "_links")]
  pub links: LicenseContentLinks,
  pub content: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub download_url: Option<String>,
  pub encoding: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub git_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub html_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub license: Option<LicenseSimple>,
  pub name: String,
  pub path: String,
  pub sha: String,
  pub size: i64,
  #[serde(rename = "type")]
  pub type_: String,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// Results of a successful merge upstream request
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct MergedUpstream {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub base_branch: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub merge_type: Option<MergedUpstreamMergeType>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub message: Option<String>,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct PagesHttpsCertificate {
  pub description: String,
  /// Array of the domain set and its alternate name (if it is configured)
  pub domains: Vec<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub expires_at: Option<String>,
  pub state: PagesHttpsCertificateState,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct PagesSourceHash {
  pub branch: String,
  pub path: String,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// The configuration for GitHub Pages for a repository.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct Page {
  /// The process in which the Page will be built.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub build_type: Option<PageBuildType>,
  /// The Pages site's custom domain
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub cname: Option<String>,
  /// Whether the Page has a custom 404 page.
  pub custom_404: bool,
  /// The web address the Page can be accessed from.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub html_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub https_certificate: Option<PagesHttpsCertificate>,
  /// Whether https is enabled on the domain
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub https_enforced: Option<bool>,
  /// The timestamp when a pending domain becomes unverified.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub pending_domain_unverified_at: Option<String>,
  /// The state if the domain is verified
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub protected_domain_state: Option<PageProtectedDomainState>,
  /// Whether the GitHub Pages site is publicly visible. If set to `true`, the site is accessible to anyone on the internet. If set to `false`, the site will only be accessible to users who have at least `read` access to the repository that published the site.
  pub public: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub source: Option<PagesSourceHash>,
  /// The status of the most recent build of the Page.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub status: Option<PageStatus>,
  /// The API address for accessing this Page resource.
  pub url: String,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct PageBuildError {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub message: Option<String>,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// Page Build
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct PageBuild {
  pub commit: String,
  pub created_at: String,
  pub duration: i64,
  pub error: PageBuildError,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub pusher: Option<SimpleUser>,
  pub status: String,
  pub updated_at: String,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// Page Build Status
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct PageBuildStatus {
  pub status: String,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// The GitHub Pages deployment status.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct PageDeployment {
  /// The ID of the GitHub Pages deployment. This is the Git SHA of the deployed commit.
  pub id: StringOrInteger,
  /// The URI to the deployed GitHub Pages.
  pub page_url: String,
  /// The URI to the deployed GitHub Pages preview.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub preview_url: Option<String>,
  /// The URI to monitor GitHub Pages deployment status.
  pub status_url: String,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct PagesDeploymentStatus {
  /// The current status of the deployment.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub status: Option<PagesDeploymentStatusStatus>,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct PagesHealthCheckAltDomain {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub caa_error: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub dns_resolves: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub enforces_https: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub has_cname_record: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub has_mx_records_present: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub host: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub https_error: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub is_a_record: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub is_apex_domain: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub is_cloudflare_ip: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub is_cname_to_fastly: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub is_cname_to_github_user_domain: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub is_cname_to_pages_dot_github_dot_com: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub is_fastly_ip: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub is_https_eligible: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub is_non_github_pages_ip_present: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub is_old_ip_address: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub is_pages_domain: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub is_pointed_to_github_pages_ip: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub is_proxied: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub is_served_by_pages: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub is_valid: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub is_valid_domain: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub nameservers: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub reason: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub responds_to_https: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub should_be_a_record: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub uri: Option<String>,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct PagesHealthCheckDomain {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub caa_error: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub dns_resolves: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub enforces_https: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub has_cname_record: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub has_mx_records_present: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub host: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub https_error: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub is_a_record: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub is_apex_domain: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub is_cloudflare_ip: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub is_cname_to_fastly: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub is_cname_to_github_user_domain: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub is_cname_to_pages_dot_github_dot_com: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub is_fastly_ip: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub is_https_eligible: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub is_non_github_pages_ip_present: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub is_old_ip_address: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub is_pages_domain: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub is_pointed_to_github_pages_ip: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub is_proxied: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub is_served_by_pages: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub is_valid: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub is_valid_domain: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub nameservers: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub reason: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub responds_to_https: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub should_be_a_record: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub uri: Option<String>,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// Pages Health Check Status
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct PagesHealthCheck {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub alt_domain: Option<PagesHealthCheckAltDomain>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub domain: Option<PagesHealthCheckDomain>,
}

#[cfg(any(feature = "full", feature = "pulls"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct PullRequestLinks {
  pub comments: Link,
  pub commits: Link,
  pub html: Link,
  pub issue: Link,
  pub review_comment: Link,
  pub review_comments: Link,
  #[serde(rename = "self")]
  pub self_: Link,
  pub statuses: Link,
}

#[cfg(any(feature = "full", feature = "pulls"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct PullRequestBaseRepoOwner {
  pub avatar_url: String,
  pub events_url: String,
  pub followers_url: String,
  pub following_url: String,
  pub gists_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub gravatar_id: Option<String>,
  pub html_url: String,
  pub id: i64,
  pub login: String,
  pub node_id: String,
  pub organizations_url: String,
  pub received_events_url: String,
  pub repos_url: String,
  pub site_admin: bool,
  pub starred_url: String,
  pub subscriptions_url: String,
  #[serde(rename = "type")]
  pub type_: String,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "pulls"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct PullRequestBaseRepoPermissions {
  pub admin: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub maintain: Option<bool>,
  pub pull: bool,
  pub push: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub triage: Option<bool>,
}

#[cfg(any(feature = "full", feature = "pulls"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct PullRequestBaseRepo {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub allow_forking: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub allow_merge_commit: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub allow_rebase_merge: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub allow_squash_merge: Option<bool>,
  pub archive_url: String,
  pub archived: bool,
  pub assignees_url: String,
  pub blobs_url: String,
  pub branches_url: String,
  pub clone_url: String,
  pub collaborators_url: String,
  pub comments_url: String,
  pub commits_url: String,
  pub compare_url: String,
  pub contents_url: String,
  pub contributors_url: String,
  pub created_at: String,
  pub default_branch: String,
  pub deployments_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub description: Option<String>,
  pub disabled: bool,
  pub downloads_url: String,
  pub events_url: String,
  pub fork: bool,
  pub forks: i64,
  pub forks_count: i64,
  pub forks_url: String,
  pub full_name: String,
  pub git_commits_url: String,
  pub git_refs_url: String,
  pub git_tags_url: String,
  pub git_url: String,
  pub has_discussions: bool,
  pub has_downloads: bool,
  pub has_issues: bool,
  pub has_pages: bool,
  pub has_projects: bool,
  pub has_wiki: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub homepage: Option<String>,
  pub hooks_url: String,
  pub html_url: String,
  pub id: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub is_template: Option<bool>,
  pub issue_comment_url: String,
  pub issue_events_url: String,
  pub issues_url: String,
  pub keys_url: String,
  pub labels_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub language: Option<String>,
  pub languages_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub license: Option<LicenseSimple>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub master_branch: Option<String>,
  pub merges_url: String,
  pub milestones_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub mirror_url: Option<String>,
  pub name: String,
  pub node_id: String,
  pub notifications_url: String,
  pub open_issues: i64,
  pub open_issues_count: i64,
  pub owner: PullRequestBaseRepoOwner,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub permissions: Option<PullRequestBaseRepoPermissions>,
  pub private: bool,
  pub pulls_url: String,
  pub pushed_at: String,
  pub releases_url: String,
  pub size: i64,
  pub ssh_url: String,
  pub stargazers_count: i64,
  pub stargazers_url: String,
  pub statuses_url: String,
  pub subscribers_url: String,
  pub subscription_url: String,
  pub svn_url: String,
  pub tags_url: String,
  pub teams_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub temp_clone_token: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub topics: Option<Vec<String>>,
  pub trees_url: String,
  pub updated_at: String,
  pub url: String,
  /// The repository visibility: public, private, or internal.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub visibility: Option<String>,
  pub watchers: i64,
  pub watchers_count: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub web_commit_signoff_required: Option<bool>,
}

#[cfg(any(feature = "full", feature = "pulls"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct PullRequestBaseUser {
  pub avatar_url: String,
  pub events_url: String,
  pub followers_url: String,
  pub following_url: String,
  pub gists_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub gravatar_id: Option<String>,
  pub html_url: String,
  pub id: i64,
  pub login: String,
  pub node_id: String,
  pub organizations_url: String,
  pub received_events_url: String,
  pub repos_url: String,
  pub site_admin: bool,
  pub starred_url: String,
  pub subscriptions_url: String,
  #[serde(rename = "type")]
  pub type_: String,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "pulls"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct PullRequestBase {
  pub label: String,
  #[serde(rename = "ref")]
  pub ref_: String,
  pub repo: PullRequestBaseRepo,
  pub sha: String,
  pub user: PullRequestBaseUser,
}

#[cfg(any(feature = "full", feature = "pulls"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct PullRequestHeadRepoLicense {
  pub key: String,
  pub name: String,
  pub node_id: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub spdx_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub url: Option<String>,
}

#[cfg(any(feature = "full", feature = "pulls"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct PullRequestHeadRepoOwner {
  pub avatar_url: String,
  pub events_url: String,
  pub followers_url: String,
  pub following_url: String,
  pub gists_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub gravatar_id: Option<String>,
  pub html_url: String,
  pub id: i64,
  pub login: String,
  pub node_id: String,
  pub organizations_url: String,
  pub received_events_url: String,
  pub repos_url: String,
  pub site_admin: bool,
  pub starred_url: String,
  pub subscriptions_url: String,
  #[serde(rename = "type")]
  pub type_: String,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "pulls"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct PullRequestHeadRepoPermissions {
  pub admin: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub maintain: Option<bool>,
  pub pull: bool,
  pub push: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub triage: Option<bool>,
}

#[cfg(any(feature = "full", feature = "pulls"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct PullRequestHeadRepo {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub allow_forking: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub allow_merge_commit: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub allow_rebase_merge: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub allow_squash_merge: Option<bool>,
  pub archive_url: String,
  pub archived: bool,
  pub assignees_url: String,
  pub blobs_url: String,
  pub branches_url: String,
  pub clone_url: String,
  pub collaborators_url: String,
  pub comments_url: String,
  pub commits_url: String,
  pub compare_url: String,
  pub contents_url: String,
  pub contributors_url: String,
  pub created_at: String,
  pub default_branch: String,
  pub deployments_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub description: Option<String>,
  pub disabled: bool,
  pub downloads_url: String,
  pub events_url: String,
  pub fork: bool,
  pub forks: i64,
  pub forks_count: i64,
  pub forks_url: String,
  pub full_name: String,
  pub git_commits_url: String,
  pub git_refs_url: String,
  pub git_tags_url: String,
  pub git_url: String,
  pub has_discussions: bool,
  pub has_downloads: bool,
  pub has_issues: bool,
  pub has_pages: bool,
  pub has_projects: bool,
  pub has_wiki: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub homepage: Option<String>,
  pub hooks_url: String,
  pub html_url: String,
  pub id: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub is_template: Option<bool>,
  pub issue_comment_url: String,
  pub issue_events_url: String,
  pub issues_url: String,
  pub keys_url: String,
  pub labels_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub language: Option<String>,
  pub languages_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub license: Option<PullRequestHeadRepoLicense>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub master_branch: Option<String>,
  pub merges_url: String,
  pub milestones_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub mirror_url: Option<String>,
  pub name: String,
  pub node_id: String,
  pub notifications_url: String,
  pub open_issues: i64,
  pub open_issues_count: i64,
  pub owner: PullRequestHeadRepoOwner,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub permissions: Option<PullRequestHeadRepoPermissions>,
  pub private: bool,
  pub pulls_url: String,
  pub pushed_at: String,
  pub releases_url: String,
  pub size: i64,
  pub ssh_url: String,
  pub stargazers_count: i64,
  pub stargazers_url: String,
  pub statuses_url: String,
  pub subscribers_url: String,
  pub subscription_url: String,
  pub svn_url: String,
  pub tags_url: String,
  pub teams_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub temp_clone_token: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub topics: Option<Vec<String>>,
  pub trees_url: String,
  pub updated_at: String,
  pub url: String,
  /// The repository visibility: public, private, or internal.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub visibility: Option<String>,
  pub watchers: i64,
  pub watchers_count: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub web_commit_signoff_required: Option<bool>,
}

#[cfg(any(feature = "full", feature = "pulls"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct PullRequestHeadUser {
  pub avatar_url: String,
  pub events_url: String,
  pub followers_url: String,
  pub following_url: String,
  pub gists_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub gravatar_id: Option<String>,
  pub html_url: String,
  pub id: i64,
  pub login: String,
  pub node_id: String,
  pub organizations_url: String,
  pub received_events_url: String,
  pub repos_url: String,
  pub site_admin: bool,
  pub starred_url: String,
  pub subscriptions_url: String,
  #[serde(rename = "type")]
  pub type_: String,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "pulls"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct PullRequestHead {
  pub label: String,
  #[serde(rename = "ref")]
  pub ref_: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub repo: Option<PullRequestHeadRepo>,
  pub sha: String,
  pub user: PullRequestHeadUser,
}

#[cfg(any(feature = "full", feature = "pulls"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct PullRequestLabels {
  pub color: String,
  #[serde(rename = "default")]
  pub default_: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub description: Option<String>,
  pub id: i64,
  pub name: String,
  pub node_id: String,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "pulls"))]
/// Pull requests let you tell others about changes you've pushed to a repository on GitHub. Once a pull request is sent, interested parties can review the set of changes, discuss potential modifications, and even push follow-up commits if necessary.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct PullRequest {
  #[serde(rename = "_links")]
  pub links: PullRequestLinks,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub active_lock_reason: Option<String>,
  pub additions: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub assignee: Option<SimpleUser>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub assignees: Option<Vec<SimpleUser>>,
  pub author_association: AuthorAssociation,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub auto_merge: Option<AutoMerge>,
  pub base: PullRequestBase,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub body: Option<String>,
  pub changed_files: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub closed_at: Option<String>,
  pub comments: i64,
  pub comments_url: String,
  pub commits: i64,
  pub commits_url: String,
  pub created_at: String,
  pub deletions: i64,
  pub diff_url: String,
  /// Indicates whether or not the pull request is a draft.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub draft: Option<bool>,
  pub head: PullRequestHead,
  pub html_url: String,
  pub id: i64,
  pub issue_url: String,
  pub labels: Vec<PullRequestLabels>,
  pub locked: bool,
  /// Indicates whether maintainers can modify the pull request.
  pub maintainer_can_modify: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub merge_commit_sha: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub mergeable: Option<bool>,
  pub mergeable_state: String,
  pub merged: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub merged_at: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub merged_by: Option<SimpleUser>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub milestone: Option<Milestone>,
  pub node_id: String,
  /// Number uniquely identifying the pull request within its repository.
  pub number: i64,
  pub patch_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub rebaseable: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub requested_reviewers: Option<Vec<SimpleUser>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub requested_teams: Option<Vec<TeamSimple>>,
  pub review_comment_url: String,
  pub review_comments: i64,
  pub review_comments_url: String,
  /// State of this Pull Request. Either `open` or `closed`.
  pub state: PullRequestState,
  pub statuses_url: String,
  /// The title of the pull request.
  pub title: String,
  pub updated_at: String,
  pub url: String,
  pub user: SimpleUser,
}

#[cfg(any(feature = "full", feature = "pulls"))]
/// Pull Request Merge Result
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct PullRequestMergeResult {
  pub merged: bool,
  pub message: String,
  pub sha: String,
}

#[cfg(any(feature = "full", feature = "pulls"))]
/// Pull Request Review Request
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct PullRequestReviewRequest {
  pub teams: Vec<Team>,
  pub users: Vec<SimpleUser>,
}

#[cfg(any(feature = "full", feature = "pulls"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct PullRequestReviewLinksHtml {
  pub href: String,
}

#[cfg(any(feature = "full", feature = "pulls"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct PullRequestReviewLinksPullRequest {
  pub href: String,
}

#[cfg(any(feature = "full", feature = "pulls"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct PullRequestReviewLinks {
  pub html: PullRequestReviewLinksHtml,
  pub pull_request: PullRequestReviewLinksPullRequest,
}

#[cfg(any(feature = "full", feature = "pulls"))]
/// Pull Request Reviews are reviews on pull requests.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct PullRequestReview {
  #[serde(rename = "_links")]
  pub links: PullRequestReviewLinks,
  pub author_association: AuthorAssociation,
  /// The text of the review.
  pub body: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub body_html: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub body_text: Option<String>,
  /// A commit SHA for the review. If the commit object was garbage collected or forcibly deleted, then it no longer exists in Git and this value will be `null`.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub commit_id: Option<String>,
  pub html_url: String,
  /// Unique identifier of the review
  pub id: i64,
  pub node_id: String,
  pub pull_request_url: String,
  pub state: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub submitted_at: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub user: Option<SimpleUser>,
}

#[cfg(any(feature = "full", feature = "pulls"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ReviewCommentLinks {
  pub html: Link,
  pub pull_request: Link,
  #[serde(rename = "self")]
  pub self_: Link,
}

#[cfg(any(feature = "full", feature = "pulls"))]
/// Legacy Review Comment
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ReviewComment {
  #[serde(rename = "_links")]
  pub links: ReviewCommentLinks,
  pub author_association: AuthorAssociation,
  pub body: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub body_html: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub body_text: Option<String>,
  pub commit_id: String,
  pub created_at: String,
  pub diff_hunk: String,
  pub html_url: String,
  pub id: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub in_reply_to_id: Option<i64>,
  /// The line of the blob to which the comment applies. The last line of the range for a multi-line comment
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub line: Option<i64>,
  pub node_id: String,
  pub original_commit_id: String,
  /// The original line of the blob to which the comment applies. The last line of the range for a multi-line comment
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub original_line: Option<i64>,
  pub original_position: i64,
  /// The original first line of the range for a multi-line comment.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub original_start_line: Option<i64>,
  pub path: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub position: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub pull_request_review_id: Option<i64>,
  pub pull_request_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub reactions: Option<ReactionRollup>,
  /// The side of the first line of the range for a multi-line comment.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub side: Option<ReviewCommentSide>,
  /// The first line of the range for a multi-line comment.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub start_line: Option<i64>,
  /// The side of the first line of the range for a multi-line comment.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub start_side: Option<ReviewCommentStartSide>,
  pub updated_at: String,
  pub url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub user: Option<SimpleUser>,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// Data related to a release.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ReleaseAsset {
  pub browser_download_url: String,
  pub content_type: String,
  pub created_at: String,
  pub download_count: i64,
  pub id: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub label: Option<String>,
  /// The file name of the asset.
  pub name: String,
  pub node_id: String,
  pub size: i64,
  /// State of the release asset.
  pub state: ReleaseAssetState,
  pub updated_at: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub uploader: Option<SimpleUser>,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// A release.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct Release {
  pub assets: Vec<ReleaseAsset>,
  pub assets_url: String,
  pub author: SimpleUser,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub body: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub body_html: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub body_text: Option<String>,
  pub created_at: String,
  /// The URL of the release discussion.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub discussion_url: Option<String>,
  /// true to create a draft (unpublished) release, false to create a published one.
  pub draft: bool,
  pub html_url: String,
  pub id: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub mentions_count: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub name: Option<String>,
  pub node_id: String,
  /// Whether to identify the release as a prerelease or a full release.
  pub prerelease: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub published_at: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub reactions: Option<ReactionRollup>,
  /// The name of the tag.
  pub tag_name: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub tarball_url: Option<String>,
  /// Specifies the commitish value that determines where the Git tag is created from.
  pub target_commitish: String,
  pub upload_url: String,
  pub url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub zipball_url: Option<String>,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// Generated name and body describing a release
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ReleaseNotesContent {
  /// The generated body describing the contents of the release supporting markdown formatting
  pub body: String,
  /// The generated name of the release
  pub name: String,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// User-defined metadata to store domain-specific information limited to 8 keys with scalar values.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RepositoryRuleRulesetInfo {
  /// The ID of the ruleset that includes this rule.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub ruleset_id: Option<i64>,
  /// The name of the source of the ruleset that includes this rule.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub ruleset_source: Option<String>,
  /// The type of source for the ruleset that includes this rule.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub ruleset_source_type: Option<RepositoryRuleRulesetInfoRulesetSourceType>,
}

#[cfg(any(feature = "full", feature = "secret_scanning"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct SecretScanningAlert {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub created_at: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub html_url: Option<String>,
  /// The REST API URL of the code locations for this alert.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub locations_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub number: Option<i64>,
  /// Whether push protection was bypassed for the detected secret.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub push_protection_bypassed: Option<bool>,
  /// The time that push protection was bypassed in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub push_protection_bypassed_at: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub push_protection_bypassed_by: Option<SimpleUser>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub resolution: Option<SecretScanningAlertResolution>,
  /// An optional comment to resolve an alert.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub resolution_comment: Option<String>,
  /// The time that the alert was resolved in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub resolved_at: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub resolved_by: Option<SimpleUser>,
  /// The secret that was detected.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub secret: Option<String>,
  /// The type of secret that secret scanning detected.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub secret_type: Option<String>,
  /// User-friendly name for the detected secret, matching the `secret_type`.
  /// For a list of built-in patterns, see "[Secret scanning patterns](https://docs.github.com/code-security/secret-scanning/secret-scanning-patterns#supported-secrets-for-advanced-security)."
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub secret_type_display_name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub state: Option<SecretScanningAlertState>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub updated_at: Option<serde_json::Value>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub url: Option<String>,
  /// The token status as of the latest validity check.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub validity: Option<SecretScanningAlertValidity>,
}

#[cfg(any(feature = "full", feature = "secret_scanning"))]
/// Represents a 'commit' secret scanning location type. This location type shows that a secret was detected inside a commit to a repository.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct SecretScanningLocationCommit {
  /// SHA-1 hash ID of the associated blob
  pub blob_sha: String,
  /// The API URL to get the associated blob resource
  pub blob_url: String,
  /// SHA-1 hash ID of the associated commit
  pub commit_sha: String,
  /// The API URL to get the associated commit resource
  pub commit_url: String,
  /// The column at which the secret ends within the end line when the file is interpreted as 8BIT ASCII
  pub end_column: f64,
  /// Line number at which the secret ends in the file
  pub end_line: f64,
  /// The file path in the repository
  pub path: String,
  /// The column at which the secret starts within the start line when the file is interpreted as 8BIT ASCII
  pub start_column: f64,
  /// Line number at which the secret starts in the file
  pub start_line: f64,
}

#[cfg(any(feature = "full", feature = "secret_scanning"))]
/// Represents a 'wiki_commit' secret scanning location type. This location type shows that a secret was detected inside a commit to a repository wiki.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct SecretScanningLocationWikiCommit {
  /// SHA-1 hash ID of the associated blob
  pub blob_sha: String,
  /// SHA-1 hash ID of the associated commit
  pub commit_sha: String,
  /// The GitHub URL to get the associated wiki commit
  pub commit_url: String,
  /// The column at which the secret ends within the end line when the file is interpreted as 8-bit ASCII.
  pub end_column: f64,
  /// Line number at which the secret ends in the file
  pub end_line: f64,
  /// The GitHub URL to get the associated wiki page
  pub page_url: String,
  /// The file path of the wiki page
  pub path: String,
  /// The column at which the secret starts within the start line when the file is interpreted as 8-bit ASCII.
  pub start_column: f64,
  /// Line number at which the secret starts in the file
  pub start_line: f64,
}

#[cfg(any(feature = "full", feature = "secret_scanning"))]
/// Represents an 'issue_title' secret scanning location type. This location type shows that a secret was detected in the title of an issue.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct SecretScanningLocationIssueTitle {
  /// The API URL to get the issue where the secret was detected.
  pub issue_title_url: String,
}

#[cfg(any(feature = "full", feature = "secret_scanning"))]
/// Represents an 'issue_body' secret scanning location type. This location type shows that a secret was detected in the body of an issue.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct SecretScanningLocationIssueBody {
  /// The API URL to get the issue where the secret was detected.
  pub issue_body_url: String,
}

#[cfg(any(feature = "full", feature = "secret_scanning"))]
/// Represents an 'issue_comment' secret scanning location type. This location type shows that a secret was detected in a comment on an issue.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct SecretScanningLocationIssueComment {
  /// The API URL to get the issue comment where the secret was detected.
  pub issue_comment_url: String,
}

#[cfg(any(feature = "full", feature = "secret_scanning"))]
/// Represents a 'discussion_title' secret scanning location type. This location type shows that a secret was detected in the title of a discussion.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct SecretScanningLocationDiscussionTitle {
  /// The URL to the discussion where the secret was detected.
  pub discussion_title_url: String,
}

#[cfg(any(feature = "full", feature = "secret_scanning"))]
/// Represents a 'discussion_body' secret scanning location type. This location type shows that a secret was detected in the body of a discussion.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct SecretScanningLocationDiscussionBody {
  /// The URL to the discussion where the secret was detected.
  pub discussion_body_url: String,
}

#[cfg(any(feature = "full", feature = "secret_scanning"))]
/// Represents a 'discussion_comment' secret scanning location type. This location type shows that a secret was detected in a comment on a discussion.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct SecretScanningLocationDiscussionComment {
  /// The API URL to get the discussion comment where the secret was detected.
  pub discussion_comment_url: String,
}

#[cfg(any(feature = "full", feature = "secret_scanning"))]
/// Represents a 'pull_request_title' secret scanning location type. This location type shows that a secret was detected in the title of a pull request.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct SecretScanningLocationPullRequestTitle {
  /// The API URL to get the pull request where the secret was detected.
  pub pull_request_title_url: String,
}

#[cfg(any(feature = "full", feature = "secret_scanning"))]
/// Represents a 'pull_request_body' secret scanning location type. This location type shows that a secret was detected in the body of a pull request.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct SecretScanningLocationPullRequestBody {
  /// The API URL to get the pull request where the secret was detected.
  pub pull_request_body_url: String,
}

#[cfg(any(feature = "full", feature = "secret_scanning"))]
/// Represents a 'pull_request_comment' secret scanning location type. This location type shows that a secret was detected in a comment on a pull request.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct SecretScanningLocationPullRequestComment {
  /// The API URL to get the pull request comment where the secret was detected.
  pub pull_request_comment_url: String,
}

#[cfg(any(feature = "full", feature = "secret_scanning"))]
/// Represents a 'pull_request_review' secret scanning location type. This location type shows that a secret was detected in a review on a pull request.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct SecretScanningLocationPullRequestReview {
  /// The API URL to get the pull request review where the secret was detected.
  pub pull_request_review_url: String,
}

#[cfg(any(feature = "full", feature = "secret_scanning"))]
/// Represents a 'pull_request_review_comment' secret scanning location type. This location type shows that a secret was detected in a review comment on a pull request.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct SecretScanningLocationPullRequestReviewComment {
  /// The API URL to get the pull request review comment where the secret was detected.
  pub pull_request_review_comment_url: String,
}

#[cfg(any(feature = "full", feature = "secret_scanning"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct SecretScanningLocation {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub details: Option<SecretScanningLocationDetails>,
  /// The location type. Because secrets may be found in different types of resources (ie. code, comments, issues, pull requests, discussions), this field identifies the type of resource where the secret was found.
  #[serde(rename = "type")]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub type_: Option<SecretScanningLocationType>,
}

#[cfg(any(feature = "full", feature = "security_advisories"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RepositoryAdvisoryCreateCredits {
  /// The username of the user credited.
  pub login: String,
  #[serde(rename = "type")]
  pub type_: SecurityAdvisoryCreditTypes,
}

#[cfg(any(feature = "full", feature = "security_advisories"))]
/// The name of the package affected by the vulnerability.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RepositoryAdvisoryCreateVulnerabilitiesPackage {
  pub ecosystem: SecurityAdvisoryEcosystems,
  /// The unique package name within its ecosystem.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub name: Option<String>,
}

#[cfg(any(feature = "full", feature = "security_advisories"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RepositoryAdvisoryCreateVulnerabilities {
  /// The name of the package affected by the vulnerability.
  pub package: RepositoryAdvisoryCreateVulnerabilitiesPackage,
  /// The package version(s) that resolve the vulnerability.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub patched_versions: Option<String>,
  /// The functions in the package that are affected.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub vulnerable_functions: Option<Vec<String>>,
  /// The range of the package versions affected by the vulnerability.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub vulnerable_version_range: Option<String>,
}

#[cfg(any(feature = "full", feature = "security_advisories"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RepositoryAdvisoryCreate {
  /// A list of users receiving credit for their participation in the security advisory.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub credits: Option<Vec<RepositoryAdvisoryCreateCredits>>,
  /// The Common Vulnerabilities and Exposures (CVE) ID.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub cve_id: Option<String>,
  /// The CVSS vector that calculates the severity of the advisory. You must choose between setting this field or `severity`.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub cvss_vector_string: Option<String>,
  /// A list of Common Weakness Enumeration (CWE) IDs.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub cwe_ids: Option<Vec<String>>,
  /// A detailed description of what the advisory impacts.
  pub description: String,
  /// The severity of the advisory. You must choose between setting this field or `cvss_vector_string`.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub severity: Option<RepositoryAdvisoryCreateSeverity>,
  /// Whether to create a temporary private fork of the repository to collaborate on a fix.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub start_private_fork: Option<bool>,
  /// A short summary of the advisory.
  pub summary: String,
  /// A product affected by the vulnerability detailed in a repository security advisory.
  pub vulnerabilities: Vec<RepositoryAdvisoryCreateVulnerabilities>,
}

#[cfg(any(feature = "full", feature = "security_advisories"))]
/// The name of the package affected by the vulnerability.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct PrivateVulnerabilityReportCreateVulnerabilitiesPackage {
  pub ecosystem: SecurityAdvisoryEcosystems,
  /// The unique package name within its ecosystem.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub name: Option<String>,
}

#[cfg(any(feature = "full", feature = "security_advisories"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct PrivateVulnerabilityReportCreateVulnerabilities {
  /// The name of the package affected by the vulnerability.
  pub package: PrivateVulnerabilityReportCreateVulnerabilitiesPackage,
  /// The package version(s) that resolve the vulnerability.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub patched_versions: Option<String>,
  /// The functions in the package that are affected.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub vulnerable_functions: Option<Vec<String>>,
  /// The range of the package versions affected by the vulnerability.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub vulnerable_version_range: Option<String>,
}

#[cfg(any(feature = "full", feature = "security_advisories"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct PrivateVulnerabilityReportCreate {
  /// The CVSS vector that calculates the severity of the advisory. You must choose between setting this field or `severity`.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub cvss_vector_string: Option<String>,
  /// A list of Common Weakness Enumeration (CWE) IDs.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub cwe_ids: Option<Vec<String>>,
  /// A detailed description of what the advisory impacts.
  pub description: String,
  /// The severity of the advisory. You must choose between setting this field or `cvss_vector_string`.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub severity: Option<PrivateVulnerabilityReportCreateSeverity>,
  /// Whether to create a temporary private fork of the repository to collaborate on a fix.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub start_private_fork: Option<bool>,
  /// A short summary of the advisory.
  pub summary: String,
  /// An array of products affected by the vulnerability detailed in a repository security advisory.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub vulnerabilities: Option<Vec<PrivateVulnerabilityReportCreateVulnerabilities>>,
}

#[cfg(any(feature = "full", feature = "security_advisories"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RepositoryAdvisoryUpdateCredits {
  /// The username of the user credited.
  pub login: String,
  #[serde(rename = "type")]
  pub type_: SecurityAdvisoryCreditTypes,
}

#[cfg(any(feature = "full", feature = "security_advisories"))]
/// The name of the package affected by the vulnerability.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RepositoryAdvisoryUpdateVulnerabilitiesPackage {
  pub ecosystem: SecurityAdvisoryEcosystems,
  /// The unique package name within its ecosystem.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub name: Option<String>,
}

#[cfg(any(feature = "full", feature = "security_advisories"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RepositoryAdvisoryUpdateVulnerabilities {
  /// The name of the package affected by the vulnerability.
  pub package: RepositoryAdvisoryUpdateVulnerabilitiesPackage,
  /// The package version(s) that resolve the vulnerability.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub patched_versions: Option<String>,
  /// The functions in the package that are affected.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub vulnerable_functions: Option<Vec<String>>,
  /// The range of the package versions affected by the vulnerability.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub vulnerable_version_range: Option<String>,
}

#[cfg(any(feature = "full", feature = "security_advisories"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RepositoryAdvisoryUpdate {
  /// A list of team slugs which have been granted write access to the advisory.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub collaborating_teams: Option<Vec<String>>,
  /// A list of usernames who have been granted write access to the advisory.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub collaborating_users: Option<Vec<String>>,
  /// A list of users receiving credit for their participation in the security advisory.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub credits: Option<Vec<RepositoryAdvisoryUpdateCredits>>,
  /// The Common Vulnerabilities and Exposures (CVE) ID.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub cve_id: Option<String>,
  /// The CVSS vector that calculates the severity of the advisory. You must choose between setting this field or `severity`.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub cvss_vector_string: Option<String>,
  /// A list of Common Weakness Enumeration (CWE) IDs.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub cwe_ids: Option<Vec<String>>,
  /// A detailed description of what the advisory impacts.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub description: Option<String>,
  /// The severity of the advisory. You must choose between setting this field or `cvss_vector_string`.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub severity: Option<RepositoryAdvisoryUpdateSeverity>,
  /// The state of the advisory.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub state: Option<RepositoryAdvisoryUpdateState>,
  /// A short summary of the advisory.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub summary: Option<String>,
  /// A product affected by the vulnerability detailed in a repository security advisory.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub vulnerabilities: Option<Vec<RepositoryAdvisoryUpdateVulnerabilities>>,
}

#[cfg(any(feature = "full", feature = "activity"))]
/// Stargazer
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct Stargazer {
  pub starred_at: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub user: Option<SimpleUser>,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// Commit Activity
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CommitActivity {
  pub days: Vec<i64>,
  pub total: i64,
  pub week: i64,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ContributorActivityWeeks {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub a: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub c: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub d: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub w: Option<i64>,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// Contributor Activity
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ContributorActivity {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub author: Option<SimpleUser>,
  pub total: i64,
  pub weeks: Vec<ContributorActivityWeeks>,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ParticipationStats {
  pub all: Vec<i64>,
  pub owner: Vec<i64>,
}

#[cfg(any(feature = "full", feature = "activity"))]
/// Repository invitations let you manage who you collaborate with.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RepositorySubscription {
  pub created_at: String,
  /// Determines if all notifications should be blocked from this repository.
  pub ignored: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub reason: Option<String>,
  pub repository_url: String,
  /// Determines if notifications should be received from this repository.
  pub subscribed: bool,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct TagCommit {
  pub sha: String,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// Tag
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct Tag {
  pub commit: TagCommit,
  pub name: String,
  pub node_id: String,
  pub tarball_url: String,
  pub zipball_url: String,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// Tag protection
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct TagProtection {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub created_at: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub enabled: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub id: Option<i64>,
  pub pattern: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub updated_at: Option<String>,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// A topic aggregates entities that are related to a subject.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct Topic {
  pub names: Vec<String>,
}

#[cfg(any(feature = "full", feature = "repos"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct Traffic {
  pub count: i64,
  pub timestamp: String,
  pub uniques: i64,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// Clone Traffic
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CloneTraffic {
  pub clones: Vec<Traffic>,
  pub count: i64,
  pub uniques: i64,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// Content Traffic
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ContentTraffic {
  pub count: i64,
  pub path: String,
  pub title: String,
  pub uniques: i64,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// Referrer Traffic
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ReferrerTraffic {
  pub count: i64,
  pub referrer: String,
  pub uniques: i64,
}

#[cfg(any(feature = "full", feature = "repos"))]
/// View Traffic
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ViewTraffic {
  pub count: i64,
  pub uniques: i64,
  pub views: Vec<Traffic>,
}

#[cfg(any(feature = "full", feature = "search"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct SearchResultTextMatchesItemMatches {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub indices: Option<Vec<i64>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub text: Option<String>,
}

#[cfg(any(feature = "full", feature = "search"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct SearchResultTextMatchesItem {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub fragment: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub matches: Option<Vec<SearchResultTextMatchesItemMatches>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub object_type: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub object_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub property: Option<String>,
}

#[cfg(any(feature = "full", feature = "search"))]
/// Code Search Result Item
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CodeSearchResultItem {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub file_size: Option<i64>,
  pub git_url: String,
  pub html_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub language: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub last_modified_at: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub line_numbers: Option<Vec<String>>,
  pub name: String,
  pub path: String,
  pub repository: MinimalRepository,
  pub score: f64,
  pub sha: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub text_matches: Option<Vec<SearchResultTextMatchesItem>>,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "search"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CommitSearchResultItemCommitAuthor {
  pub date: String,
  pub email: String,
  pub name: String,
}

#[cfg(any(feature = "full", feature = "search"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CommitSearchResultItemCommitTree {
  pub sha: String,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "search"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CommitSearchResultItemCommit {
  pub author: CommitSearchResultItemCommitAuthor,
  pub comment_count: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub committer: Option<GitUser>,
  pub message: String,
  pub tree: CommitSearchResultItemCommitTree,
  pub url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub verification: Option<Verification>,
}

#[cfg(any(feature = "full", feature = "search"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CommitSearchResultItemParents {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub html_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub sha: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub url: Option<String>,
}

#[cfg(any(feature = "full", feature = "search"))]
/// Commit Search Result Item
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CommitSearchResultItem {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub author: Option<SimpleUser>,
  pub comments_url: String,
  pub commit: CommitSearchResultItemCommit,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub committer: Option<GitUser>,
  pub html_url: String,
  pub node_id: String,
  pub parents: Vec<CommitSearchResultItemParents>,
  pub repository: MinimalRepository,
  pub score: f64,
  pub sha: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub text_matches: Option<Vec<SearchResultTextMatchesItem>>,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "search"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct IssueSearchResultItemLabels {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub color: Option<String>,
  #[serde(rename = "default")]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub default_: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub description: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub id: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub node_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub url: Option<String>,
}

#[cfg(any(feature = "full", feature = "search"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct IssueSearchResultItemPullRequest {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub diff_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub html_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub merged_at: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub patch_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub url: Option<String>,
}

#[cfg(any(feature = "full", feature = "search"))]
/// Issue Search Result Item
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct IssueSearchResultItem {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub active_lock_reason: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub assignee: Option<SimpleUser>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub assignees: Option<Vec<SimpleUser>>,
  pub author_association: AuthorAssociation,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub body: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub body_html: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub body_text: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub closed_at: Option<String>,
  pub comments: i64,
  pub comments_url: String,
  pub created_at: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub draft: Option<bool>,
  pub events_url: String,
  pub html_url: String,
  pub id: i64,
  pub labels: Vec<IssueSearchResultItemLabels>,
  pub labels_url: String,
  pub locked: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub milestone: Option<Milestone>,
  pub node_id: String,
  pub number: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub performed_via_github_app: Option<Integration>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub pull_request: Option<IssueSearchResultItemPullRequest>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub reactions: Option<ReactionRollup>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub repository: Option<Repository>,
  pub repository_url: String,
  pub score: f64,
  pub state: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub state_reason: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub text_matches: Option<Vec<SearchResultTextMatchesItem>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub timeline_url: Option<String>,
  pub title: String,
  pub updated_at: String,
  pub url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub user: Option<SimpleUser>,
}

#[cfg(any(feature = "full", feature = "search"))]
/// Label Search Result Item
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct LabelSearchResultItem {
  pub color: String,
  #[serde(rename = "default")]
  pub default_: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub description: Option<String>,
  pub id: i64,
  pub name: String,
  pub node_id: String,
  pub score: f64,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub text_matches: Option<Vec<SearchResultTextMatchesItem>>,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "search"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RepoSearchResultItemPermissions {
  pub admin: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub maintain: Option<bool>,
  pub pull: bool,
  pub push: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub triage: Option<bool>,
}

#[cfg(any(feature = "full", feature = "search"))]
/// Repo Search Result Item
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct RepoSearchResultItem {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub allow_auto_merge: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub allow_forking: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub allow_merge_commit: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub allow_rebase_merge: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub allow_squash_merge: Option<bool>,
  pub archive_url: String,
  pub archived: bool,
  pub assignees_url: String,
  pub blobs_url: String,
  pub branches_url: String,
  pub clone_url: String,
  pub collaborators_url: String,
  pub comments_url: String,
  pub commits_url: String,
  pub compare_url: String,
  pub contents_url: String,
  pub contributors_url: String,
  pub created_at: String,
  pub default_branch: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub delete_branch_on_merge: Option<bool>,
  pub deployments_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub description: Option<String>,
  /// Returns whether or not this repository disabled.
  pub disabled: bool,
  pub downloads_url: String,
  pub events_url: String,
  pub fork: bool,
  pub forks: i64,
  pub forks_count: i64,
  pub forks_url: String,
  pub full_name: String,
  pub git_commits_url: String,
  pub git_refs_url: String,
  pub git_tags_url: String,
  pub git_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub has_discussions: Option<bool>,
  pub has_downloads: bool,
  pub has_issues: bool,
  pub has_pages: bool,
  pub has_projects: bool,
  pub has_wiki: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub homepage: Option<String>,
  pub hooks_url: String,
  pub html_url: String,
  pub id: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub is_template: Option<bool>,
  pub issue_comment_url: String,
  pub issue_events_url: String,
  pub issues_url: String,
  pub keys_url: String,
  pub labels_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub language: Option<String>,
  pub languages_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub license: Option<LicenseSimple>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub master_branch: Option<String>,
  pub merges_url: String,
  pub milestones_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub mirror_url: Option<String>,
  pub name: String,
  pub node_id: String,
  pub notifications_url: String,
  pub open_issues: i64,
  pub open_issues_count: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub owner: Option<SimpleUser>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub permissions: Option<RepoSearchResultItemPermissions>,
  pub private: bool,
  pub pulls_url: String,
  pub pushed_at: String,
  pub releases_url: String,
  pub score: f64,
  pub size: i64,
  pub ssh_url: String,
  pub stargazers_count: i64,
  pub stargazers_url: String,
  pub statuses_url: String,
  pub subscribers_url: String,
  pub subscription_url: String,
  pub svn_url: String,
  pub tags_url: String,
  pub teams_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub temp_clone_token: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub text_matches: Option<Vec<SearchResultTextMatchesItem>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub topics: Option<Vec<String>>,
  pub trees_url: String,
  pub updated_at: String,
  pub url: String,
  /// The repository visibility: public, private, or internal.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub visibility: Option<String>,
  pub watchers: i64,
  pub watchers_count: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub web_commit_signoff_required: Option<bool>,
}

#[cfg(any(feature = "full", feature = "search"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct TopicSearchResultItemAliasesTopicRelation {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub id: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub relation_type: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub topic_id: Option<i64>,
}

#[cfg(any(feature = "full", feature = "search"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct TopicSearchResultItemAliases {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub topic_relation: Option<TopicSearchResultItemAliasesTopicRelation>,
}

#[cfg(any(feature = "full", feature = "search"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct TopicSearchResultItemRelatedTopicRelation {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub id: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub relation_type: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub topic_id: Option<i64>,
}

#[cfg(any(feature = "full", feature = "search"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct TopicSearchResultItemRelated {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub topic_relation: Option<TopicSearchResultItemRelatedTopicRelation>,
}

#[cfg(any(feature = "full", feature = "search"))]
/// Topic Search Result Item
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct TopicSearchResultItem {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub aliases: Option<Vec<TopicSearchResultItemAliases>>,
  pub created_at: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub created_by: Option<String>,
  pub curated: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub description: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub display_name: Option<String>,
  pub featured: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub logo_url: Option<String>,
  pub name: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub related: Option<Vec<TopicSearchResultItemRelated>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub released: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub repository_count: Option<i64>,
  pub score: f64,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub short_description: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub text_matches: Option<Vec<SearchResultTextMatchesItem>>,
  pub updated_at: String,
}

#[cfg(any(feature = "full", feature = "search"))]
/// User Search Result Item
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct UserSearchResultItem {
  pub avatar_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub bio: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub blog: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub company: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub created_at: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub email: Option<String>,
  pub events_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub followers: Option<i64>,
  pub followers_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub following: Option<i64>,
  pub following_url: String,
  pub gists_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub gravatar_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub hireable: Option<bool>,
  pub html_url: String,
  pub id: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub location: Option<String>,
  pub login: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub name: Option<String>,
  pub node_id: String,
  pub organizations_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub public_gists: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub public_repos: Option<i64>,
  pub received_events_url: String,
  pub repos_url: String,
  pub score: f64,
  pub site_admin: bool,
  pub starred_url: String,
  pub subscriptions_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub suspended_at: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub text_matches: Option<Vec<SearchResultTextMatchesItem>>,
  #[serde(rename = "type")]
  pub type_: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub updated_at: Option<String>,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "users"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct PrivateUserPlan {
  pub collaborators: i64,
  pub name: String,
  pub private_repos: i64,
  pub space: i64,
}

#[cfg(any(feature = "full", feature = "users"))]
/// Private User
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct PrivateUser {
  pub avatar_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub bio: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub blog: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub business_plus: Option<bool>,
  pub collaborators: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub company: Option<String>,
  pub created_at: String,
  pub disk_usage: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub email: Option<String>,
  pub events_url: String,
  pub followers: i64,
  pub followers_url: String,
  pub following: i64,
  pub following_url: String,
  pub gists_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub gravatar_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub hireable: Option<bool>,
  pub html_url: String,
  pub id: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub ldap_dn: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub location: Option<String>,
  pub login: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub name: Option<String>,
  pub node_id: String,
  pub organizations_url: String,
  pub owned_private_repos: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub plan: Option<PrivateUserPlan>,
  pub private_gists: i64,
  pub public_gists: i64,
  pub public_repos: i64,
  pub received_events_url: String,
  pub repos_url: String,
  pub site_admin: bool,
  pub starred_url: String,
  pub subscriptions_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub suspended_at: Option<String>,
  pub total_private_repos: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub twitter_username: Option<String>,
  pub two_factor_authentication: bool,
  #[serde(rename = "type")]
  pub type_: String,
  pub updated_at: String,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "codespaces"))]
/// Secrets for a GitHub Codespace.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CodespacesSecret {
  /// The date and time at which the secret was created, in ISO 8601 format':' YYYY-MM-DDTHH:MM:SSZ.
  pub created_at: String,
  /// The name of the secret
  pub name: String,
  /// The API URL at which the list of repositories this secret is visible to can be retrieved
  pub selected_repositories_url: String,
  /// The date and time at which the secret was last updated, in ISO 8601 format':' YYYY-MM-DDTHH:MM:SSZ.
  pub updated_at: String,
  /// The type of repositories in the organization that the secret is visible to
  pub visibility: Visibility,
}

#[cfg(any(feature = "full", feature = "codespaces"))]
/// The public key used for setting user Codespaces' Secrets.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CodespacesUserPublicKey {
  /// The Base64 encoded public key.
  pub key: String,
  /// The identifier for the key.
  pub key_id: String,
}

#[cfg(any(feature = "full", feature = "codespaces"))]
/// An export of a codespace. Also, latest export details for a codespace can be fetched with id = latest
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CodespaceExportDetails {
  /// Name of the exported branch
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub branch: Option<String>,
  /// Completion time of the last export operation
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub completed_at: Option<String>,
  /// Url for fetching export details
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub export_url: Option<String>,
  /// Web url for the exported branch
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub html_url: Option<String>,
  /// Id for the export details
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub id: Option<String>,
  /// Git commit SHA of the exported branch
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub sha: Option<String>,
  /// State of the latest export
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub state: Option<String>,
}

#[cfg(any(feature = "full", feature = "codespaces"))]
/// Details about the codespace's git repository.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CodespaceWithFullRepositoryGitStatus {
  /// The number of commits the local repository is ahead of the remote.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub ahead: Option<i64>,
  /// The number of commits the local repository is behind the remote.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub behind: Option<i64>,
  /// Whether the local repository has uncommitted changes.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub has_uncommitted_changes: Option<bool>,
  /// Whether the local repository has unpushed changes.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub has_unpushed_changes: Option<bool>,
  /// The current branch (or SHA if in detached HEAD state) of the local repository.
  #[serde(rename = "ref")]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub ref_: Option<String>,
}

#[cfg(any(feature = "full", feature = "codespaces"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CodespaceWithFullRepositoryRuntimeConstraints {
  /// The privacy settings a user can select from when forwarding a port.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub allowed_port_privacy_settings: Option<Vec<String>>,
}

#[cfg(any(feature = "full", feature = "codespaces"))]
/// A codespace.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CodespaceWithFullRepository {
  pub billable_owner: SimpleUser,
  pub created_at: String,
  /// Path to devcontainer.json from repo root used to create Codespace.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub devcontainer_path: Option<String>,
  /// Display name for this codespace.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub display_name: Option<String>,
  /// UUID identifying this codespace's environment.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub environment_id: Option<String>,
  /// Details about the codespace's git repository.
  pub git_status: CodespaceWithFullRepositoryGitStatus,
  pub id: i64,
  /// The number of minutes of inactivity after which this codespace will be automatically stopped.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub idle_timeout_minutes: Option<i64>,
  /// Text to show user when codespace idle timeout minutes has been overriden by an organization policy
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub idle_timeout_notice: Option<String>,
  /// Last known time this codespace was started.
  pub last_used_at: String,
  /// The initally assigned location of a new codespace.
  pub location: CodespaceWithFullRepositoryLocation,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub machine: Option<CodespaceMachine>,
  /// API URL to access available alternate machine types for this codespace.
  pub machines_url: String,
  /// Automatically generated name of this codespace.
  pub name: String,
  pub owner: SimpleUser,
  /// Whether or not a codespace has a pending async operation. This would mean that the codespace is temporarily unavailable. The only thing that you can do with a codespace in this state is delete it.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub pending_operation: Option<bool>,
  /// Text to show user when codespace is disabled by a pending operation
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub pending_operation_disabled_reason: Option<String>,
  /// Whether the codespace was created from a prebuild.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub prebuild: Option<bool>,
  /// API URL to publish this codespace to a new repository.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub publish_url: Option<String>,
  /// API URL for the Pull Request associated with this codespace, if any.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub pulls_url: Option<String>,
  pub recent_folders: Vec<String>,
  pub repository: FullRepository,
  /// When a codespace will be auto-deleted based on the "retention_period_minutes" and "last_used_at"
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub retention_expires_at: Option<String>,
  /// Duration in minutes after codespace has gone idle in which it will be deleted. Must be integer minutes between 0 and 43200 (30 days).
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub retention_period_minutes: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub runtime_constraints: Option<CodespaceWithFullRepositoryRuntimeConstraints>,
  /// API URL to start this codespace.
  pub start_url: String,
  /// State of this codespace.
  pub state: CodespaceWithFullRepositoryState,
  /// API URL to stop this codespace.
  pub stop_url: String,
  pub updated_at: String,
  /// API URL for this codespace.
  pub url: String,
  /// URL to access this codespace on the web.
  pub web_url: String,
}

#[cfg(any(feature = "full", feature = "users"))]
/// Email
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct Email {
  pub email: String,
  pub primary: bool,
  pub verified: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub visibility: Option<String>,
}

#[cfg(any(feature = "full", feature = "users"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct GpgKeyEmails {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub email: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub verified: Option<bool>,
}

#[cfg(any(feature = "full", feature = "users"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct GpgKeySubkeysEmails {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub email: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub verified: Option<bool>,
}

#[cfg(any(feature = "full", feature = "users"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct GpgKeySubkeys {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub can_certify: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub can_encrypt_comms: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub can_encrypt_storage: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub can_sign: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub created_at: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub emails: Option<Vec<GpgKeySubkeysEmails>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub expires_at: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub id: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub key_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub primary_key_id: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub public_key: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub raw_key: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub revoked: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub subkeys: Option<Vec<serde_json::Value>>,
}

#[cfg(any(feature = "full", feature = "users"))]
/// A unique encryption key
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct GpgKey {
  pub can_certify: bool,
  pub can_encrypt_comms: bool,
  pub can_encrypt_storage: bool,
  pub can_sign: bool,
  pub created_at: String,
  pub emails: Vec<GpgKeyEmails>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub expires_at: Option<String>,
  pub id: i64,
  pub key_id: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub primary_key_id: Option<i64>,
  pub public_key: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub raw_key: Option<String>,
  pub revoked: bool,
  pub subkeys: Vec<GpgKeySubkeys>,
}

#[cfg(any(feature = "full", feature = "users"))]
/// Key
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct Key {
  pub created_at: String,
  pub id: i64,
  pub key: String,
  pub read_only: bool,
  pub title: String,
  pub url: String,
  pub verified: bool,
}

#[cfg(any(feature = "full", feature = "apps"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct MarketplaceAccount {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub email: Option<String>,
  pub id: i64,
  pub login: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub node_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub organization_billing_email: Option<String>,
  #[serde(rename = "type")]
  pub type_: String,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "apps"))]
/// User Marketplace Purchase
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct UserMarketplacePurchase {
  pub account: MarketplaceAccount,
  pub billing_cycle: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub free_trial_ends_on: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub next_billing_date: Option<String>,
  pub on_free_trial: bool,
  pub plan: MarketplaceListingPlan,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub unit_count: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(default, setter(strip_option))]
  pub updated_at: Option<String>,
}

#[cfg(any(feature = "full", feature = "users"))]
/// Social media account
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct SocialAccount {
  pub provider: String,
  pub url: String,
}

#[cfg(any(feature = "full", feature = "users"))]
/// A public SSH key used to sign Git commits
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct SshSigningKey {
  pub created_at: String,
  pub id: i64,
  pub key: String,
  pub title: String,
}

#[cfg(any(feature = "full", feature = "users"))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct HovercardContexts {
  pub message: String,
  pub octicon: String,
}

#[cfg(any(feature = "full", feature = "users"))]
/// Hovercard
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct Hovercard {
  pub contexts: Vec<HovercardContexts>,
}

#[cfg(any(feature = "full", feature = "users"))]
/// Key Simple
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct KeySimple {
  pub id: i64,
  pub key: String,
}

#[cfg(any(feature = "full", feature = "activity"))]
/// Starred Repository
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct StarredRepository {
  pub repo: Repository,
  pub starred_at: String,
}
