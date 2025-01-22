use octocrate_core::*;
#[allow(unused_imports)]
use octocrate_types::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use typed_builder::TypedBuilder;

pub mod list_for_org {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<MinimalRepository>;

  #[allow(clippy::large_enum_variant)]
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QueryType {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "forks")]
    Forks,
    #[serde(rename = "sources")]
    Sources,
    #[serde(rename = "member")]
    Member,
  }

  impl std::fmt::Display for QueryType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match self {
        QueryType::All => write!(f, "all"),
        QueryType::Public => write!(f, "public"),
        QueryType::Private => write!(f, "private"),
        QueryType::Forks => write!(f, "forks"),
        QueryType::Sources => write!(f, "sources"),
        QueryType::Member => write!(f, "member"),
      }
    }
  }

  #[allow(clippy::large_enum_variant)]
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QuerySort {
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "updated")]
    Updated,
    #[serde(rename = "pushed")]
    Pushed,
    #[serde(rename = "full_name")]
    FullName,
  }

  impl std::fmt::Display for QuerySort {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match self {
        QuerySort::Created => write!(f, "created"),
        QuerySort::Updated => write!(f, "updated"),
        QuerySort::Pushed => write!(f, "pushed"),
        QuerySort::FullName => write!(f, "full_name"),
      }
    }
  }

  #[allow(clippy::large_enum_variant)]
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QueryDirection {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
  }

  impl std::fmt::Display for QueryDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match self {
        QueryDirection::Asc => write!(f, "asc"),
        QueryDirection::Desc => write!(f, "desc"),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// Specifies the types of repositories you want returned.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub type_: Option<QueryType>,
    /// The property to sort the results by.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub sort: Option<QuerySort>,
    /// The order to sort by. Default: `asc` when using `full_name`, otherwise `desc`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub direction: Option<QueryDirection>,
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
  }
}

pub mod create_in_org {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = FullRepository;

  #[allow(clippy::large_enum_variant)]
  /// The visibility of the repository.
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum RequestVisibility {
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "private")]
    Private,
  }

  impl std::fmt::Display for RequestVisibility {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match self {
        RequestVisibility::Public => write!(f, "public"),
        RequestVisibility::Private => write!(f, "private"),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// Either `true` to allow auto-merge on pull requests, or `false` to disallow auto-merge.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub allow_auto_merge: Option<bool>,
    /// Either `true` to allow merging pull requests with a merge commit, or `false` to prevent merging pull requests with merge commits.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub allow_merge_commit: Option<bool>,
    /// Either `true` to allow rebase-merging pull requests, or `false` to prevent rebase-merging.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub allow_rebase_merge: Option<bool>,
    /// Either `true` to allow squash-merging pull requests, or `false` to prevent squash-merging.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub allow_squash_merge: Option<bool>,
    /// Pass `true` to create an initial commit with empty README.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub auto_init: Option<bool>,
    /// The custom properties for the new repository. The keys are the custom property names, and the values are the corresponding custom property values.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub custom_properties: Option<serde_json::Value>,
    /// Either `true` to allow automatically deleting head branches when pull requests are merged, or `false` to prevent automatic deletion. **The authenticated user must be an organization owner to set this property to `true`.**
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub delete_branch_on_merge: Option<bool>,
    /// A short description of the repository.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub description: Option<String>,
    /// Desired language or platform [.gitignore template](https://github.com/github/gitignore) to apply. Use the name of the template without the extension. For example, "Haskell".
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub gitignore_template: Option<String>,
    /// Whether downloads are enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub has_downloads: Option<bool>,
    /// Either `true` to enable issues for this repository or `false` to disable them.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub has_issues: Option<bool>,
    /// Either `true` to enable projects for this repository or `false` to disable them. **Note:** If you're creating a repository in an organization that has disabled repository projects, the default is `false`, and if you pass `true`, the API returns an error.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub has_projects: Option<bool>,
    /// Either `true` to enable the wiki for this repository or `false` to disable it.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub has_wiki: Option<bool>,
    /// A URL with more information about the repository.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub homepage: Option<String>,
    /// Either `true` to make this repo available as a template repository or `false` to prevent it.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub is_template: Option<bool>,
    /// Choose an [open source license template](https://choosealicense.com/) that best suits your needs, and then use the [license keyword](https://docs.github.com/articles/licensing-a-repository/#searching-github-by-license-type) as the `license_template` string. For example, "mit" or "mpl-2.0".
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub license_template: Option<String>,
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
    /// The name of the repository.
    pub name: String,
    /// Whether the repository is private.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub private: Option<bool>,
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
    /// The id of the team that will be granted access to this repository. This is only valid when creating a repository in an organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub team_id: Option<i64>,
    /// Either `true` to allow squash-merge commits to use pull request title, or `false` to use commit message. **This property has been deprecated. Please use `squash_merge_commit_title` instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub use_squash_pr_title_as_default: Option<bool>,
    /// The visibility of the repository.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub visibility: Option<RequestVisibility>,
  }
}

pub mod get_org_rulesets {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<RepositoryRuleset>;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
  }
}

pub mod create_org_ruleset {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = RepositoryRuleset;

  #[allow(clippy::large_enum_variant)]
  /// The target of the ruleset.
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum RequestTarget {
    #[serde(rename = "branch")]
    Branch,
    #[serde(rename = "tag")]
    Tag,
  }

  impl std::fmt::Display for RequestTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match self {
        RequestTarget::Branch => write!(f, "branch"),
        RequestTarget::Tag => write!(f, "tag"),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The actors that can bypass the rules in this ruleset
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub bypass_actors: Option<Vec<RepositoryRulesetBypassActor>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub conditions: Option<OrgRulesetConditions>,
    pub enforcement: RepositoryRuleEnforcement,
    /// The name of the ruleset.
    pub name: String,
    /// An array of rules within the ruleset.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub rules: Option<Vec<RepositoryRule>>,
    /// The target of the ruleset.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub target: Option<RequestTarget>,
  }
}

pub mod get_org_rule_suites {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = RuleSuites;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The name of the repository to filter on. When specified, only rule evaluations from this repository will be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub repository_name: Option<i64>,
    /// The time period to filter by.
    ///
    /// For example, `day` will filter for rule suites that occurred in the past 24 hours, and `week` will filter for insights that occurred in the past 7 days (168 hours).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub time_period: Option<parameters::TimePeriod>,
    /// The handle for the GitHub user account to filter on. When specified, only rule evaluations triggered by this actor will be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub actor_name: Option<String>,
    /// The rule results to filter on. When specified, only suites with this result will be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub rule_suite_result: Option<parameters::RuleSuiteResult>,
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
  }
}

pub mod get_org_rule_suite {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = RuleSuite;
}

pub mod get_org_ruleset {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = RepositoryRuleset;
}

pub mod update_org_ruleset {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = RepositoryRuleset;

  #[allow(clippy::large_enum_variant)]
  /// The target of the ruleset.
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum RequestTarget {
    #[serde(rename = "branch")]
    Branch,
    #[serde(rename = "tag")]
    Tag,
  }

  impl std::fmt::Display for RequestTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match self {
        RequestTarget::Branch => write!(f, "branch"),
        RequestTarget::Tag => write!(f, "tag"),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The actors that can bypass the rules in this ruleset
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub bypass_actors: Option<Vec<RepositoryRulesetBypassActor>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub conditions: Option<OrgRulesetConditions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub enforcement: Option<RepositoryRuleEnforcement>,
    /// The name of the ruleset.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub name: Option<String>,
    /// An array of rules within the ruleset.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub rules: Option<Vec<RepositoryRule>>,
    /// The target of the ruleset.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub target: Option<RequestTarget>,
  }
}

pub mod get {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = FullRepository;
}

pub mod update {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = FullRepository;

  #[allow(clippy::large_enum_variant)]
  /// The visibility of the repository.
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum RequestVisibility {
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "private")]
    Private,
  }

  impl std::fmt::Display for RequestVisibility {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match self {
        RequestVisibility::Public => write!(f, "public"),
        RequestVisibility::Private => write!(f, "private"),
      }
    }
  }

  /// Use the `status` property to enable or disable GitHub Advanced Security for this repository. For more information, see "[About GitHub Advanced Security](/github/getting-started-with-github/learning-about-github/about-github-advanced-security)."
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct RequestSecurityAndAnalysisAdvancedSecurity {
    /// Can be `enabled` or `disabled`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub status: Option<String>,
  }

  /// Use the `status` property to enable or disable secret scanning for this repository. For more information, see "[About secret scanning](/code-security/secret-security/about-secret-scanning)."
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct RequestSecurityAndAnalysisSecretScanning {
    /// Can be `enabled` or `disabled`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub status: Option<String>,
  }

  /// Use the `status` property to enable or disable secret scanning push protection for this repository. For more information, see "[Protecting pushes with secret scanning](/code-security/secret-scanning/protecting-pushes-with-secret-scanning)."
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct RequestSecurityAndAnalysisSecretScanningPushProtection {
    /// Can be `enabled` or `disabled`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub status: Option<String>,
  }

  /// Specify which security and analysis features to enable or disable for the repository.
  ///
  /// To use this parameter, you must have admin permissions for the repository or be an owner or security manager for the organization that owns the repository. For more information, see "[Managing security managers in your organization](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/managing-security-managers-in-your-organization)."
  ///
  /// For example, to enable GitHub Advanced Security, use this data in the body of the `PATCH` request:
  /// `{ "security_and_analysis": {"advanced_security": { "status": "enabled" } } }`.
  ///
  /// You can check which security and analysis features are currently enabled by using a `GET /repos/{owner}/{repo}` request.
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct RequestSecurityAndAnalysis {
    /// Use the `status` property to enable or disable GitHub Advanced Security for this repository. For more information, see "[About GitHub Advanced Security](/github/getting-started-with-github/learning-about-github/about-github-advanced-security)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub advanced_security: Option<RequestSecurityAndAnalysisAdvancedSecurity>,
    /// Use the `status` property to enable or disable secret scanning for this repository. For more information, see "[About secret scanning](/code-security/secret-security/about-secret-scanning)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub secret_scanning: Option<RequestSecurityAndAnalysisSecretScanning>,
    /// Use the `status` property to enable or disable secret scanning push protection for this repository. For more information, see "[Protecting pushes with secret scanning](/code-security/secret-scanning/protecting-pushes-with-secret-scanning)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub secret_scanning_push_protection:
      Option<RequestSecurityAndAnalysisSecretScanningPushProtection>,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// Either `true` to allow auto-merge on pull requests, or `false` to disallow auto-merge.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub allow_auto_merge: Option<bool>,
    /// Either `true` to allow private forks, or `false` to prevent private forks.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub allow_forking: Option<bool>,
    /// Either `true` to allow merging pull requests with a merge commit, or `false` to prevent merging pull requests with merge commits.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub allow_merge_commit: Option<bool>,
    /// Either `true` to allow rebase-merging pull requests, or `false` to prevent rebase-merging.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub allow_rebase_merge: Option<bool>,
    /// Either `true` to allow squash-merging pull requests, or `false` to prevent squash-merging.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub allow_squash_merge: Option<bool>,
    /// Either `true` to always allow a pull request head branch that is behind its base branch to be updated even if it is not required to be up to date before merging, or false otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub allow_update_branch: Option<bool>,
    /// Whether to archive this repository. `false` will unarchive a previously archived repository.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub archived: Option<bool>,
    /// Updates the default branch for this repository.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub default_branch: Option<String>,
    /// Either `true` to allow automatically deleting head branches when pull requests are merged, or `false` to prevent automatic deletion.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub delete_branch_on_merge: Option<bool>,
    /// A short description of the repository.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub description: Option<String>,
    /// Either `true` to enable issues for this repository or `false` to disable them.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub has_issues: Option<bool>,
    /// Either `true` to enable projects for this repository or `false` to disable them. **Note:** If you're creating a repository in an organization that has disabled repository projects, the default is `false`, and if you pass `true`, the API returns an error.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub has_projects: Option<bool>,
    /// Either `true` to enable the wiki for this repository or `false` to disable it.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub has_wiki: Option<bool>,
    /// A URL with more information about the repository.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub homepage: Option<String>,
    /// Either `true` to make this repo available as a template repository or `false` to prevent it.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub is_template: Option<bool>,
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
    /// The name of the repository.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub name: Option<String>,
    /// Either `true` to make the repository private or `false` to make it public. Default: `false`.  
    /// **Note**: You will get a `422` error if the organization restricts [changing repository visibility](https://docs.github.com/articles/repository-permission-levels-for-an-organization#changing-the-visibility-of-repositories) to organization owners and a non-owner tries to change the value of private.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub private: Option<bool>,
    /// Specify which security and analysis features to enable or disable for the repository.
    ///
    /// To use this parameter, you must have admin permissions for the repository or be an owner or security manager for the organization that owns the repository. For more information, see "[Managing security managers in your organization](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/managing-security-managers-in-your-organization)."
    ///
    /// For example, to enable GitHub Advanced Security, use this data in the body of the `PATCH` request:
    /// `{ "security_and_analysis": {"advanced_security": { "status": "enabled" } } }`.
    ///
    /// You can check which security and analysis features are currently enabled by using a `GET /repos/{owner}/{repo}` request.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub security_and_analysis: Option<RequestSecurityAndAnalysis>,
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
    /// Either `true` to allow squash-merge commits to use pull request title, or `false` to use commit message. **This property has been deprecated. Please use `squash_merge_commit_title` instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub use_squash_pr_title_as_default: Option<bool>,
    /// The visibility of the repository.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub visibility: Option<RequestVisibility>,
    /// Either `true` to require contributors to sign off on web-based commits, or `false` to not require contributors to sign off on web-based commits.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub web_commit_signoff_required: Option<bool>,
  }
}

pub mod list_activities {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Activity>;

  #[allow(clippy::large_enum_variant)]
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QueryTimePeriod {
    #[serde(rename = "day")]
    Day,
    #[serde(rename = "week")]
    Week,
    #[serde(rename = "month")]
    Month,
    #[serde(rename = "quarter")]
    Quarter,
    #[serde(rename = "year")]
    Year,
  }

  impl std::fmt::Display for QueryTimePeriod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match self {
        QueryTimePeriod::Day => write!(f, "day"),
        QueryTimePeriod::Week => write!(f, "week"),
        QueryTimePeriod::Month => write!(f, "month"),
        QueryTimePeriod::Quarter => write!(f, "quarter"),
        QueryTimePeriod::Year => write!(f, "year"),
      }
    }
  }

  #[allow(clippy::large_enum_variant)]
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QueryActivityType {
    #[serde(rename = "push")]
    Push,
    #[serde(rename = "force_push")]
    ForcePush,
    #[serde(rename = "branch_creation")]
    BranchCreation,
    #[serde(rename = "branch_deletion")]
    BranchDeletion,
    #[serde(rename = "pr_merge")]
    PrMerge,
    #[serde(rename = "merge_queue_merge")]
    MergeQueueMerge,
  }

  impl std::fmt::Display for QueryActivityType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match self {
        QueryActivityType::Push => write!(f, "push"),
        QueryActivityType::ForcePush => write!(f, "force_push"),
        QueryActivityType::BranchCreation => write!(f, "branch_creation"),
        QueryActivityType::BranchDeletion => write!(f, "branch_deletion"),
        QueryActivityType::PrMerge => write!(f, "pr_merge"),
        QueryActivityType::MergeQueueMerge => write!(f, "merge_queue_merge"),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The direction to sort the results by.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub direction: Option<parameters::Direction>,
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results before this cursor. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub before: Option<String>,
    /// A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results after this cursor. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub after: Option<String>,
    /// The Git reference for the activities you want to list.
    ///
    /// The `ref` for a branch can be formatted either as `refs/heads/BRANCH_NAME` or `BRANCH_NAME`, where `BRANCH_NAME` is the name of your branch.
    #[serde(rename = "ref")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub ref_: Option<String>,
    /// The GitHub username to use to filter by the actor who performed the activity.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub actor: Option<String>,
    /// The time period to filter by.
    ///
    /// For example, `day` will filter for activity that occurred in the past 24 hours, and `week` will filter for activity that occurred in the past 7 days (168 hours).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub time_period: Option<QueryTimePeriod>,
    /// The activity type to filter by.
    ///
    /// For example, you can choose to filter by "force_push", to see all force pushes to the repository.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub activity_type: Option<QueryActivityType>,
  }
}

pub mod list_autolinks {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Autolink>;
}

pub mod create_autolink {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Autolink;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// Whether this autolink reference matches alphanumeric characters. If true, the `<num>` parameter of the `url_template` matches alphanumeric characters `A-Z` (case insensitive), `0-9`, and `-`. If false, this autolink reference only matches numeric characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub is_alphanumeric: Option<bool>,
    /// This prefix appended by certain characters will generate a link any time it is found in an issue, pull request, or commit.
    pub key_prefix: String,
    /// The URL must contain `<num>` for the reference number. `<num>` matches different characters depending on the value of `is_alphanumeric`.
    pub url_template: String,
  }
}

pub mod get_autolink {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Autolink;
}

pub mod check_automated_security_fixes {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = CheckAutomatedSecurityFixes;
}

pub mod list_branches {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<ShortBranch>;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// Setting to `true` returns only protected branches. When set to `false`, only unprotected branches are returned. Omitting this parameter returns all branches.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub protected: Option<bool>,
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
  }
}

pub mod get_branch {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = BranchWithProtection;
}

pub mod get_branch_protection {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = BranchProtection;
}

pub mod update_branch_protection {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = ProtectedBranch;

  /// Allow specific users, teams, or apps to bypass pull request requirements.
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct RequestRequiredPullRequestReviewsBypassPullRequestAllowances {
    /// The list of app `slug`s allowed to bypass pull request requirements.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub apps: Option<Vec<String>>,
    /// The list of team `slug`s allowed to bypass pull request requirements.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub teams: Option<Vec<String>>,
    /// The list of user `login`s allowed to bypass pull request requirements.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub users: Option<Vec<String>>,
  }

  /// Specify which users, teams, and apps can dismiss pull request reviews. Pass an empty `dismissal_restrictions` object to disable. User and team `dismissal_restrictions` are only available for organization-owned repositories. Omit this parameter for personal repositories.
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct RequestRequiredPullRequestReviewsDismissalRestrictions {
    /// The list of app `slug`s with dismissal access
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub apps: Option<Vec<String>>,
    /// The list of team `slug`s with dismissal access
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub teams: Option<Vec<String>>,
    /// The list of user `login`s with dismissal access
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub users: Option<Vec<String>>,
  }

  /// Require at least one approving review on a pull request, before merging. Set to `null` to disable.
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct RequestRequiredPullRequestReviews {
    /// Allow specific users, teams, or apps to bypass pull request requirements.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub bypass_pull_request_allowances:
      Option<RequestRequiredPullRequestReviewsBypassPullRequestAllowances>,
    /// Set to `true` if you want to automatically dismiss approving reviews when someone pushes a new commit.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub dismiss_stale_reviews: Option<bool>,
    /// Specify which users, teams, and apps can dismiss pull request reviews. Pass an empty `dismissal_restrictions` object to disable. User and team `dismissal_restrictions` are only available for organization-owned repositories. Omit this parameter for personal repositories.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub dismissal_restrictions: Option<RequestRequiredPullRequestReviewsDismissalRestrictions>,
    /// Blocks merging pull requests until [code owners](https://docs.github.com/articles/about-code-owners/) review them.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub require_code_owner_reviews: Option<bool>,
    /// Whether the most recent push must be approved by someone other than the person who pushed it. Default: `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub require_last_push_approval: Option<bool>,
    /// Specify the number of reviewers required to approve pull requests. Use a number between 1 and 6 or 0 to not require reviewers.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub required_approving_review_count: Option<i64>,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct RequestRequiredStatusChecksChecks {
    /// The ID of the GitHub App that must provide this check. Omit this field to automatically select the GitHub App that has recently provided this check, or any app if it was not set by a GitHub App. Pass -1 to explicitly allow any app to set the status.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub app_id: Option<i64>,
    /// The name of the required check
    pub context: String,
  }

  /// Require status checks to pass before merging. Set to `null` to disable.
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct RequestRequiredStatusChecks {
    /// The list of status checks to require in order to merge into this branch.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub checks: Option<Vec<RequestRequiredStatusChecksChecks>>,
    /// **Deprecated**: The list of status checks to require in order to merge into this branch. If any of these checks have recently been set by a particular GitHub App, they will be required to come from that app in future for the branch to merge. Use `checks` instead of `contexts` for more fine-grained control.
    ///
    pub contexts: Vec<String>,
    /// Require branches to be up to date before merging.
    pub strict: bool,
  }

  /// Restrict who can push to the protected branch. User, app, and team `restrictions` are only available for organization-owned repositories. Set to `null` to disable.
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct RequestRestrictions {
    /// The list of app `slug`s with push access
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub apps: Option<Vec<String>>,
    /// The list of team `slug`s with push access
    pub teams: Vec<String>,
    /// The list of user `login`s with push access
    pub users: Vec<String>,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// Allows deletion of the protected branch by anyone with write access to the repository. Set to `false` to prevent deletion of the protected branch. Default: `false`. For more information, see "[Enabling force pushes to a protected branch](https://docs.github.com/github/administering-a-repository/enabling-force-pushes-to-a-protected-branch)" in the GitHub Help documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub allow_deletions: Option<bool>,
    /// Permits force pushes to the protected branch by anyone with write access to the repository. Set to `true` to allow force pushes. Set to `false` or `null` to block force pushes. Default: `false`. For more information, see "[Enabling force pushes to a protected branch](https://docs.github.com/github/administering-a-repository/enabling-force-pushes-to-a-protected-branch)" in the GitHub Help documentation."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub allow_force_pushes: Option<bool>,
    /// Whether users can pull changes from upstream when the branch is locked. Set to `true` to allow fork syncing. Set to `false` to prevent fork syncing. Default: `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub allow_fork_syncing: Option<bool>,
    /// If set to `true`, the `restrictions` branch protection settings which limits who can push will also block pushes which create new branches, unless the push is initiated by a user, team, or app which has the ability to push. Set to `true` to restrict new branch creation. Default: `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub block_creations: Option<bool>,
    /// Enforce all configured restrictions for administrators. Set to `true` to enforce required status checks for repository administrators. Set to `null` to disable.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub enforce_admins: Option<bool>,
    /// Whether to set the branch as read-only. If this is true, users will not be able to push to the branch. Default: `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub lock_branch: Option<bool>,
    /// Requires all conversations on code to be resolved before a pull request can be merged into a branch that matches this rule. Set to `false` to disable. Default: `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub required_conversation_resolution: Option<bool>,
    /// Enforces a linear commit Git history, which prevents anyone from pushing merge commits to a branch. Set to `true` to enforce a linear commit history. Set to `false` to disable a linear commit Git history. Your repository must allow squash merging or rebase merging before you can enable a linear commit history. Default: `false`. For more information, see "[Requiring a linear commit history](https://docs.github.com/github/administering-a-repository/requiring-a-linear-commit-history)" in the GitHub Help documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub required_linear_history: Option<bool>,
    /// Require at least one approving review on a pull request, before merging. Set to `null` to disable.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub required_pull_request_reviews: Option<RequestRequiredPullRequestReviews>,
    /// Require status checks to pass before merging. Set to `null` to disable.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub required_status_checks: Option<RequestRequiredStatusChecks>,
    /// Restrict who can push to the protected branch. User, app, and team `restrictions` are only available for organization-owned repositories. Set to `null` to disable.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub restrictions: Option<RequestRestrictions>,
  }
}

pub mod get_admin_branch_protection {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = ProtectedBranchAdminEnforced;
}

pub mod set_admin_branch_protection {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = ProtectedBranchAdminEnforced;
}

pub mod get_pull_request_review_protection {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = ProtectedBranchPullRequestReview;
}

pub mod update_pull_request_review_protection {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = ProtectedBranchPullRequestReview;

  /// Allow specific users, teams, or apps to bypass pull request requirements.
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct RequestBypassPullRequestAllowances {
    /// The list of app `slug`s allowed to bypass pull request requirements.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub apps: Option<Vec<String>>,
    /// The list of team `slug`s allowed to bypass pull request requirements.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub teams: Option<Vec<String>>,
    /// The list of user `login`s allowed to bypass pull request requirements.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub users: Option<Vec<String>>,
  }

  /// Specify which users, teams, and apps can dismiss pull request reviews. Pass an empty `dismissal_restrictions` object to disable. User and team `dismissal_restrictions` are only available for organization-owned repositories. Omit this parameter for personal repositories.
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct RequestDismissalRestrictions {
    /// The list of app `slug`s with dismissal access
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub apps: Option<Vec<String>>,
    /// The list of team `slug`s with dismissal access
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub teams: Option<Vec<String>>,
    /// The list of user `login`s with dismissal access
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub users: Option<Vec<String>>,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// Allow specific users, teams, or apps to bypass pull request requirements.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub bypass_pull_request_allowances: Option<RequestBypassPullRequestAllowances>,
    /// Set to `true` if you want to automatically dismiss approving reviews when someone pushes a new commit.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub dismiss_stale_reviews: Option<bool>,
    /// Specify which users, teams, and apps can dismiss pull request reviews. Pass an empty `dismissal_restrictions` object to disable. User and team `dismissal_restrictions` are only available for organization-owned repositories. Omit this parameter for personal repositories.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub dismissal_restrictions: Option<RequestDismissalRestrictions>,
    /// Blocks merging pull requests until [code owners](https://docs.github.com/articles/about-code-owners/) have reviewed.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub require_code_owner_reviews: Option<bool>,
    /// Whether the most recent push must be approved by someone other than the person who pushed it. Default: `false`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub require_last_push_approval: Option<bool>,
    /// Specifies the number of reviewers required to approve pull requests. Use a number between 1 and 6 or 0 to not require reviewers.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub required_approving_review_count: Option<i64>,
  }
}

pub mod get_commit_signature_protection {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = ProtectedBranchAdminEnforced;
}

pub mod create_commit_signature_protection {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = ProtectedBranchAdminEnforced;
}

pub mod get_status_checks_protection {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = StatusCheckPolicy;
}

pub mod update_status_check_protection {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = StatusCheckPolicy;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct RequestChecks {
    /// The ID of the GitHub App that must provide this check. Omit this field to automatically select the GitHub App that has recently provided this check, or any app if it was not set by a GitHub App. Pass -1 to explicitly allow any app to set the status.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub app_id: Option<i64>,
    /// The name of the required check
    pub context: String,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The list of status checks to require in order to merge into this branch.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub checks: Option<Vec<RequestChecks>>,
    /// **Deprecated**: The list of status checks to require in order to merge into this branch. If any of these checks have recently been set by a particular GitHub App, they will be required to come from that app in future for the branch to merge. Use `checks` instead of `contexts` for more fine-grained control.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub contexts: Option<Vec<String>>,
    /// Require branches to be up to date before merging.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub strict: Option<bool>,
  }
}

pub mod get_all_status_check_contexts {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<String>;
}

pub mod add_status_check_contexts {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<String>;

  #[allow(clippy::large_enum_variant)]
  #[derive(Debug, Clone, Serialize, Deserialize)]
  #[serde(untagged)]
  pub enum Request {
    RequestItem1(RequestItem1),
    StringArray(Vec<String>),
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct RequestItem1 {
    /// The name of the status checks
    pub contexts: Vec<String>,
  }
}

pub mod set_status_check_contexts {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<String>;

  #[allow(clippy::large_enum_variant)]
  #[derive(Debug, Clone, Serialize, Deserialize)]
  #[serde(untagged)]
  pub enum Request {
    RequestItem1(RequestItem1),
    StringArray(Vec<String>),
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct RequestItem1 {
    /// The name of the status checks
    pub contexts: Vec<String>,
  }
}

pub mod remove_status_check_contexts {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<String>;

  #[allow(clippy::large_enum_variant)]
  #[derive(Debug, Clone, Serialize, Deserialize)]
  #[serde(untagged)]
  pub enum Request {
    RequestItem1(RequestItem1),
    StringArray(Vec<String>),
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct RequestItem1 {
    /// The name of the status checks
    pub contexts: Vec<String>,
  }
}

pub mod get_access_restrictions {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = BranchRestrictionPolicy;
}

pub mod get_apps_with_access_to_protected_branch {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Integration>;
}

pub mod add_app_access_restrictions {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Integration>;

  #[allow(clippy::large_enum_variant)]
  #[derive(Debug, Clone, Serialize, Deserialize)]
  #[serde(untagged)]
  pub enum Request {
    RequestItem1(RequestItem1),
    StringArray(Vec<String>),
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct RequestItem1 {
    /// The GitHub Apps that have push access to this branch. Use the slugified version of the app name. **Note**: The list of users, apps, and teams in total is limited to 100 items.
    pub apps: Vec<String>,
  }
}

pub mod set_app_access_restrictions {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Integration>;

  #[allow(clippy::large_enum_variant)]
  #[derive(Debug, Clone, Serialize, Deserialize)]
  #[serde(untagged)]
  pub enum Request {
    RequestItem1(RequestItem1),
    StringArray(Vec<String>),
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct RequestItem1 {
    /// The GitHub Apps that have push access to this branch. Use the slugified version of the app name. **Note**: The list of users, apps, and teams in total is limited to 100 items.
    pub apps: Vec<String>,
  }
}

pub mod remove_app_access_restrictions {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Integration>;

  #[allow(clippy::large_enum_variant)]
  #[derive(Debug, Clone, Serialize, Deserialize)]
  #[serde(untagged)]
  pub enum Request {
    RequestItem1(RequestItem1),
    StringArray(Vec<String>),
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct RequestItem1 {
    /// The GitHub Apps that have push access to this branch. Use the slugified version of the app name. **Note**: The list of users, apps, and teams in total is limited to 100 items.
    pub apps: Vec<String>,
  }
}

pub mod get_teams_with_access_to_protected_branch {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Team>;
}

pub mod add_team_access_restrictions {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Team>;

  #[allow(clippy::large_enum_variant)]
  #[derive(Debug, Clone, Serialize, Deserialize)]
  #[serde(untagged)]
  pub enum Request {
    RequestItem1(RequestItem1),
    StringArray(Vec<String>),
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct RequestItem1 {
    /// The slug values for teams
    pub teams: Vec<String>,
  }
}

pub mod set_team_access_restrictions {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Team>;

  #[allow(clippy::large_enum_variant)]
  #[derive(Debug, Clone, Serialize, Deserialize)]
  #[serde(untagged)]
  pub enum Request {
    RequestItem1(RequestItem1),
    StringArray(Vec<String>),
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct RequestItem1 {
    /// The slug values for teams
    pub teams: Vec<String>,
  }
}

pub mod remove_team_access_restrictions {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Team>;

  #[allow(clippy::large_enum_variant)]
  #[derive(Debug, Clone, Serialize, Deserialize)]
  #[serde(untagged)]
  pub enum Request {
    RequestItem1(RequestItem1),
    StringArray(Vec<String>),
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct RequestItem1 {
    /// The slug values for teams
    pub teams: Vec<String>,
  }
}

pub mod get_users_with_access_to_protected_branch {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<SimpleUser>;
}

pub mod add_user_access_restrictions {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<SimpleUser>;

  #[allow(clippy::large_enum_variant)]
  #[derive(Debug, Clone, Serialize, Deserialize)]
  #[serde(untagged)]
  pub enum Request {
    RequestItem1(RequestItem1),
    StringArray(Vec<String>),
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct RequestItem1 {
    /// The username for users
    pub users: Vec<String>,
  }
}

pub mod set_user_access_restrictions {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<SimpleUser>;

  #[allow(clippy::large_enum_variant)]
  #[derive(Debug, Clone, Serialize, Deserialize)]
  #[serde(untagged)]
  pub enum Request {
    RequestItem1(RequestItem1),
    StringArray(Vec<String>),
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct RequestItem1 {
    /// The username for users
    pub users: Vec<String>,
  }
}

pub mod remove_user_access_restrictions {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<SimpleUser>;

  #[allow(clippy::large_enum_variant)]
  #[derive(Debug, Clone, Serialize, Deserialize)]
  #[serde(untagged)]
  pub enum Request {
    RequestItem1(RequestItem1),
    StringArray(Vec<String>),
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct RequestItem1 {
    /// The username for users
    pub users: Vec<String>,
  }
}

pub mod rename_branch {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = BranchWithProtection;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The new name of the branch.
    pub new_name: String,
  }
}

pub mod codeowners_errors {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = CodeownersErrors;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// A branch, tag or commit name used to determine which version of the CODEOWNERS file to use. Default: the repository's default branch (e.g. `main`)
    #[serde(rename = "ref")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub ref_: Option<String>,
  }
}

pub mod list_collaborators {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Collaborator>;

  #[allow(clippy::large_enum_variant)]
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QueryAffiliation {
    #[serde(rename = "outside")]
    Outside,
    #[serde(rename = "direct")]
    Direct,
    #[serde(rename = "all")]
    All,
  }

  impl std::fmt::Display for QueryAffiliation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match self {
        QueryAffiliation::Outside => write!(f, "outside"),
        QueryAffiliation::Direct => write!(f, "direct"),
        QueryAffiliation::All => write!(f, "all"),
      }
    }
  }

  #[allow(clippy::large_enum_variant)]
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QueryPermission {
    #[serde(rename = "pull")]
    Pull,
    #[serde(rename = "triage")]
    Triage,
    #[serde(rename = "push")]
    Push,
    #[serde(rename = "maintain")]
    Maintain,
    #[serde(rename = "admin")]
    Admin,
  }

  impl std::fmt::Display for QueryPermission {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match self {
        QueryPermission::Pull => write!(f, "pull"),
        QueryPermission::Triage => write!(f, "triage"),
        QueryPermission::Push => write!(f, "push"),
        QueryPermission::Maintain => write!(f, "maintain"),
        QueryPermission::Admin => write!(f, "admin"),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// Filter collaborators returned by their affiliation. `outside` means all outside collaborators of an organization-owned repository. `direct` means all collaborators with permissions to an organization-owned repository, regardless of organization membership status. `all` means all collaborators the authenticated user can see.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub affiliation: Option<QueryAffiliation>,
    /// Filter collaborators by the permissions they have on the repository. If not specified, all collaborators will be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub permission: Option<QueryPermission>,
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
  }
}

pub mod add_collaborator {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = RepositoryInvitation;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The permission to grant the collaborator. **Only valid on organization-owned repositories.** We accept the following permissions to be set: `pull`, `triage`, `push`, `maintain`, `admin` and you can also specify a custom repository role name, if the owning organization has defined any.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub permission: Option<String>,
  }
}

pub mod get_collaborator_permission_level {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = RepositoryCollaboratorPermission;
}

pub mod list_commit_comments_for_repo {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<CommitComment>;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
  }
}

pub mod get_commit_comment {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = CommitComment;
}

pub mod update_commit_comment {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = CommitComment;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The contents of the comment
    pub body: String,
  }
}

pub mod list_commits {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Commit>;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// SHA or branch to start listing commits from. Default: the repository’s default branch (usually `main`).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub sha: Option<String>,
    /// Only commits containing this file path will be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub path: Option<String>,
    /// GitHub username or email address to use to filter by commit author.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub author: Option<String>,
    /// GitHub username or email address to use to filter by commit committer.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub committer: Option<String>,
    /// Only show results that were last updated after the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub since: Option<String>,
    /// Only commits before this date will be returned. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub until: Option<String>,
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
  }
}

pub mod list_branches_for_head_commit {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<BranchShort>;
}

pub mod list_comments_for_commit {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<CommitComment>;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
  }
}

pub mod create_commit_comment {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = CommitComment;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The contents of the comment.
    pub body: String,
    /// **Deprecated**. Use **position** parameter instead. Line number in the file to comment on.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub line: Option<i64>,
    /// Relative path of the file to comment on.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub path: Option<String>,
    /// Line index in the diff to comment on.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub position: Option<i64>,
  }
}

pub mod list_pull_requests_associated_with_commit {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<PullRequestSimple>;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
  }
}

pub mod get_commit {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Commit;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
  }
}

pub mod get_combined_status_for_ref {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = CombinedCommitStatus;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
  }
}

pub mod list_commit_statuses_for_ref {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Status>;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
  }
}

pub mod get_community_profile_metrics {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = CommunityProfile;
}

pub mod compare_commits {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = CommitComparison;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
  }
}

pub mod get_content {
  #[allow(unused_imports)]
  use super::*;

  #[allow(clippy::large_enum_variant)]
  #[derive(Debug, Clone, Serialize, Deserialize)]
  #[serde(untagged)]
  pub enum Response {
    ContentDirectory(Vec<ContentDirectoryItem>),
    /// Content File
    ContentFile(ContentFile),
    /// An object describing a symlink
    ContentSymlink(ContentSymlink),
    /// An object describing a submodule
    ContentSubmodule(ContentSubmodule),
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The name of the commit/branch/tag. Default: the repository’s default branch.
    #[serde(rename = "ref")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub ref_: Option<String>,
  }
}

pub mod create_or_update_file_contents {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = FileCommit;

  /// The author of the file. Default: The `committer` or the authenticated user if you omit `committer`.
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct RequestAuthor {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub date: Option<String>,
    /// The email of the author or committer of the commit. You'll receive a `422` status code if `email` is omitted.
    pub email: String,
    /// The name of the author or committer of the commit. You'll receive a `422` status code if `name` is omitted.
    pub name: String,
  }

  /// The person that committed the file. Default: the authenticated user.
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct RequestCommitter {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub date: Option<String>,
    /// The email of the author or committer of the commit. You'll receive a `422` status code if `email` is omitted.
    pub email: String,
    /// The name of the author or committer of the commit. You'll receive a `422` status code if `name` is omitted.
    pub name: String,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The author of the file. Default: The `committer` or the authenticated user if you omit `committer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub author: Option<RequestAuthor>,
    /// The branch name. Default: the repository’s default branch.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub branch: Option<String>,
    /// The person that committed the file. Default: the authenticated user.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub committer: Option<RequestCommitter>,
    /// The new file content, using Base64 encoding.
    pub content: String,
    /// The commit message.
    pub message: String,
    /// **Required if you are updating a file**. The blob SHA of the file being replaced.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub sha: Option<String>,
  }
}

pub mod delete_file {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = FileCommit;

  /// object containing information about the author.
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct RequestAuthor {
    /// The email of the author (or committer) of the commit
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub email: Option<String>,
    /// The name of the author (or committer) of the commit
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub name: Option<String>,
  }

  /// object containing information about the committer.
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct RequestCommitter {
    /// The email of the author (or committer) of the commit
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub email: Option<String>,
    /// The name of the author (or committer) of the commit
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub name: Option<String>,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// object containing information about the author.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub author: Option<RequestAuthor>,
    /// The branch name. Default: the repository’s default branch
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub branch: Option<String>,
    /// object containing information about the committer.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub committer: Option<RequestCommitter>,
    /// The commit message.
    pub message: String,
    /// The blob SHA of the file being deleted.
    pub sha: String,
  }
}

pub mod list_contributors {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Contributor>;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// Set to `1` or `true` to include anonymous contributors in results.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub anon: Option<String>,
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
  }
}

pub mod list_deployments {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Deployment>;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The SHA recorded at creation time.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub sha: Option<String>,
    /// The name of the ref. This can be a branch, tag, or SHA.
    #[serde(rename = "ref")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub ref_: Option<String>,
    /// The name of the task for the deployment (e.g., `deploy` or `deploy:migrations`).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub task: Option<String>,
    /// The name of the environment that was deployed to (e.g., `staging` or `production`).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub environment: Option<String>,
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
  }
}

pub mod create_deployment {
  #[allow(unused_imports)]
  use super::*;

  #[allow(clippy::large_enum_variant)]
  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub enum Response {
    Created(Deployment),
    Accepted(AcceptedResponse),
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// Attempts to automatically merge the default branch into the requested ref, if it's behind the default branch.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub auto_merge: Option<bool>,
    /// Short description of the deployment.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub description: Option<String>,
    /// Name for the target deployment environment (e.g., `production`, `staging`, `qa`).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub environment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub payload: Option<serde_json::Value>,
    /// Specifies if the given environment is one that end-users directly interact with. Default: `true` when `environment` is `production` and `false` otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub production_environment: Option<bool>,
    /// The ref to deploy. This can be a branch, tag, or SHA.
    #[serde(rename = "ref")]
    pub ref_: String,
    /// The [status](https://docs.github.com/rest/commits/statuses) contexts to verify against commit status checks. If you omit this parameter, GitHub verifies all unique contexts before creating a deployment. To bypass checking entirely, pass an empty array. Defaults to all unique contexts.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub required_contexts: Option<Vec<String>>,
    /// Specifies a task to execute (e.g., `deploy` or `deploy:migrations`).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub task: Option<String>,
    /// Specifies if the given environment is specific to the deployment and will no longer exist at some point in the future. Default: `false`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub transient_environment: Option<bool>,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct AcceptedResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub message: Option<String>,
  }
}

pub mod get_deployment {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Deployment;
}

pub mod list_deployment_statuses {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<DeploymentStatus>;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
  }
}

pub mod create_deployment_status {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = DeploymentStatus;

  #[allow(clippy::large_enum_variant)]
  /// The state of the status. When you set a transient deployment to `inactive`, the deployment will be shown as `destroyed` in GitHub.
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum RequestState {
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "failure")]
    Failure,
    #[serde(rename = "inactive")]
    Inactive,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "success")]
    Success,
  }

  impl std::fmt::Display for RequestState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match self {
        RequestState::Error => write!(f, "error"),
        RequestState::Failure => write!(f, "failure"),
        RequestState::Inactive => write!(f, "inactive"),
        RequestState::InProgress => write!(f, "in_progress"),
        RequestState::Queued => write!(f, "queued"),
        RequestState::Pending => write!(f, "pending"),
        RequestState::Success => write!(f, "success"),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// Adds a new `inactive` status to all prior non-transient, non-production environment deployments with the same repository and `environment` name as the created status's deployment. An `inactive` status is only added to deployments that had a `success` state. Default: `true`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub auto_inactive: Option<bool>,
    /// A short description of the status. The maximum description length is 140 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub description: Option<String>,
    /// Name for the target deployment environment, which can be changed when setting a deploy status. For example, `production`, `staging`, or `qa`. If not defined, the environment of the previous status on the deployment will be used, if it exists. Otherwise, the environment of the deployment will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub environment: Option<String>,
    /// Sets the URL for accessing your environment. Default: `""`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub environment_url: Option<String>,
    /// The full URL of the deployment's output. This parameter replaces `target_url`. We will continue to accept `target_url` to support legacy uses, but we recommend replacing `target_url` with `log_url`. Setting `log_url` will automatically set `target_url` to the same value. Default: `""`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub log_url: Option<String>,
    /// The state of the status. When you set a transient deployment to `inactive`, the deployment will be shown as `destroyed` in GitHub.
    pub state: RequestState,
    /// The target URL to associate with this status. This URL should contain output to keep the user updated while the task is running or serve as historical information for what happened in the deployment. **Note:** It's recommended to use the `log_url` parameter, which replaces `target_url`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub target_url: Option<String>,
  }
}

pub mod get_deployment_status {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = DeploymentStatus;
}

pub mod create_dispatch_event {
  #[allow(unused_imports)]
  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// JSON payload with extra information about the webhook event that your action or workflow may use. The maximum number of top-level properties is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub client_payload: Option<serde_json::Value>,
    /// A custom webhook event name. Must be 100 characters or fewer.
    pub event_type: String,
  }
}

pub mod get_all_environments {
  #[allow(unused_imports)]
  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub environments: Option<Vec<Environment>>,
    /// The number of environments in this repository
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub total_count: Option<i64>,
  }
}

pub mod get_environment {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Environment;
}

pub mod create_or_update_environment {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Environment;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct RequestReviewers {
    /// The id of the user or team who can review the deployment
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub id: Option<i64>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub type_: Option<DeploymentReviewerType>,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub deployment_branch_policy: Option<DeploymentBranchPolicySettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub prevent_self_review: Option<bool>,
    /// The people or teams that may review jobs that reference the environment. You can list up to six users or teams as reviewers. The reviewers must have at least read access to the repository. Only one of the required reviewers needs to approve the job for it to proceed.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub reviewers: Option<Vec<RequestReviewers>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub wait_timer: Option<i64>,
  }
}

pub mod list_deployment_branch_policies {
  #[allow(unused_imports)]
  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Response {
    pub branch_policies: Vec<DeploymentBranchPolicy>,
    /// The number of deployment branch policies for the environment.
    pub total_count: i64,
  }
}

pub mod create_deployment_branch_policy {
  #[allow(unused_imports)]
  use super::*;

  pub type Request = DeploymentBranchPolicyNamePatternWithType;
  pub type Response = DeploymentBranchPolicy;
}

pub mod get_deployment_branch_policy {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = DeploymentBranchPolicy;
}

pub mod update_deployment_branch_policy {
  #[allow(unused_imports)]
  use super::*;

  pub type Request = DeploymentBranchPolicyNamePattern;
  pub type Response = DeploymentBranchPolicy;
}

pub mod get_all_deployment_protection_rules {
  #[allow(unused_imports)]
  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub custom_deployment_protection_rules: Option<Vec<DeploymentProtectionRule>>,
    /// The number of enabled custom deployment protection rules for this environment
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub total_count: Option<i64>,
  }
}

pub mod create_deployment_protection_rule {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = DeploymentProtectionRule;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The ID of the custom app that will be enabled on the environment.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub integration_id: Option<i64>,
  }
}

pub mod list_custom_deployment_rule_integrations {
  #[allow(unused_imports)]
  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub available_custom_deployment_protection_rule_integrations:
      Option<Vec<CustomDeploymentRuleApp>>,
    /// The total number of custom deployment protection rule integrations available for this environment.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub total_count: Option<i64>,
  }
}

pub mod get_custom_deployment_protection_rule {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = DeploymentProtectionRule;
}

pub mod list_forks {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<MinimalRepository>;

  #[allow(clippy::large_enum_variant)]
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QuerySort {
    #[serde(rename = "newest")]
    Newest,
    #[serde(rename = "oldest")]
    Oldest,
    #[serde(rename = "stargazers")]
    Stargazers,
    #[serde(rename = "watchers")]
    Watchers,
  }

  impl std::fmt::Display for QuerySort {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match self {
        QuerySort::Newest => write!(f, "newest"),
        QuerySort::Oldest => write!(f, "oldest"),
        QuerySort::Stargazers => write!(f, "stargazers"),
        QuerySort::Watchers => write!(f, "watchers"),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The sort order. `stargazers` will sort by star count.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub sort: Option<QuerySort>,
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
  }
}

pub mod create_fork {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = FullRepository;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// When forking from an existing repository, fork with only the default branch.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub default_branch_only: Option<bool>,
    /// When forking from an existing repository, a new name for the fork.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub name: Option<String>,
    /// Optional parameter to specify the organization name if forking into an organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub organization: Option<String>,
  }
}

pub mod list_webhooks {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Hook>;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
  }
}

pub mod create_webhook {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Hook;

  /// Key/value pairs to provide settings for this webhook.
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct RequestConfig {
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

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// Determines if notifications are sent when the webhook is triggered. Set to `true` to send notifications.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub active: Option<bool>,
    /// Key/value pairs to provide settings for this webhook.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub config: Option<RequestConfig>,
    /// Determines what [events](https://docs.github.com/webhooks/event-payloads) the hook is triggered for.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub events: Option<Vec<String>>,
    /// Use `web` to create a webhook. Default: `web`. This parameter only accepts the value `web`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub name: Option<String>,
  }
}

pub mod get_webhook {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Hook;
}

pub mod update_webhook {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Hook;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// Determines if notifications are sent when the webhook is triggered. Set to `true` to send notifications.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub active: Option<bool>,
    /// Determines a list of events to be added to the list of events that the Hook triggers for.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub add_events: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub config: Option<WebhookConfig>,
    /// Determines what [events](https://docs.github.com/webhooks/event-payloads) the hook is triggered for. This replaces the entire array of events.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub events: Option<Vec<String>>,
    /// Determines a list of events to be removed from the list of events that the Hook triggers for.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub remove_events: Option<Vec<String>>,
  }
}

pub mod get_webhook_config_for_repo {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = WebhookConfig;
}

pub mod update_webhook_config_for_repo {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = WebhookConfig;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
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
}

pub mod list_webhook_deliveries {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<HookDeliveryItem>;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// Used for pagination: the starting delivery from which the page of deliveries is fetched. Refer to the `link` header for the next and previous page cursors.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub redelivery: Option<bool>,
  }
}

pub mod get_webhook_delivery {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = HookDelivery;
}

pub mod list_invitations {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<RepositoryInvitation>;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
  }
}

pub mod update_invitation {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = RepositoryInvitation;

  #[allow(clippy::large_enum_variant)]
  /// The permissions that the associated user will have on the repository. Valid values are `read`, `write`, `maintain`, `triage`, and `admin`.
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum RequestPermissions {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
    #[serde(rename = "maintain")]
    Maintain,
    #[serde(rename = "triage")]
    Triage,
    #[serde(rename = "admin")]
    Admin,
  }

  impl std::fmt::Display for RequestPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match self {
        RequestPermissions::Read => write!(f, "read"),
        RequestPermissions::Write => write!(f, "write"),
        RequestPermissions::Maintain => write!(f, "maintain"),
        RequestPermissions::Triage => write!(f, "triage"),
        RequestPermissions::Admin => write!(f, "admin"),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The permissions that the associated user will have on the repository. Valid values are `read`, `write`, `maintain`, `triage`, and `admin`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub permissions: Option<RequestPermissions>,
  }
}

pub mod list_deploy_keys {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<DeployKey>;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
  }
}

pub mod create_deploy_key {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = DeployKey;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The contents of the key.
    pub key: String,
    /// If `true`, the key will only be able to read repository contents. Otherwise, the key will be able to read and write.  
    ///   
    /// Deploy keys with write access can perform the same actions as an organization member with admin access, or a collaborator on a personal repository. For more information, see "[Repository permission levels for an organization](https://docs.github.com/articles/repository-permission-levels-for-an-organization/)" and "[Permission levels for a user account repository](https://docs.github.com/articles/permission-levels-for-a-user-account-repository/)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub read_only: Option<bool>,
    /// A name for the key.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub title: Option<String>,
  }
}

pub mod get_deploy_key {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = DeployKey;
}

pub mod list_languages {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = serde_json::Value;
}

pub mod merge_upstream {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = MergedUpstream;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The name of the branch which should be updated to match upstream.
    pub branch: String,
  }
}

pub mod merge {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Commit;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The name of the base branch that the head will be merged into.
    pub base: String,
    /// Commit message to use for the merge commit. If omitted, a default message will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub commit_message: Option<String>,
    /// The head to merge. This can be a branch name or a commit SHA1.
    pub head: String,
  }
}

pub mod get_pages {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Page;
}

pub mod create_pages_site {
  #[allow(unused_imports)]
  use super::*;

  pub type Request = Option<serde_json::Value>;
  pub type Response = Page;
}

pub mod update_information_about_pages_site {
  #[allow(unused_imports)]
  use super::*;

  pub type Request = serde_json::Value;
}

pub mod list_pages_builds {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<PageBuild>;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
  }
}

pub mod request_pages_build {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = PageBuildStatus;
}

pub mod get_latest_pages_build {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = PageBuild;
}

pub mod get_pages_build {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = PageBuild;
}

pub mod create_pages_deployment {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = PageDeployment;

  /// The object used to create GitHub Pages deployment
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The ID of an artifact that contains the .zip or .tar of static assets to deploy. The artifact belongs to the repository. Either `artifact_id` or `artifact_url` are required.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub artifact_id: Option<f64>,
    /// The URL of an artifact that contains the .zip or .tar of static assets to deploy. The artifact belongs to the repository. Either `artifact_id` or `artifact_url` are required.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub artifact_url: Option<String>,
    /// The target environment for this GitHub Pages deployment.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub environment: Option<String>,
    /// The OIDC token issued by GitHub Actions certifying the origin of the deployment.
    pub oidc_token: String,
    /// A unique string that represents the version of the build for this deployment.
    pub pages_build_version: String,
  }
}

pub mod get_pages_deployment {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = PagesDeploymentStatus;
}

pub mod get_pages_health_check {
  #[allow(unused_imports)]
  use super::*;

  #[allow(clippy::large_enum_variant)]
  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub enum Response {
    Success(PagesHealthCheck),
    Accepted(EmptyObject),
  }
}

pub mod check_private_vulnerability_reporting {
  #[allow(unused_imports)]
  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Response {
    /// Whether or not private vulnerability reporting is enabled for the repository.
    pub enabled: bool,
  }
}

pub mod get_custom_properties_values {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<CustomPropertyValue>;
}

pub mod create_or_update_custom_properties_values {
  #[allow(unused_imports)]
  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// A list of custom property names and associated values to apply to the repositories.
    pub properties: Vec<CustomPropertyValue>,
  }
}

pub mod get_readme {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = ContentFile;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The name of the commit/branch/tag. Default: the repository’s default branch.
    #[serde(rename = "ref")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub ref_: Option<String>,
  }
}

pub mod get_readme_in_directory {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = ContentFile;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The name of the commit/branch/tag. Default: the repository’s default branch.
    #[serde(rename = "ref")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub ref_: Option<String>,
  }
}

pub mod list_releases {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Release>;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
  }
}

pub mod create_release {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Release;

  #[allow(clippy::large_enum_variant)]
  /// Specifies whether this release should be set as the latest release for the repository. Drafts and prereleases cannot be set as latest. Defaults to `true` for newly published releases. `legacy` specifies that the latest release should be determined based on the release creation date and higher semantic version.
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum RequestMakeLatest {
    #[serde(rename = "true")]
    True,
    #[serde(rename = "false")]
    False,
    #[serde(rename = "legacy")]
    Legacy,
  }

  impl std::fmt::Display for RequestMakeLatest {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match self {
        RequestMakeLatest::True => write!(f, "true"),
        RequestMakeLatest::False => write!(f, "false"),
        RequestMakeLatest::Legacy => write!(f, "legacy"),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// Text describing the contents of the tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub body: Option<String>,
    /// If specified, a discussion of the specified category is created and linked to the release. The value must be a category that already exists in the repository. For more information, see "[Managing categories for discussions in your repository](https://docs.github.com/discussions/managing-discussions-for-your-community/managing-categories-for-discussions-in-your-repository)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub discussion_category_name: Option<String>,
    /// `true` to create a draft (unpublished) release, `false` to create a published one.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub draft: Option<bool>,
    /// Whether to automatically generate the name and body for this release. If `name` is specified, the specified name will be used; otherwise, a name will be automatically generated. If `body` is specified, the body will be pre-pended to the automatically generated notes.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub generate_release_notes: Option<bool>,
    /// Specifies whether this release should be set as the latest release for the repository. Drafts and prereleases cannot be set as latest. Defaults to `true` for newly published releases. `legacy` specifies that the latest release should be determined based on the release creation date and higher semantic version.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub make_latest: Option<RequestMakeLatest>,
    /// The name of the release.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub name: Option<String>,
    /// `true` to identify the release as a prerelease. `false` to identify the release as a full release.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub prerelease: Option<bool>,
    /// The name of the tag.
    pub tag_name: String,
    /// Specifies the commitish value that determines where the Git tag is created from. Can be any branch or commit SHA. Unused if the Git tag already exists. Default: the repository's default branch.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub target_commitish: Option<String>,
  }
}

pub mod get_release_asset {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = ReleaseAsset;
}

pub mod update_release_asset {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = ReleaseAsset;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// An alternate short description of the asset. Used in place of the filename.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub label: Option<String>,
    /// The file name of the asset.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub state: Option<String>,
  }
}

pub mod generate_release_notes {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = ReleaseNotesContent;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// Specifies a path to a file in the repository containing configuration settings used for generating the release notes. If unspecified, the configuration file located in the repository at '.github/release.yml' or '.github/release.yaml' will be used. If that is not present, the default configuration will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub configuration_file_path: Option<String>,
    /// The name of the previous tag to use as the starting point for the release notes. Use to manually specify the range for the set of changes considered as part this release.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub previous_tag_name: Option<String>,
    /// The tag name for the release. This can be an existing tag or a new one.
    pub tag_name: String,
    /// Specifies the commitish value that will be the target for the release's tag. Required if the supplied tag_name does not reference an existing tag. Ignored if the tag_name already exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub target_commitish: Option<String>,
  }
}

pub mod get_latest_release {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Release;
}

pub mod get_release_by_tag {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Release;
}

pub mod get_release {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Release;
}

pub mod update_release {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Release;

  #[allow(clippy::large_enum_variant)]
  /// Specifies whether this release should be set as the latest release for the repository. Drafts and prereleases cannot be set as latest. Defaults to `true` for newly published releases. `legacy` specifies that the latest release should be determined based on the release creation date and higher semantic version.
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum RequestMakeLatest {
    #[serde(rename = "true")]
    True,
    #[serde(rename = "false")]
    False,
    #[serde(rename = "legacy")]
    Legacy,
  }

  impl std::fmt::Display for RequestMakeLatest {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match self {
        RequestMakeLatest::True => write!(f, "true"),
        RequestMakeLatest::False => write!(f, "false"),
        RequestMakeLatest::Legacy => write!(f, "legacy"),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// Text describing the contents of the tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub body: Option<String>,
    /// If specified, a discussion of the specified category is created and linked to the release. The value must be a category that already exists in the repository. If there is already a discussion linked to the release, this parameter is ignored. For more information, see "[Managing categories for discussions in your repository](https://docs.github.com/discussions/managing-discussions-for-your-community/managing-categories-for-discussions-in-your-repository)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub discussion_category_name: Option<String>,
    /// `true` makes the release a draft, and `false` publishes the release.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub draft: Option<bool>,
    /// Specifies whether this release should be set as the latest release for the repository. Drafts and prereleases cannot be set as latest. Defaults to `true` for newly published releases. `legacy` specifies that the latest release should be determined based on the release creation date and higher semantic version.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub make_latest: Option<RequestMakeLatest>,
    /// The name of the release.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub name: Option<String>,
    /// `true` to identify the release as a prerelease, `false` to identify the release as a full release.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub prerelease: Option<bool>,
    /// The name of the tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub tag_name: Option<String>,
    /// Specifies the commitish value that determines where the Git tag is created from. Can be any branch or commit SHA. Unused if the Git tag already exists. Default: the repository's default branch.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub target_commitish: Option<String>,
  }
}

pub mod list_release_assets {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<ReleaseAsset>;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
  }
}

pub mod upload_release_asset {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = ReleaseAsset;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub label: Option<String>,
  }
}

pub mod get_branch_rules {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<RepositoryRuleDetailed>;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
  }
}

pub mod get_repo_rulesets {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<RepositoryRuleset>;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
    /// Include rulesets configured at higher levels that apply to this repository
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub includes_parents: Option<bool>,
  }
}

pub mod create_repo_ruleset {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = RepositoryRuleset;

  #[allow(clippy::large_enum_variant)]
  /// The target of the ruleset.
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum RequestTarget {
    #[serde(rename = "branch")]
    Branch,
    #[serde(rename = "tag")]
    Tag,
  }

  impl std::fmt::Display for RequestTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match self {
        RequestTarget::Branch => write!(f, "branch"),
        RequestTarget::Tag => write!(f, "tag"),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The actors that can bypass the rules in this ruleset
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub bypass_actors: Option<Vec<RepositoryRulesetBypassActor>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub conditions: Option<RepositoryRulesetConditions>,
    pub enforcement: RepositoryRuleEnforcement,
    /// The name of the ruleset.
    pub name: String,
    /// An array of rules within the ruleset.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub rules: Option<Vec<RepositoryRule>>,
    /// The target of the ruleset.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub target: Option<RequestTarget>,
  }
}

pub mod get_repo_rule_suites {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = RuleSuites;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The name of the ref. Cannot contain wildcard characters. When specified, only rule evaluations triggered for this ref will be returned.
    #[serde(rename = "ref")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub ref_: Option<String>,
    /// The time period to filter by.
    ///
    /// For example, `day` will filter for rule suites that occurred in the past 24 hours, and `week` will filter for insights that occurred in the past 7 days (168 hours).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub time_period: Option<parameters::TimePeriod>,
    /// The handle for the GitHub user account to filter on. When specified, only rule evaluations triggered by this actor will be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub actor_name: Option<String>,
    /// The rule results to filter on. When specified, only suites with this result will be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub rule_suite_result: Option<parameters::RuleSuiteResult>,
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
  }
}

pub mod get_repo_rule_suite {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = RuleSuite;
}

pub mod get_repo_ruleset {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = RepositoryRuleset;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// Include rulesets configured at higher levels that apply to this repository
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub includes_parents: Option<bool>,
  }
}

pub mod update_repo_ruleset {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = RepositoryRuleset;

  #[allow(clippy::large_enum_variant)]
  /// The target of the ruleset.
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum RequestTarget {
    #[serde(rename = "branch")]
    Branch,
    #[serde(rename = "tag")]
    Tag,
  }

  impl std::fmt::Display for RequestTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match self {
        RequestTarget::Branch => write!(f, "branch"),
        RequestTarget::Tag => write!(f, "tag"),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The actors that can bypass the rules in this ruleset
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub bypass_actors: Option<Vec<RepositoryRulesetBypassActor>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub conditions: Option<RepositoryRulesetConditions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub enforcement: Option<RepositoryRuleEnforcement>,
    /// The name of the ruleset.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub name: Option<String>,
    /// An array of rules within the ruleset.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub rules: Option<Vec<RepositoryRule>>,
    /// The target of the ruleset.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub target: Option<RequestTarget>,
  }
}

pub mod get_code_frequency_stats {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Vec<i64>>;
}

pub mod get_commit_activity_stats {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<CommitActivity>;
}

pub mod get_contributors_stats {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<ContributorActivity>;
}

pub mod get_participation_stats {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = ParticipationStats;
}

pub mod get_punch_card_stats {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Vec<i64>>;
}

pub mod create_commit_status {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Status;

  #[allow(clippy::large_enum_variant)]
  /// The state of the status.
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum RequestState {
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "failure")]
    Failure,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "success")]
    Success,
  }

  impl std::fmt::Display for RequestState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match self {
        RequestState::Error => write!(f, "error"),
        RequestState::Failure => write!(f, "failure"),
        RequestState::Pending => write!(f, "pending"),
        RequestState::Success => write!(f, "success"),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// A string label to differentiate this status from the status of other systems. This field is case-insensitive.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub context: Option<String>,
    /// A short description of the status.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub description: Option<String>,
    /// The state of the status.
    pub state: RequestState,
    /// The target URL to associate with this status. This URL will be linked from the GitHub UI to allow users to easily see the source of the status.  
    /// For example, if your continuous integration system is posting build status, you would want to provide the deep link for the build output for this specific SHA:  
    /// `http://ci.example.com/user/repo/build/sha`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub target_url: Option<String>,
  }
}

pub mod list_tags {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Tag>;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
  }
}

pub mod list_tag_protection {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<TagProtection>;
}

pub mod create_tag_protection {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = TagProtection;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// An optional glob pattern to match against when enforcing tag protection.
    pub pattern: String,
  }
}

pub mod list_teams {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Team>;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
  }
}

pub mod get_all_topics {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Topic;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
  }
}

pub mod replace_all_topics {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Topic;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// An array of topics to add to the repository. Pass one or more topics to _replace_ the set of existing topics. Send an empty array (`[]`) to clear all topics from the repository. **Note:** Topic `names` cannot contain uppercase letters.
    pub names: Vec<String>,
  }
}

pub mod get_clones {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = CloneTraffic;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The time frame to display results for.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per: Option<parameters::Per>,
  }
}

pub mod get_top_paths {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<ContentTraffic>;
}

pub mod get_top_referrers {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<ReferrerTraffic>;
}

pub mod get_views {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = ViewTraffic;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The time frame to display results for.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per: Option<parameters::Per>,
  }
}

pub mod transfer {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = MinimalRepository;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The new name to be given to the repository.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub new_name: Option<String>,
    /// The username or organization name the repository will be transferred to.
    pub new_owner: String,
    /// ID of the team or teams to add to the repository. Teams can only be added to organization-owned repositories.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub team_ids: Option<Vec<i64>>,
  }
}

pub mod create_using_template {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = FullRepository;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// A short description of the new repository.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub description: Option<String>,
    /// Set to `true` to include the directory structure and files from all branches in the template repository, and not just the default branch. Default: `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub include_all_branches: Option<bool>,
    /// The name of the new repository.
    pub name: String,
    /// The organization or person who will own the new repository. To create a new repository in an organization, the authenticated user must be a member of the specified organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub owner: Option<String>,
    /// Either `true` to create a new private repository or `false` to create a new public one.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub private: Option<bool>,
  }
}

pub mod list_public {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<MinimalRepository>;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// A repository ID. Only return repositories with an ID greater than this ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub since: Option<i64>,
  }
}

pub mod list_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Repository>;

  #[allow(clippy::large_enum_variant)]
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QueryVisibility {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "private")]
    Private,
  }

  impl std::fmt::Display for QueryVisibility {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match self {
        QueryVisibility::All => write!(f, "all"),
        QueryVisibility::Public => write!(f, "public"),
        QueryVisibility::Private => write!(f, "private"),
      }
    }
  }

  #[allow(clippy::large_enum_variant)]
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QueryType {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "owner")]
    Owner,
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "member")]
    Member,
  }

  impl std::fmt::Display for QueryType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match self {
        QueryType::All => write!(f, "all"),
        QueryType::Owner => write!(f, "owner"),
        QueryType::Public => write!(f, "public"),
        QueryType::Private => write!(f, "private"),
        QueryType::Member => write!(f, "member"),
      }
    }
  }

  #[allow(clippy::large_enum_variant)]
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QuerySort {
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "updated")]
    Updated,
    #[serde(rename = "pushed")]
    Pushed,
    #[serde(rename = "full_name")]
    FullName,
  }

  impl std::fmt::Display for QuerySort {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match self {
        QuerySort::Created => write!(f, "created"),
        QuerySort::Updated => write!(f, "updated"),
        QuerySort::Pushed => write!(f, "pushed"),
        QuerySort::FullName => write!(f, "full_name"),
      }
    }
  }

  #[allow(clippy::large_enum_variant)]
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QueryDirection {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
  }

  impl std::fmt::Display for QueryDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match self {
        QueryDirection::Asc => write!(f, "asc"),
        QueryDirection::Desc => write!(f, "desc"),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// Limit results to repositories with the specified visibility.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub visibility: Option<QueryVisibility>,
    /// Comma-separated list of values. Can include:  
    ///  * `owner`: Repositories that are owned by the authenticated user.  
    ///  * `collaborator`: Repositories that the user has been added to as a collaborator.  
    ///  * `organization_member`: Repositories that the user has access to through being a member of an organization. This includes every repository on every team that the user is on.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub affiliation: Option<String>,
    /// Limit results to repositories of the specified type. Will cause a `422` error if used in the same request as **visibility** or **affiliation**.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub type_: Option<QueryType>,
    /// The property to sort the results by.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub sort: Option<QuerySort>,
    /// The order to sort by. Default: `asc` when using `full_name`, otherwise `desc`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub direction: Option<QueryDirection>,
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
    /// Only show repositories updated after the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub since: Option<String>,
    /// Only show repositories updated before the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub before: Option<String>,
  }
}

pub mod create_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = FullRepository;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// Whether to allow Auto-merge to be used on pull requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub allow_auto_merge: Option<bool>,
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
    /// Whether the repository is initialized with a minimal README.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub auto_init: Option<bool>,
    /// Whether to delete head branches when pull requests are merged
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub delete_branch_on_merge: Option<bool>,
    /// A short description of the repository.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub description: Option<String>,
    /// The desired language or platform to apply to the .gitignore.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub gitignore_template: Option<String>,
    /// Whether discussions are enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub has_discussions: Option<bool>,
    /// Whether downloads are enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub has_downloads: Option<bool>,
    /// Whether issues are enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub has_issues: Option<bool>,
    /// Whether projects are enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub has_projects: Option<bool>,
    /// Whether the wiki is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub has_wiki: Option<bool>,
    /// A URL with more information about the repository.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub homepage: Option<String>,
    /// Whether this repository acts as a template that can be used to generate new repositories.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub is_template: Option<bool>,
    /// The license keyword of the open source license for this repository.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub license_template: Option<String>,
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
    /// The name of the repository.
    pub name: String,
    /// Whether the repository is private.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub private: Option<bool>,
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
    /// The id of the team that will be granted access to this repository. This is only valid when creating a repository in an organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub team_id: Option<i64>,
  }
}

pub mod list_invitations_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<RepositoryInvitation>;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
  }
}

pub mod list_for_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<MinimalRepository>;

  #[allow(clippy::large_enum_variant)]
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QueryType {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "owner")]
    Owner,
    #[serde(rename = "member")]
    Member,
  }

  impl std::fmt::Display for QueryType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match self {
        QueryType::All => write!(f, "all"),
        QueryType::Owner => write!(f, "owner"),
        QueryType::Member => write!(f, "member"),
      }
    }
  }

  #[allow(clippy::large_enum_variant)]
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QuerySort {
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "updated")]
    Updated,
    #[serde(rename = "pushed")]
    Pushed,
    #[serde(rename = "full_name")]
    FullName,
  }

  impl std::fmt::Display for QuerySort {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match self {
        QuerySort::Created => write!(f, "created"),
        QuerySort::Updated => write!(f, "updated"),
        QuerySort::Pushed => write!(f, "pushed"),
        QuerySort::FullName => write!(f, "full_name"),
      }
    }
  }

  #[allow(clippy::large_enum_variant)]
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QueryDirection {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
  }

  impl std::fmt::Display for QueryDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match self {
        QueryDirection::Asc => write!(f, "asc"),
        QueryDirection::Desc => write!(f, "desc"),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// Limit results to repositories of the specified type.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub type_: Option<QueryType>,
    /// The property to sort the results by.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub sort: Option<QuerySort>,
    /// The order to sort by. Default: `asc` when using `full_name`, otherwise `desc`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub direction: Option<QueryDirection>,
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
  }
}

/// Interact with GitHub Repos.
pub struct GitHubReposAPI {
  config: SharedAPIConfig,
}

impl GitHubReposAPI {
  pub fn new(config: &SharedAPIConfig) -> Self {
    Self {
      config: config.clone(),
    }
  }

  /// **List organization repositories**
  ///
  /// Lists repositories for the specified organization.
  ///
  /// **Note:** In order to see the `security_and_analysis` block for a repository you must have admin permissions for the repository or be an owner or security manager for the organization that owns the repository. For more information, see "[Managing security managers in your organization](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/managing-security-managers-in-your-organization)."
  ///
  /// *Documentation*: [https://docs.github.com/rest/repos/repos#list-organization-repositories](https://docs.github.com/rest/repos/repos#list-organization-repositories)
  pub fn list_for_org(
    &self,
    org: impl Into<String>,
  ) -> Request<(), list_for_org::Query, list_for_org::Response> {
    let org = org.into();
    let url = format!("/orgs/{org}/repos");

    Request::<(), list_for_org::Query, list_for_org::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create an organization repository**
  ///
  /// Creates a new repository in the specified organization. The authenticated user must be a member of the organization.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `public_repo` or `repo` scope to create a public repository, and `repo` scope to create a private repository.
  ///
  /// *Documentation*: [https://docs.github.com/rest/repos/repos#create-an-organization-repository](https://docs.github.com/rest/repos/repos#create-an-organization-repository)
  pub fn create_in_org(
    &self,
    org: impl Into<String>,
  ) -> Request<create_in_org::Request, (), create_in_org::Response> {
    let org = org.into();
    let url = format!("/orgs/{org}/repos");

    Request::<create_in_org::Request, (), create_in_org::Response>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Get all organization repository rulesets**
  ///
  /// Get all the repository rulesets for an organization.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/rules#get-all-organization-repository-rulesets](https://docs.github.com/rest/orgs/rules#get-all-organization-repository-rulesets)
  pub fn get_org_rulesets(
    &self,
    org: impl Into<String>,
  ) -> Request<(), get_org_rulesets::Query, get_org_rulesets::Response> {
    let org = org.into();
    let url = format!("/orgs/{org}/rulesets");

    Request::<(), get_org_rulesets::Query, get_org_rulesets::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create an organization repository ruleset**
  ///
  /// Create a repository ruleset for an organization.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/rules#create-an-organization-repository-ruleset](https://docs.github.com/rest/orgs/rules#create-an-organization-repository-ruleset)
  pub fn create_org_ruleset(
    &self,
    org: impl Into<String>,
  ) -> Request<create_org_ruleset::Request, (), create_org_ruleset::Response> {
    let org = org.into();
    let url = format!("/orgs/{org}/rulesets");

    Request::<create_org_ruleset::Request, (), create_org_ruleset::Response>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **List organization rule suites**
  ///
  /// Lists suites of rule evaluations at the organization level.
  /// For more information, see "[Managing rulesets for repositories in your organization](https://docs.github.com/organizations/managing-organization-settings/managing-rulesets-for-repositories-in-your-organization#viewing-insights-for-rulesets)."
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/rule-suites#list-organization-rule-suites](https://docs.github.com/rest/orgs/rule-suites#list-organization-rule-suites)
  pub fn get_org_rule_suites(
    &self,
    org: impl Into<String>,
  ) -> Request<(), get_org_rule_suites::Query, get_org_rule_suites::Response> {
    let org = org.into();
    let url = format!("/orgs/{org}/rulesets/rule-suites");

    Request::<(), get_org_rule_suites::Query, get_org_rule_suites::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get an organization rule suite**
  ///
  /// Gets information about a suite of rule evaluations from within an organization.
  /// For more information, see "[Managing rulesets for repositories in your organization](https://docs.github.com/organizations/managing-organization-settings/managing-rulesets-for-repositories-in-your-organization#viewing-insights-for-rulesets)."
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/rule-suites#get-an-organization-rule-suite](https://docs.github.com/rest/orgs/rule-suites#get-an-organization-rule-suite)
  pub fn get_org_rule_suite(
    &self,
    org: impl Into<String>,
    rule_suite_id: impl Into<i64>,
  ) -> Request<(), (), get_org_rule_suite::Response> {
    let org = org.into();
    let rule_suite_id = rule_suite_id.into();
    let url = format!("/orgs/{org}/rulesets/rule-suites/{rule_suite_id}");

    Request::<(), (), get_org_rule_suite::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get an organization repository ruleset**
  ///
  /// Get a repository ruleset for an organization.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/rules#get-an-organization-repository-ruleset](https://docs.github.com/rest/orgs/rules#get-an-organization-repository-ruleset)
  pub fn get_org_ruleset(
    &self,
    org: impl Into<String>,
    ruleset_id: impl Into<i64>,
  ) -> Request<(), (), get_org_ruleset::Response> {
    let org = org.into();
    let ruleset_id = ruleset_id.into();
    let url = format!("/orgs/{org}/rulesets/{ruleset_id}");

    Request::<(), (), get_org_ruleset::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Update an organization repository ruleset**
  ///
  /// Update a ruleset for an organization.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/rules#update-an-organization-repository-ruleset](https://docs.github.com/rest/orgs/rules#update-an-organization-repository-ruleset)
  pub fn update_org_ruleset(
    &self,
    org: impl Into<String>,
    ruleset_id: impl Into<i64>,
  ) -> Request<update_org_ruleset::Request, (), update_org_ruleset::Response> {
    let org = org.into();
    let ruleset_id = ruleset_id.into();
    let url = format!("/orgs/{org}/rulesets/{ruleset_id}");

    Request::<update_org_ruleset::Request, (), update_org_ruleset::Response>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **Delete an organization repository ruleset**
  ///
  /// Delete a ruleset for an organization.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/rules#delete-an-organization-repository-ruleset](https://docs.github.com/rest/orgs/rules#delete-an-organization-repository-ruleset)
  pub fn delete_org_ruleset(
    &self,
    org: impl Into<String>,
    ruleset_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let ruleset_id = ruleset_id.into();
    let url = format!("/orgs/{org}/rulesets/{ruleset_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Get a repository**
  ///
  /// The `parent` and `source` objects are present when the repository is a fork. `parent` is the repository this repository was forked from, `source` is the ultimate source for the network.
  ///
  /// **Note:** In order to see the `security_and_analysis` block for a repository you must have admin permissions for the repository or be an owner or security manager for the organization that owns the repository. For more information, see "[Managing security managers in your organization](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/managing-security-managers-in-your-organization)."
  ///
  /// *Documentation*: [https://docs.github.com/rest/repos/repos#get-a-repository](https://docs.github.com/rest/repos/repos#get-a-repository)
  pub fn get(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), (), get::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}");

    Request::<(), (), get::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Update a repository**
  ///
  /// **Note**: To edit a repository's topics, use the [Replace all repository topics](https://docs.github.com/rest/repos/repos#replace-all-repository-topics) endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/repos/repos#update-a-repository](https://docs.github.com/rest/repos/repos#update-a-repository)
  pub fn update(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<update::Request, (), update::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}");

    Request::<update::Request, (), update::Response>::builder(&self.config)
      .patch(url)
      .build()
  }

  /// **Delete a repository**
  ///
  /// Deleting a repository requires admin access.
  ///
  /// If an organization owner has configured the organization to prevent members from deleting organization-owned
  /// repositories, you will get a `403 Forbidden` response.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `delete_repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/repos/repos#delete-a-repository](https://docs.github.com/rest/repos/repos#delete-a-repository)
  pub fn delete(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List repository activities**
  ///
  /// Lists a detailed history of changes to a repository, such as pushes, merges, force pushes, and branch changes, and associates these changes with commits and users.
  ///
  /// For more information about viewing repository activity,
  /// see "[Viewing activity and data for your repository](https://docs.github.com/repositories/viewing-activity-and-data-for-your-repository)."
  ///
  /// *Documentation*: [https://docs.github.com/rest/repos/repos#list-repository-activities](https://docs.github.com/rest/repos/repos#list-repository-activities)
  pub fn list_activities(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), list_activities::Query, list_activities::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/activity");

    Request::<(), list_activities::Query, list_activities::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get all autolinks of a repository**
  ///
  /// Gets all autolinks that are configured for a repository.
  ///
  /// Information about autolinks are only available to repository administrators.
  ///
  /// *Documentation*: [https://docs.github.com/rest/repos/autolinks#get-all-autolinks-of-a-repository](https://docs.github.com/rest/repos/autolinks#get-all-autolinks-of-a-repository)
  pub fn list_autolinks(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), (), list_autolinks::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/autolinks");

    Request::<(), (), list_autolinks::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create an autolink reference for a repository**
  ///
  /// Users with admin access to the repository can create an autolink.
  ///
  /// *Documentation*: [https://docs.github.com/rest/repos/autolinks#create-an-autolink-reference-for-a-repository](https://docs.github.com/rest/repos/autolinks#create-an-autolink-reference-for-a-repository)
  pub fn create_autolink(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<create_autolink::Request, (), create_autolink::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/autolinks");

    Request::<create_autolink::Request, (), create_autolink::Response>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Get an autolink reference of a repository**
  ///
  /// This returns a single autolink reference by ID that was configured for the given repository.
  ///
  /// Information about autolinks are only available to repository administrators.
  ///
  /// *Documentation*: [https://docs.github.com/rest/repos/autolinks#get-an-autolink-reference-of-a-repository](https://docs.github.com/rest/repos/autolinks#get-an-autolink-reference-of-a-repository)
  pub fn get_autolink(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    autolink_id: impl Into<i64>,
  ) -> Request<(), (), get_autolink::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let autolink_id = autolink_id.into();
    let url = format!("/repos/{owner}/{repo}/autolinks/{autolink_id}");

    Request::<(), (), get_autolink::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Delete an autolink reference from a repository**
  ///
  /// This deletes a single autolink reference by ID that was configured for the given repository.
  ///
  /// Information about autolinks are only available to repository administrators.
  ///
  /// *Documentation*: [https://docs.github.com/rest/repos/autolinks#delete-an-autolink-reference-from-a-repository](https://docs.github.com/rest/repos/autolinks#delete-an-autolink-reference-from-a-repository)
  pub fn delete_autolink(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    autolink_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let autolink_id = autolink_id.into();
    let url = format!("/repos/{owner}/{repo}/autolinks/{autolink_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Check if automated security fixes are enabled for a repository**
  ///
  /// Shows whether automated security fixes are enabled, disabled or paused for a repository. The authenticated user must have admin read access to the repository. For more information, see "[Configuring automated security fixes](https://docs.github.com/articles/configuring-automated-security-fixes)".
  ///
  /// *Documentation*: [https://docs.github.com/rest/repos/repos#check-if-automated-security-fixes-are-enabled-for-a-repository](https://docs.github.com/rest/repos/repos#check-if-automated-security-fixes-are-enabled-for-a-repository)
  pub fn check_automated_security_fixes(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), (), check_automated_security_fixes::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/automated-security-fixes");

    Request::<(), (), check_automated_security_fixes::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Enable automated security fixes**
  ///
  /// Enables automated security fixes for a repository. The authenticated user must have admin access to the repository. For more information, see "[Configuring automated security fixes](https://docs.github.com/articles/configuring-automated-security-fixes)".
  ///
  /// *Documentation*: [https://docs.github.com/rest/repos/repos#enable-automated-security-fixes](https://docs.github.com/rest/repos/repos#enable-automated-security-fixes)
  pub fn enable_automated_security_fixes(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/automated-security-fixes");

    NoContentRequest::<(), ()>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **Disable automated security fixes**
  ///
  /// Disables automated security fixes for a repository. The authenticated user must have admin access to the repository. For more information, see "[Configuring automated security fixes](https://docs.github.com/articles/configuring-automated-security-fixes)".
  ///
  /// *Documentation*: [https://docs.github.com/rest/repos/repos#disable-automated-security-fixes](https://docs.github.com/rest/repos/repos#disable-automated-security-fixes)
  pub fn disable_automated_security_fixes(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/automated-security-fixes");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List branches**
  ///
  ///
  /// *Documentation*: [https://docs.github.com/rest/branches/branches#list-branches](https://docs.github.com/rest/branches/branches#list-branches)
  pub fn list_branches(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), list_branches::Query, list_branches::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/branches");

    Request::<(), list_branches::Query, list_branches::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get a branch**
  ///
  ///
  /// *Documentation*: [https://docs.github.com/rest/branches/branches#get-a-branch](https://docs.github.com/rest/branches/branches#get-a-branch)
  pub fn get_branch(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    branch: impl Into<String>,
  ) -> Request<(), (), get_branch::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let branch = branch.into();
    let url = format!("/repos/{owner}/{repo}/branches/{branch}");

    Request::<(), (), get_branch::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get branch protection**
  ///
  /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
  ///
  /// *Documentation*: [https://docs.github.com/rest/branches/branch-protection#get-branch-protection](https://docs.github.com/rest/branches/branch-protection#get-branch-protection)
  pub fn get_branch_protection(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    branch: impl Into<String>,
  ) -> Request<(), (), get_branch_protection::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let branch = branch.into();
    let url = format!("/repos/{owner}/{repo}/branches/{branch}/protection");

    Request::<(), (), get_branch_protection::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Update branch protection**
  ///
  /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
  ///
  /// Protecting a branch requires admin or owner permissions to the repository.
  ///
  /// **Note**: Passing new arrays of `users` and `teams` replaces their previous values.
  ///
  /// **Note**: The list of users, apps, and teams in total is limited to 100 items.
  ///
  /// *Documentation*: [https://docs.github.com/rest/branches/branch-protection#update-branch-protection](https://docs.github.com/rest/branches/branch-protection#update-branch-protection)
  pub fn update_branch_protection(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    branch: impl Into<String>,
  ) -> Request<update_branch_protection::Request, (), update_branch_protection::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let branch = branch.into();
    let url = format!("/repos/{owner}/{repo}/branches/{branch}/protection");

    Request::<update_branch_protection::Request, (), update_branch_protection::Response>::builder(
      &self.config,
    )
    .put(url)
    .build()
  }

  /// **Delete branch protection**
  ///
  /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
  ///
  /// *Documentation*: [https://docs.github.com/rest/branches/branch-protection#delete-branch-protection](https://docs.github.com/rest/branches/branch-protection#delete-branch-protection)
  pub fn delete_branch_protection(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    branch: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let branch = branch.into();
    let url = format!("/repos/{owner}/{repo}/branches/{branch}/protection");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Get admin branch protection**
  ///
  /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
  ///
  /// *Documentation*: [https://docs.github.com/rest/branches/branch-protection#get-admin-branch-protection](https://docs.github.com/rest/branches/branch-protection#get-admin-branch-protection)
  pub fn get_admin_branch_protection(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    branch: impl Into<String>,
  ) -> Request<(), (), get_admin_branch_protection::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let branch = branch.into();
    let url = format!("/repos/{owner}/{repo}/branches/{branch}/protection/enforce_admins");

    Request::<(), (), get_admin_branch_protection::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Set admin branch protection**
  ///
  /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
  ///
  /// Adding admin enforcement requires admin or owner permissions to the repository and branch protection to be enabled.
  ///
  /// *Documentation*: [https://docs.github.com/rest/branches/branch-protection#set-admin-branch-protection](https://docs.github.com/rest/branches/branch-protection#set-admin-branch-protection)
  pub fn set_admin_branch_protection(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    branch: impl Into<String>,
  ) -> Request<(), (), set_admin_branch_protection::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let branch = branch.into();
    let url = format!("/repos/{owner}/{repo}/branches/{branch}/protection/enforce_admins");

    Request::<(), (), set_admin_branch_protection::Response>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Delete admin branch protection**
  ///
  /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
  ///
  /// Removing admin enforcement requires admin or owner permissions to the repository and branch protection to be enabled.
  ///
  /// *Documentation*: [https://docs.github.com/rest/branches/branch-protection#delete-admin-branch-protection](https://docs.github.com/rest/branches/branch-protection#delete-admin-branch-protection)
  pub fn delete_admin_branch_protection(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    branch: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let branch = branch.into();
    let url = format!("/repos/{owner}/{repo}/branches/{branch}/protection/enforce_admins");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Get pull request review protection**
  ///
  /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
  ///
  /// *Documentation*: [https://docs.github.com/rest/branches/branch-protection#get-pull-request-review-protection](https://docs.github.com/rest/branches/branch-protection#get-pull-request-review-protection)
  pub fn get_pull_request_review_protection(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    branch: impl Into<String>,
  ) -> Request<(), (), get_pull_request_review_protection::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let branch = branch.into();
    let url =
      format!("/repos/{owner}/{repo}/branches/{branch}/protection/required_pull_request_reviews");

    Request::<(), (), get_pull_request_review_protection::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Update pull request review protection**
  ///
  /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
  ///
  /// Updating pull request review enforcement requires admin or owner permissions to the repository and branch protection to be enabled.
  ///
  /// **Note**: Passing new arrays of `users` and `teams` replaces their previous values.
  ///
  /// *Documentation*: [https://docs.github.com/rest/branches/branch-protection#update-pull-request-review-protection](https://docs.github.com/rest/branches/branch-protection#update-pull-request-review-protection)
  pub fn update_pull_request_review_protection(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    branch: impl Into<String>,
  ) -> Request<
    update_pull_request_review_protection::Request,
    (),
    update_pull_request_review_protection::Response,
  > {
    let owner = owner.into();
    let repo = repo.into();
    let branch = branch.into();
    let url =
      format!("/repos/{owner}/{repo}/branches/{branch}/protection/required_pull_request_reviews");

    Request::<
      update_pull_request_review_protection::Request,
      (),
      update_pull_request_review_protection::Response,
    >::builder(&self.config)
    .patch(url)
    .build()
  }

  /// **Delete pull request review protection**
  ///
  /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
  ///
  /// *Documentation*: [https://docs.github.com/rest/branches/branch-protection#delete-pull-request-review-protection](https://docs.github.com/rest/branches/branch-protection#delete-pull-request-review-protection)
  pub fn delete_pull_request_review_protection(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    branch: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let branch = branch.into();
    let url =
      format!("/repos/{owner}/{repo}/branches/{branch}/protection/required_pull_request_reviews");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Get commit signature protection**
  ///
  /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
  ///
  /// When authenticated with admin or owner permissions to the repository, you can use this endpoint to check whether a branch requires signed commits. An enabled status of `true` indicates you must sign commits on this branch. For more information, see [Signing commits with GPG](https://docs.github.com/articles/signing-commits-with-gpg) in GitHub Help.
  ///
  /// **Note**: You must enable branch protection to require signed commits.
  ///
  /// *Documentation*: [https://docs.github.com/rest/branches/branch-protection#get-commit-signature-protection](https://docs.github.com/rest/branches/branch-protection#get-commit-signature-protection)
  pub fn get_commit_signature_protection(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    branch: impl Into<String>,
  ) -> Request<(), (), get_commit_signature_protection::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let branch = branch.into();
    let url = format!("/repos/{owner}/{repo}/branches/{branch}/protection/required_signatures");

    Request::<(), (), get_commit_signature_protection::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create commit signature protection**
  ///
  /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
  ///
  /// When authenticated with admin or owner permissions to the repository, you can use this endpoint to require signed commits on a branch. You must enable branch protection to require signed commits.
  ///
  /// *Documentation*: [https://docs.github.com/rest/branches/branch-protection#create-commit-signature-protection](https://docs.github.com/rest/branches/branch-protection#create-commit-signature-protection)
  pub fn create_commit_signature_protection(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    branch: impl Into<String>,
  ) -> Request<(), (), create_commit_signature_protection::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let branch = branch.into();
    let url = format!("/repos/{owner}/{repo}/branches/{branch}/protection/required_signatures");

    Request::<(), (), create_commit_signature_protection::Response>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Delete commit signature protection**
  ///
  /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
  ///
  /// When authenticated with admin or owner permissions to the repository, you can use this endpoint to disable required signed commits on a branch. You must enable branch protection to require signed commits.
  ///
  /// *Documentation*: [https://docs.github.com/rest/branches/branch-protection#delete-commit-signature-protection](https://docs.github.com/rest/branches/branch-protection#delete-commit-signature-protection)
  pub fn delete_commit_signature_protection(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    branch: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let branch = branch.into();
    let url = format!("/repos/{owner}/{repo}/branches/{branch}/protection/required_signatures");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Get status checks protection**
  ///
  /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
  ///
  /// *Documentation*: [https://docs.github.com/rest/branches/branch-protection#get-status-checks-protection](https://docs.github.com/rest/branches/branch-protection#get-status-checks-protection)
  pub fn get_status_checks_protection(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    branch: impl Into<String>,
  ) -> Request<(), (), get_status_checks_protection::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let branch = branch.into();
    let url = format!("/repos/{owner}/{repo}/branches/{branch}/protection/required_status_checks");

    Request::<(), (), get_status_checks_protection::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Update status check protection**
  ///
  /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
  ///
  /// Updating required status checks requires admin or owner permissions to the repository and branch protection to be enabled.
  ///
  /// *Documentation*: [https://docs.github.com/rest/branches/branch-protection#update-status-check-protection](https://docs.github.com/rest/branches/branch-protection#update-status-check-protection)
  pub fn update_status_check_protection(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    branch: impl Into<String>,
  ) -> Request<update_status_check_protection::Request, (), update_status_check_protection::Response>
  {
    let owner = owner.into();
    let repo = repo.into();
    let branch = branch.into();
    let url = format!("/repos/{owner}/{repo}/branches/{branch}/protection/required_status_checks");

    Request::<update_status_check_protection::Request, (), update_status_check_protection::Response>::builder(&self.config)
      .patch(url)
      .build()
  }

  /// **Remove status check protection**
  ///
  /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
  ///
  /// *Documentation*: [https://docs.github.com/rest/branches/branch-protection#remove-status-check-protection](https://docs.github.com/rest/branches/branch-protection#remove-status-check-protection)
  pub fn remove_status_check_protection(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    branch: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let branch = branch.into();
    let url = format!("/repos/{owner}/{repo}/branches/{branch}/protection/required_status_checks");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Get all status check contexts**
  ///
  /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
  ///
  /// *Documentation*: [https://docs.github.com/rest/branches/branch-protection#get-all-status-check-contexts](https://docs.github.com/rest/branches/branch-protection#get-all-status-check-contexts)
  pub fn get_all_status_check_contexts(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    branch: impl Into<String>,
  ) -> Request<(), (), get_all_status_check_contexts::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let branch = branch.into();
    let url =
      format!("/repos/{owner}/{repo}/branches/{branch}/protection/required_status_checks/contexts");

    Request::<(), (), get_all_status_check_contexts::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Add status check contexts**
  ///
  /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
  ///
  /// *Documentation*: [https://docs.github.com/rest/branches/branch-protection#add-status-check-contexts](https://docs.github.com/rest/branches/branch-protection#add-status-check-contexts)
  pub fn add_status_check_contexts(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    branch: impl Into<String>,
  ) -> Request<add_status_check_contexts::Request, (), add_status_check_contexts::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let branch = branch.into();
    let url =
      format!("/repos/{owner}/{repo}/branches/{branch}/protection/required_status_checks/contexts");

    Request::<add_status_check_contexts::Request, (), add_status_check_contexts::Response>::builder(
      &self.config,
    )
    .post(url)
    .build()
  }

  /// **Set status check contexts**
  ///
  /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
  ///
  /// *Documentation*: [https://docs.github.com/rest/branches/branch-protection#set-status-check-contexts](https://docs.github.com/rest/branches/branch-protection#set-status-check-contexts)
  pub fn set_status_check_contexts(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    branch: impl Into<String>,
  ) -> Request<set_status_check_contexts::Request, (), set_status_check_contexts::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let branch = branch.into();
    let url =
      format!("/repos/{owner}/{repo}/branches/{branch}/protection/required_status_checks/contexts");

    Request::<set_status_check_contexts::Request, (), set_status_check_contexts::Response>::builder(
      &self.config,
    )
    .put(url)
    .build()
  }

  /// **Remove status check contexts**
  ///
  /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
  ///
  /// *Documentation*: [https://docs.github.com/rest/branches/branch-protection#remove-status-check-contexts](https://docs.github.com/rest/branches/branch-protection#remove-status-check-contexts)
  pub fn remove_status_check_contexts(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    branch: impl Into<String>,
  ) -> Request<remove_status_check_contexts::Request, (), remove_status_check_contexts::Response>
  {
    let owner = owner.into();
    let repo = repo.into();
    let branch = branch.into();
    let url =
      format!("/repos/{owner}/{repo}/branches/{branch}/protection/required_status_checks/contexts");

    Request::<remove_status_check_contexts::Request, (), remove_status_check_contexts::Response>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Get access restrictions**
  ///
  /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
  ///
  /// Lists who has access to this protected branch.
  ///
  /// **Note**: Users, apps, and teams `restrictions` are only available for organization-owned repositories.
  ///
  /// *Documentation*: [https://docs.github.com/rest/branches/branch-protection#get-access-restrictions](https://docs.github.com/rest/branches/branch-protection#get-access-restrictions)
  pub fn get_access_restrictions(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    branch: impl Into<String>,
  ) -> Request<(), (), get_access_restrictions::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let branch = branch.into();
    let url = format!("/repos/{owner}/{repo}/branches/{branch}/protection/restrictions");

    Request::<(), (), get_access_restrictions::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Delete access restrictions**
  ///
  /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
  ///
  /// Disables the ability to restrict who can push to this branch.
  ///
  /// *Documentation*: [https://docs.github.com/rest/branches/branch-protection#delete-access-restrictions](https://docs.github.com/rest/branches/branch-protection#delete-access-restrictions)
  pub fn delete_access_restrictions(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    branch: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let branch = branch.into();
    let url = format!("/repos/{owner}/{repo}/branches/{branch}/protection/restrictions");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Get apps with access to the protected branch**
  ///
  /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
  ///
  /// Lists the GitHub Apps that have push access to this branch. Only GitHub Apps that are installed on the repository and that have been granted write access to the repository contents can be added as authorized actors on a protected branch.
  ///
  /// *Documentation*: [https://docs.github.com/rest/branches/branch-protection#get-apps-with-access-to-the-protected-branch](https://docs.github.com/rest/branches/branch-protection#get-apps-with-access-to-the-protected-branch)
  pub fn get_apps_with_access_to_protected_branch(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    branch: impl Into<String>,
  ) -> Request<(), (), get_apps_with_access_to_protected_branch::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let branch = branch.into();
    let url = format!("/repos/{owner}/{repo}/branches/{branch}/protection/restrictions/apps");

    Request::<(), (), get_apps_with_access_to_protected_branch::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Add app access restrictions**
  ///
  /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
  ///
  /// Grants the specified apps push access for this branch. Only GitHub Apps that are installed on the repository and that have been granted write access to the repository contents can be added as authorized actors on a protected branch.
  ///
  /// *Documentation*: [https://docs.github.com/rest/branches/branch-protection#add-app-access-restrictions](https://docs.github.com/rest/branches/branch-protection#add-app-access-restrictions)
  pub fn add_app_access_restrictions(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    branch: impl Into<String>,
  ) -> Request<add_app_access_restrictions::Request, (), add_app_access_restrictions::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let branch = branch.into();
    let url = format!("/repos/{owner}/{repo}/branches/{branch}/protection/restrictions/apps");

    Request::<add_app_access_restrictions::Request, (), add_app_access_restrictions::Response>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Set app access restrictions**
  ///
  /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
  ///
  /// Replaces the list of apps that have push access to this branch. This removes all apps that previously had push access and grants push access to the new list of apps. Only GitHub Apps that are installed on the repository and that have been granted write access to the repository contents can be added as authorized actors on a protected branch.
  ///
  /// *Documentation*: [https://docs.github.com/rest/branches/branch-protection#set-app-access-restrictions](https://docs.github.com/rest/branches/branch-protection#set-app-access-restrictions)
  pub fn set_app_access_restrictions(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    branch: impl Into<String>,
  ) -> Request<set_app_access_restrictions::Request, (), set_app_access_restrictions::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let branch = branch.into();
    let url = format!("/repos/{owner}/{repo}/branches/{branch}/protection/restrictions/apps");

    Request::<set_app_access_restrictions::Request, (), set_app_access_restrictions::Response>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **Remove app access restrictions**
  ///
  /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
  ///
  /// Removes the ability of an app to push to this branch. Only GitHub Apps that are installed on the repository and that have been granted write access to the repository contents can be added as authorized actors on a protected branch.
  ///
  /// *Documentation*: [https://docs.github.com/rest/branches/branch-protection#remove-app-access-restrictions](https://docs.github.com/rest/branches/branch-protection#remove-app-access-restrictions)
  pub fn remove_app_access_restrictions(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    branch: impl Into<String>,
  ) -> Request<remove_app_access_restrictions::Request, (), remove_app_access_restrictions::Response>
  {
    let owner = owner.into();
    let repo = repo.into();
    let branch = branch.into();
    let url = format!("/repos/{owner}/{repo}/branches/{branch}/protection/restrictions/apps");

    Request::<remove_app_access_restrictions::Request, (), remove_app_access_restrictions::Response>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Get teams with access to the protected branch**
  ///
  /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
  ///
  /// Lists the teams who have push access to this branch. The list includes child teams.
  ///
  /// *Documentation*: [https://docs.github.com/rest/branches/branch-protection#get-teams-with-access-to-the-protected-branch](https://docs.github.com/rest/branches/branch-protection#get-teams-with-access-to-the-protected-branch)
  pub fn get_teams_with_access_to_protected_branch(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    branch: impl Into<String>,
  ) -> Request<(), (), get_teams_with_access_to_protected_branch::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let branch = branch.into();
    let url = format!("/repos/{owner}/{repo}/branches/{branch}/protection/restrictions/teams");

    Request::<(), (), get_teams_with_access_to_protected_branch::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Add team access restrictions**
  ///
  /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
  ///
  /// Grants the specified teams push access for this branch. You can also give push access to child teams.
  ///
  /// *Documentation*: [https://docs.github.com/rest/branches/branch-protection#add-team-access-restrictions](https://docs.github.com/rest/branches/branch-protection#add-team-access-restrictions)
  pub fn add_team_access_restrictions(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    branch: impl Into<String>,
  ) -> Request<add_team_access_restrictions::Request, (), add_team_access_restrictions::Response>
  {
    let owner = owner.into();
    let repo = repo.into();
    let branch = branch.into();
    let url = format!("/repos/{owner}/{repo}/branches/{branch}/protection/restrictions/teams");

    Request::<add_team_access_restrictions::Request, (), add_team_access_restrictions::Response>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Set team access restrictions**
  ///
  /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
  ///
  /// Replaces the list of teams that have push access to this branch. This removes all teams that previously had push access and grants push access to the new list of teams. Team restrictions include child teams.
  ///
  /// *Documentation*: [https://docs.github.com/rest/branches/branch-protection#set-team-access-restrictions](https://docs.github.com/rest/branches/branch-protection#set-team-access-restrictions)
  pub fn set_team_access_restrictions(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    branch: impl Into<String>,
  ) -> Request<set_team_access_restrictions::Request, (), set_team_access_restrictions::Response>
  {
    let owner = owner.into();
    let repo = repo.into();
    let branch = branch.into();
    let url = format!("/repos/{owner}/{repo}/branches/{branch}/protection/restrictions/teams");

    Request::<set_team_access_restrictions::Request, (), set_team_access_restrictions::Response>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **Remove team access restrictions**
  ///
  /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
  ///
  /// Removes the ability of a team to push to this branch. You can also remove push access for child teams.
  ///
  /// *Documentation*: [https://docs.github.com/rest/branches/branch-protection#remove-team-access-restrictions](https://docs.github.com/rest/branches/branch-protection#remove-team-access-restrictions)
  pub fn remove_team_access_restrictions(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    branch: impl Into<String>,
  ) -> Request<
    remove_team_access_restrictions::Request,
    (),
    remove_team_access_restrictions::Response,
  > {
    let owner = owner.into();
    let repo = repo.into();
    let branch = branch.into();
    let url = format!("/repos/{owner}/{repo}/branches/{branch}/protection/restrictions/teams");

    Request::<remove_team_access_restrictions::Request, (), remove_team_access_restrictions::Response>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Get users with access to the protected branch**
  ///
  /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
  ///
  /// Lists the people who have push access to this branch.
  ///
  /// *Documentation*: [https://docs.github.com/rest/branches/branch-protection#get-users-with-access-to-the-protected-branch](https://docs.github.com/rest/branches/branch-protection#get-users-with-access-to-the-protected-branch)
  pub fn get_users_with_access_to_protected_branch(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    branch: impl Into<String>,
  ) -> Request<(), (), get_users_with_access_to_protected_branch::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let branch = branch.into();
    let url = format!("/repos/{owner}/{repo}/branches/{branch}/protection/restrictions/users");

    Request::<(), (), get_users_with_access_to_protected_branch::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Add user access restrictions**
  ///
  /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
  ///
  /// Grants the specified people push access for this branch.
  ///
  /// | Type    | Description                                                                                                                   |
  /// | ------- | ----------------------------------------------------------------------------------------------------------------------------- |
  /// | `array` | Usernames for people who can have push access. **Note**: The list of users, apps, and teams in total is limited to 100 items. |
  ///
  /// *Documentation*: [https://docs.github.com/rest/branches/branch-protection#add-user-access-restrictions](https://docs.github.com/rest/branches/branch-protection#add-user-access-restrictions)
  pub fn add_user_access_restrictions(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    branch: impl Into<String>,
  ) -> Request<add_user_access_restrictions::Request, (), add_user_access_restrictions::Response>
  {
    let owner = owner.into();
    let repo = repo.into();
    let branch = branch.into();
    let url = format!("/repos/{owner}/{repo}/branches/{branch}/protection/restrictions/users");

    Request::<add_user_access_restrictions::Request, (), add_user_access_restrictions::Response>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Set user access restrictions**
  ///
  /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
  ///
  /// Replaces the list of people that have push access to this branch. This removes all people that previously had push access and grants push access to the new list of people.
  ///
  /// | Type    | Description                                                                                                                   |
  /// | ------- | ----------------------------------------------------------------------------------------------------------------------------- |
  /// | `array` | Usernames for people who can have push access. **Note**: The list of users, apps, and teams in total is limited to 100 items. |
  ///
  /// *Documentation*: [https://docs.github.com/rest/branches/branch-protection#set-user-access-restrictions](https://docs.github.com/rest/branches/branch-protection#set-user-access-restrictions)
  pub fn set_user_access_restrictions(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    branch: impl Into<String>,
  ) -> Request<set_user_access_restrictions::Request, (), set_user_access_restrictions::Response>
  {
    let owner = owner.into();
    let repo = repo.into();
    let branch = branch.into();
    let url = format!("/repos/{owner}/{repo}/branches/{branch}/protection/restrictions/users");

    Request::<set_user_access_restrictions::Request, (), set_user_access_restrictions::Response>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **Remove user access restrictions**
  ///
  /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
  ///
  /// Removes the ability of a user to push to this branch.
  ///
  /// | Type    | Description                                                                                                                                   |
  /// | ------- | --------------------------------------------------------------------------------------------------------------------------------------------- |
  /// | `array` | Usernames of the people who should no longer have push access. **Note**: The list of users, apps, and teams in total is limited to 100 items. |
  ///
  /// *Documentation*: [https://docs.github.com/rest/branches/branch-protection#remove-user-access-restrictions](https://docs.github.com/rest/branches/branch-protection#remove-user-access-restrictions)
  pub fn remove_user_access_restrictions(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    branch: impl Into<String>,
  ) -> Request<
    remove_user_access_restrictions::Request,
    (),
    remove_user_access_restrictions::Response,
  > {
    let owner = owner.into();
    let repo = repo.into();
    let branch = branch.into();
    let url = format!("/repos/{owner}/{repo}/branches/{branch}/protection/restrictions/users");

    Request::<remove_user_access_restrictions::Request, (), remove_user_access_restrictions::Response>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Rename a branch**
  ///
  /// Renames a branch in a repository.
  ///
  /// **Note:** Although the API responds immediately, the branch rename process might take some extra time to complete in the background. You won't be able to push to the old branch name while the rename process is in progress. For more information, see "[Renaming a branch](https://docs.github.com/github/administering-a-repository/renaming-a-branch)".
  ///
  /// The authenticated user must have push access to the branch. If the branch is the default branch, the authenticated user must also have admin or owner permissions.
  ///
  /// In order to rename the default branch, fine-grained access tokens also need the `administration:write` repository permission.
  ///
  /// *Documentation*: [https://docs.github.com/rest/branches/branches#rename-a-branch](https://docs.github.com/rest/branches/branches#rename-a-branch)
  pub fn rename_branch(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    branch: impl Into<String>,
  ) -> Request<rename_branch::Request, (), rename_branch::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let branch = branch.into();
    let url = format!("/repos/{owner}/{repo}/branches/{branch}/rename");

    Request::<rename_branch::Request, (), rename_branch::Response>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **List CODEOWNERS errors**
  ///
  /// List any syntax errors that are detected in the CODEOWNERS
  /// file.
  ///
  /// For more information about the correct CODEOWNERS syntax,
  /// see "[About code owners](https://docs.github.com/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/about-code-owners)."
  ///
  /// *Documentation*: [https://docs.github.com/rest/repos/repos#list-codeowners-errors](https://docs.github.com/rest/repos/repos#list-codeowners-errors)
  pub fn codeowners_errors(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), codeowners_errors::Query, codeowners_errors::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/codeowners/errors");

    Request::<(), codeowners_errors::Query, codeowners_errors::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List repository collaborators**
  ///
  /// For organization-owned repositories, the list of collaborators includes outside collaborators, organization members that are direct collaborators, organization members with access through team memberships, organization members with access through default organization permissions, and organization owners.
  /// Organization members with write, maintain, or admin privileges on the organization-owned repository can use this endpoint.
  ///
  /// Team members will include the members of child teams.
  ///
  /// The authenticated user must have push access to the repository to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `read:org` and `repo` scopes to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/collaborators/collaborators#list-repository-collaborators](https://docs.github.com/rest/collaborators/collaborators#list-repository-collaborators)
  pub fn list_collaborators(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), list_collaborators::Query, list_collaborators::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/collaborators");

    Request::<(), list_collaborators::Query, list_collaborators::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Check if a user is a repository collaborator**
  ///
  /// For organization-owned repositories, the list of collaborators includes outside collaborators, organization members that are direct collaborators, organization members with access through team memberships, organization members with access through default organization permissions, and organization owners.
  ///
  /// Team members will include the members of child teams.
  ///
  /// The authenticated user must have push access to the repository to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `read:org` and `repo` scopes to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/collaborators/collaborators#check-if-a-user-is-a-repository-collaborator](https://docs.github.com/rest/collaborators/collaborators#check-if-a-user-is-a-repository-collaborator)
  pub fn check_collaborator(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    username: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let username = username.into();
    let url = format!("/repos/{owner}/{repo}/collaborators/{username}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Add a repository collaborator**
  ///
  /// This endpoint triggers [notifications](https://docs.github.com/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. For more information, see "[Rate limits for the API](https://docs.github.com/rest/overview/rate-limits-for-the-rest-api#about-secondary-rate-limits)" and "[Best practices for using the REST API](https://docs.github.com/rest/guides/best-practices-for-using-the-rest-api)."
  ///
  /// Adding an outside collaborator may be restricted by enterprise administrators. For more information, see "[Enforcing repository management policies in your enterprise](https://docs.github.com/admin/policies/enforcing-policies-for-your-enterprise/enforcing-repository-management-policies-in-your-enterprise#enforcing-a-policy-for-inviting-outside-collaborators-to-repositories)."
  ///
  /// For more information on permission levels, see "[Repository permission levels for an organization](https://docs.github.com/github/setting-up-and-managing-organizations-and-teams/repository-permission-levels-for-an-organization#permission-levels-for-repositories-owned-by-an-organization)". There are restrictions on which permissions can be granted to organization members when an organization base role is in place. In this case, the permission being given must be equal to or higher than the org base permission. Otherwise, the request will fail with:
  ///
  /// ```
  /// Cannot assign {member} permission of {role name}
  /// ```
  ///
  /// Note that, if you choose not to pass any parameters, you'll need to set `Content-Length` to zero when calling out to this endpoint. For more information, see "[HTTP method](https://docs.github.com/rest/guides/getting-started-with-the-rest-api#http-method)."
  ///
  /// The invitee will receive a notification that they have been invited to the repository, which they must accept or decline. They may do this via the notifications page, the email they receive, or by using the [API](https://docs.github.com/rest/collaborators/invitations).
  ///
  /// **Updating an existing collaborator's permission level**
  ///
  /// The endpoint can also be used to change the permissions of an existing collaborator without first removing and re-adding the collaborator. To change the permissions, use the same endpoint and pass a different `permission` parameter. The response will be a `204`, with no other indication that the permission level changed.
  ///
  /// **Rate limits**
  ///
  /// You are limited to sending 50 invitations to a repository per 24 hour period. Note there is no limit if you are inviting organization members to an organization repository.
  ///
  /// *Documentation*: [https://docs.github.com/rest/collaborators/collaborators#add-a-repository-collaborator](https://docs.github.com/rest/collaborators/collaborators#add-a-repository-collaborator)
  pub fn add_collaborator(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    username: impl Into<String>,
  ) -> Request<add_collaborator::Request, (), add_collaborator::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let username = username.into();
    let url = format!("/repos/{owner}/{repo}/collaborators/{username}");

    Request::<add_collaborator::Request, (), add_collaborator::Response>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **Remove a repository collaborator**
  ///
  /// Removes a collaborator from a repository.
  ///
  /// To use this endpoint, the authenticated user must either be an administrator of the repository or target themselves for removal.
  ///
  /// This endpoint also:
  /// - Cancels any outstanding invitations
  /// - Unasigns the user from any issues
  /// - Removes access to organization projects if the user is not an organization member and is not a collaborator on any other organization repositories.
  /// - Unstars the repository
  /// - Updates access permissions to packages
  ///
  /// Removing a user as a collaborator has the following effects on forks:
  ///  - If the user had access to a fork through their membership to this repository, the user will also be removed from the fork.
  ///  - If the user had their own fork of the repository, the fork will be deleted.
  ///  - If the user still has read access to the repository, open pull requests by this user from a fork will be denied.
  ///
  /// **Note**: A user can still have access to the repository through organization permissions like base repository permissions.
  ///
  /// Although the API responds immediately, the additional permission updates might take some extra time to complete in the background.
  ///
  /// For more information on fork permissions, see "[About permissions and visibility of forks](https://docs.github.com/pull-requests/collaborating-with-pull-requests/working-with-forks/about-permissions-and-visibility-of-forks)".
  ///
  /// *Documentation*: [https://docs.github.com/rest/collaborators/collaborators#remove-a-repository-collaborator](https://docs.github.com/rest/collaborators/collaborators#remove-a-repository-collaborator)
  pub fn remove_collaborator(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    username: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let username = username.into();
    let url = format!("/repos/{owner}/{repo}/collaborators/{username}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Get repository permissions for a user**
  ///
  /// Checks the repository permission of a collaborator. The possible repository
  /// permissions are `admin`, `write`, `read`, and `none`.
  ///
  /// *Note*: The `permission` attribute provides the legacy base roles of `admin`, `write`, `read`, and `none`, where the
  /// `maintain` role is mapped to `write` and the `triage` role is mapped to `read`. To determine the role assigned to the
  /// collaborator, see the `role_name` attribute, which will provide the full role name, including custom roles. The
  /// `permissions` hash can also be used to determine which base level of access the collaborator has to the repository.
  ///
  /// *Documentation*: [https://docs.github.com/rest/collaborators/collaborators#get-repository-permissions-for-a-user](https://docs.github.com/rest/collaborators/collaborators#get-repository-permissions-for-a-user)
  pub fn get_collaborator_permission_level(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    username: impl Into<String>,
  ) -> Request<(), (), get_collaborator_permission_level::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let username = username.into();
    let url = format!("/repos/{owner}/{repo}/collaborators/{username}/permission");

    Request::<(), (), get_collaborator_permission_level::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List commit comments for a repository**
  ///
  /// Lists the commit comments for a specified repository. Comments are ordered by ascending ID.
  ///
  /// This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
  ///
  /// - **`application/vnd.github-commitcomment.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
  /// - **`application/vnd.github-commitcomment.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
  /// - **`application/vnd.github-commitcomment.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
  /// - **`application/vnd.github-commitcomment.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
  ///
  /// *Documentation*: [https://docs.github.com/rest/commits/comments#list-commit-comments-for-a-repository](https://docs.github.com/rest/commits/comments#list-commit-comments-for-a-repository)
  pub fn list_commit_comments_for_repo(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), list_commit_comments_for_repo::Query, list_commit_comments_for_repo::Response>
  {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/comments");

    Request::<(), list_commit_comments_for_repo::Query, list_commit_comments_for_repo::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get a commit comment**
  ///
  /// Gets a specified commit comment.
  ///
  /// This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
  ///
  /// - **`application/vnd.github-commitcomment.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
  /// - **`application/vnd.github-commitcomment.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
  /// - **`application/vnd.github-commitcomment.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
  /// - **`application/vnd.github-commitcomment.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
  ///
  /// *Documentation*: [https://docs.github.com/rest/commits/comments#get-a-commit-comment](https://docs.github.com/rest/commits/comments#get-a-commit-comment)
  pub fn get_commit_comment(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    comment_id: impl Into<i64>,
  ) -> Request<(), (), get_commit_comment::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let comment_id = comment_id.into();
    let url = format!("/repos/{owner}/{repo}/comments/{comment_id}");

    Request::<(), (), get_commit_comment::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Update a commit comment**
  ///
  /// Updates the contents of a specified commit comment.
  ///
  /// This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
  ///
  /// - **`application/vnd.github-commitcomment.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
  /// - **`application/vnd.github-commitcomment.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
  /// - **`application/vnd.github-commitcomment.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
  /// - **`application/vnd.github-commitcomment.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
  ///
  /// *Documentation*: [https://docs.github.com/rest/commits/comments#update-a-commit-comment](https://docs.github.com/rest/commits/comments#update-a-commit-comment)
  pub fn update_commit_comment(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    comment_id: impl Into<i64>,
  ) -> Request<update_commit_comment::Request, (), update_commit_comment::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let comment_id = comment_id.into();
    let url = format!("/repos/{owner}/{repo}/comments/{comment_id}");

    Request::<update_commit_comment::Request, (), update_commit_comment::Response>::builder(
      &self.config,
    )
    .patch(url)
    .build()
  }

  /// **Delete a commit comment**
  ///
  ///
  /// *Documentation*: [https://docs.github.com/rest/commits/comments#delete-a-commit-comment](https://docs.github.com/rest/commits/comments#delete-a-commit-comment)
  pub fn delete_commit_comment(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    comment_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let comment_id = comment_id.into();
    let url = format!("/repos/{owner}/{repo}/comments/{comment_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List commits**
  ///
  /// **Signature verification object**
  ///
  /// The response will include a `verification` object that describes the result of verifying the commit's signature. The following fields are included in the `verification` object:
  ///
  /// | Name | Type | Description |
  /// | ---- | ---- | ----------- |
  /// | `verified` | `boolean` | Indicates whether GitHub considers the signature in this commit to be verified. |
  /// | `reason` | `string` | The reason for verified value. Possible values and their meanings are enumerated in table below. |
  /// | `signature` | `string` | The signature that was extracted from the commit. |
  /// | `payload` | `string` | The value that was signed. |
  ///
  /// These are the possible values for `reason` in the `verification` object:
  ///
  /// | Value | Description |
  /// | ----- | ----------- |
  /// | `expired_key` | The key that made the signature is expired. |
  /// | `not_signing_key` | The "signing" flag is not among the usage flags in the GPG key that made the signature. |
  /// | `gpgverify_error` | There was an error communicating with the signature verification service. |
  /// | `gpgverify_unavailable` | The signature verification service is currently unavailable. |
  /// | `unsigned` | The object does not include a signature. |
  /// | `unknown_signature_type` | A non-PGP signature was found in the commit. |
  /// | `no_user` | No user was associated with the `committer` email address in the commit. |
  /// | `unverified_email` | The `committer` email address in the commit was associated with a user, but the email address is not verified on their account. |
  /// | `bad_email` | The `committer` email address in the commit is not included in the identities of the PGP key that made the signature. |
  /// | `unknown_key` | The key that made the signature has not been registered with any user's account. |
  /// | `malformed_signature` | There was an error parsing the signature. |
  /// | `invalid` | The signature could not be cryptographically verified using the key whose key-id was found in the signature. |
  /// | `valid` | None of the above errors applied, so the signature is considered to be verified. |
  ///
  /// *Documentation*: [https://docs.github.com/rest/commits/commits#list-commits](https://docs.github.com/rest/commits/commits#list-commits)
  pub fn list_commits(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), list_commits::Query, list_commits::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/commits");

    Request::<(), list_commits::Query, list_commits::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List branches for HEAD commit**
  ///
  /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
  ///
  /// Returns all branches where the given commit SHA is the HEAD, or latest commit for the branch.
  ///
  /// *Documentation*: [https://docs.github.com/rest/commits/commits#list-branches-for-head-commit](https://docs.github.com/rest/commits/commits#list-branches-for-head-commit)
  pub fn list_branches_for_head_commit(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    commit_sha: impl Into<String>,
  ) -> Request<(), (), list_branches_for_head_commit::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let commit_sha = commit_sha.into();
    let url = format!("/repos/{owner}/{repo}/commits/{commit_sha}/branches-where-head");

    Request::<(), (), list_branches_for_head_commit::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List commit comments**
  ///
  /// Lists the comments for a specified commit.
  ///
  /// This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
  ///
  /// - **`application/vnd.github-commitcomment.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
  /// - **`application/vnd.github-commitcomment.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
  /// - **`application/vnd.github-commitcomment.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
  /// - **`application/vnd.github-commitcomment.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
  ///
  /// *Documentation*: [https://docs.github.com/rest/commits/comments#list-commit-comments](https://docs.github.com/rest/commits/comments#list-commit-comments)
  pub fn list_comments_for_commit(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    commit_sha: impl Into<String>,
  ) -> Request<(), list_comments_for_commit::Query, list_comments_for_commit::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let commit_sha = commit_sha.into();
    let url = format!("/repos/{owner}/{repo}/commits/{commit_sha}/comments");

    Request::<(), list_comments_for_commit::Query, list_comments_for_commit::Response>::builder(
      &self.config,
    )
    .get(url)
    .build()
  }

  /// **Create a commit comment**
  ///
  /// Create a comment for a commit using its `:commit_sha`.
  ///
  /// This endpoint triggers [notifications](https://docs.github.com/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. For more information, see "[Rate limits for the API](https://docs.github.com/rest/overview/rate-limits-for-the-rest-api#about-secondary-rate-limits)" and "[Best practices for using the REST API](https://docs.github.com/rest/guides/best-practices-for-using-the-rest-api)."
  ///
  /// This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
  ///
  /// - **`application/vnd.github-commitcomment.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
  /// - **`application/vnd.github-commitcomment.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
  /// - **`application/vnd.github-commitcomment.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
  /// - **`application/vnd.github-commitcomment.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
  ///
  /// *Documentation*: [https://docs.github.com/rest/commits/comments#create-a-commit-comment](https://docs.github.com/rest/commits/comments#create-a-commit-comment)
  pub fn create_commit_comment(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    commit_sha: impl Into<String>,
  ) -> Request<create_commit_comment::Request, (), create_commit_comment::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let commit_sha = commit_sha.into();
    let url = format!("/repos/{owner}/{repo}/commits/{commit_sha}/comments");

    Request::<create_commit_comment::Request, (), create_commit_comment::Response>::builder(
      &self.config,
    )
    .post(url)
    .build()
  }

  /// **List pull requests associated with a commit**
  ///
  /// Lists the merged pull request that introduced the commit to the repository. If the commit is not present in the default branch, will only return open pull requests associated with the commit.
  ///
  /// To list the open or merged pull requests associated with a branch, you can set the `commit_sha` parameter to the branch name.
  ///
  /// *Documentation*: [https://docs.github.com/rest/commits/commits#list-pull-requests-associated-with-a-commit](https://docs.github.com/rest/commits/commits#list-pull-requests-associated-with-a-commit)
  pub fn list_pull_requests_associated_with_commit(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    commit_sha: impl Into<String>,
  ) -> Request<
    (),
    list_pull_requests_associated_with_commit::Query,
    list_pull_requests_associated_with_commit::Response,
  > {
    let owner = owner.into();
    let repo = repo.into();
    let commit_sha = commit_sha.into();
    let url = format!("/repos/{owner}/{repo}/commits/{commit_sha}/pulls");

    Request::<
      (),
      list_pull_requests_associated_with_commit::Query,
      list_pull_requests_associated_with_commit::Response,
    >::builder(&self.config)
    .get(url)
    .build()
  }

  /// **Get a commit**
  ///
  /// Returns the contents of a single commit reference. You must have `read` access for the repository to use this endpoint.
  ///
  /// **Note:** If there are more than 300 files in the commit diff and the default JSON media type is requested, the response will include pagination link headers for the remaining files, up to a limit of 3000 files. Each page contains the static commit information, and the only changes are to the file listing.
  ///
  /// This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)." Pagination query parameters are not supported for these media types.
  ///
  /// - **`application/vnd.github.diff`**: Returns the diff of the commit. Larger diffs may time out and return a 5xx status code.
  /// - **`application/vnd.github.patch`**: Returns the patch of the commit. Diffs with binary data will have no `patch` property. Larger diffs may time out and return a 5xx status code.
  /// - **`application/vnd.github.sha`**: Returns the commit's SHA-1 hash. You can use this endpoint to check if a remote reference's SHA-1 hash is the same as your local reference's SHA-1 hash by providing the local SHA-1 reference as the ETag.
  ///
  /// **Signature verification object**
  ///
  /// The response will include a `verification` object that describes the result of verifying the commit's signature. The following fields are included in the `verification` object:
  ///
  /// | Name | Type | Description |
  /// | ---- | ---- | ----------- |
  /// | `verified` | `boolean` | Indicates whether GitHub considers the signature in this commit to be verified. |
  /// | `reason` | `string` | The reason for verified value. Possible values and their meanings are enumerated in table below. |
  /// | `signature` | `string` | The signature that was extracted from the commit. |
  /// | `payload` | `string` | The value that was signed. |
  ///
  /// These are the possible values for `reason` in the `verification` object:
  ///
  /// | Value | Description |
  /// | ----- | ----------- |
  /// | `expired_key` | The key that made the signature is expired. |
  /// | `not_signing_key` | The "signing" flag is not among the usage flags in the GPG key that made the signature. |
  /// | `gpgverify_error` | There was an error communicating with the signature verification service. |
  /// | `gpgverify_unavailable` | The signature verification service is currently unavailable. |
  /// | `unsigned` | The object does not include a signature. |
  /// | `unknown_signature_type` | A non-PGP signature was found in the commit. |
  /// | `no_user` | No user was associated with the `committer` email address in the commit. |
  /// | `unverified_email` | The `committer` email address in the commit was associated with a user, but the email address is not verified on their account. |
  /// | `bad_email` | The `committer` email address in the commit is not included in the identities of the PGP key that made the signature. |
  /// | `unknown_key` | The key that made the signature has not been registered with any user's account. |
  /// | `malformed_signature` | There was an error parsing the signature. |
  /// | `invalid` | The signature could not be cryptographically verified using the key whose key-id was found in the signature. |
  /// | `valid` | None of the above errors applied, so the signature is considered to be verified. |
  ///
  /// *Documentation*: [https://docs.github.com/rest/commits/commits#get-a-commit](https://docs.github.com/rest/commits/commits#get-a-commit)
  pub fn get_commit(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    ref_: impl Into<String>,
  ) -> Request<(), get_commit::Query, get_commit::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let ref_ = ref_.into();
    let url = format!("/repos/{owner}/{repo}/commits/{ref_}");

    Request::<(), get_commit::Query, get_commit::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get the combined status for a specific reference**
  ///
  /// Users with pull access in a repository can access a combined view of commit statuses for a given ref. The ref can be a SHA, a branch name, or a tag name.
  ///
  ///
  /// Additionally, a combined `state` is returned. The `state` is one of:
  ///
  /// *   **failure** if any of the contexts report as `error` or `failure`
  /// *   **pending** if there are no statuses or a context is `pending`
  /// *   **success** if the latest status for all contexts is `success`
  ///
  /// *Documentation*: [https://docs.github.com/rest/commits/statuses#get-the-combined-status-for-a-specific-reference](https://docs.github.com/rest/commits/statuses#get-the-combined-status-for-a-specific-reference)
  pub fn get_combined_status_for_ref(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    ref_: impl Into<String>,
  ) -> Request<(), get_combined_status_for_ref::Query, get_combined_status_for_ref::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let ref_ = ref_.into();
    let url = format!("/repos/{owner}/{repo}/commits/{ref_}/status");

    Request::<(), get_combined_status_for_ref::Query, get_combined_status_for_ref::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List commit statuses for a reference**
  ///
  /// Users with pull access in a repository can view commit statuses for a given ref. The ref can be a SHA, a branch name, or a tag name. Statuses are returned in reverse chronological order. The first status in the list will be the latest one.
  ///
  /// This resource is also available via a legacy route: `GET /repos/:owner/:repo/statuses/:ref`.
  ///
  /// *Documentation*: [https://docs.github.com/rest/commits/statuses#list-commit-statuses-for-a-reference](https://docs.github.com/rest/commits/statuses#list-commit-statuses-for-a-reference)
  pub fn list_commit_statuses_for_ref(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    ref_: impl Into<String>,
  ) -> Request<(), list_commit_statuses_for_ref::Query, list_commit_statuses_for_ref::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let ref_ = ref_.into();
    let url = format!("/repos/{owner}/{repo}/commits/{ref_}/statuses");

    Request::<(), list_commit_statuses_for_ref::Query, list_commit_statuses_for_ref::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get community profile metrics**
  ///
  /// Returns all community profile metrics for a repository. The repository cannot be a fork.
  ///
  /// The returned metrics include an overall health score, the repository description, the presence of documentation, the
  /// detected code of conduct, the detected license, and the presence of ISSUE\_TEMPLATE, PULL\_REQUEST\_TEMPLATE,
  /// README, and CONTRIBUTING files.
  ///
  /// The `health_percentage` score is defined as a percentage of how many of
  /// the recommended community health files are present. For more information, see
  /// "[About community profiles for public repositories](https://docs.github.com/communities/setting-up-your-project-for-healthy-contributions/about-community-profiles-for-public-repositories)."
  ///
  /// `content_reports_enabled` is only returned for organization-owned repositories.
  ///
  /// *Documentation*: [https://docs.github.com/rest/metrics/community#get-community-profile-metrics](https://docs.github.com/rest/metrics/community#get-community-profile-metrics)
  pub fn get_community_profile_metrics(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), (), get_community_profile_metrics::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/community/profile");

    Request::<(), (), get_community_profile_metrics::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Compare two commits**
  ///
  /// Compares two commits against one another. You can compare refs (branches or tags) and commit SHAs in the same repository, or you can compare refs and commit SHAs that exist in different repositories within the same repository network, including fork branches. For more information about how to view a repository's network, see "[Understanding connections between repositories](https://docs.github.com/repositories/viewing-activity-and-data-for-your-repository/understanding-connections-between-repositories)."
  ///
  /// This endpoint is equivalent to running the `git log BASE..HEAD` command, but it returns commits in a different order. The `git log BASE..HEAD` command returns commits in reverse chronological order, whereas the API returns commits in chronological order.
  ///
  /// This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
  ///
  /// - **`application/vnd.github.diff`**: Returns the diff of the commit.
  /// - **`application/vnd.github.patch`**: Returns the patch of the commit. Diffs with binary data will have no `patch` property.
  ///
  /// The API response includes details about the files that were changed between the two commits. This includes the status of the change (if a file was added, removed, modified, or renamed), and details of the change itself. For example, files with a `renamed` status have a `previous_filename` field showing the previous filename of the file, and files with a `modified` status have a `patch` field showing the changes made to the file.
  ///
  /// When calling this endpoint without any paging parameter (`per_page` or `page`), the returned list is limited to 250 commits, and the last commit in the list is the most recent of the entire comparison.
  ///
  /// **Working with large comparisons**
  ///
  /// To process a response with a large number of commits, use a query parameter (`per_page` or `page`) to paginate the results. When using pagination:
  ///
  /// - The list of changed files is only shown on the first page of results, but it includes all changed files for the entire comparison.
  /// - The results are returned in chronological order, but the last commit in the returned list may not be the most recent one in the entire set if there are more pages of results.
  ///
  /// For more information on working with pagination, see "[Using pagination in the REST API](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api)."
  ///
  /// **Signature verification object**
  ///
  /// The response will include a `verification` object that describes the result of verifying the commit's signature. The `verification` object includes the following fields:
  ///
  /// | Name | Type | Description |
  /// | ---- | ---- | ----------- |
  /// | `verified` | `boolean` | Indicates whether GitHub considers the signature in this commit to be verified. |
  /// | `reason` | `string` | The reason for verified value. Possible values and their meanings are enumerated in table below. |
  /// | `signature` | `string` | The signature that was extracted from the commit. |
  /// | `payload` | `string` | The value that was signed. |
  ///
  /// These are the possible values for `reason` in the `verification` object:
  ///
  /// | Value | Description |
  /// | ----- | ----------- |
  /// | `expired_key` | The key that made the signature is expired. |
  /// | `not_signing_key` | The "signing" flag is not among the usage flags in the GPG key that made the signature. |
  /// | `gpgverify_error` | There was an error communicating with the signature verification service. |
  /// | `gpgverify_unavailable` | The signature verification service is currently unavailable. |
  /// | `unsigned` | The object does not include a signature. |
  /// | `unknown_signature_type` | A non-PGP signature was found in the commit. |
  /// | `no_user` | No user was associated with the `committer` email address in the commit. |
  /// | `unverified_email` | The `committer` email address in the commit was associated with a user, but the email address is not verified on their account. |
  /// | `bad_email` | The `committer` email address in the commit is not included in the identities of the PGP key that made the signature. |
  /// | `unknown_key` | The key that made the signature has not been registered with any user's account. |
  /// | `malformed_signature` | There was an error parsing the signature. |
  /// | `invalid` | The signature could not be cryptographically verified using the key whose key-id was found in the signature. |
  /// | `valid` | None of the above errors applied, so the signature is considered to be verified. |
  ///
  /// *Documentation*: [https://docs.github.com/rest/commits/commits#compare-two-commits](https://docs.github.com/rest/commits/commits#compare-two-commits)
  pub fn compare_commits(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    basehead: impl Into<String>,
  ) -> Request<(), compare_commits::Query, compare_commits::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let basehead = basehead.into();
    let url = format!("/repos/{owner}/{repo}/compare/{basehead}");

    Request::<(), compare_commits::Query, compare_commits::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get repository content**
  ///
  /// Gets the contents of a file or directory in a repository. Specify the file path or directory with the `path` parameter. If you omit the `path` parameter, you will receive the contents of the repository's root directory.
  ///
  /// This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
  ///
  /// - **`application/vnd.github.raw+json`**: Returns the raw file contents for files and symlinks.
  /// - **`application/vnd.github.html+json`**: Returns the file contents in HTML. Markup languages are rendered to HTML using GitHub's open-source [Markup library](https://github.com/github/markup).
  /// - **`application/vnd.github.object+json`**: Returns the contents in a consistent object format regardless of the content type. For example, instead of an array of objects for a directory, the response will be an object with an `entries` attribute containing the array of objects.
  ///
  /// If the content is a directory, the response will be an array of objects, one object for each item in the directory. When listing the contents of a directory, submodules have their "type" specified as "file". Logically, the value _should_ be "submodule". This behavior exists [for backwards compatibility purposes](https://git.io/v1YCW). In the next major version of the API, the type will be returned as "submodule".
  ///
  /// If the content is a symlink and the symlink's target is a normal file in the repository, then the API responds with the content of the file. Otherwise, the API responds with an object describing the symlink itself.
  ///
  /// If the content is a submodule, the `submodule_git_url` field identifies the location of the submodule repository, and the `sha` identifies a specific commit within the submodule repository. Git uses the given URL when cloning the submodule repository, and checks out the submodule at that specific commit. If the submodule repository is not hosted on github.com, the Git URLs (`git_url` and `_links["git"]`) and the github.com URLs (`html_url` and `_links["html"]`) will have null values.
  ///
  /// **Notes**:
  ///
  /// - To get a repository's contents recursively, you can [recursively get the tree](https://docs.github.com/rest/git/trees#get-a-tree).
  /// - This API has an upper limit of 1,000 files for a directory. If you need to retrieve
  /// more files, use the [Git Trees API](https://docs.github.com/rest/git/trees#get-a-tree).
  /// - Download URLs expire and are meant to be used just once. To ensure the download URL does not expire, please use the contents API to obtain a fresh download URL for each download.
  /// - If the requested file's size is:
  ///   - 1 MB or smaller: All features of this endpoint are supported.
  ///   - Between 1-100 MB: Only the `raw` or `object` custom media types are supported. Both will work as normal, except that when using the `object` media type, the `content` field will be an empty
  /// string and the `encoding` field will be `"none"`. To get the contents of these larger files, use the `raw` media type.
  ///   - Greater than 100 MB: This endpoint is not supported.
  ///
  /// *Documentation*: [https://docs.github.com/rest/repos/contents#get-repository-content](https://docs.github.com/rest/repos/contents#get-repository-content)
  pub fn get_content(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    path: impl Into<String>,
  ) -> Request<(), get_content::Query, get_content::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let path = path.into();
    let url = format!("/repos/{owner}/{repo}/contents/{path}");

    Request::<(), get_content::Query, get_content::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create or update file contents**
  ///
  /// Creates a new file or replaces an existing file in a repository.
  ///
  /// **Note:** If you use this endpoint and the "[Delete a file](https://docs.github.com/rest/repos/contents/#delete-a-file)" endpoint in parallel, the concurrent requests will conflict and you will receive errors. You must use these endpoints serially instead.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint. The `workflow` scope is also required in order to modify files in the `.github/workflows` directory.
  ///
  /// *Documentation*: [https://docs.github.com/rest/repos/contents#create-or-update-file-contents](https://docs.github.com/rest/repos/contents#create-or-update-file-contents)
  pub fn create_or_update_file_contents(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    path: impl Into<String>,
  ) -> Request<create_or_update_file_contents::Request, (), create_or_update_file_contents::Response>
  {
    let owner = owner.into();
    let repo = repo.into();
    let path = path.into();
    let url = format!("/repos/{owner}/{repo}/contents/{path}");

    Request::<create_or_update_file_contents::Request, (), create_or_update_file_contents::Response>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **Delete a file**
  ///
  /// Deletes a file in a repository.
  ///
  /// You can provide an additional `committer` parameter, which is an object containing information about the committer. Or, you can provide an `author` parameter, which is an object containing information about the author.
  ///
  /// The `author` section is optional and is filled in with the `committer` information if omitted. If the `committer` information is omitted, the authenticated user's information is used.
  ///
  /// You must provide values for both `name` and `email`, whether you choose to use `author` or `committer`. Otherwise, you'll receive a `422` status code.
  ///
  /// **Note:** If you use this endpoint and the "[Create or update file contents](https://docs.github.com/rest/repos/contents/#create-or-update-file-contents)" endpoint in parallel, the concurrent requests will conflict and you will receive errors. You must use these endpoints serially instead.
  ///
  /// *Documentation*: [https://docs.github.com/rest/repos/contents#delete-a-file](https://docs.github.com/rest/repos/contents#delete-a-file)
  pub fn delete_file(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    path: impl Into<String>,
  ) -> Request<delete_file::Request, (), delete_file::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let path = path.into();
    let url = format!("/repos/{owner}/{repo}/contents/{path}");

    Request::<delete_file::Request, (), delete_file::Response>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List repository contributors**
  ///
  /// Lists contributors to the specified repository and sorts them by the number of commits per contributor in descending order. This endpoint may return information that is a few hours old because the GitHub REST API caches contributor data to improve performance.
  ///
  /// GitHub identifies contributors by author email address. This endpoint groups contribution counts by GitHub user, which includes all associated email addresses. To improve performance, only the first 500 author email addresses in the repository link to GitHub users. The rest will appear as anonymous contributors without associated GitHub user information.
  ///
  /// *Documentation*: [https://docs.github.com/rest/repos/repos#list-repository-contributors](https://docs.github.com/rest/repos/repos#list-repository-contributors)
  pub fn list_contributors(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), list_contributors::Query, list_contributors::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/contributors");

    Request::<(), list_contributors::Query, list_contributors::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List deployments**
  ///
  /// Simple filtering of deployments is available via query parameters:
  ///
  /// *Documentation*: [https://docs.github.com/rest/deployments/deployments#list-deployments](https://docs.github.com/rest/deployments/deployments#list-deployments)
  pub fn list_deployments(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), list_deployments::Query, list_deployments::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/deployments");

    Request::<(), list_deployments::Query, list_deployments::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create a deployment**
  ///
  /// Deployments offer a few configurable parameters with certain defaults.
  ///
  /// The `ref` parameter can be any named branch, tag, or SHA. At GitHub we often deploy branches and verify them
  /// before we merge a pull request.
  ///
  /// The `environment` parameter allows deployments to be issued to different runtime environments. Teams often have
  /// multiple environments for verifying their applications, such as `production`, `staging`, and `qa`. This parameter
  /// makes it easier to track which environments have requested deployments. The default environment is `production`.
  ///
  /// The `auto_merge` parameter is used to ensure that the requested ref is not behind the repository's default branch. If
  /// the ref _is_ behind the default branch for the repository, we will attempt to merge it for you. If the merge succeeds,
  /// the API will return a successful merge commit. If merge conflicts prevent the merge from succeeding, the API will
  /// return a failure response.
  ///
  /// By default, [commit statuses](https://docs.github.com/rest/commits/statuses) for every submitted context must be in a `success`
  /// state. The `required_contexts` parameter allows you to specify a subset of contexts that must be `success`, or to
  /// specify contexts that have not yet been submitted. You are not required to use commit statuses to deploy. If you do
  /// not require any contexts or create any commit statuses, the deployment will always succeed.
  ///
  /// The `payload` parameter is available for any extra information that a deployment system might need. It is a JSON text
  /// field that will be passed on when a deployment event is dispatched.
  ///
  /// The `task` parameter is used by the deployment system to allow different execution paths. In the web world this might
  /// be `deploy:migrations` to run schema changes on the system. In the compiled world this could be a flag to compile an
  /// application with debugging enabled.
  ///
  /// Merged branch response:
  ///
  /// You will see this response when GitHub automatically merges the base branch into the topic branch instead of creating
  /// a deployment. This auto-merge happens when:
  /// *   Auto-merge option is enabled in the repository
  /// *   Topic branch does not include the latest changes on the base branch, which is `master` in the response example
  /// *   There are no merge conflicts
  ///
  /// If there are no new commits in the base branch, a new request to create a deployment should give a successful
  /// response.
  ///
  /// Merge conflict response:
  ///
  /// This error happens when the `auto_merge` option is enabled and when the default branch (in this case `master`), can't
  /// be merged into the branch that's being deployed (in this case `topic-branch`), due to merge conflicts.
  ///
  /// Failed commit status checks:
  ///
  /// This error happens when the `required_contexts` parameter indicates that one or more contexts need to have a `success`
  /// status for the commit to be deployed, but one or more of the required contexts do not have a state of `success`.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` or `repo_deployment` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/deployments/deployments#create-a-deployment](https://docs.github.com/rest/deployments/deployments#create-a-deployment)
  pub fn create_deployment(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<create_deployment::Request, (), create_deployment::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/deployments");

    Request::<create_deployment::Request, (), create_deployment::Response>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Get a deployment**
  ///
  ///
  /// *Documentation*: [https://docs.github.com/rest/deployments/deployments#get-a-deployment](https://docs.github.com/rest/deployments/deployments#get-a-deployment)
  pub fn get_deployment(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    deployment_id: impl Into<i64>,
  ) -> Request<(), (), get_deployment::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let deployment_id = deployment_id.into();
    let url = format!("/repos/{owner}/{repo}/deployments/{deployment_id}");

    Request::<(), (), get_deployment::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Delete a deployment**
  ///
  /// If the repository only has one deployment, you can delete the deployment regardless of its status. If the repository has more than one deployment, you can only delete inactive deployments. This ensures that repositories with multiple deployments will always have an active deployment.
  ///
  /// To set a deployment as inactive, you must:
  ///
  /// *   Create a new deployment that is active so that the system has a record of the current state, then delete the previously active deployment.
  /// *   Mark the active deployment as inactive by adding any non-successful deployment status.
  ///
  /// For more information, see "[Create a deployment](https://docs.github.com/rest/deployments/deployments/#create-a-deployment)" and "[Create a deployment status](https://docs.github.com/rest/deployments/statuses#create-a-deployment-status)."
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` or `repo_deployment` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/deployments/deployments#delete-a-deployment](https://docs.github.com/rest/deployments/deployments#delete-a-deployment)
  pub fn delete_deployment(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    deployment_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let deployment_id = deployment_id.into();
    let url = format!("/repos/{owner}/{repo}/deployments/{deployment_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List deployment statuses**
  ///
  /// Users with pull access can view deployment statuses for a deployment:
  ///
  /// *Documentation*: [https://docs.github.com/rest/deployments/statuses#list-deployment-statuses](https://docs.github.com/rest/deployments/statuses#list-deployment-statuses)
  pub fn list_deployment_statuses(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    deployment_id: impl Into<i64>,
  ) -> Request<(), list_deployment_statuses::Query, list_deployment_statuses::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let deployment_id = deployment_id.into();
    let url = format!("/repos/{owner}/{repo}/deployments/{deployment_id}/statuses");

    Request::<(), list_deployment_statuses::Query, list_deployment_statuses::Response>::builder(
      &self.config,
    )
    .get(url)
    .build()
  }

  /// **Create a deployment status**
  ///
  /// Users with `push` access can create deployment statuses for a given deployment.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo_deployment` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/deployments/statuses#create-a-deployment-status](https://docs.github.com/rest/deployments/statuses#create-a-deployment-status)
  pub fn create_deployment_status(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    deployment_id: impl Into<i64>,
  ) -> Request<create_deployment_status::Request, (), create_deployment_status::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let deployment_id = deployment_id.into();
    let url = format!("/repos/{owner}/{repo}/deployments/{deployment_id}/statuses");

    Request::<create_deployment_status::Request, (), create_deployment_status::Response>::builder(
      &self.config,
    )
    .post(url)
    .build()
  }

  /// **Get a deployment status**
  ///
  /// Users with pull access can view a deployment status for a deployment:
  ///
  /// *Documentation*: [https://docs.github.com/rest/deployments/statuses#get-a-deployment-status](https://docs.github.com/rest/deployments/statuses#get-a-deployment-status)
  pub fn get_deployment_status(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    deployment_id: impl Into<i64>,
    status_id: impl Into<i64>,
  ) -> Request<(), (), get_deployment_status::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let deployment_id = deployment_id.into();
    let status_id = status_id.into();
    let url = format!("/repos/{owner}/{repo}/deployments/{deployment_id}/statuses/{status_id}");

    Request::<(), (), get_deployment_status::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create a repository dispatch event**
  ///
  /// You can use this endpoint to trigger a webhook event called `repository_dispatch` when you want activity that happens outside of GitHub to trigger a GitHub Actions workflow or GitHub App webhook. You must configure your GitHub Actions workflow or GitHub App to run when the `repository_dispatch` event occurs. For an example `repository_dispatch` webhook payload, see "[RepositoryDispatchEvent](https://docs.github.com/webhooks/event-payloads/#repository_dispatch)."
  ///
  /// The `client_payload` parameter is available for any extra information that your workflow might need. This parameter is a JSON payload that will be passed on when the webhook event is dispatched. For example, the `client_payload` can include a message that a user would like to send using a GitHub Actions workflow. Or the `client_payload` can be used as a test to debug your workflow.
  ///
  /// This input example shows how you can use the `client_payload` as a test to debug your workflow.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/repos/repos#create-a-repository-dispatch-event](https://docs.github.com/rest/repos/repos#create-a-repository-dispatch-event)
  pub fn create_dispatch_event(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> NoContentRequest<create_dispatch_event::Request, ()> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/dispatches");

    NoContentRequest::<create_dispatch_event::Request, ()>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **List environments**
  ///
  /// Lists the environments for a repository.
  ///
  /// Anyone with read access to the repository can use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with a private repository.
  ///
  /// *Documentation*: [https://docs.github.com/rest/deployments/environments#list-environments](https://docs.github.com/rest/deployments/environments#list-environments)
  pub fn get_all_environments(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), get_all_environments::Query, get_all_environments::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/environments");

    Request::<(), get_all_environments::Query, get_all_environments::Response>::builder(
      &self.config,
    )
    .get(url)
    .build()
  }

  /// **Get an environment**
  ///
  /// **Note:** To get information about name patterns that branches must match in order to deploy to this environment, see "[Get a deployment branch policy](/rest/deployments/branch-policies#get-a-deployment-branch-policy)."
  ///
  /// Anyone with read access to the repository can use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with a private repository.
  ///
  /// *Documentation*: [https://docs.github.com/rest/deployments/environments#get-an-environment](https://docs.github.com/rest/deployments/environments#get-an-environment)
  pub fn get_environment(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    environment_name: impl Into<String>,
  ) -> Request<(), (), get_environment::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let environment_name = environment_name.into();
    let url = format!("/repos/{owner}/{repo}/environments/{environment_name}");

    Request::<(), (), get_environment::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create or update an environment**
  ///
  /// Create or update an environment with protection rules, such as required reviewers. For more information about environment protection rules, see "[Environments](/actions/reference/environments#environment-protection-rules)."
  ///
  /// **Note:** To create or update name patterns that branches must match in order to deploy to this environment, see "[Deployment branch policies](/rest/deployments/branch-policies)."
  ///
  /// **Note:** To create or update secrets for an environment, see "[GitHub Actions secrets](/rest/actions/secrets)."
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/deployments/environments#create-or-update-an-environment](https://docs.github.com/rest/deployments/environments#create-or-update-an-environment)
  pub fn create_or_update_environment(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    environment_name: impl Into<String>,
  ) -> Request<create_or_update_environment::Request, (), create_or_update_environment::Response>
  {
    let owner = owner.into();
    let repo = repo.into();
    let environment_name = environment_name.into();
    let url = format!("/repos/{owner}/{repo}/environments/{environment_name}");

    Request::<create_or_update_environment::Request, (), create_or_update_environment::Response>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **Delete an environment**
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/deployments/environments#delete-an-environment](https://docs.github.com/rest/deployments/environments#delete-an-environment)
  pub fn delete_an_environment(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    environment_name: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let environment_name = environment_name.into();
    let url = format!("/repos/{owner}/{repo}/environments/{environment_name}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List deployment branch policies**
  ///
  /// Lists the deployment branch policies for an environment.
  ///
  /// Anyone with read access to the repository can use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with a private repository.
  ///
  /// *Documentation*: [https://docs.github.com/rest/deployments/branch-policies#list-deployment-branch-policies](https://docs.github.com/rest/deployments/branch-policies#list-deployment-branch-policies)
  pub fn list_deployment_branch_policies(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    environment_name: impl Into<String>,
  ) -> Request<(), list_deployment_branch_policies::Query, list_deployment_branch_policies::Response>
  {
    let owner = owner.into();
    let repo = repo.into();
    let environment_name = environment_name.into();
    let url =
      format!("/repos/{owner}/{repo}/environments/{environment_name}/deployment-branch-policies");

    Request::<(), list_deployment_branch_policies::Query, list_deployment_branch_policies::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create a deployment branch policy**
  ///
  /// Creates a deployment branch or tag policy for an environment.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/deployments/branch-policies#create-a-deployment-branch-policy](https://docs.github.com/rest/deployments/branch-policies#create-a-deployment-branch-policy)
  pub fn create_deployment_branch_policy(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    environment_name: impl Into<String>,
  ) -> Request<
    create_deployment_branch_policy::Request,
    (),
    create_deployment_branch_policy::Response,
  > {
    let owner = owner.into();
    let repo = repo.into();
    let environment_name = environment_name.into();
    let url =
      format!("/repos/{owner}/{repo}/environments/{environment_name}/deployment-branch-policies");

    Request::<create_deployment_branch_policy::Request, (), create_deployment_branch_policy::Response>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Get a deployment branch policy**
  ///
  /// Gets a deployment branch or tag policy for an environment.
  ///
  /// Anyone with read access to the repository can use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with a private repository.
  ///
  /// *Documentation*: [https://docs.github.com/rest/deployments/branch-policies#get-a-deployment-branch-policy](https://docs.github.com/rest/deployments/branch-policies#get-a-deployment-branch-policy)
  pub fn get_deployment_branch_policy(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    environment_name: impl Into<String>,
    branch_policy_id: impl Into<i64>,
  ) -> Request<(), (), get_deployment_branch_policy::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let environment_name = environment_name.into();
    let branch_policy_id = branch_policy_id.into();
    let url = format!("/repos/{owner}/{repo}/environments/{environment_name}/deployment-branch-policies/{branch_policy_id}");

    Request::<(), (), get_deployment_branch_policy::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Update a deployment branch policy**
  ///
  /// Updates a deployment branch or tag policy for an environment.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/deployments/branch-policies#update-a-deployment-branch-policy](https://docs.github.com/rest/deployments/branch-policies#update-a-deployment-branch-policy)
  pub fn update_deployment_branch_policy(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    environment_name: impl Into<String>,
    branch_policy_id: impl Into<i64>,
  ) -> Request<
    update_deployment_branch_policy::Request,
    (),
    update_deployment_branch_policy::Response,
  > {
    let owner = owner.into();
    let repo = repo.into();
    let environment_name = environment_name.into();
    let branch_policy_id = branch_policy_id.into();
    let url = format!("/repos/{owner}/{repo}/environments/{environment_name}/deployment-branch-policies/{branch_policy_id}");

    Request::<update_deployment_branch_policy::Request, (), update_deployment_branch_policy::Response>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **Delete a deployment branch policy**
  ///
  /// Deletes a deployment branch or tag policy for an environment.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/deployments/branch-policies#delete-a-deployment-branch-policy](https://docs.github.com/rest/deployments/branch-policies#delete-a-deployment-branch-policy)
  pub fn delete_deployment_branch_policy(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    environment_name: impl Into<String>,
    branch_policy_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let environment_name = environment_name.into();
    let branch_policy_id = branch_policy_id.into();
    let url = format!("/repos/{owner}/{repo}/environments/{environment_name}/deployment-branch-policies/{branch_policy_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Get all deployment protection rules for an environment**
  ///
  /// Gets all custom deployment protection rules that are enabled for an environment. Anyone with read access to the repository can use this endpoint. For more information about environments, see "[Using environments for deployment](https://docs.github.com/actions/deployment/targeting-different-environments/using-environments-for-deployment)."
  ///
  /// For more information about the app that is providing this custom deployment rule, see the [documentation for the `GET /apps/{app_slug}` endpoint](https://docs.github.com/rest/apps/apps#get-an-app).
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with a private repository.
  ///
  /// *Documentation*: [https://docs.github.com/rest/deployments/protection-rules#get-all-deployment-protection-rules-for-an-environment](https://docs.github.com/rest/deployments/protection-rules#get-all-deployment-protection-rules-for-an-environment)
  pub fn get_all_deployment_protection_rules(
    &self,
    environment_name: impl Into<String>,
    repo: impl Into<String>,
    owner: impl Into<String>,
  ) -> Request<(), (), get_all_deployment_protection_rules::Response> {
    let environment_name = environment_name.into();
    let repo = repo.into();
    let owner = owner.into();
    let url =
      format!("/repos/{owner}/{repo}/environments/{environment_name}/deployment_protection_rules");

    Request::<(), (), get_all_deployment_protection_rules::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create a custom deployment protection rule on an environment**
  ///
  /// Enable a custom deployment protection rule for an environment.
  ///
  /// The authenticated user must have admin or owner permissions to the repository to use this endpoint.
  ///
  /// For more information about the app that is providing this custom deployment rule, see the [documentation for the `GET /apps/{app_slug}` endpoint](https://docs.github.com/rest/apps/apps#get-an-app).
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/deployments/protection-rules#create-a-custom-deployment-protection-rule-on-an-environment](https://docs.github.com/rest/deployments/protection-rules#create-a-custom-deployment-protection-rule-on-an-environment)
  pub fn create_deployment_protection_rule(
    &self,
    environment_name: impl Into<String>,
    repo: impl Into<String>,
    owner: impl Into<String>,
  ) -> Request<
    create_deployment_protection_rule::Request,
    (),
    create_deployment_protection_rule::Response,
  > {
    let environment_name = environment_name.into();
    let repo = repo.into();
    let owner = owner.into();
    let url =
      format!("/repos/{owner}/{repo}/environments/{environment_name}/deployment_protection_rules");

    Request::<
      create_deployment_protection_rule::Request,
      (),
      create_deployment_protection_rule::Response,
    >::builder(&self.config)
    .post(url)
    .build()
  }

  /// **List custom deployment rule integrations available for an environment**
  ///
  /// Gets all custom deployment protection rule integrations that are available for an environment. Anyone with read access to the repository can use this endpoint.
  ///
  /// For more information about environments, see "[Using environments for deployment](https://docs.github.com/actions/deployment/targeting-different-environments/using-environments-for-deployment)."
  ///
  /// For more information about the app that is providing this custom deployment rule, see "[GET an app](https://docs.github.com/rest/apps/apps#get-an-app)".
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with a private repository.
  ///
  /// *Documentation*: [https://docs.github.com/rest/deployments/protection-rules#list-custom-deployment-rule-integrations-available-for-an-environment](https://docs.github.com/rest/deployments/protection-rules#list-custom-deployment-rule-integrations-available-for-an-environment)
  pub fn list_custom_deployment_rule_integrations(
    &self,
    environment_name: impl Into<String>,
    repo: impl Into<String>,
    owner: impl Into<String>,
  ) -> Request<
    (),
    list_custom_deployment_rule_integrations::Query,
    list_custom_deployment_rule_integrations::Response,
  > {
    let environment_name = environment_name.into();
    let repo = repo.into();
    let owner = owner.into();
    let url = format!(
      "/repos/{owner}/{repo}/environments/{environment_name}/deployment_protection_rules/apps"
    );

    Request::<
      (),
      list_custom_deployment_rule_integrations::Query,
      list_custom_deployment_rule_integrations::Response,
    >::builder(&self.config)
    .get(url)
    .build()
  }

  /// **Get a custom deployment protection rule**
  ///
  /// Gets an enabled custom deployment protection rule for an environment. Anyone with read access to the repository can use this endpoint. For more information about environments, see "[Using environments for deployment](https://docs.github.com/actions/deployment/targeting-different-environments/using-environments-for-deployment)."
  ///
  /// For more information about the app that is providing this custom deployment rule, see [`GET /apps/{app_slug}`](https://docs.github.com/rest/apps/apps#get-an-app).
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with a private repository.
  ///
  /// *Documentation*: [https://docs.github.com/rest/deployments/protection-rules#get-a-custom-deployment-protection-rule](https://docs.github.com/rest/deployments/protection-rules#get-a-custom-deployment-protection-rule)
  pub fn get_custom_deployment_protection_rule(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    environment_name: impl Into<String>,
    protection_rule_id: impl Into<i64>,
  ) -> Request<(), (), get_custom_deployment_protection_rule::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let environment_name = environment_name.into();
    let protection_rule_id = protection_rule_id.into();
    let url = format!("/repos/{owner}/{repo}/environments/{environment_name}/deployment_protection_rules/{protection_rule_id}");

    Request::<(), (), get_custom_deployment_protection_rule::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Disable a custom protection rule for an environment**
  ///
  /// Disables a custom deployment protection rule for an environment.
  ///
  /// The authenticated user must have admin or owner permissions to the repository to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/deployments/protection-rules#disable-a-custom-protection-rule-for-an-environment](https://docs.github.com/rest/deployments/protection-rules#disable-a-custom-protection-rule-for-an-environment)
  pub fn disable_deployment_protection_rule(
    &self,
    environment_name: impl Into<String>,
    repo: impl Into<String>,
    owner: impl Into<String>,
    protection_rule_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let environment_name = environment_name.into();
    let repo = repo.into();
    let owner = owner.into();
    let protection_rule_id = protection_rule_id.into();
    let url = format!("/repos/{owner}/{repo}/environments/{environment_name}/deployment_protection_rules/{protection_rule_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List forks**
  ///
  ///
  /// *Documentation*: [https://docs.github.com/rest/repos/forks#list-forks](https://docs.github.com/rest/repos/forks#list-forks)
  pub fn list_forks(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), list_forks::Query, list_forks::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/forks");

    Request::<(), list_forks::Query, list_forks::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create a fork**
  ///
  /// Create a fork for the authenticated user.
  ///
  /// **Note**: Forking a Repository happens asynchronously. You may have to wait a short period of time before you can access the git objects. If this takes longer than 5 minutes, be sure to contact [GitHub Support](https://support.github.com/contact?tags=dotcom-rest-api).
  ///
  /// **Note**: Although this endpoint works with GitHub Apps, the GitHub App must be installed on the destination account with access to all repositories and on the source account with access to the source repository.
  ///
  /// *Documentation*: [https://docs.github.com/rest/repos/forks#create-a-fork](https://docs.github.com/rest/repos/forks#create-a-fork)
  pub fn create_fork(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<create_fork::Request, (), create_fork::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/forks");

    Request::<create_fork::Request, (), create_fork::Response>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **List repository webhooks**
  ///
  /// Lists webhooks for a repository. `last response` may return null if there have not been any deliveries within 30 days.
  ///
  /// *Documentation*: [https://docs.github.com/rest/repos/webhooks#list-repository-webhooks](https://docs.github.com/rest/repos/webhooks#list-repository-webhooks)
  pub fn list_webhooks(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), list_webhooks::Query, list_webhooks::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/hooks");

    Request::<(), list_webhooks::Query, list_webhooks::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create a repository webhook**
  ///
  /// Repositories can have multiple webhooks installed. Each webhook should have a unique `config`. Multiple webhooks can
  /// share the same `config` as long as those webhooks do not have any `events` that overlap.
  ///
  /// *Documentation*: [https://docs.github.com/rest/repos/webhooks#create-a-repository-webhook](https://docs.github.com/rest/repos/webhooks#create-a-repository-webhook)
  pub fn create_webhook(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<create_webhook::Request, (), create_webhook::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/hooks");

    Request::<create_webhook::Request, (), create_webhook::Response>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Get a repository webhook**
  ///
  /// Returns a webhook configured in a repository. To get only the webhook `config` properties, see "[Get a webhook configuration for a repository](/rest/webhooks/repo-config#get-a-webhook-configuration-for-a-repository)."
  ///
  /// *Documentation*: [https://docs.github.com/rest/repos/webhooks#get-a-repository-webhook](https://docs.github.com/rest/repos/webhooks#get-a-repository-webhook)
  pub fn get_webhook(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    hook_id: impl Into<i64>,
  ) -> Request<(), (), get_webhook::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let hook_id = hook_id.into();
    let url = format!("/repos/{owner}/{repo}/hooks/{hook_id}");

    Request::<(), (), get_webhook::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Update a repository webhook**
  ///
  /// Updates a webhook configured in a repository. If you previously had a `secret` set, you must provide the same `secret` or set a new `secret` or the secret will be removed. If you are only updating individual webhook `config` properties, use "[Update a webhook configuration for a repository](/rest/webhooks/repo-config#update-a-webhook-configuration-for-a-repository)."
  ///
  /// *Documentation*: [https://docs.github.com/rest/repos/webhooks#update-a-repository-webhook](https://docs.github.com/rest/repos/webhooks#update-a-repository-webhook)
  pub fn update_webhook(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    hook_id: impl Into<i64>,
  ) -> Request<update_webhook::Request, (), update_webhook::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let hook_id = hook_id.into();
    let url = format!("/repos/{owner}/{repo}/hooks/{hook_id}");

    Request::<update_webhook::Request, (), update_webhook::Response>::builder(&self.config)
      .patch(url)
      .build()
  }

  /// **Delete a repository webhook**
  ///
  ///
  /// *Documentation*: [https://docs.github.com/rest/repos/webhooks#delete-a-repository-webhook](https://docs.github.com/rest/repos/webhooks#delete-a-repository-webhook)
  pub fn delete_webhook(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    hook_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let hook_id = hook_id.into();
    let url = format!("/repos/{owner}/{repo}/hooks/{hook_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Get a webhook configuration for a repository**
  ///
  /// Returns the webhook configuration for a repository. To get more information about the webhook, including the `active` state and `events`, use "[Get a repository webhook](/rest/webhooks/repos#get-a-repository-webhook)."
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `read:repo_hook` or `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/repos/webhooks#get-a-webhook-configuration-for-a-repository](https://docs.github.com/rest/repos/webhooks#get-a-webhook-configuration-for-a-repository)
  pub fn get_webhook_config_for_repo(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    hook_id: impl Into<i64>,
  ) -> Request<(), (), get_webhook_config_for_repo::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let hook_id = hook_id.into();
    let url = format!("/repos/{owner}/{repo}/hooks/{hook_id}/config");

    Request::<(), (), get_webhook_config_for_repo::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Update a webhook configuration for a repository**
  ///
  /// Updates the webhook configuration for a repository. To update more information about the webhook, including the `active` state and `events`, use "[Update a repository webhook](/rest/webhooks/repos#update-a-repository-webhook)."
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `write:repo_hook` or `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/repos/webhooks#update-a-webhook-configuration-for-a-repository](https://docs.github.com/rest/repos/webhooks#update-a-webhook-configuration-for-a-repository)
  pub fn update_webhook_config_for_repo(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    hook_id: impl Into<i64>,
  ) -> Request<update_webhook_config_for_repo::Request, (), update_webhook_config_for_repo::Response>
  {
    let owner = owner.into();
    let repo = repo.into();
    let hook_id = hook_id.into();
    let url = format!("/repos/{owner}/{repo}/hooks/{hook_id}/config");

    Request::<update_webhook_config_for_repo::Request, (), update_webhook_config_for_repo::Response>::builder(&self.config)
      .patch(url)
      .build()
  }

  /// **List deliveries for a repository webhook**
  ///
  /// Returns a list of webhook deliveries for a webhook configured in a repository.
  ///
  /// *Documentation*: [https://docs.github.com/rest/repos/webhooks#list-deliveries-for-a-repository-webhook](https://docs.github.com/rest/repos/webhooks#list-deliveries-for-a-repository-webhook)
  pub fn list_webhook_deliveries(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    hook_id: impl Into<i64>,
  ) -> Request<(), list_webhook_deliveries::Query, list_webhook_deliveries::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let hook_id = hook_id.into();
    let url = format!("/repos/{owner}/{repo}/hooks/{hook_id}/deliveries");

    Request::<(), list_webhook_deliveries::Query, list_webhook_deliveries::Response>::builder(
      &self.config,
    )
    .get(url)
    .build()
  }

  /// **Get a delivery for a repository webhook**
  ///
  /// Returns a delivery for a webhook configured in a repository.
  ///
  /// *Documentation*: [https://docs.github.com/rest/repos/webhooks#get-a-delivery-for-a-repository-webhook](https://docs.github.com/rest/repos/webhooks#get-a-delivery-for-a-repository-webhook)
  pub fn get_webhook_delivery(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    hook_id: impl Into<i64>,
    delivery_id: impl Into<i64>,
  ) -> Request<(), (), get_webhook_delivery::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let hook_id = hook_id.into();
    let delivery_id = delivery_id.into();
    let url = format!("/repos/{owner}/{repo}/hooks/{hook_id}/deliveries/{delivery_id}");

    Request::<(), (), get_webhook_delivery::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Redeliver a delivery for a repository webhook**
  ///
  /// Redeliver a webhook delivery for a webhook configured in a repository.
  ///
  /// *Documentation*: [https://docs.github.com/rest/repos/webhooks#redeliver-a-delivery-for-a-repository-webhook](https://docs.github.com/rest/repos/webhooks#redeliver-a-delivery-for-a-repository-webhook)
  pub fn redeliver_webhook_delivery(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    hook_id: impl Into<i64>,
    delivery_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let hook_id = hook_id.into();
    let delivery_id = delivery_id.into();
    let url = format!("/repos/{owner}/{repo}/hooks/{hook_id}/deliveries/{delivery_id}/attempts");

    NoContentRequest::<(), ()>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Ping a repository webhook**
  ///
  /// This will trigger a [ping event](https://docs.github.com/webhooks/#ping-event) to be sent to the hook.
  ///
  /// *Documentation*: [https://docs.github.com/rest/repos/webhooks#ping-a-repository-webhook](https://docs.github.com/rest/repos/webhooks#ping-a-repository-webhook)
  pub fn ping_webhook(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    hook_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let hook_id = hook_id.into();
    let url = format!("/repos/{owner}/{repo}/hooks/{hook_id}/pings");

    NoContentRequest::<(), ()>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Test the push repository webhook**
  ///
  /// This will trigger the hook with the latest push to the current repository if the hook is subscribed to `push` events. If the hook is not subscribed to `push` events, the server will respond with 204 but no test POST will be generated.
  ///
  /// **Note**: Previously `/repos/:owner/:repo/hooks/:hook_id/test`
  ///
  /// *Documentation*: [https://docs.github.com/rest/repos/webhooks#test-the-push-repository-webhook](https://docs.github.com/rest/repos/webhooks#test-the-push-repository-webhook)
  pub fn test_push_webhook(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    hook_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let hook_id = hook_id.into();
    let url = format!("/repos/{owner}/{repo}/hooks/{hook_id}/tests");

    NoContentRequest::<(), ()>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **List repository invitations**
  ///
  /// When authenticating as a user with admin rights to a repository, this endpoint will list all currently open repository invitations.
  ///
  /// *Documentation*: [https://docs.github.com/rest/collaborators/invitations#list-repository-invitations](https://docs.github.com/rest/collaborators/invitations#list-repository-invitations)
  pub fn list_invitations(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), list_invitations::Query, list_invitations::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/invitations");

    Request::<(), list_invitations::Query, list_invitations::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Update a repository invitation**
  ///
  ///
  /// *Documentation*: [https://docs.github.com/rest/collaborators/invitations#update-a-repository-invitation](https://docs.github.com/rest/collaborators/invitations#update-a-repository-invitation)
  pub fn update_invitation(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    invitation_id: impl Into<i64>,
  ) -> Request<update_invitation::Request, (), update_invitation::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let invitation_id = invitation_id.into();
    let url = format!("/repos/{owner}/{repo}/invitations/{invitation_id}");

    Request::<update_invitation::Request, (), update_invitation::Response>::builder(&self.config)
      .patch(url)
      .build()
  }

  /// **Delete a repository invitation**
  ///
  ///
  /// *Documentation*: [https://docs.github.com/rest/collaborators/invitations#delete-a-repository-invitation](https://docs.github.com/rest/collaborators/invitations#delete-a-repository-invitation)
  pub fn delete_invitation(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    invitation_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let invitation_id = invitation_id.into();
    let url = format!("/repos/{owner}/{repo}/invitations/{invitation_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List deploy keys**
  ///
  ///
  /// *Documentation*: [https://docs.github.com/rest/deploy-keys/deploy-keys#list-deploy-keys](https://docs.github.com/rest/deploy-keys/deploy-keys#list-deploy-keys)
  pub fn list_deploy_keys(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), list_deploy_keys::Query, list_deploy_keys::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/keys");

    Request::<(), list_deploy_keys::Query, list_deploy_keys::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create a deploy key**
  ///
  /// You can create a read-only deploy key.
  ///
  /// *Documentation*: [https://docs.github.com/rest/deploy-keys/deploy-keys#create-a-deploy-key](https://docs.github.com/rest/deploy-keys/deploy-keys#create-a-deploy-key)
  pub fn create_deploy_key(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<create_deploy_key::Request, (), create_deploy_key::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/keys");

    Request::<create_deploy_key::Request, (), create_deploy_key::Response>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Get a deploy key**
  ///
  ///
  /// *Documentation*: [https://docs.github.com/rest/deploy-keys/deploy-keys#get-a-deploy-key](https://docs.github.com/rest/deploy-keys/deploy-keys#get-a-deploy-key)
  pub fn get_deploy_key(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    key_id: impl Into<i64>,
  ) -> Request<(), (), get_deploy_key::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let key_id = key_id.into();
    let url = format!("/repos/{owner}/{repo}/keys/{key_id}");

    Request::<(), (), get_deploy_key::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Delete a deploy key**
  ///
  /// Deploy keys are immutable. If you need to update a key, remove the key and create a new one instead.
  ///
  /// *Documentation*: [https://docs.github.com/rest/deploy-keys/deploy-keys#delete-a-deploy-key](https://docs.github.com/rest/deploy-keys/deploy-keys#delete-a-deploy-key)
  pub fn delete_deploy_key(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    key_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let key_id = key_id.into();
    let url = format!("/repos/{owner}/{repo}/keys/{key_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List repository languages**
  ///
  /// Lists languages for the specified repository. The value shown for each language is the number of bytes of code written in that language.
  ///
  /// *Documentation*: [https://docs.github.com/rest/repos/repos#list-repository-languages](https://docs.github.com/rest/repos/repos#list-repository-languages)
  pub fn list_languages(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), (), list_languages::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/languages");

    Request::<(), (), list_languages::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Sync a fork branch with the upstream repository**
  ///
  /// Sync a branch of a forked repository to keep it up-to-date with the upstream repository.
  ///
  /// *Documentation*: [https://docs.github.com/rest/branches/branches#sync-a-fork-branch-with-the-upstream-repository](https://docs.github.com/rest/branches/branches#sync-a-fork-branch-with-the-upstream-repository)
  pub fn merge_upstream(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<merge_upstream::Request, (), merge_upstream::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/merge-upstream");

    Request::<merge_upstream::Request, (), merge_upstream::Response>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Merge a branch**
  ///
  ///
  /// *Documentation*: [https://docs.github.com/rest/branches/branches#merge-a-branch](https://docs.github.com/rest/branches/branches#merge-a-branch)
  pub fn merge(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<merge::Request, (), merge::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/merges");

    Request::<merge::Request, (), merge::Response>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Get a GitHub Pages site**
  ///
  /// Gets information about a GitHub Pages site.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/pages/pages#get-a-apiname-pages-site](https://docs.github.com/rest/pages/pages#get-a-apiname-pages-site)
  pub fn get_pages(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), (), get_pages::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/pages");

    Request::<(), (), get_pages::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create a GitHub Pages site**
  ///
  /// Configures a GitHub Pages site. For more information, see "[About GitHub Pages](/github/working-with-github-pages/about-github-pages)."
  ///
  /// The authenticated user must be a repository administrator, maintainer, or have the 'manage GitHub Pages settings' permission.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/pages/pages#create-a-apiname-pages-site](https://docs.github.com/rest/pages/pages#create-a-apiname-pages-site)
  pub fn create_pages_site(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<create_pages_site::Request, (), create_pages_site::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/pages");

    Request::<create_pages_site::Request, (), create_pages_site::Response>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Update information about a GitHub Pages site**
  ///
  /// Updates information for a GitHub Pages site. For more information, see "[About GitHub Pages](/github/working-with-github-pages/about-github-pages).
  ///
  /// The authenticated user must be a repository administrator, maintainer, or have the 'manage GitHub Pages settings' permission.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/pages/pages#update-information-about-a-apiname-pages-site](https://docs.github.com/rest/pages/pages#update-information-about-a-apiname-pages-site)
  pub fn update_information_about_pages_site(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> NoContentRequest<update_information_about_pages_site::Request, ()> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/pages");

    NoContentRequest::<update_information_about_pages_site::Request, ()>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **Delete a GitHub Pages site**
  ///
  /// Deletes a GitHub Pages site. For more information, see "[About GitHub Pages](/github/working-with-github-pages/about-github-pages).
  ///
  /// The authenticated user must be a repository administrator, maintainer, or have the 'manage GitHub Pages settings' permission.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/pages/pages#delete-a-apiname-pages-site](https://docs.github.com/rest/pages/pages#delete-a-apiname-pages-site)
  pub fn delete_pages_site(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/pages");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List GitHub Pages builds**
  ///
  /// Lists builts of a GitHub Pages site.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/pages/pages#list-apiname-pages-builds](https://docs.github.com/rest/pages/pages#list-apiname-pages-builds)
  pub fn list_pages_builds(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), list_pages_builds::Query, list_pages_builds::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/pages/builds");

    Request::<(), list_pages_builds::Query, list_pages_builds::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Request a GitHub Pages build**
  ///
  /// You can request that your site be built from the latest revision on the default branch. This has the same effect as pushing a commit to your default branch, but does not require an additional commit. Manually triggering page builds can be helpful when diagnosing build warnings and failures.
  ///
  /// Build requests are limited to one concurrent build per repository and one concurrent build per requester. If you request a build while another is still in progress, the second request will be queued until the first completes.
  ///
  /// *Documentation*: [https://docs.github.com/rest/pages/pages#request-a-apiname-pages-build](https://docs.github.com/rest/pages/pages#request-a-apiname-pages-build)
  pub fn request_pages_build(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), (), request_pages_build::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/pages/builds");

    Request::<(), (), request_pages_build::Response>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Get latest Pages build**
  ///
  /// Gets information about the single most recent build of a GitHub Pages site.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/pages/pages#get-latest-pages-build](https://docs.github.com/rest/pages/pages#get-latest-pages-build)
  pub fn get_latest_pages_build(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), (), get_latest_pages_build::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/pages/builds/latest");

    Request::<(), (), get_latest_pages_build::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get GitHub Pages build**
  ///
  /// Gets information about a GitHub Pages build.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/pages/pages#get-apiname-pages-build](https://docs.github.com/rest/pages/pages#get-apiname-pages-build)
  pub fn get_pages_build(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    build_id: impl Into<i64>,
  ) -> Request<(), (), get_pages_build::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let build_id = build_id.into();
    let url = format!("/repos/{owner}/{repo}/pages/builds/{build_id}");

    Request::<(), (), get_pages_build::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create a GitHub Pages deployment**
  ///
  /// Create a GitHub Pages deployment for a repository.
  ///
  /// The authenticated user must have write permission to the repository.
  ///
  /// *Documentation*: [https://docs.github.com/rest/pages/pages#create-a-github-pages-deployment](https://docs.github.com/rest/pages/pages#create-a-github-pages-deployment)
  pub fn create_pages_deployment(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<create_pages_deployment::Request, (), create_pages_deployment::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/pages/deployments");

    Request::<create_pages_deployment::Request, (), create_pages_deployment::Response>::builder(
      &self.config,
    )
    .post(url)
    .build()
  }

  /// **Get the status of a GitHub Pages deployment**
  ///
  /// Gets the current status of a GitHub Pages deployment.
  ///
  /// The authenticated user must have read permission for the GitHub Pages site.
  ///
  /// *Documentation*: [https://docs.github.com/rest/pages/pages#get-the-status-of-a-github-pages-deployment](https://docs.github.com/rest/pages/pages#get-the-status-of-a-github-pages-deployment)
  pub fn get_pages_deployment(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    pages_deployment_id: impl Into<StringOrInteger>,
  ) -> Request<(), (), get_pages_deployment::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let pages_deployment_id = pages_deployment_id.into();
    let url = format!("/repos/{owner}/{repo}/pages/deployments/{pages_deployment_id}");

    Request::<(), (), get_pages_deployment::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Cancel a GitHub Pages deployment**
  ///
  /// Cancels a GitHub Pages deployment.
  ///
  /// The authenticated user must have write permissions for the GitHub Pages site.
  ///
  /// *Documentation*: [https://docs.github.com/rest/pages/pages#cancel-a-github-pages-deployment](https://docs.github.com/rest/pages/pages#cancel-a-github-pages-deployment)
  pub fn cancel_pages_deployment(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    pages_deployment_id: impl Into<StringOrInteger>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let pages_deployment_id = pages_deployment_id.into();
    let url = format!("/repos/{owner}/{repo}/pages/deployments/{pages_deployment_id}/cancel");

    NoContentRequest::<(), ()>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Get a DNS health check for GitHub Pages**
  ///
  /// Gets a health check of the DNS settings for the `CNAME` record configured for a repository's GitHub Pages.
  ///
  /// The first request to this endpoint returns a `202 Accepted` status and starts an asynchronous background task to get the results for the domain. After the background task completes, subsequent requests to this endpoint return a `200 OK` status with the health check results in the response.
  ///
  /// The authenticated user must be a repository administrator, maintainer, or have the 'manage GitHub Pages settings' permission to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/pages/pages#get-a-dns-health-check-for-github-pages](https://docs.github.com/rest/pages/pages#get-a-dns-health-check-for-github-pages)
  pub fn get_pages_health_check(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), (), get_pages_health_check::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/pages/health");

    Request::<(), (), get_pages_health_check::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Check if private vulnerability reporting is enabled for a repository**
  ///
  /// Returns a boolean indicating whether or not private vulnerability reporting is enabled for the repository. For more information, see "[Evaluating the security settings of a repository](https://docs.github.com/code-security/security-advisories/working-with-repository-security-advisories/evaluating-the-security-settings-of-a-repository)".
  ///
  /// *Documentation*: [https://docs.github.com/rest/repos/repos#check-if-private-vulnerability-reporting-is-enabled-for-a-repository](https://docs.github.com/rest/repos/repos#check-if-private-vulnerability-reporting-is-enabled-for-a-repository)
  pub fn check_private_vulnerability_reporting(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), (), check_private_vulnerability_reporting::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/private-vulnerability-reporting");

    Request::<(), (), check_private_vulnerability_reporting::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Enable private vulnerability reporting for a repository**
  ///
  /// Enables private vulnerability reporting for a repository. The authenticated user must have admin access to the repository. For more information, see "[Privately reporting a security vulnerability](https://docs.github.com/code-security/security-advisories/guidance-on-reporting-and-writing/privately-reporting-a-security-vulnerability)."
  ///
  /// *Documentation*: [https://docs.github.com/rest/repos/repos#enable-private-vulnerability-reporting-for-a-repository](https://docs.github.com/rest/repos/repos#enable-private-vulnerability-reporting-for-a-repository)
  pub fn enable_private_vulnerability_reporting(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/private-vulnerability-reporting");

    NoContentRequest::<(), ()>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **Disable private vulnerability reporting for a repository**
  ///
  /// Disables private vulnerability reporting for a repository. The authenticated user must have admin access to the repository. For more information, see "[Privately reporting a security vulnerability](https://docs.github.com/code-security/security-advisories/guidance-on-reporting-and-writing/privately-reporting-a-security-vulnerability)".
  ///
  /// *Documentation*: [https://docs.github.com/rest/repos/repos#disable-private-vulnerability-reporting-for-a-repository](https://docs.github.com/rest/repos/repos#disable-private-vulnerability-reporting-for-a-repository)
  pub fn disable_private_vulnerability_reporting(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/private-vulnerability-reporting");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Get all custom property values for a repository**
  ///
  /// Gets all custom property values that are set for a repository.
  /// Users with read access to the repository can use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/repos/custom-properties#get-all-custom-property-values-for-a-repository](https://docs.github.com/rest/repos/custom-properties#get-all-custom-property-values-for-a-repository)
  pub fn get_custom_properties_values(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), (), get_custom_properties_values::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/properties/values");

    Request::<(), (), get_custom_properties_values::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create or update custom property values for a repository**
  ///
  /// Create new or update existing custom property values for a repository.
  /// Using a value of `null` for a custom property will remove or 'unset' the property value from the repository.
  ///
  /// Repository admins and other users with the repository-level "edit custom property values" fine-grained permission can use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/repos/custom-properties#create-or-update-custom-property-values-for-a-repository](https://docs.github.com/rest/repos/custom-properties#create-or-update-custom-property-values-for-a-repository)
  pub fn create_or_update_custom_properties_values(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> NoContentRequest<create_or_update_custom_properties_values::Request, ()> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/properties/values");

    NoContentRequest::<create_or_update_custom_properties_values::Request, ()>::builder(
      &self.config,
    )
    .patch(url)
    .build()
  }

  /// **Get a repository README**
  ///
  /// Gets the preferred README for a repository.
  ///
  /// This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
  ///
  /// - **`application/vnd.github.raw+json`**: Returns the raw file contents. This is the default if you do not specify a media type.
  /// - **`application/vnd.github.html+json`**: Returns the README in HTML. Markup languages are rendered to HTML using GitHub's open-source [Markup library](https://github.com/github/markup).
  ///
  /// *Documentation*: [https://docs.github.com/rest/repos/contents#get-a-repository-readme](https://docs.github.com/rest/repos/contents#get-a-repository-readme)
  pub fn get_readme(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), get_readme::Query, get_readme::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/readme");

    Request::<(), get_readme::Query, get_readme::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get a repository README for a directory**
  ///
  /// Gets the README from a repository directory.
  ///
  /// This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
  ///
  /// - **`application/vnd.github.raw+json`**: Returns the raw file contents. This is the default if you do not specify a media type.
  /// - **`application/vnd.github.html+json`**: Returns the README in HTML. Markup languages are rendered to HTML using GitHub's open-source [Markup library](https://github.com/github/markup).
  ///
  /// *Documentation*: [https://docs.github.com/rest/repos/contents#get-a-repository-readme-for-a-directory](https://docs.github.com/rest/repos/contents#get-a-repository-readme-for-a-directory)
  pub fn get_readme_in_directory(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    dir: impl Into<String>,
  ) -> Request<(), get_readme_in_directory::Query, get_readme_in_directory::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let dir = dir.into();
    let url = format!("/repos/{owner}/{repo}/readme/{dir}");

    Request::<(), get_readme_in_directory::Query, get_readme_in_directory::Response>::builder(
      &self.config,
    )
    .get(url)
    .build()
  }

  /// **List releases**
  ///
  /// This returns a list of releases, which does not include regular Git tags that have not been associated with a release. To get a list of Git tags, use the [Repository Tags API](https://docs.github.com/rest/repos/repos#list-repository-tags).
  ///
  /// Information about published releases are available to everyone. Only users with push access will receive listings for draft releases.
  ///
  /// *Documentation*: [https://docs.github.com/rest/releases/releases#list-releases](https://docs.github.com/rest/releases/releases#list-releases)
  pub fn list_releases(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), list_releases::Query, list_releases::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/releases");

    Request::<(), list_releases::Query, list_releases::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create a release**
  ///
  /// Users with push access to the repository can create a release.
  ///
  /// This endpoint triggers [notifications](https://docs.github.com/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. For more information, see "[Rate limits for the API](https://docs.github.com/rest/overview/rate-limits-for-the-rest-api#about-secondary-rate-limits)" and "[Best practices for using the REST API](https://docs.github.com/rest/guides/best-practices-for-using-the-rest-api)."
  ///
  /// *Documentation*: [https://docs.github.com/rest/releases/releases#create-a-release](https://docs.github.com/rest/releases/releases#create-a-release)
  pub fn create_release(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<create_release::Request, (), create_release::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/releases");

    Request::<create_release::Request, (), create_release::Response>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Get a release asset**
  ///
  /// To download the asset's binary content, set the `Accept` header of the request to [`application/octet-stream`](https://docs.github.com/rest/overview/media-types). The API will either redirect the client to the location, or stream it directly if possible. API clients should handle both a `200` or `302` response.
  ///
  /// *Documentation*: [https://docs.github.com/rest/releases/assets#get-a-release-asset](https://docs.github.com/rest/releases/assets#get-a-release-asset)
  pub fn get_release_asset(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    asset_id: impl Into<i64>,
  ) -> Request<(), (), get_release_asset::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let asset_id = asset_id.into();
    let url = format!("/repos/{owner}/{repo}/releases/assets/{asset_id}");

    Request::<(), (), get_release_asset::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Update a release asset**
  ///
  /// Users with push access to the repository can edit a release asset.
  ///
  /// *Documentation*: [https://docs.github.com/rest/releases/assets#update-a-release-asset](https://docs.github.com/rest/releases/assets#update-a-release-asset)
  pub fn update_release_asset(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    asset_id: impl Into<i64>,
  ) -> Request<update_release_asset::Request, (), update_release_asset::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let asset_id = asset_id.into();
    let url = format!("/repos/{owner}/{repo}/releases/assets/{asset_id}");

    Request::<update_release_asset::Request, (), update_release_asset::Response>::builder(
      &self.config,
    )
    .patch(url)
    .build()
  }

  /// **Delete a release asset**
  ///
  ///
  /// *Documentation*: [https://docs.github.com/rest/releases/assets#delete-a-release-asset](https://docs.github.com/rest/releases/assets#delete-a-release-asset)
  pub fn delete_release_asset(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    asset_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let asset_id = asset_id.into();
    let url = format!("/repos/{owner}/{repo}/releases/assets/{asset_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Generate release notes content for a release**
  ///
  /// Generate a name and body describing a [release](https://docs.github.com/rest/releases/releases#get-a-release). The body content will be markdown formatted and contain information like the changes since last release and users who contributed. The generated release notes are not saved anywhere. They are intended to be generated and used when creating a new release.
  ///
  /// *Documentation*: [https://docs.github.com/rest/releases/releases#generate-release-notes-content-for-a-release](https://docs.github.com/rest/releases/releases#generate-release-notes-content-for-a-release)
  pub fn generate_release_notes(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<generate_release_notes::Request, (), generate_release_notes::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/releases/generate-notes");

    Request::<generate_release_notes::Request, (), generate_release_notes::Response>::builder(
      &self.config,
    )
    .post(url)
    .build()
  }

  /// **Get the latest release**
  ///
  /// View the latest published full release for the repository.
  ///
  /// The latest release is the most recent non-prerelease, non-draft release, sorted by the `created_at` attribute. The `created_at` attribute is the date of the commit used for the release, and not the date when the release was drafted or published.
  ///
  /// *Documentation*: [https://docs.github.com/rest/releases/releases#get-the-latest-release](https://docs.github.com/rest/releases/releases#get-the-latest-release)
  pub fn get_latest_release(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), (), get_latest_release::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/releases/latest");

    Request::<(), (), get_latest_release::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get a release by tag name**
  ///
  /// Get a published release with the specified tag.
  ///
  /// *Documentation*: [https://docs.github.com/rest/releases/releases#get-a-release-by-tag-name](https://docs.github.com/rest/releases/releases#get-a-release-by-tag-name)
  pub fn get_release_by_tag(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    tag: impl Into<String>,
  ) -> Request<(), (), get_release_by_tag::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let tag = tag.into();
    let url = format!("/repos/{owner}/{repo}/releases/tags/{tag}");

    Request::<(), (), get_release_by_tag::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get a release**
  ///
  /// Gets a public release with the specified release ID.
  ///
  /// **Note:** This returns an `upload_url` key corresponding to the endpoint
  /// for uploading release assets. This key is a hypermedia resource. For more information, see
  /// "[Getting started with the REST API](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#hypermedia)."
  ///
  /// *Documentation*: [https://docs.github.com/rest/releases/releases#get-a-release](https://docs.github.com/rest/releases/releases#get-a-release)
  pub fn get_release(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    release_id: impl Into<i64>,
  ) -> Request<(), (), get_release::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let release_id = release_id.into();
    let url = format!("/repos/{owner}/{repo}/releases/{release_id}");

    Request::<(), (), get_release::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Update a release**
  ///
  /// Users with push access to the repository can edit a release.
  ///
  /// *Documentation*: [https://docs.github.com/rest/releases/releases#update-a-release](https://docs.github.com/rest/releases/releases#update-a-release)
  pub fn update_release(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    release_id: impl Into<i64>,
  ) -> Request<update_release::Request, (), update_release::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let release_id = release_id.into();
    let url = format!("/repos/{owner}/{repo}/releases/{release_id}");

    Request::<update_release::Request, (), update_release::Response>::builder(&self.config)
      .patch(url)
      .build()
  }

  /// **Delete a release**
  ///
  /// Users with push access to the repository can delete a release.
  ///
  /// *Documentation*: [https://docs.github.com/rest/releases/releases#delete-a-release](https://docs.github.com/rest/releases/releases#delete-a-release)
  pub fn delete_release(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    release_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let release_id = release_id.into();
    let url = format!("/repos/{owner}/{repo}/releases/{release_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List release assets**
  ///
  ///
  /// *Documentation*: [https://docs.github.com/rest/releases/assets#list-release-assets](https://docs.github.com/rest/releases/assets#list-release-assets)
  pub fn list_release_assets(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    release_id: impl Into<i64>,
  ) -> Request<(), list_release_assets::Query, list_release_assets::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let release_id = release_id.into();
    let url = format!("/repos/{owner}/{repo}/releases/{release_id}/assets");

    Request::<(), list_release_assets::Query, list_release_assets::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Upload a release asset**
  ///
  /// This endpoint makes use of a [Hypermedia relation](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#hypermedia) to determine which URL to access. The endpoint you call to upload release assets is specific to your release. Use the `upload_url` returned in
  /// the response of the [Create a release endpoint](https://docs.github.com/rest/releases/releases#create-a-release) to upload a release asset.
  ///
  /// You need to use an HTTP client which supports [SNI](http://en.wikipedia.org/wiki/Server_Name_Indication) to make calls to this endpoint.
  ///
  /// Most libraries will set the required `Content-Length` header automatically. Use the required `Content-Type` header to provide the media type of the asset. For a list of media types, see [Media Types](https://www.iana.org/assignments/media-types/media-types.xhtml). For example:
  ///
  /// `application/zip`
  ///
  /// GitHub expects the asset data in its raw binary form, rather than JSON. You will send the raw binary content of the asset as the request body. Everything else about the endpoint is the same as the rest of the API. For example,
  /// you'll still need to pass your authentication to be able to upload an asset.
  ///
  /// When an upstream failure occurs, you will receive a `502 Bad Gateway` status. This may leave an empty asset with a state of `starter`. It can be safely deleted.
  ///
  /// **Notes:**
  /// *   GitHub renames asset filenames that have special characters, non-alphanumeric characters, and leading or trailing periods. The "[List release assets](https://docs.github.com/rest/releases/assets#list-release-assets)"
  /// endpoint lists the renamed filenames. For more information and help, contact [GitHub Support](https://support.github.com/contact?tags=dotcom-rest-api).
  /// *   To find the `release_id` query the [`GET /repos/{owner}/{repo}/releases/latest` endpoint](https://docs.github.com/rest/releases/releases#get-the-latest-release).
  /// *   If you upload an asset with the same filename as another uploaded asset, you'll receive an error and must delete the old file before you can re-upload the new asset.
  ///
  /// *Documentation*: [https://docs.github.com/rest/releases/assets#upload-a-release-asset](https://docs.github.com/rest/releases/assets#upload-a-release-asset)
  pub fn upload_release_asset(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    release_id: impl Into<i64>,
  ) -> Request<(), upload_release_asset::Query, upload_release_asset::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let release_id = release_id.into();
    let url = format!("/repos/{owner}/{repo}/releases/{release_id}/assets");

    Request::<(), upload_release_asset::Query, upload_release_asset::Response>::builder(
      &self.config,
    )
    .post(url)
    .build()
  }

  /// **Get rules for a branch**
  ///
  /// Returns all active rules that apply to the specified branch. The branch does not need to exist; rules that would apply
  /// to a branch with that name will be returned. All active rules that apply will be returned, regardless of the level
  /// at which they are configured (e.g. repository or organization). Rules in rulesets with "evaluate" or "disabled"
  /// enforcement statuses are not returned.
  ///
  /// *Documentation*: [https://docs.github.com/rest/repos/rules#get-rules-for-a-branch](https://docs.github.com/rest/repos/rules#get-rules-for-a-branch)
  pub fn get_branch_rules(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    branch: impl Into<String>,
  ) -> Request<(), get_branch_rules::Query, get_branch_rules::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let branch = branch.into();
    let url = format!("/repos/{owner}/{repo}/rules/branches/{branch}");

    Request::<(), get_branch_rules::Query, get_branch_rules::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get all repository rulesets**
  ///
  /// Get all the rulesets for a repository.
  ///
  /// *Documentation*: [https://docs.github.com/rest/repos/rules#get-all-repository-rulesets](https://docs.github.com/rest/repos/rules#get-all-repository-rulesets)
  pub fn get_repo_rulesets(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), get_repo_rulesets::Query, get_repo_rulesets::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/rulesets");

    Request::<(), get_repo_rulesets::Query, get_repo_rulesets::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create a repository ruleset**
  ///
  /// Create a ruleset for a repository.
  ///
  /// *Documentation*: [https://docs.github.com/rest/repos/rules#create-a-repository-ruleset](https://docs.github.com/rest/repos/rules#create-a-repository-ruleset)
  pub fn create_repo_ruleset(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<create_repo_ruleset::Request, (), create_repo_ruleset::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/rulesets");

    Request::<create_repo_ruleset::Request, (), create_repo_ruleset::Response>::builder(
      &self.config,
    )
    .post(url)
    .build()
  }

  /// **List repository rule suites**
  ///
  /// Lists suites of rule evaluations at the repository level.
  /// For more information, see "[Managing rulesets for a repository](https://docs.github.com/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/managing-rulesets-for-a-repository#viewing-insights-for-rulesets)."
  ///
  /// *Documentation*: [https://docs.github.com/rest/repos/rule-suites#list-repository-rule-suites](https://docs.github.com/rest/repos/rule-suites#list-repository-rule-suites)
  pub fn get_repo_rule_suites(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), get_repo_rule_suites::Query, get_repo_rule_suites::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/rulesets/rule-suites");

    Request::<(), get_repo_rule_suites::Query, get_repo_rule_suites::Response>::builder(
      &self.config,
    )
    .get(url)
    .build()
  }

  /// **Get a repository rule suite**
  ///
  /// Gets information about a suite of rule evaluations from within a repository.
  /// For more information, see "[Managing rulesets for a repository](https://docs.github.com/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/managing-rulesets-for-a-repository#viewing-insights-for-rulesets)."
  ///
  /// *Documentation*: [https://docs.github.com/rest/repos/rule-suites#get-a-repository-rule-suite](https://docs.github.com/rest/repos/rule-suites#get-a-repository-rule-suite)
  pub fn get_repo_rule_suite(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    rule_suite_id: impl Into<i64>,
  ) -> Request<(), (), get_repo_rule_suite::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let rule_suite_id = rule_suite_id.into();
    let url = format!("/repos/{owner}/{repo}/rulesets/rule-suites/{rule_suite_id}");

    Request::<(), (), get_repo_rule_suite::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get a repository ruleset**
  ///
  /// Get a ruleset for a repository.
  ///
  /// *Documentation*: [https://docs.github.com/rest/repos/rules#get-a-repository-ruleset](https://docs.github.com/rest/repos/rules#get-a-repository-ruleset)
  pub fn get_repo_ruleset(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    ruleset_id: impl Into<i64>,
  ) -> Request<(), get_repo_ruleset::Query, get_repo_ruleset::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let ruleset_id = ruleset_id.into();
    let url = format!("/repos/{owner}/{repo}/rulesets/{ruleset_id}");

    Request::<(), get_repo_ruleset::Query, get_repo_ruleset::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Update a repository ruleset**
  ///
  /// Update a ruleset for a repository.
  ///
  /// *Documentation*: [https://docs.github.com/rest/repos/rules#update-a-repository-ruleset](https://docs.github.com/rest/repos/rules#update-a-repository-ruleset)
  pub fn update_repo_ruleset(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    ruleset_id: impl Into<i64>,
  ) -> Request<update_repo_ruleset::Request, (), update_repo_ruleset::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let ruleset_id = ruleset_id.into();
    let url = format!("/repos/{owner}/{repo}/rulesets/{ruleset_id}");

    Request::<update_repo_ruleset::Request, (), update_repo_ruleset::Response>::builder(
      &self.config,
    )
    .put(url)
    .build()
  }

  /// **Delete a repository ruleset**
  ///
  /// Delete a ruleset for a repository.
  ///
  /// *Documentation*: [https://docs.github.com/rest/repos/rules#delete-a-repository-ruleset](https://docs.github.com/rest/repos/rules#delete-a-repository-ruleset)
  pub fn delete_repo_ruleset(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    ruleset_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let ruleset_id = ruleset_id.into();
    let url = format!("/repos/{owner}/{repo}/rulesets/{ruleset_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Get the weekly commit activity**
  ///
  ///
  /// Returns a weekly aggregate of the number of additions and deletions pushed to a repository.
  ///
  /// **Note:** This endpoint can only be used for repositories with fewer than 10,000 commits. If the repository contains
  /// 10,000 or more commits, a 422 status code will be returned.
  ///
  ///
  /// *Documentation*: [https://docs.github.com/rest/metrics/statistics#get-the-weekly-commit-activity](https://docs.github.com/rest/metrics/statistics#get-the-weekly-commit-activity)
  pub fn get_code_frequency_stats(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), (), get_code_frequency_stats::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/stats/code_frequency");

    Request::<(), (), get_code_frequency_stats::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get the last year of commit activity**
  ///
  /// Returns the last year of commit activity grouped by week. The `days` array is a group of commits per day, starting on `Sunday`.
  ///
  /// *Documentation*: [https://docs.github.com/rest/metrics/statistics#get-the-last-year-of-commit-activity](https://docs.github.com/rest/metrics/statistics#get-the-last-year-of-commit-activity)
  pub fn get_commit_activity_stats(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), (), get_commit_activity_stats::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/stats/commit_activity");

    Request::<(), (), get_commit_activity_stats::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get all contributor commit activity**
  ///
  ///
  /// Returns the `total` number of commits authored by the contributor. In addition, the response includes a Weekly Hash (`weeks` array) with the following information:
  ///
  /// *   `w` - Start of the week, given as a [Unix timestamp](https://en.wikipedia.org/wiki/Unix_time).
  /// *   `a` - Number of additions
  /// *   `d` - Number of deletions
  /// *   `c` - Number of commits
  ///
  /// **Note:** This endpoint will return `0` values for all addition and deletion counts in repositories with 10,000 or more commits.
  ///
  /// *Documentation*: [https://docs.github.com/rest/metrics/statistics#get-all-contributor-commit-activity](https://docs.github.com/rest/metrics/statistics#get-all-contributor-commit-activity)
  pub fn get_contributors_stats(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), (), get_contributors_stats::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/stats/contributors");

    Request::<(), (), get_contributors_stats::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get the weekly commit count**
  ///
  /// Returns the total commit counts for the `owner` and total commit counts in `all`. `all` is everyone combined, including the `owner` in the last 52 weeks. If you'd like to get the commit counts for non-owners, you can subtract `owner` from `all`.
  ///
  /// The array order is oldest week (index 0) to most recent week.
  ///
  /// The most recent week is seven days ago at UTC midnight to today at UTC midnight.
  ///
  /// *Documentation*: [https://docs.github.com/rest/metrics/statistics#get-the-weekly-commit-count](https://docs.github.com/rest/metrics/statistics#get-the-weekly-commit-count)
  pub fn get_participation_stats(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), (), get_participation_stats::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/stats/participation");

    Request::<(), (), get_participation_stats::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get the hourly commit count for each day**
  ///
  /// Each array contains the day number, hour number, and number of commits:
  ///
  /// *   `0-6`: Sunday - Saturday
  /// *   `0-23`: Hour of day
  /// *   Number of commits
  ///
  /// For example, `[2, 14, 25]` indicates that there were 25 total commits, during the 2:00pm hour on Tuesdays. All times are based on the time zone of individual commits.
  ///
  /// *Documentation*: [https://docs.github.com/rest/metrics/statistics#get-the-hourly-commit-count-for-each-day](https://docs.github.com/rest/metrics/statistics#get-the-hourly-commit-count-for-each-day)
  pub fn get_punch_card_stats(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), (), get_punch_card_stats::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/stats/punch_card");

    Request::<(), (), get_punch_card_stats::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create a commit status**
  ///
  /// Users with push access in a repository can create commit statuses for a given SHA.
  ///
  /// Note: there is a limit of 1000 statuses per `sha` and `context` within a repository. Attempts to create more than 1000 statuses will result in a validation error.
  ///
  /// *Documentation*: [https://docs.github.com/rest/commits/statuses#create-a-commit-status](https://docs.github.com/rest/commits/statuses#create-a-commit-status)
  pub fn create_commit_status(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    sha: impl Into<String>,
  ) -> Request<create_commit_status::Request, (), create_commit_status::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let sha = sha.into();
    let url = format!("/repos/{owner}/{repo}/statuses/{sha}");

    Request::<create_commit_status::Request, (), create_commit_status::Response>::builder(
      &self.config,
    )
    .post(url)
    .build()
  }

  /// **List repository tags**
  ///
  ///
  /// *Documentation*: [https://docs.github.com/rest/repos/repos#list-repository-tags](https://docs.github.com/rest/repos/repos#list-repository-tags)
  pub fn list_tags(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), list_tags::Query, list_tags::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/tags");

    Request::<(), list_tags::Query, list_tags::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List tag protection states for a repository**
  ///
  /// This returns the tag protection states of a repository.
  ///
  /// This information is only available to repository administrators.
  ///
  /// *Documentation*: [https://docs.github.com/rest/repos/tags#list-tag-protection-states-for-a-repository](https://docs.github.com/rest/repos/tags#list-tag-protection-states-for-a-repository)
  pub fn list_tag_protection(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), (), list_tag_protection::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/tags/protection");

    Request::<(), (), list_tag_protection::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create a tag protection state for a repository**
  ///
  /// This creates a tag protection state for a repository.
  /// This endpoint is only available to repository administrators.
  ///
  /// *Documentation*: [https://docs.github.com/rest/repos/tags#create-a-tag-protection-state-for-a-repository](https://docs.github.com/rest/repos/tags#create-a-tag-protection-state-for-a-repository)
  pub fn create_tag_protection(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<create_tag_protection::Request, (), create_tag_protection::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/tags/protection");

    Request::<create_tag_protection::Request, (), create_tag_protection::Response>::builder(
      &self.config,
    )
    .post(url)
    .build()
  }

  /// **Delete a tag protection state for a repository**
  ///
  /// This deletes a tag protection state for a repository.
  /// This endpoint is only available to repository administrators.
  ///
  /// *Documentation*: [https://docs.github.com/rest/repos/tags#delete-a-tag-protection-state-for-a-repository](https://docs.github.com/rest/repos/tags#delete-a-tag-protection-state-for-a-repository)
  pub fn delete_tag_protection(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    tag_protection_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let tag_protection_id = tag_protection_id.into();
    let url = format!("/repos/{owner}/{repo}/tags/protection/{tag_protection_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Download a repository archive (tar)**
  ///
  /// Gets a redirect URL to download a tar archive for a repository. If you omit `:ref`, the repository’s default branch (usually
  /// `main`) will be used. Please make sure your HTTP framework is configured to follow redirects or you will need to use
  /// the `Location` header to make a second `GET` request.
  /// **Note**: For private repositories, these links are temporary and expire after five minutes.
  ///
  /// *Documentation*: [https://docs.github.com/rest/repos/contents#download-a-repository-archive-tar](https://docs.github.com/rest/repos/contents#download-a-repository-archive-tar)
  pub fn download_tarball_archive(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    ref_: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let ref_ = ref_.into();
    let url = format!("/repos/{owner}/{repo}/tarball/{ref_}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List repository teams**
  ///
  /// Lists the teams that have access to the specified repository and that are also visible to the authenticated user.
  ///
  /// For a public repository, a team is listed only if that team added the public repository explicitly.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `public_repo` or `repo` scope to use this endpoint with a public repository, and `repo` scope to use this endpoint with a private repository.
  ///
  /// *Documentation*: [https://docs.github.com/rest/repos/repos#list-repository-teams](https://docs.github.com/rest/repos/repos#list-repository-teams)
  pub fn list_teams(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), list_teams::Query, list_teams::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/teams");

    Request::<(), list_teams::Query, list_teams::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get all repository topics**
  ///
  ///
  /// *Documentation*: [https://docs.github.com/rest/repos/repos#get-all-repository-topics](https://docs.github.com/rest/repos/repos#get-all-repository-topics)
  pub fn get_all_topics(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), get_all_topics::Query, get_all_topics::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/topics");

    Request::<(), get_all_topics::Query, get_all_topics::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Replace all repository topics**
  ///
  ///
  /// *Documentation*: [https://docs.github.com/rest/repos/repos#replace-all-repository-topics](https://docs.github.com/rest/repos/repos#replace-all-repository-topics)
  pub fn replace_all_topics(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<replace_all_topics::Request, (), replace_all_topics::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/topics");

    Request::<replace_all_topics::Request, (), replace_all_topics::Response>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **Get repository clones**
  ///
  /// Get the total number of clones and breakdown per day or week for the last 14 days. Timestamps are aligned to UTC midnight of the beginning of the day or week. Week begins on Monday.
  ///
  /// *Documentation*: [https://docs.github.com/rest/metrics/traffic#get-repository-clones](https://docs.github.com/rest/metrics/traffic#get-repository-clones)
  pub fn get_clones(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), get_clones::Query, get_clones::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/traffic/clones");

    Request::<(), get_clones::Query, get_clones::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get top referral paths**
  ///
  /// Get the top 10 popular contents over the last 14 days.
  ///
  /// *Documentation*: [https://docs.github.com/rest/metrics/traffic#get-top-referral-paths](https://docs.github.com/rest/metrics/traffic#get-top-referral-paths)
  pub fn get_top_paths(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), (), get_top_paths::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/traffic/popular/paths");

    Request::<(), (), get_top_paths::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get top referral sources**
  ///
  /// Get the top 10 referrers over the last 14 days.
  ///
  /// *Documentation*: [https://docs.github.com/rest/metrics/traffic#get-top-referral-sources](https://docs.github.com/rest/metrics/traffic#get-top-referral-sources)
  pub fn get_top_referrers(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), (), get_top_referrers::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/traffic/popular/referrers");

    Request::<(), (), get_top_referrers::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get page views**
  ///
  /// Get the total number of views and breakdown per day or week for the last 14 days. Timestamps are aligned to UTC midnight of the beginning of the day or week. Week begins on Monday.
  ///
  /// *Documentation*: [https://docs.github.com/rest/metrics/traffic#get-page-views](https://docs.github.com/rest/metrics/traffic#get-page-views)
  pub fn get_views(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), get_views::Query, get_views::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/traffic/views");

    Request::<(), get_views::Query, get_views::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Transfer a repository**
  ///
  /// A transfer request will need to be accepted by the new owner when transferring a personal repository to another user. The response will contain the original `owner`, and the transfer will continue asynchronously. For more details on the requirements to transfer personal and organization-owned repositories, see [about repository transfers](https://docs.github.com/articles/about-repository-transfers/).
  ///
  /// *Documentation*: [https://docs.github.com/rest/repos/repos#transfer-a-repository](https://docs.github.com/rest/repos/repos#transfer-a-repository)
  pub fn transfer(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<transfer::Request, (), transfer::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/transfer");

    Request::<transfer::Request, (), transfer::Response>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Check if vulnerability alerts are enabled for a repository**
  ///
  /// Shows whether dependency alerts are enabled or disabled for a repository. The authenticated user must have admin read access to the repository. For more information, see "[About security alerts for vulnerable dependencies](https://docs.github.com/articles/about-security-alerts-for-vulnerable-dependencies)".
  ///
  /// *Documentation*: [https://docs.github.com/rest/repos/repos#check-if-vulnerability-alerts-are-enabled-for-a-repository](https://docs.github.com/rest/repos/repos#check-if-vulnerability-alerts-are-enabled-for-a-repository)
  pub fn check_vulnerability_alerts(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/vulnerability-alerts");

    NoContentRequest::<(), ()>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Enable vulnerability alerts**
  ///
  /// Enables dependency alerts and the dependency graph for a repository. The authenticated user must have admin access to the repository. For more information, see "[About security alerts for vulnerable dependencies](https://docs.github.com/articles/about-security-alerts-for-vulnerable-dependencies)".
  ///
  /// *Documentation*: [https://docs.github.com/rest/repos/repos#enable-vulnerability-alerts](https://docs.github.com/rest/repos/repos#enable-vulnerability-alerts)
  pub fn enable_vulnerability_alerts(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/vulnerability-alerts");

    NoContentRequest::<(), ()>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **Disable vulnerability alerts**
  ///
  /// Disables dependency alerts and the dependency graph for a repository.
  /// The authenticated user must have admin access to the repository. For more information,
  /// see "[About security alerts for vulnerable dependencies](https://docs.github.com/articles/about-security-alerts-for-vulnerable-dependencies)".
  ///
  /// *Documentation*: [https://docs.github.com/rest/repos/repos#disable-vulnerability-alerts](https://docs.github.com/rest/repos/repos#disable-vulnerability-alerts)
  pub fn disable_vulnerability_alerts(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/vulnerability-alerts");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Download a repository archive (zip)**
  ///
  /// Gets a redirect URL to download a zip archive for a repository. If you omit `:ref`, the repository’s default branch (usually
  /// `main`) will be used. Please make sure your HTTP framework is configured to follow redirects or you will need to use
  /// the `Location` header to make a second `GET` request.
  ///
  /// **Note**: For private repositories, these links are temporary and expire after five minutes. If the repository is empty, you will receive a 404 when you follow the redirect.
  ///
  /// *Documentation*: [https://docs.github.com/rest/repos/contents#download-a-repository-archive-zip](https://docs.github.com/rest/repos/contents#download-a-repository-archive-zip)
  pub fn download_zipball_archive(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    ref_: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let ref_ = ref_.into();
    let url = format!("/repos/{owner}/{repo}/zipball/{ref_}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create a repository using a template**
  ///
  /// Creates a new repository using a repository template. Use the `template_owner` and `template_repo` route parameters to specify the repository to use as the template. If the repository is not public, the authenticated user must own or be a member of an organization that owns the repository. To check if a repository is available to use as a template, get the repository's information using the [Get a repository](https://docs.github.com/rest/repos/repos#get-a-repository) endpoint and check that the `is_template` key is `true`.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `public_repo` or `repo` scope to create a public repository, and `repo` scope to create a private repository.
  ///
  /// *Documentation*: [https://docs.github.com/rest/repos/repos#create-a-repository-using-a-template](https://docs.github.com/rest/repos/repos#create-a-repository-using-a-template)
  pub fn create_using_template(
    &self,
    template_owner: impl Into<String>,
    template_repo: impl Into<String>,
  ) -> Request<create_using_template::Request, (), create_using_template::Response> {
    let template_owner = template_owner.into();
    let template_repo = template_repo.into();
    let url = format!("/repos/{template_owner}/{template_repo}/generate");

    Request::<create_using_template::Request, (), create_using_template::Response>::builder(
      &self.config,
    )
    .post(url)
    .build()
  }

  /// **List public repositories**
  ///
  /// Lists all public repositories in the order that they were created.
  ///
  /// Note:
  /// - For GitHub Enterprise Server, this endpoint will only list repositories available to all users on the enterprise.
  /// - Pagination is powered exclusively by the `since` parameter. Use the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers) to get the URL for the next page of repositories.
  ///
  /// *Documentation*: [https://docs.github.com/rest/repos/repos#list-public-repositories](https://docs.github.com/rest/repos/repos#list-public-repositories)
  pub fn list_public(&self) -> Request<(), list_public::Query, list_public::Response> {
    let url = format!("/repositories");

    Request::<(), list_public::Query, list_public::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List repositories for the authenticated user**
  ///
  /// Lists repositories that the authenticated user has explicit permission (`:read`, `:write`, or `:admin`) to access.
  ///
  /// The authenticated user has explicit permission to access repositories they own, repositories where they are a collaborator, and repositories that they can access through an organization membership.
  ///
  /// *Documentation*: [https://docs.github.com/rest/repos/repos#list-repositories-for-the-authenticated-user](https://docs.github.com/rest/repos/repos#list-repositories-for-the-authenticated-user)
  pub fn list_for_authenticated_user(
    &self,
  ) -> Request<(), list_for_authenticated_user::Query, list_for_authenticated_user::Response> {
    let url = format!("/user/repos");

    Request::<(), list_for_authenticated_user::Query, list_for_authenticated_user::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create a repository for the authenticated user**
  ///
  /// Creates a new repository for the authenticated user.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `public_repo` or `repo` scope to create a public repository, and `repo` scope to create a private repository.
  ///
  /// *Documentation*: [https://docs.github.com/rest/repos/repos#create-a-repository-for-the-authenticated-user](https://docs.github.com/rest/repos/repos#create-a-repository-for-the-authenticated-user)
  pub fn create_for_authenticated_user(
    &self,
  ) -> Request<create_for_authenticated_user::Request, (), create_for_authenticated_user::Response>
  {
    let url = format!("/user/repos");

    Request::<create_for_authenticated_user::Request, (), create_for_authenticated_user::Response>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **List repository invitations for the authenticated user**
  ///
  /// When authenticating as a user, this endpoint will list all currently open repository invitations for that user.
  ///
  /// *Documentation*: [https://docs.github.com/rest/collaborators/invitations#list-repository-invitations-for-the-authenticated-user](https://docs.github.com/rest/collaborators/invitations#list-repository-invitations-for-the-authenticated-user)
  pub fn list_invitations_for_authenticated_user(
    &self,
  ) -> Request<
    (),
    list_invitations_for_authenticated_user::Query,
    list_invitations_for_authenticated_user::Response,
  > {
    let url = format!("/user/repository_invitations");

    Request::<
      (),
      list_invitations_for_authenticated_user::Query,
      list_invitations_for_authenticated_user::Response,
    >::builder(&self.config)
    .get(url)
    .build()
  }

  /// **Accept a repository invitation**
  ///
  ///
  /// *Documentation*: [https://docs.github.com/rest/collaborators/invitations#accept-a-repository-invitation](https://docs.github.com/rest/collaborators/invitations#accept-a-repository-invitation)
  pub fn accept_invitation_for_authenticated_user(
    &self,
    invitation_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let invitation_id = invitation_id.into();
    let url = format!("/user/repository_invitations/{invitation_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .patch(url)
      .build()
  }

  /// **Decline a repository invitation**
  ///
  ///
  /// *Documentation*: [https://docs.github.com/rest/collaborators/invitations#decline-a-repository-invitation](https://docs.github.com/rest/collaborators/invitations#decline-a-repository-invitation)
  pub fn decline_invitation_for_authenticated_user(
    &self,
    invitation_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let invitation_id = invitation_id.into();
    let url = format!("/user/repository_invitations/{invitation_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List repositories for a user**
  ///
  /// Lists public repositories for the specified user.
  ///
  /// *Documentation*: [https://docs.github.com/rest/repos/repos#list-repositories-for-a-user](https://docs.github.com/rest/repos/repos#list-repositories-for-a-user)
  pub fn list_for_user(
    &self,
    username: impl Into<String>,
  ) -> Request<(), list_for_user::Query, list_for_user::Response> {
    let username = username.into();
    let url = format!("/users/{username}/repos");

    Request::<(), list_for_user::Query, list_for_user::Response>::builder(&self.config)
      .get(url)
      .build()
  }
}
