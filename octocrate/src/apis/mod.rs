#[cfg(any(feature = "full", feature = "actions"))]
pub mod actions;
#[cfg(any(feature = "full", feature = "activity"))]
pub mod activity;
#[cfg(any(feature = "full", feature = "apps"))]
pub mod apps;
#[cfg(any(feature = "full", feature = "billing"))]
pub mod billing;
#[cfg(any(feature = "full", feature = "checks"))]
pub mod checks;
#[cfg(any(feature = "full", feature = "classroom"))]
pub mod classroom;
#[cfg(any(feature = "full", feature = "code_scanning"))]
pub mod code_scanning;
#[cfg(any(feature = "full", feature = "codes_of_conduct"))]
pub mod codes_of_conduct;
#[cfg(any(feature = "full", feature = "codespaces"))]
pub mod codespaces;
#[cfg(any(feature = "full", feature = "copilot"))]
pub mod copilot;
#[cfg(any(feature = "full", feature = "dependabot"))]
pub mod dependabot;
#[cfg(any(feature = "full", feature = "dependency_graph"))]
pub mod dependency_graph;
#[cfg(any(feature = "full", feature = "emojis"))]
pub mod emojis;
#[cfg(any(feature = "full", feature = "gists"))]
pub mod gists;
#[cfg(any(feature = "full", feature = "git"))]
pub mod git;
#[cfg(any(feature = "full", feature = "gitignore"))]
pub mod gitignore;
#[cfg(any(feature = "full", feature = "interactions"))]
pub mod interactions;
#[cfg(any(feature = "full", feature = "issues"))]
pub mod issues;
#[cfg(any(feature = "full", feature = "licenses"))]
pub mod licenses;
#[cfg(any(feature = "full", feature = "markdown"))]
pub mod markdown;
#[cfg(any(feature = "full", feature = "meta"))]
pub mod meta;
#[cfg(any(feature = "full", feature = "migrations"))]
pub mod migrations;
#[cfg(any(feature = "full", feature = "oidc"))]
pub mod oidc;
#[cfg(any(feature = "full", feature = "orgs"))]
pub mod orgs;
#[cfg(any(feature = "full", feature = "packages"))]
pub mod packages;
#[cfg(any(feature = "full", feature = "projects"))]
pub mod projects;
#[cfg(any(feature = "full", feature = "pulls"))]
pub mod pulls;
#[cfg(any(feature = "full", feature = "rate_limit"))]
pub mod rate_limit;
#[cfg(any(feature = "full", feature = "reactions"))]
pub mod reactions;
#[cfg(any(feature = "full", feature = "repos"))]
pub mod repos;
#[cfg(any(feature = "full", feature = "search"))]
pub mod search;
#[cfg(any(feature = "full", feature = "secret_scanning"))]
pub mod secret_scanning;
#[cfg(any(feature = "full", feature = "security_advisories"))]
pub mod security_advisories;
#[cfg(any(feature = "full", feature = "teams"))]
pub mod teams;
#[cfg(any(feature = "full", feature = "users"))]
pub mod users;

use octocrate_core::SharedAPIConfig;

pub struct GitHubAPI {
  #[cfg(any(feature = "full", feature = "codespaces"))]
  pub codespaces: codespaces::GitHubCodespacesAPI,
  #[cfg(any(feature = "full", feature = "rate_limit"))]
  pub rate_limit: rate_limit::GitHubRateLimitAPI,
  #[cfg(any(feature = "full", feature = "oidc"))]
  pub oidc: oidc::GitHubOidcAPI,
  #[cfg(any(feature = "full", feature = "migrations"))]
  pub migrations: migrations::GitHubMigrationsAPI,
  #[cfg(any(feature = "full", feature = "orgs"))]
  pub orgs: orgs::GitHubOrgsAPI,
  #[cfg(any(feature = "full", feature = "issues"))]
  pub issues: issues::GitHubIssuesAPI,
  #[cfg(any(feature = "full", feature = "repos"))]
  pub repos: repos::GitHubReposAPI,
  #[cfg(any(feature = "full", feature = "users"))]
  pub users: users::GitHubUsersAPI,
  #[cfg(any(feature = "full", feature = "actions"))]
  pub actions: actions::GitHubActionsAPI,
  #[cfg(any(feature = "full", feature = "pulls"))]
  pub pulls: pulls::GitHubPullsAPI,
  #[cfg(any(feature = "full", feature = "interactions"))]
  pub interactions: interactions::GitHubInteractionsAPI,
  #[cfg(any(feature = "full", feature = "projects"))]
  pub projects: projects::GitHubProjectsAPI,
  #[cfg(any(feature = "full", feature = "dependabot"))]
  pub dependabot: dependabot::GitHubDependabotAPI,
  #[cfg(any(feature = "full", feature = "codes_of_conduct"))]
  pub codes_of_conduct: codes_of_conduct::GitHubCodesOfConductAPI,
  #[cfg(any(feature = "full", feature = "teams"))]
  pub teams: teams::GitHubTeamsAPI,
  #[cfg(any(feature = "full", feature = "git"))]
  pub git: git::GitHubGitAPI,
  #[cfg(any(feature = "full", feature = "billing"))]
  pub billing: billing::GitHubBillingAPI,
  #[cfg(any(feature = "full", feature = "apps"))]
  pub apps: apps::GitHubAppsAPI,
  #[cfg(any(feature = "full", feature = "packages"))]
  pub packages: packages::GitHubPackagesAPI,
  #[cfg(any(feature = "full", feature = "code_scanning"))]
  pub code_scanning: code_scanning::GitHubCodeScanningAPI,
  #[cfg(any(feature = "full", feature = "activity"))]
  pub activity: activity::GitHubActivityAPI,
  #[cfg(any(feature = "full", feature = "emojis"))]
  pub emojis: emojis::GitHubEmojisAPI,
  #[cfg(any(feature = "full", feature = "secret_scanning"))]
  pub secret_scanning: secret_scanning::GitHubSecretScanningAPI,
  #[cfg(any(feature = "full", feature = "search"))]
  pub search: search::GitHubSearchAPI,
  #[cfg(any(feature = "full", feature = "security_advisories"))]
  pub security_advisories: security_advisories::GitHubSecurityAdvisoriesAPI,
  #[cfg(any(feature = "full", feature = "reactions"))]
  pub reactions: reactions::GitHubReactionsAPI,
  #[cfg(any(feature = "full", feature = "markdown"))]
  pub markdown: markdown::GitHubMarkdownAPI,
  #[cfg(any(feature = "full", feature = "dependency_graph"))]
  pub dependency_graph: dependency_graph::GitHubDependencyGraphAPI,
  #[cfg(any(feature = "full", feature = "gitignore"))]
  pub gitignore: gitignore::GitHubGitignoreAPI,
  #[cfg(any(feature = "full", feature = "licenses"))]
  pub licenses: licenses::GitHubLicensesAPI,
  #[cfg(any(feature = "full", feature = "classroom"))]
  pub classroom: classroom::GitHubClassroomAPI,
  #[cfg(any(feature = "full", feature = "checks"))]
  pub checks: checks::GitHubChecksAPI,
  #[cfg(any(feature = "full", feature = "copilot"))]
  pub copilot: copilot::GitHubCopilotAPI,
  #[cfg(any(feature = "full", feature = "gists"))]
  pub gists: gists::GitHubGistsAPI,
  #[cfg(any(feature = "full", feature = "meta"))]
  pub meta: meta::GitHubMetaAPI,
}

impl GitHubAPI {
  pub fn new(config: &SharedAPIConfig) -> Self {
    Self {
      #[cfg(any(feature = "full", feature = "codespaces"))]
      codespaces: codespaces::GitHubCodespacesAPI::new(&config),
      #[cfg(any(feature = "full", feature = "rate_limit"))]
      rate_limit: rate_limit::GitHubRateLimitAPI::new(&config),
      #[cfg(any(feature = "full", feature = "oidc"))]
      oidc: oidc::GitHubOidcAPI::new(&config),
      #[cfg(any(feature = "full", feature = "migrations"))]
      migrations: migrations::GitHubMigrationsAPI::new(&config),
      #[cfg(any(feature = "full", feature = "orgs"))]
      orgs: orgs::GitHubOrgsAPI::new(&config),
      #[cfg(any(feature = "full", feature = "issues"))]
      issues: issues::GitHubIssuesAPI::new(&config),
      #[cfg(any(feature = "full", feature = "repos"))]
      repos: repos::GitHubReposAPI::new(&config),
      #[cfg(any(feature = "full", feature = "users"))]
      users: users::GitHubUsersAPI::new(&config),
      #[cfg(any(feature = "full", feature = "actions"))]
      actions: actions::GitHubActionsAPI::new(&config),
      #[cfg(any(feature = "full", feature = "pulls"))]
      pulls: pulls::GitHubPullsAPI::new(&config),
      #[cfg(any(feature = "full", feature = "interactions"))]
      interactions: interactions::GitHubInteractionsAPI::new(&config),
      #[cfg(any(feature = "full", feature = "projects"))]
      projects: projects::GitHubProjectsAPI::new(&config),
      #[cfg(any(feature = "full", feature = "dependabot"))]
      dependabot: dependabot::GitHubDependabotAPI::new(&config),
      #[cfg(any(feature = "full", feature = "codes_of_conduct"))]
      codes_of_conduct: codes_of_conduct::GitHubCodesOfConductAPI::new(&config),
      #[cfg(any(feature = "full", feature = "teams"))]
      teams: teams::GitHubTeamsAPI::new(&config),
      #[cfg(any(feature = "full", feature = "git"))]
      git: git::GitHubGitAPI::new(&config),
      #[cfg(any(feature = "full", feature = "billing"))]
      billing: billing::GitHubBillingAPI::new(&config),
      #[cfg(any(feature = "full", feature = "apps"))]
      apps: apps::GitHubAppsAPI::new(&config),
      #[cfg(any(feature = "full", feature = "packages"))]
      packages: packages::GitHubPackagesAPI::new(&config),
      #[cfg(any(feature = "full", feature = "code_scanning"))]
      code_scanning: code_scanning::GitHubCodeScanningAPI::new(&config),
      #[cfg(any(feature = "full", feature = "activity"))]
      activity: activity::GitHubActivityAPI::new(&config),
      #[cfg(any(feature = "full", feature = "emojis"))]
      emojis: emojis::GitHubEmojisAPI::new(&config),
      #[cfg(any(feature = "full", feature = "secret_scanning"))]
      secret_scanning: secret_scanning::GitHubSecretScanningAPI::new(&config),
      #[cfg(any(feature = "full", feature = "search"))]
      search: search::GitHubSearchAPI::new(&config),
      #[cfg(any(feature = "full", feature = "security_advisories"))]
      security_advisories: security_advisories::GitHubSecurityAdvisoriesAPI::new(&config),
      #[cfg(any(feature = "full", feature = "reactions"))]
      reactions: reactions::GitHubReactionsAPI::new(&config),
      #[cfg(any(feature = "full", feature = "markdown"))]
      markdown: markdown::GitHubMarkdownAPI::new(&config),
      #[cfg(any(feature = "full", feature = "dependency_graph"))]
      dependency_graph: dependency_graph::GitHubDependencyGraphAPI::new(&config),
      #[cfg(any(feature = "full", feature = "gitignore"))]
      gitignore: gitignore::GitHubGitignoreAPI::new(&config),
      #[cfg(any(feature = "full", feature = "licenses"))]
      licenses: licenses::GitHubLicensesAPI::new(&config),
      #[cfg(any(feature = "full", feature = "classroom"))]
      classroom: classroom::GitHubClassroomAPI::new(&config),
      #[cfg(any(feature = "full", feature = "checks"))]
      checks: checks::GitHubChecksAPI::new(&config),
      #[cfg(any(feature = "full", feature = "copilot"))]
      copilot: copilot::GitHubCopilotAPI::new(&config),
      #[cfg(any(feature = "full", feature = "gists"))]
      gists: gists::GitHubGistsAPI::new(&config),
      #[cfg(any(feature = "full", feature = "meta"))]
      meta: meta::GitHubMetaAPI::new(&config),
    }
  }
}
