use octocrate_core::*;
#[allow(unused_imports)]
use octocrate_types::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use typed_builder::TypedBuilder;

pub mod get_actions_cache_usage_for_org {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = ActionsCacheUsageOrgEnterprise;
}

pub mod get_actions_cache_usage_by_repo_for_org {
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
    pub repository_cache_usages: Vec<ActionsCacheUsageByRepository>,
    pub total_count: i64,
  }
}

pub mod get_github_actions_permissions_organization {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = ActionsOrganizationPermissions;
}

pub mod set_github_actions_permissions_organization {
  #[allow(unused_imports)]
  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub allowed_actions: Option<AllowedActions>,
    pub enabled_repositories: EnabledRepositories,
  }
}

pub mod list_selected_repositories_enabled_github_actions_organization {
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
    pub repositories: Vec<Repository>,
    pub total_count: f64,
  }
}

pub mod set_selected_repositories_enabled_github_actions_organization {
  #[allow(unused_imports)]
  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// List of repository IDs to enable for GitHub Actions.
    pub selected_repository_ids: Vec<i64>,
  }
}

pub mod get_allowed_actions_organization {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = SelectedActions;
}

pub mod set_allowed_actions_organization {
  #[allow(unused_imports)]
  use super::*;

  pub type Request = SelectedActions;
}

pub mod get_github_actions_default_workflow_permissions_organization {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = ActionsGetDefaultWorkflowPermissions;
}

pub mod set_github_actions_default_workflow_permissions_organization {
  #[allow(unused_imports)]
  use super::*;

  pub type Request = ActionsSetDefaultWorkflowPermissions;
}

pub mod list_self_hosted_runners_for_org {
  #[allow(unused_imports)]
  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The name of a self-hosted runner.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub name: Option<String>,
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
    pub runners: Vec<Runner>,
    pub total_count: i64,
  }
}

pub mod list_runner_applications_for_org {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<RunnerApplication>;
}

pub mod generate_runner_jitconfig_for_org {
  #[allow(unused_imports)]
  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The names of the custom labels to add to the runner. **Minimum items**: 1. **Maximum items**: 100.
    pub labels: Vec<String>,
    /// The name of the new runner.
    pub name: String,
    /// The ID of the runner group to register the runner to.
    pub runner_group_id: i64,
    /// The working directory to be used for job execution, relative to the runner install directory.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub work_folder: Option<String>,
  }
}

pub mod create_registration_token_for_org {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = AuthenticationToken;
}

pub mod create_remove_token_for_org {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = AuthenticationToken;
}

pub mod get_self_hosted_runner_for_org {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Runner;
}

pub mod add_custom_labels_to_self_hosted_runner_for_org {
  #[allow(unused_imports)]
  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The names of the custom labels to add to the runner.
    pub labels: Vec<String>,
  }
}

pub mod set_custom_labels_for_self_hosted_runner_for_org {
  #[allow(unused_imports)]
  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The names of the custom labels to set for the runner. You can pass an empty array to remove all custom labels.
    pub labels: Vec<String>,
  }
}

pub mod list_org_secrets {
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
    pub secrets: Vec<OrganizationActionsSecret>,
    pub total_count: i64,
  }
}

pub mod get_org_public_key {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = ActionsPublicKey;
}

pub mod get_org_secret {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = OrganizationActionsSecret;
}

pub mod create_or_update_org_secret {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = EmptyObject;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// Value for your secret, encrypted with [LibSodium](https://libsodium.gitbook.io/doc/bindings_for_other_languages) using the public key retrieved from the [Get an organization public key](https://docs.github.com/rest/actions/secrets#get-an-organization-public-key) endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub encrypted_value: Option<String>,
    /// ID of the key you used to encrypt the secret.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub key_id: Option<String>,
    /// An array of repository ids that can access the organization secret. You can only provide a list of repository ids when the `visibility` is set to `selected`. You can manage the list of selected repositories using the [List selected repositories for an organization secret](https://docs.github.com/rest/actions/secrets#list-selected-repositories-for-an-organization-secret), [Set selected repositories for an organization secret](https://docs.github.com/rest/actions/secrets#set-selected-repositories-for-an-organization-secret), and [Remove selected repository from an organization secret](https://docs.github.com/rest/actions/secrets#remove-selected-repository-from-an-organization-secret) endpoints.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub selected_repository_ids: Option<Vec<i64>>,
    /// Which type of organization repositories have access to the organization secret. `selected` means only the repositories specified by `selected_repository_ids` can access the secret.
    pub visibility: Visibility,
  }
}

pub mod list_selected_repos_for_org_secret {
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
    pub repositories: Vec<MinimalRepository>,
    pub total_count: i64,
  }
}

pub mod set_selected_repos_for_org_secret {
  #[allow(unused_imports)]
  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// An array of repository ids that can access the organization secret. You can only provide a list of repository ids when the `visibility` is set to `selected`. You can add and remove individual repositories using the [Add selected repository to an organization secret](https://docs.github.com/rest/actions/secrets#add-selected-repository-to-an-organization-secret) and [Remove selected repository from an organization secret](https://docs.github.com/rest/actions/secrets#remove-selected-repository-from-an-organization-secret) endpoints.
    pub selected_repository_ids: Vec<i64>,
  }
}

pub mod list_org_variables {
  #[allow(unused_imports)]
  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The number of results per page (max 30). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
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
    pub total_count: i64,
    pub variables: Vec<OrganizationActionsVariable>,
  }
}

pub mod create_org_variable {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = EmptyObject;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The name of the variable.
    pub name: String,
    /// An array of repository ids that can access the organization variable. You can only provide a list of repository ids when the `visibility` is set to `selected`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub selected_repository_ids: Option<Vec<i64>>,
    /// The value of the variable.
    pub value: String,
    /// The type of repositories in the organization that can access the variable. `selected` means only the repositories specified by `selected_repository_ids` can access the variable.
    pub visibility: Visibility,
  }
}

pub mod get_org_variable {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = OrganizationActionsVariable;
}

pub mod update_org_variable {
  #[allow(unused_imports)]
  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The name of the variable.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub name: Option<String>,
    /// An array of repository ids that can access the organization variable. You can only provide a list of repository ids when the `visibility` is set to `selected`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub selected_repository_ids: Option<Vec<i64>>,
    /// The value of the variable.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub value: Option<String>,
    /// The type of repositories in the organization that can access the variable. `selected` means only the repositories specified by `selected_repository_ids` can access the variable.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub visibility: Option<Visibility>,
  }
}

pub mod list_selected_repos_for_org_variable {
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
    pub repositories: Vec<MinimalRepository>,
    pub total_count: i64,
  }
}

pub mod set_selected_repos_for_org_variable {
  #[allow(unused_imports)]
  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The IDs of the repositories that can access the organization variable.
    pub selected_repository_ids: Vec<i64>,
  }
}

pub mod list_artifacts_for_repo {
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
    /// The name field of an artifact. When specified, only artifacts with this name will be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub name: Option<String>,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Response {
    pub artifacts: Vec<Artifact>,
    pub total_count: i64,
  }
}

pub mod get_artifact {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Artifact;
}

pub mod get_actions_cache_usage {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = ActionsCacheUsageByRepository;
}

pub mod get_actions_cache_list {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = ActionsCacheList;

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
    /// The full Git reference for narrowing down the cache. The `ref` for a branch should be formatted as `refs/heads/<branch name>`. To reference a pull request use `refs/pull/<number>/merge`.
    #[serde(rename = "ref")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub ref_: Option<String>,
    /// An explicit key or prefix for identifying the cache
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub key: Option<String>,
    /// The property to sort the results by. `created_at` means when the cache was created. `last_accessed_at` means when the cache was last accessed. `size_in_bytes` is the size of the cache in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub sort: Option<parameters::ActionsCacheListSort>,
    /// The direction to sort the results by.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub direction: Option<parameters::Direction>,
  }
}

pub mod delete_actions_cache_by_key {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = ActionsCacheList;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// A key for identifying the cache.
    pub key: String,
    /// The full Git reference for narrowing down the cache. The `ref` for a branch should be formatted as `refs/heads/<branch name>`. To reference a pull request use `refs/pull/<number>/merge`.
    #[serde(rename = "ref")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub ref_: Option<String>,
  }
}

pub mod get_job_for_workflow_run {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Job;
}

pub mod re_run_job_for_workflow_run {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = EmptyObject;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// Whether to enable debug logging for the re-run.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub enable_debug_logging: Option<bool>,
  }
}

pub mod get_custom_oidc_sub_claim_for_repo {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = OidcCustomSubRepo;
}

pub mod set_custom_oidc_sub_claim_for_repo {
  #[allow(unused_imports)]
  use super::*;

  pub type Request = ActionsOidcSubjectCustomizationForARepository;
  pub type Response = EmptyObject;

  /// Actions OIDC subject customization for a repository
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct ActionsOidcSubjectCustomizationForARepository {
    /// Array of unique strings. Each claim key can only contain alphanumeric characters and underscores.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub include_claim_keys: Option<Vec<String>>,
    /// Whether to use the default template or not. If `true`, the `include_claim_keys` field is ignored.
    pub use_default: bool,
  }
}

pub mod list_repo_organization_secrets {
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
    pub secrets: Vec<ActionsSecret>,
    pub total_count: i64,
  }
}

pub mod list_repo_organization_variables {
  #[allow(unused_imports)]
  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The number of results per page (max 30). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
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
    pub total_count: i64,
    pub variables: Vec<ActionsVariable>,
  }
}

pub mod get_github_actions_permissions_repository {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = ActionsRepositoryPermissions;
}

pub mod set_github_actions_permissions_repository {
  #[allow(unused_imports)]
  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub allowed_actions: Option<AllowedActions>,
    pub enabled: bool,
  }
}

pub mod get_workflow_access_to_repository {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = ActionsWorkflowAccessToRepository;
}

pub mod set_workflow_access_to_repository {
  #[allow(unused_imports)]
  use super::*;

  pub type Request = ActionsWorkflowAccessToRepository;
}

pub mod get_allowed_actions_repository {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = SelectedActions;
}

pub mod set_allowed_actions_repository {
  #[allow(unused_imports)]
  use super::*;

  pub type Request = SelectedActions;
}

pub mod get_github_actions_default_workflow_permissions_repository {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = ActionsGetDefaultWorkflowPermissions;
}

pub mod set_github_actions_default_workflow_permissions_repository {
  #[allow(unused_imports)]
  use super::*;

  pub type Request = ActionsSetDefaultWorkflowPermissions;
}

pub mod list_self_hosted_runners_for_repo {
  #[allow(unused_imports)]
  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The name of a self-hosted runner.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub name: Option<String>,
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
    pub runners: Vec<Runner>,
    pub total_count: i64,
  }
}

pub mod list_runner_applications_for_repo {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<RunnerApplication>;
}

pub mod generate_runner_jitconfig_for_repo {
  #[allow(unused_imports)]
  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The names of the custom labels to add to the runner. **Minimum items**: 1. **Maximum items**: 100.
    pub labels: Vec<String>,
    /// The name of the new runner.
    pub name: String,
    /// The ID of the runner group to register the runner to.
    pub runner_group_id: i64,
    /// The working directory to be used for job execution, relative to the runner install directory.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub work_folder: Option<String>,
  }
}

pub mod create_registration_token_for_repo {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = AuthenticationToken;
}

pub mod create_remove_token_for_repo {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = AuthenticationToken;
}

pub mod get_self_hosted_runner_for_repo {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Runner;
}

pub mod add_custom_labels_to_self_hosted_runner_for_repo {
  #[allow(unused_imports)]
  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The names of the custom labels to add to the runner.
    pub labels: Vec<String>,
  }
}

pub mod set_custom_labels_for_self_hosted_runner_for_repo {
  #[allow(unused_imports)]
  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The names of the custom labels to set for the runner. You can pass an empty array to remove all custom labels.
    pub labels: Vec<String>,
  }
}

pub mod list_workflow_runs_for_repo {
  #[allow(unused_imports)]
  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// Returns someone's workflow runs. Use the login for the user who created the `push` associated with the check suite or workflow run.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub actor: Option<String>,
    /// Returns workflow runs associated with a branch. Use the name of the branch of the `push`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub branch: Option<String>,
    /// Returns workflow run triggered by the event you specify. For example, `push`, `pull_request` or `issue`. For more information, see "[Events that trigger workflows](https://docs.github.com/actions/automating-your-workflow-with-github-actions/events-that-trigger-workflows)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub event: Option<String>,
    /// Returns workflow runs with the check run `status` or `conclusion` that you specify. For example, a conclusion can be `success` or a status can be `in_progress`. Only GitHub Actions can set a status of `waiting`, `pending`, or `requested`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub status: Option<parameters::WorkflowRunStatus>,
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
    /// Returns workflow runs created within the given date-time range. For more information on the syntax, see "[Understanding the search syntax](https://docs.github.com/search-github/getting-started-with-searching-on-github/understanding-the-search-syntax#query-for-dates)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub created: Option<String>,
    /// If `true` pull requests are omitted from the response (empty array).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub exclude_pull_requests: Option<bool>,
    /// Returns workflow runs with the `check_suite_id` that you specify.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub check_suite_id: Option<i64>,
    /// Only returns workflow runs that are associated with the specified `head_sha`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub head_sha: Option<String>,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Response {
    pub total_count: i64,
    pub workflow_runs: Vec<WorkflowRun>,
  }
}

pub mod get_workflow_run {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = WorkflowRun;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// If `true` pull requests are omitted from the response (empty array).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub exclude_pull_requests: Option<bool>,
  }
}

pub mod get_reviews_for_run {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<EnvironmentApprovals>;
}

pub mod approve_workflow_run {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = EmptyObject;
}

pub mod list_workflow_run_artifacts {
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
    /// The name field of an artifact. When specified, only artifacts with this name will be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub name: Option<String>,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Response {
    pub artifacts: Vec<Artifact>,
    pub total_count: i64,
  }
}

pub mod get_workflow_run_attempt {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = WorkflowRun;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// If `true` pull requests are omitted from the response (empty array).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub exclude_pull_requests: Option<bool>,
  }
}

pub mod list_jobs_for_workflow_run_attempt {
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
    pub jobs: Vec<Job>,
    pub total_count: i64,
  }
}

pub mod cancel_workflow_run {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = EmptyObject;
}

pub mod review_custom_gates_for_run {
  #[allow(unused_imports)]
  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize)]
  #[serde(untagged)]
  pub enum Request {
    ReviewCustomGatesCommentRequired(ReviewCustomGatesCommentRequired),
    ReviewCustomGatesStateRequired(ReviewCustomGatesStateRequired),
  }
}

pub mod force_cancel_workflow_run {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = EmptyObject;
}

pub mod list_jobs_for_workflow_run {
  #[allow(unused_imports)]
  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QueryFilter {
    #[serde(rename = "latest")]
    Latest,
    #[serde(rename = "all")]
    All,
  }

  impl ToString for QueryFilter {
    fn to_string(&self) -> String {
      match self {
        QueryFilter::Latest => "latest".to_string(),
        QueryFilter::All => "all".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// Filters jobs by their `completed_at` timestamp. `latest` returns jobs from the most recent execution of the workflow run. `all` returns all jobs for a workflow run, including from old executions of the workflow run.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub filter: Option<QueryFilter>,
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
    pub jobs: Vec<Job>,
    pub total_count: i64,
  }
}

pub mod get_pending_deployments_for_run {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<PendingDeployment>;
}

pub mod review_pending_deployments_for_run {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Deployment>;

  /// Whether to approve or reject deployment to the specified environments.
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum RequestState {
    #[serde(rename = "approved")]
    Approved,
    #[serde(rename = "rejected")]
    Rejected,
  }

  impl ToString for RequestState {
    fn to_string(&self) -> String {
      match self {
        RequestState::Approved => "approved".to_string(),
        RequestState::Rejected => "rejected".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// A comment to accompany the deployment review
    pub comment: String,
    /// The list of environment ids to approve or reject
    pub environment_ids: Vec<i64>,
    /// Whether to approve or reject deployment to the specified environments.
    pub state: RequestState,
  }
}

pub mod re_run_workflow {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = EmptyObject;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// Whether to enable debug logging for the re-run.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub enable_debug_logging: Option<bool>,
  }
}

pub mod re_run_workflow_failed_jobs {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = EmptyObject;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// Whether to enable debug logging for the re-run.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub enable_debug_logging: Option<bool>,
  }
}

pub mod get_workflow_run_usage {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = WorkflowRunUsage;
}

pub mod list_repo_secrets {
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
    pub secrets: Vec<ActionsSecret>,
    pub total_count: i64,
  }
}

pub mod get_repo_public_key {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = ActionsPublicKey;
}

pub mod get_repo_secret {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = ActionsSecret;
}

pub mod create_or_update_repo_secret {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = EmptyObject;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// Value for your secret, encrypted with [LibSodium](https://libsodium.gitbook.io/doc/bindings_for_other_languages) using the public key retrieved from the [Get a repository public key](https://docs.github.com/rest/actions/secrets#get-a-repository-public-key) endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub encrypted_value: Option<String>,
    /// ID of the key you used to encrypt the secret.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub key_id: Option<String>,
  }
}

pub mod list_repo_variables {
  #[allow(unused_imports)]
  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The number of results per page (max 30). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
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
    pub total_count: i64,
    pub variables: Vec<ActionsVariable>,
  }
}

pub mod create_repo_variable {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = EmptyObject;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The name of the variable.
    pub name: String,
    /// The value of the variable.
    pub value: String,
  }
}

pub mod get_repo_variable {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = ActionsVariable;
}

pub mod update_repo_variable {
  #[allow(unused_imports)]
  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The name of the variable.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub name: Option<String>,
    /// The value of the variable.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub value: Option<String>,
  }
}

pub mod list_repo_workflows {
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
    pub total_count: i64,
    pub workflows: Vec<Workflow>,
  }
}

pub mod get_workflow {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Workflow;
}

pub mod create_workflow_dispatch {
  #[allow(unused_imports)]
  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// Input keys and values configured in the workflow file. The maximum number of properties is 10. Any default properties configured in the workflow file will be used when `inputs` are omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub inputs: Option<serde_json::Value>,
    /// The git reference for the workflow. The reference can be a branch or tag name.
    #[serde(rename = "ref")]
    pub ref_: String,
  }
}

pub mod list_workflow_runs {
  #[allow(unused_imports)]
  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// Returns someone's workflow runs. Use the login for the user who created the `push` associated with the check suite or workflow run.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub actor: Option<String>,
    /// Returns workflow runs associated with a branch. Use the name of the branch of the `push`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub branch: Option<String>,
    /// Returns workflow run triggered by the event you specify. For example, `push`, `pull_request` or `issue`. For more information, see "[Events that trigger workflows](https://docs.github.com/actions/automating-your-workflow-with-github-actions/events-that-trigger-workflows)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub event: Option<String>,
    /// Returns workflow runs with the check run `status` or `conclusion` that you specify. For example, a conclusion can be `success` or a status can be `in_progress`. Only GitHub Actions can set a status of `waiting`, `pending`, or `requested`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub status: Option<parameters::WorkflowRunStatus>,
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
    /// Returns workflow runs created within the given date-time range. For more information on the syntax, see "[Understanding the search syntax](https://docs.github.com/search-github/getting-started-with-searching-on-github/understanding-the-search-syntax#query-for-dates)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub created: Option<String>,
    /// If `true` pull requests are omitted from the response (empty array).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub exclude_pull_requests: Option<bool>,
    /// Returns workflow runs with the `check_suite_id` that you specify.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub check_suite_id: Option<i64>,
    /// Only returns workflow runs that are associated with the specified `head_sha`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub head_sha: Option<String>,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Response {
    pub total_count: i64,
    pub workflow_runs: Vec<WorkflowRun>,
  }
}

pub mod get_workflow_usage {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = WorkflowUsage;
}

pub mod list_environment_secrets {
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
    pub secrets: Vec<ActionsSecret>,
    pub total_count: i64,
  }
}

pub mod get_environment_public_key {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = ActionsPublicKey;
}

pub mod get_environment_secret {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = ActionsSecret;
}

pub mod create_or_update_environment_secret {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = EmptyObject;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// Value for your secret, encrypted with [LibSodium](https://libsodium.gitbook.io/doc/bindings_for_other_languages) using the public key retrieved from the [Get an environment public key](https://docs.github.com/rest/actions/secrets#get-an-environment-public-key) endpoint.
    pub encrypted_value: String,
    /// ID of the key you used to encrypt the secret.
    pub key_id: String,
  }
}

pub mod list_environment_variables {
  #[allow(unused_imports)]
  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The number of results per page (max 30). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
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
    pub total_count: i64,
    pub variables: Vec<ActionsVariable>,
  }
}

pub mod create_environment_variable {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = EmptyObject;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The name of the variable.
    pub name: String,
    /// The value of the variable.
    pub value: String,
  }
}

pub mod get_environment_variable {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = ActionsVariable;
}

pub mod update_environment_variable {
  #[allow(unused_imports)]
  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The name of the variable.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub name: Option<String>,
    /// The value of the variable.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub value: Option<String>,
  }
}

/// Endpoints to manage GitHub Actions using the REST API.
pub struct GitHubActionsAPI {
  config: SharedAPIConfig,
}

impl GitHubActionsAPI {
  pub fn new(config: &SharedAPIConfig) -> Self {
    Self {
      config: config.clone(),
    }
  }

  /// **Get GitHub Actions cache usage for an organization**
  ///
  /// Gets the total GitHub Actions cache usage for an organization.
  /// The data fetched using this API is refreshed approximately every 5 minutes, so values returned from this endpoint may take at least 5 minutes to get updated.
  ///
  /// OAuth tokens and personal access tokens (classic) need the `read:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/cache#get-github-actions-cache-usage-for-an-organization](https://docs.github.com/rest/actions/cache#get-github-actions-cache-usage-for-an-organization)
  pub fn get_actions_cache_usage_for_org(
    &self,
    org: impl Into<String>,
  ) -> Request<(), (), get_actions_cache_usage_for_org::Response> {
    let org = org.into();
    let url = format!("/orgs/{org}/actions/cache/usage");

    Request::<(), (), get_actions_cache_usage_for_org::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List repositories with GitHub Actions cache usage for an organization**
  ///
  /// Lists repositories and their GitHub Actions cache usage for an organization.
  /// The data fetched using this API is refreshed approximately every 5 minutes, so values returned from this endpoint may take at least 5 minutes to get updated.
  ///
  /// OAuth tokens and personal access tokens (classic) need the `read:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/cache#list-repositories-with-github-actions-cache-usage-for-an-organization](https://docs.github.com/rest/actions/cache#list-repositories-with-github-actions-cache-usage-for-an-organization)
  pub fn get_actions_cache_usage_by_repo_for_org(
    &self,
    org: impl Into<String>,
  ) -> Request<
    (),
    get_actions_cache_usage_by_repo_for_org::Query,
    get_actions_cache_usage_by_repo_for_org::Response,
  > {
    let org = org.into();
    let url = format!("/orgs/{org}/actions/cache/usage-by-repository");

    Request::<
      (),
      get_actions_cache_usage_by_repo_for_org::Query,
      get_actions_cache_usage_by_repo_for_org::Response,
    >::builder(&self.config)
    .get(url)
    .build()
  }

  /// **Get GitHub Actions permissions for an organization**
  ///
  /// Gets the GitHub Actions permissions policy for repositories and allowed actions and reusable workflows in an organization.
  ///
  /// OAuth tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/permissions#get-github-actions-permissions-for-an-organization](https://docs.github.com/rest/actions/permissions#get-github-actions-permissions-for-an-organization)
  pub fn get_github_actions_permissions_organization(
    &self,
    org: impl Into<String>,
  ) -> Request<(), (), get_github_actions_permissions_organization::Response> {
    let org = org.into();
    let url = format!("/orgs/{org}/actions/permissions");

    Request::<(), (), get_github_actions_permissions_organization::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Set GitHub Actions permissions for an organization**
  ///
  /// Sets the GitHub Actions permissions policy for repositories and allowed actions and reusable workflows in an organization.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/permissions#set-github-actions-permissions-for-an-organization](https://docs.github.com/rest/actions/permissions#set-github-actions-permissions-for-an-organization)
  pub fn set_github_actions_permissions_organization(
    &self,
    org: impl Into<String>,
  ) -> NoContentRequest<set_github_actions_permissions_organization::Request, ()> {
    let org = org.into();
    let url = format!("/orgs/{org}/actions/permissions");

    NoContentRequest::<set_github_actions_permissions_organization::Request, ()>::builder(
      &self.config,
    )
    .put(url)
    .build()
  }

  /// **List selected repositories enabled for GitHub Actions in an organization**
  ///
  /// Lists the selected repositories that are enabled for GitHub Actions in an organization. To use this endpoint, the organization permission policy for `enabled_repositories` must be configured to `selected`. For more information, see "[Set GitHub Actions permissions for an organization](#set-github-actions-permissions-for-an-organization)."
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/permissions#list-selected-repositories-enabled-for-github-actions-in-an-organization](https://docs.github.com/rest/actions/permissions#list-selected-repositories-enabled-for-github-actions-in-an-organization)
  pub fn list_selected_repositories_enabled_github_actions_organization(
    &self,
    org: impl Into<String>,
  ) -> Request<
    (),
    list_selected_repositories_enabled_github_actions_organization::Query,
    list_selected_repositories_enabled_github_actions_organization::Response,
  > {
    let org = org.into();
    let url = format!("/orgs/{org}/actions/permissions/repositories");

    Request::<
      (),
      list_selected_repositories_enabled_github_actions_organization::Query,
      list_selected_repositories_enabled_github_actions_organization::Response,
    >::builder(&self.config)
    .get(url)
    .build()
  }

  /// **Set selected repositories enabled for GitHub Actions in an organization**
  ///
  /// Replaces the list of selected repositories that are enabled for GitHub Actions in an organization. To use this endpoint, the organization permission policy for `enabled_repositories` must be configured to `selected`. For more information, see "[Set GitHub Actions permissions for an organization](#set-github-actions-permissions-for-an-organization)."
  ///
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/permissions#set-selected-repositories-enabled-for-github-actions-in-an-organization](https://docs.github.com/rest/actions/permissions#set-selected-repositories-enabled-for-github-actions-in-an-organization)
  pub fn set_selected_repositories_enabled_github_actions_organization(
    &self,
    org: impl Into<String>,
  ) -> NoContentRequest<set_selected_repositories_enabled_github_actions_organization::Request, ()>
  {
    let org = org.into();
    let url = format!("/orgs/{org}/actions/permissions/repositories");

    NoContentRequest::<set_selected_repositories_enabled_github_actions_organization::Request, ()>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **Enable a selected repository for GitHub Actions in an organization**
  ///
  /// Adds a repository to the list of selected repositories that are enabled for GitHub Actions in an organization. To use this endpoint, the organization permission policy for `enabled_repositories` must be must be configured to `selected`. For more information, see "[Set GitHub Actions permissions for an organization](#set-github-actions-permissions-for-an-organization)."
  ///
  /// OAuth tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/permissions#enable-a-selected-repository-for-github-actions-in-an-organization](https://docs.github.com/rest/actions/permissions#enable-a-selected-repository-for-github-actions-in-an-organization)
  pub fn enable_selected_repository_github_actions_organization(
    &self,
    org: impl Into<String>,
    repository_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let repository_id = repository_id.into();
    let url = format!("/orgs/{org}/actions/permissions/repositories/{repository_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **Disable a selected repository for GitHub Actions in an organization**
  ///
  /// Removes a repository from the list of selected repositories that are enabled for GitHub Actions in an organization. To use this endpoint, the organization permission policy for `enabled_repositories` must be configured to `selected`. For more information, see "[Set GitHub Actions permissions for an organization](#set-github-actions-permissions-for-an-organization)."
  ///
  /// OAuth tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/permissions#disable-a-selected-repository-for-github-actions-in-an-organization](https://docs.github.com/rest/actions/permissions#disable-a-selected-repository-for-github-actions-in-an-organization)
  pub fn disable_selected_repository_github_actions_organization(
    &self,
    org: impl Into<String>,
    repository_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let repository_id = repository_id.into();
    let url = format!("/orgs/{org}/actions/permissions/repositories/{repository_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Get allowed actions and reusable workflows for an organization**
  ///
  /// Gets the selected actions and reusable workflows that are allowed in an organization. To use this endpoint, the organization permission policy for `allowed_actions` must be configured to `selected`. For more information, see "[Set GitHub Actions permissions for an organization](#set-github-actions-permissions-for-an-organization)."
  ///
  /// OAuth tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/permissions#get-allowed-actions-and-reusable-workflows-for-an-organization](https://docs.github.com/rest/actions/permissions#get-allowed-actions-and-reusable-workflows-for-an-organization)
  pub fn get_allowed_actions_organization(
    &self,
    org: impl Into<String>,
  ) -> Request<(), (), get_allowed_actions_organization::Response> {
    let org = org.into();
    let url = format!("/orgs/{org}/actions/permissions/selected-actions");

    Request::<(), (), get_allowed_actions_organization::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Set allowed actions and reusable workflows for an organization**
  ///
  /// Sets the actions and reusable workflows that are allowed in an organization. To use this endpoint, the organization permission policy for `allowed_actions` must be configured to `selected`. For more information, see "[Set GitHub Actions permissions for an organization](#set-github-actions-permissions-for-an-organization)."
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/permissions#set-allowed-actions-and-reusable-workflows-for-an-organization](https://docs.github.com/rest/actions/permissions#set-allowed-actions-and-reusable-workflows-for-an-organization)
  pub fn set_allowed_actions_organization(
    &self,
    org: impl Into<String>,
  ) -> NoContentRequest<set_allowed_actions_organization::Request, ()> {
    let org = org.into();
    let url = format!("/orgs/{org}/actions/permissions/selected-actions");

    NoContentRequest::<set_allowed_actions_organization::Request, ()>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **Get default workflow permissions for an organization**
  ///
  /// Gets the default workflow permissions granted to the `GITHUB_TOKEN` when running workflows in an organization,
  /// as well as whether GitHub Actions can submit approving pull request reviews. For more information, see
  /// "[Setting the permissions of the GITHUB_TOKEN for your organization](https://docs.github.com/organizations/managing-organization-settings/disabling-or-limiting-github-actions-for-your-organization#setting-the-permissions-of-the-github_token-for-your-organization)."
  ///
  /// OAuth tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/permissions#get-default-workflow-permissions-for-an-organization](https://docs.github.com/rest/actions/permissions#get-default-workflow-permissions-for-an-organization)
  pub fn get_github_actions_default_workflow_permissions_organization(
    &self,
    org: impl Into<String>,
  ) -> Request<(), (), get_github_actions_default_workflow_permissions_organization::Response> {
    let org = org.into();
    let url = format!("/orgs/{org}/actions/permissions/workflow");

    Request::<(), (), get_github_actions_default_workflow_permissions_organization::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Set default workflow permissions for an organization**
  ///
  /// Sets the default workflow permissions granted to the `GITHUB_TOKEN` when running workflows in an organization, and sets if GitHub Actions
  /// can submit approving pull request reviews. For more information, see
  /// "[Setting the permissions of the GITHUB_TOKEN for your organization](https://docs.github.com/organizations/managing-organization-settings/disabling-or-limiting-github-actions-for-your-organization#setting-the-permissions-of-the-github_token-for-your-organization)."
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/permissions#set-default-workflow-permissions-for-an-organization](https://docs.github.com/rest/actions/permissions#set-default-workflow-permissions-for-an-organization)
  pub fn set_github_actions_default_workflow_permissions_organization(
    &self,
    org: impl Into<String>,
  ) -> NoContentRequest<set_github_actions_default_workflow_permissions_organization::Request, ()>
  {
    let org = org.into();
    let url = format!("/orgs/{org}/actions/permissions/workflow");

    NoContentRequest::<set_github_actions_default_workflow_permissions_organization::Request, ()>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **List self-hosted runners for an organization**
  ///
  /// Lists all self-hosted runners configured in an organization.
  ///
  /// Authenticated users must have admin access to the organization to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint. If the repository is private, the `repo` scope is also required.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/self-hosted-runners#list-self-hosted-runners-for-an-organization](https://docs.github.com/rest/actions/self-hosted-runners#list-self-hosted-runners-for-an-organization)
  pub fn list_self_hosted_runners_for_org(
    &self,
    org: impl Into<String>,
  ) -> Request<
    (),
    list_self_hosted_runners_for_org::Query,
    list_self_hosted_runners_for_org::Response,
  > {
    let org = org.into();
    let url = format!("/orgs/{org}/actions/runners");

    Request::<(), list_self_hosted_runners_for_org::Query, list_self_hosted_runners_for_org::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List runner applications for an organization**
  ///
  /// Lists binaries for the runner application that you can download and run.
  ///
  /// Authenticated users must have admin access to the organization to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.  If the repository is private, the `repo` scope is also required.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/self-hosted-runners#list-runner-applications-for-an-organization](https://docs.github.com/rest/actions/self-hosted-runners#list-runner-applications-for-an-organization)
  pub fn list_runner_applications_for_org(
    &self,
    org: impl Into<String>,
  ) -> Request<(), (), list_runner_applications_for_org::Response> {
    let org = org.into();
    let url = format!("/orgs/{org}/actions/runners/downloads");

    Request::<(), (), list_runner_applications_for_org::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create configuration for a just-in-time runner for an organization**
  ///
  /// Generates a configuration that can be passed to the runner application at startup.
  ///
  /// The authenticated user must have admin access to the organization.
  ///
  /// OAuth tokens and personal access tokens (classic) need the`admin:org` scope to use this endpoint. If the repository is private, OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/self-hosted-runners#create-configuration-for-a-just-in-time-runner-for-an-organization](https://docs.github.com/rest/actions/self-hosted-runners#create-configuration-for-a-just-in-time-runner-for-an-organization)
  pub fn generate_runner_jitconfig_for_org(
    &self,
    org: impl Into<String>,
  ) -> NoContentRequest<generate_runner_jitconfig_for_org::Request, ()> {
    let org = org.into();
    let url = format!("/orgs/{org}/actions/runners/generate-jitconfig");

    NoContentRequest::<generate_runner_jitconfig_for_org::Request, ()>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Create a registration token for an organization**
  ///
  /// Returns a token that you can pass to the `config` script. The token expires after one hour.
  ///
  /// For example, you can replace `TOKEN` in the following example with the registration token provided by this endpoint to configure your self-hosted runner:
  ///
  /// ```
  /// ./config.sh --url https://github.com/octo-org --token TOKEN
  /// ```
  ///
  /// Authenticated users must have admin access to the organization to use this endpoint.
  ///
  /// OAuth tokens and personal access tokens (classic) need the`admin:org` scope to use this endpoint. If the repository is private, OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/self-hosted-runners#create-a-registration-token-for-an-organization](https://docs.github.com/rest/actions/self-hosted-runners#create-a-registration-token-for-an-organization)
  pub fn create_registration_token_for_org(
    &self,
    org: impl Into<String>,
  ) -> Request<(), (), create_registration_token_for_org::Response> {
    let org = org.into();
    let url = format!("/orgs/{org}/actions/runners/registration-token");

    Request::<(), (), create_registration_token_for_org::Response>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Create a remove token for an organization**
  ///
  /// Returns a token that you can pass to the `config` script to remove a self-hosted runner from an organization. The token expires after one hour.
  ///
  /// For example, you can replace `TOKEN` in the following example with the registration token provided by this endpoint to remove your self-hosted runner from an organization:
  ///
  /// ```
  /// ./config.sh remove --token TOKEN
  /// ```
  ///
  /// Authenticated users must have admin access to the organization to use this endpoint.
  ///
  /// OAuth tokens and personal access tokens (classic) need the`admin:org` scope to use this endpoint. If the repository is private, OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/self-hosted-runners#create-a-remove-token-for-an-organization](https://docs.github.com/rest/actions/self-hosted-runners#create-a-remove-token-for-an-organization)
  pub fn create_remove_token_for_org(
    &self,
    org: impl Into<String>,
  ) -> Request<(), (), create_remove_token_for_org::Response> {
    let org = org.into();
    let url = format!("/orgs/{org}/actions/runners/remove-token");

    Request::<(), (), create_remove_token_for_org::Response>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Get a self-hosted runner for an organization**
  ///
  /// Gets a specific self-hosted runner configured in an organization.
  ///
  /// Authenticated users must have admin access to the organization to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint. If the repository is private, the `repo` scope is also required.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/self-hosted-runners#get-a-self-hosted-runner-for-an-organization](https://docs.github.com/rest/actions/self-hosted-runners#get-a-self-hosted-runner-for-an-organization)
  pub fn get_self_hosted_runner_for_org(
    &self,
    org: impl Into<String>,
    runner_id: impl Into<i64>,
  ) -> Request<(), (), get_self_hosted_runner_for_org::Response> {
    let org = org.into();
    let runner_id = runner_id.into();
    let url = format!("/orgs/{org}/actions/runners/{runner_id}");

    Request::<(), (), get_self_hosted_runner_for_org::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Delete a self-hosted runner from an organization**
  ///
  /// Forces the removal of a self-hosted runner from an organization. You can use this endpoint to completely remove the runner when the machine you were using no longer exists.
  ///
  /// Authenticated users must have admin access to the organization to use this endpoint.
  ///
  /// OAuth tokens and personal access tokens (classic) need the`admin:org` scope to use this endpoint. If the repository is private, OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/self-hosted-runners#delete-a-self-hosted-runner-from-an-organization](https://docs.github.com/rest/actions/self-hosted-runners#delete-a-self-hosted-runner-from-an-organization)
  pub fn delete_self_hosted_runner_from_org(
    &self,
    org: impl Into<String>,
    runner_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let runner_id = runner_id.into();
    let url = format!("/orgs/{org}/actions/runners/{runner_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List labels for a self-hosted runner for an organization**
  ///
  /// Lists all labels for a self-hosted runner configured in an organization.
  ///
  /// Authenticated users must have admin access to the organization to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint. If the repository is private, the `repo` scope is also required.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/self-hosted-runners#list-labels-for-a-self-hosted-runner-for-an-organization](https://docs.github.com/rest/actions/self-hosted-runners#list-labels-for-a-self-hosted-runner-for-an-organization)
  pub fn list_labels_for_self_hosted_runner_for_org(
    &self,
    org: impl Into<String>,
    runner_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let runner_id = runner_id.into();
    let url = format!("/orgs/{org}/actions/runners/{runner_id}/labels");

    NoContentRequest::<(), ()>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Add custom labels to a self-hosted runner for an organization**
  ///
  /// Adds custom labels to a self-hosted runner configured in an organization.
  ///
  /// Authenticated users must have admin access to the organization to use this endpoint.
  ///
  /// OAuth tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/self-hosted-runners#add-custom-labels-to-a-self-hosted-runner-for-an-organization](https://docs.github.com/rest/actions/self-hosted-runners#add-custom-labels-to-a-self-hosted-runner-for-an-organization)
  pub fn add_custom_labels_to_self_hosted_runner_for_org(
    &self,
    org: impl Into<String>,
    runner_id: impl Into<i64>,
  ) -> NoContentRequest<add_custom_labels_to_self_hosted_runner_for_org::Request, ()> {
    let org = org.into();
    let runner_id = runner_id.into();
    let url = format!("/orgs/{org}/actions/runners/{runner_id}/labels");

    NoContentRequest::<add_custom_labels_to_self_hosted_runner_for_org::Request, ()>::builder(
      &self.config,
    )
    .post(url)
    .build()
  }

  /// **Set custom labels for a self-hosted runner for an organization**
  ///
  /// Remove all previous custom labels and set the new custom labels for a specific
  /// self-hosted runner configured in an organization.
  ///
  /// Authenticated users must have admin access to the organization to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint. If the repository is private, the `repo` scope is also required.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/self-hosted-runners#set-custom-labels-for-a-self-hosted-runner-for-an-organization](https://docs.github.com/rest/actions/self-hosted-runners#set-custom-labels-for-a-self-hosted-runner-for-an-organization)
  pub fn set_custom_labels_for_self_hosted_runner_for_org(
    &self,
    org: impl Into<String>,
    runner_id: impl Into<i64>,
  ) -> NoContentRequest<set_custom_labels_for_self_hosted_runner_for_org::Request, ()> {
    let org = org.into();
    let runner_id = runner_id.into();
    let url = format!("/orgs/{org}/actions/runners/{runner_id}/labels");

    NoContentRequest::<set_custom_labels_for_self_hosted_runner_for_org::Request, ()>::builder(
      &self.config,
    )
    .put(url)
    .build()
  }

  /// **Remove all custom labels from a self-hosted runner for an organization**
  ///
  /// Remove all custom labels from a self-hosted runner configured in an
  /// organization. Returns the remaining read-only labels from the runner.
  ///
  /// Authenticated users must have admin access to the organization to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint. If the repository is private, the `repo` scope is also required.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/self-hosted-runners#remove-all-custom-labels-from-a-self-hosted-runner-for-an-organization](https://docs.github.com/rest/actions/self-hosted-runners#remove-all-custom-labels-from-a-self-hosted-runner-for-an-organization)
  pub fn remove_all_custom_labels_from_self_hosted_runner_for_org(
    &self,
    org: impl Into<String>,
    runner_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let runner_id = runner_id.into();
    let url = format!("/orgs/{org}/actions/runners/{runner_id}/labels");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Remove a custom label from a self-hosted runner for an organization**
  ///
  /// Remove a custom label from a self-hosted runner configured
  /// in an organization. Returns the remaining labels from the runner.
  ///
  /// This endpoint returns a `404 Not Found` status if the custom label is not
  /// present on the runner.
  ///
  /// Authenticated users must have admin access to the organization to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint. If the repository is private, the `repo` scope is also required.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/self-hosted-runners#remove-a-custom-label-from-a-self-hosted-runner-for-an-organization](https://docs.github.com/rest/actions/self-hosted-runners#remove-a-custom-label-from-a-self-hosted-runner-for-an-organization)
  pub fn remove_custom_label_from_self_hosted_runner_for_org(
    &self,
    org: impl Into<String>,
    runner_id: impl Into<i64>,
    name: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let runner_id = runner_id.into();
    let name = name.into();
    let url = format!("/orgs/{org}/actions/runners/{runner_id}/labels/{name}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List organization secrets**
  ///
  /// Lists all secrets available in an organization without revealing their
  /// encrypted values.
  ///
  /// Authenticated users must have collaborator access to a repository to create, update, or read secrets.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint. If the repository is private, the `repo` scope is also required.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/secrets#list-organization-secrets](https://docs.github.com/rest/actions/secrets#list-organization-secrets)
  pub fn list_org_secrets(
    &self,
    org: impl Into<String>,
  ) -> Request<(), list_org_secrets::Query, list_org_secrets::Response> {
    let org = org.into();
    let url = format!("/orgs/{org}/actions/secrets");

    Request::<(), list_org_secrets::Query, list_org_secrets::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get an organization public key**
  ///
  /// Gets your public key, which you need to encrypt secrets. You need to
  /// encrypt a secret before you can create or update secrets.
  ///
  /// The authenticated user must have collaborator access to a repository to create, update, or read secrets.
  ///
  /// OAuth tokens and personal access tokens (classic) need the`admin:org` scope to use this endpoint. If the repository is private, OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/secrets#get-an-organization-public-key](https://docs.github.com/rest/actions/secrets#get-an-organization-public-key)
  pub fn get_org_public_key(
    &self,
    org: impl Into<String>,
  ) -> Request<(), (), get_org_public_key::Response> {
    let org = org.into();
    let url = format!("/orgs/{org}/actions/secrets/public-key");

    Request::<(), (), get_org_public_key::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get an organization secret**
  ///
  /// Gets a single organization secret without revealing its encrypted value.
  ///
  /// The authenticated user must have collaborator access to a repository to create, update, or read secrets
  ///
  /// OAuth tokens and personal access tokens (classic) need the`admin:org` scope to use this endpoint. If the repository is private, OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/secrets#get-an-organization-secret](https://docs.github.com/rest/actions/secrets#get-an-organization-secret)
  pub fn get_org_secret(
    &self,
    org: impl Into<String>,
    secret_name: impl Into<String>,
  ) -> Request<(), (), get_org_secret::Response> {
    let org = org.into();
    let secret_name = secret_name.into();
    let url = format!("/orgs/{org}/actions/secrets/{secret_name}");

    Request::<(), (), get_org_secret::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create or update an organization secret**
  ///
  /// Creates or updates an organization secret with an encrypted value. Encrypt your secret using
  /// [LibSodium](https://libsodium.gitbook.io/doc/bindings_for_other_languages). For more information, see "[Encrypting secrets for the REST API](https://docs.github.com/rest/guides/encrypting-secrets-for-the-rest-api)."
  ///
  /// Authenticated users must have collaborator access to a repository to create, update, or read secrets.
  ///
  /// OAuth tokens and personal access tokens (classic) need the`admin:org` scope to use this endpoint. If the repository is private, OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/secrets#create-or-update-an-organization-secret](https://docs.github.com/rest/actions/secrets#create-or-update-an-organization-secret)
  pub fn create_or_update_org_secret(
    &self,
    org: impl Into<String>,
    secret_name: impl Into<String>,
  ) -> Request<create_or_update_org_secret::Request, (), create_or_update_org_secret::Response> {
    let org = org.into();
    let secret_name = secret_name.into();
    let url = format!("/orgs/{org}/actions/secrets/{secret_name}");

    Request::<create_or_update_org_secret::Request, (), create_or_update_org_secret::Response>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **Delete an organization secret**
  ///
  /// Deletes a secret in an organization using the secret name.
  ///
  /// Authenticated users must have collaborator access to a repository to create, update, or read secrets.
  ///
  /// OAuth tokens and personal access tokens (classic) need the`admin:org` scope to use this endpoint. If the repository is private, OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/secrets#delete-an-organization-secret](https://docs.github.com/rest/actions/secrets#delete-an-organization-secret)
  pub fn delete_org_secret(
    &self,
    org: impl Into<String>,
    secret_name: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let secret_name = secret_name.into();
    let url = format!("/orgs/{org}/actions/secrets/{secret_name}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List selected repositories for an organization secret**
  ///
  /// Lists all repositories that have been selected when the `visibility`
  /// for repository access to a secret is set to `selected`.
  ///
  /// Authenticated users must have collaborator access to a repository to create, update, or read secrets.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint. If the repository is private, the `repo` scope is also required.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/secrets#list-selected-repositories-for-an-organization-secret](https://docs.github.com/rest/actions/secrets#list-selected-repositories-for-an-organization-secret)
  pub fn list_selected_repos_for_org_secret(
    &self,
    org: impl Into<String>,
    secret_name: impl Into<String>,
  ) -> Request<
    (),
    list_selected_repos_for_org_secret::Query,
    list_selected_repos_for_org_secret::Response,
  > {
    let org = org.into();
    let secret_name = secret_name.into();
    let url = format!("/orgs/{org}/actions/secrets/{secret_name}/repositories");

    Request::<
      (),
      list_selected_repos_for_org_secret::Query,
      list_selected_repos_for_org_secret::Response,
    >::builder(&self.config)
    .get(url)
    .build()
  }

  /// **Set selected repositories for an organization secret**
  ///
  /// Replaces all repositories for an organization secret when the `visibility`
  /// for repository access is set to `selected`. The visibility is set when you [Create
  /// or update an organization secret](https://docs.github.com/rest/actions/secrets#create-or-update-an-organization-secret).
  ///
  /// Authenticated users must have collaborator access to a repository to create, update, or read secrets.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint. If the repository is private, the `repo` scope is also required.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/secrets#set-selected-repositories-for-an-organization-secret](https://docs.github.com/rest/actions/secrets#set-selected-repositories-for-an-organization-secret)
  pub fn set_selected_repos_for_org_secret(
    &self,
    org: impl Into<String>,
    secret_name: impl Into<String>,
  ) -> NoContentRequest<set_selected_repos_for_org_secret::Request, ()> {
    let org = org.into();
    let secret_name = secret_name.into();
    let url = format!("/orgs/{org}/actions/secrets/{secret_name}/repositories");

    NoContentRequest::<set_selected_repos_for_org_secret::Request, ()>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **Add selected repository to an organization secret**
  ///
  /// Adds a repository to an organization secret when the `visibility` for
  /// repository access is set to `selected`. For more information about setting the visibility, see [Create or
  /// update an organization secret](https://docs.github.com/rest/actions/secrets#create-or-update-an-organization-secret).
  ///
  /// Authenticated users must have collaborator access to a repository to create, update, or read secrets.
  ///
  /// OAuth tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint. If the repository is private, OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/secrets#add-selected-repository-to-an-organization-secret](https://docs.github.com/rest/actions/secrets#add-selected-repository-to-an-organization-secret)
  pub fn add_selected_repo_to_org_secret(
    &self,
    org: impl Into<String>,
    secret_name: impl Into<String>,
    repository_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let secret_name = secret_name.into();
    let repository_id = repository_id.into();
    let url = format!("/orgs/{org}/actions/secrets/{secret_name}/repositories/{repository_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **Remove selected repository from an organization secret**
  ///
  /// Removes a repository from an organization secret when the `visibility`
  /// for repository access is set to `selected`. The visibility is set when you [Create
  /// or update an organization secret](https://docs.github.com/rest/actions/secrets#create-or-update-an-organization-secret).
  ///
  /// Authenticated users must have collaborator access to a repository to create, update, or read secrets.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint. If the repository is private, the `repo` scope is also required.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/secrets#remove-selected-repository-from-an-organization-secret](https://docs.github.com/rest/actions/secrets#remove-selected-repository-from-an-organization-secret)
  pub fn remove_selected_repo_from_org_secret(
    &self,
    org: impl Into<String>,
    secret_name: impl Into<String>,
    repository_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let secret_name = secret_name.into();
    let repository_id = repository_id.into();
    let url = format!("/orgs/{org}/actions/secrets/{secret_name}/repositories/{repository_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List organization variables**
  ///
  /// Lists all organization variables.
  ///
  /// Authenticated users must have collaborator access to a repository to create, update, or read variables.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint. If the repository is private, the `repo` scope is also required.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/variables#list-organization-variables](https://docs.github.com/rest/actions/variables#list-organization-variables)
  pub fn list_org_variables(
    &self,
    org: impl Into<String>,
  ) -> Request<(), list_org_variables::Query, list_org_variables::Response> {
    let org = org.into();
    let url = format!("/orgs/{org}/actions/variables");

    Request::<(), list_org_variables::Query, list_org_variables::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create an organization variable**
  ///
  /// Creates an organization variable that you can reference in a GitHub Actions workflow.
  ///
  /// Authenticated users must have collaborator access to a repository to create, update, or read variables.
  ///
  /// OAuth tokens and personal access tokens (classic) need the`admin:org` scope to use this endpoint. If the repository is private, OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/variables#create-an-organization-variable](https://docs.github.com/rest/actions/variables#create-an-organization-variable)
  pub fn create_org_variable(
    &self,
    org: impl Into<String>,
  ) -> Request<create_org_variable::Request, (), create_org_variable::Response> {
    let org = org.into();
    let url = format!("/orgs/{org}/actions/variables");

    Request::<create_org_variable::Request, (), create_org_variable::Response>::builder(
      &self.config,
    )
    .post(url)
    .build()
  }

  /// **Get an organization variable**
  ///
  /// Gets a specific variable in an organization.
  ///
  /// The authenticated user must have collaborator access to a repository to create, update, or read variables.
  ///
  /// OAuth tokens and personal access tokens (classic) need the`admin:org` scope to use this endpoint. If the repository is private, OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/variables#get-an-organization-variable](https://docs.github.com/rest/actions/variables#get-an-organization-variable)
  pub fn get_org_variable(
    &self,
    org: impl Into<String>,
    name: impl Into<String>,
  ) -> Request<(), (), get_org_variable::Response> {
    let org = org.into();
    let name = name.into();
    let url = format!("/orgs/{org}/actions/variables/{name}");

    Request::<(), (), get_org_variable::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Update an organization variable**
  ///
  /// Updates an organization variable that you can reference in a GitHub Actions workflow.
  ///
  /// Authenticated users must have collaborator access to a repository to create, update, or read variables.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint. If the repository is private, the `repo` scope is also required.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/variables#update-an-organization-variable](https://docs.github.com/rest/actions/variables#update-an-organization-variable)
  pub fn update_org_variable(
    &self,
    org: impl Into<String>,
    name: impl Into<String>,
  ) -> NoContentRequest<update_org_variable::Request, ()> {
    let org = org.into();
    let name = name.into();
    let url = format!("/orgs/{org}/actions/variables/{name}");

    NoContentRequest::<update_org_variable::Request, ()>::builder(&self.config)
      .patch(url)
      .build()
  }

  /// **Delete an organization variable**
  ///
  /// Deletes an organization variable using the variable name.
  ///
  /// Authenticated users must have collaborator access to a repository to create, update, or read variables.
  ///
  /// OAuth tokens and personal access tokens (classic) need the`admin:org` scope to use this endpoint. If the repository is private, OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/variables#delete-an-organization-variable](https://docs.github.com/rest/actions/variables#delete-an-organization-variable)
  pub fn delete_org_variable(
    &self,
    org: impl Into<String>,
    name: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let name = name.into();
    let url = format!("/orgs/{org}/actions/variables/{name}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List selected repositories for an organization variable**
  ///
  /// Lists all repositories that can access an organization variable
  /// that is available to selected repositories.
  ///
  /// Authenticated users must have collaborator access to a repository to create, update, or read variables.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint. If the repository is private, the `repo` scope is also required.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/variables#list-selected-repositories-for-an-organization-variable](https://docs.github.com/rest/actions/variables#list-selected-repositories-for-an-organization-variable)
  pub fn list_selected_repos_for_org_variable(
    &self,
    org: impl Into<String>,
    name: impl Into<String>,
  ) -> Request<
    (),
    list_selected_repos_for_org_variable::Query,
    list_selected_repos_for_org_variable::Response,
  > {
    let org = org.into();
    let name = name.into();
    let url = format!("/orgs/{org}/actions/variables/{name}/repositories");

    Request::<
      (),
      list_selected_repos_for_org_variable::Query,
      list_selected_repos_for_org_variable::Response,
    >::builder(&self.config)
    .get(url)
    .build()
  }

  /// **Set selected repositories for an organization variable**
  ///
  /// Replaces all repositories for an organization variable that is available
  /// to selected repositories. Organization variables that are available to selected
  /// repositories have their `visibility` field set to `selected`.
  ///
  /// Authenticated users must have collaborator access to a repository to create, update, or read variables.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint. If the repository is private, the `repo` scope is also required.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/variables#set-selected-repositories-for-an-organization-variable](https://docs.github.com/rest/actions/variables#set-selected-repositories-for-an-organization-variable)
  pub fn set_selected_repos_for_org_variable(
    &self,
    org: impl Into<String>,
    name: impl Into<String>,
  ) -> NoContentRequest<set_selected_repos_for_org_variable::Request, ()> {
    let org = org.into();
    let name = name.into();
    let url = format!("/orgs/{org}/actions/variables/{name}/repositories");

    NoContentRequest::<set_selected_repos_for_org_variable::Request, ()>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **Add selected repository to an organization variable**
  ///
  /// Adds a repository to an organization variable that is available to selected repositories.
  /// Organization variables that are available to selected repositories have their `visibility` field set to `selected`.
  ///
  /// Authenticated users must have collaborator access to a repository to create, update, or read secrets.
  ///
  /// OAuth tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint. If the repository is private, OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/variables#add-selected-repository-to-an-organization-variable](https://docs.github.com/rest/actions/variables#add-selected-repository-to-an-organization-variable)
  pub fn add_selected_repo_to_org_variable(
    &self,
    org: impl Into<String>,
    name: impl Into<String>,
    repository_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let name = name.into();
    let repository_id = repository_id.into();
    let url = format!("/orgs/{org}/actions/variables/{name}/repositories/{repository_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **Remove selected repository from an organization variable**
  ///
  /// Removes a repository from an organization variable that is
  /// available to selected repositories. Organization variables that are available to
  /// selected repositories have their `visibility` field set to `selected`.
  ///
  /// Authenticated users must have collaborator access to a repository to create, update, or read variables.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint. If the repository is private, the `repo` scope is also required.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/variables#remove-selected-repository-from-an-organization-variable](https://docs.github.com/rest/actions/variables#remove-selected-repository-from-an-organization-variable)
  pub fn remove_selected_repo_from_org_variable(
    &self,
    org: impl Into<String>,
    name: impl Into<String>,
    repository_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let name = name.into();
    let repository_id = repository_id.into();
    let url = format!("/orgs/{org}/actions/variables/{name}/repositories/{repository_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List artifacts for a repository**
  ///
  /// Lists all artifacts for a repository.
  ///
  /// Anyone with read access to the repository can use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with a private repository.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/artifacts#list-artifacts-for-a-repository](https://docs.github.com/rest/actions/artifacts#list-artifacts-for-a-repository)
  pub fn list_artifacts_for_repo(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), list_artifacts_for_repo::Query, list_artifacts_for_repo::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/actions/artifacts");

    Request::<(), list_artifacts_for_repo::Query, list_artifacts_for_repo::Response>::builder(
      &self.config,
    )
    .get(url)
    .build()
  }

  /// **Get an artifact**
  ///
  /// Gets a specific artifact for a workflow run.
  ///
  /// Anyone with read access to the repository can use this endpoint.
  ///
  /// If the repository is private, OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/artifacts#get-an-artifact](https://docs.github.com/rest/actions/artifacts#get-an-artifact)
  pub fn get_artifact(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    artifact_id: impl Into<i64>,
  ) -> Request<(), (), get_artifact::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let artifact_id = artifact_id.into();
    let url = format!("/repos/{owner}/{repo}/actions/artifacts/{artifact_id}");

    Request::<(), (), get_artifact::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Delete an artifact**
  ///
  /// Deletes an artifact for a workflow run.
  /// OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/artifacts#delete-an-artifact](https://docs.github.com/rest/actions/artifacts#delete-an-artifact)
  pub fn delete_artifact(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    artifact_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let artifact_id = artifact_id.into();
    let url = format!("/repos/{owner}/{repo}/actions/artifacts/{artifact_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Download an artifact**
  ///
  /// Gets a redirect URL to download an archive for a repository. This URL expires after 1 minute. Look for `Location:` in
  /// the response header to find the URL for the download. The `:archive_format` must be `zip`.
  ///
  /// OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/artifacts#download-an-artifact](https://docs.github.com/rest/actions/artifacts#download-an-artifact)
  pub fn download_artifact(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    artifact_id: impl Into<i64>,
    archive_format: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let artifact_id = artifact_id.into();
    let archive_format = archive_format.into();
    let url = format!("/repos/{owner}/{repo}/actions/artifacts/{artifact_id}/{archive_format}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get GitHub Actions cache usage for a repository**
  ///
  /// Gets GitHub Actions cache usage for a repository.
  /// The data fetched using this API is refreshed approximately every 5 minutes, so values returned from this endpoint may take at least 5 minutes to get updated.
  ///
  /// Anyone with read access to the repository can use this endpoint.
  ///
  /// If the repository is private, OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/cache#get-github-actions-cache-usage-for-a-repository](https://docs.github.com/rest/actions/cache#get-github-actions-cache-usage-for-a-repository)
  pub fn get_actions_cache_usage(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), (), get_actions_cache_usage::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/actions/cache/usage");

    Request::<(), (), get_actions_cache_usage::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List GitHub Actions caches for a repository**
  ///
  /// Lists the GitHub Actions caches for a repository.
  ///
  /// OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/cache#list-github-actions-caches-for-a-repository](https://docs.github.com/rest/actions/cache#list-github-actions-caches-for-a-repository)
  pub fn get_actions_cache_list(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), get_actions_cache_list::Query, get_actions_cache_list::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/actions/caches");

    Request::<(), get_actions_cache_list::Query, get_actions_cache_list::Response>::builder(
      &self.config,
    )
    .get(url)
    .build()
  }

  /// **Delete GitHub Actions caches for a repository (using a cache key)**
  ///
  /// Deletes one or more GitHub Actions caches for a repository, using a complete cache key. By default, all caches that match the provided key are deleted, but you can optionally provide a Git ref to restrict deletions to caches that match both the provided key and the Git ref.
  ///
  /// OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/cache#delete-github-actions-caches-for-a-repository-using-a-cache-key](https://docs.github.com/rest/actions/cache#delete-github-actions-caches-for-a-repository-using-a-cache-key)
  pub fn delete_actions_cache_by_key(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), delete_actions_cache_by_key::Query, delete_actions_cache_by_key::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/actions/caches");

    Request::<(), delete_actions_cache_by_key::Query, delete_actions_cache_by_key::Response>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Delete a GitHub Actions cache for a repository (using a cache ID)**
  ///
  /// Deletes a GitHub Actions cache for a repository, using a cache ID.
  ///
  /// OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/cache#delete-a-github-actions-cache-for-a-repository-using-a-cache-id](https://docs.github.com/rest/actions/cache#delete-a-github-actions-cache-for-a-repository-using-a-cache-id)
  pub fn delete_actions_cache_by_id(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    cache_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let cache_id = cache_id.into();
    let url = format!("/repos/{owner}/{repo}/actions/caches/{cache_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Get a job for a workflow run**
  ///
  /// Gets a specific job in a workflow run.
  ///
  /// Anyone with read access to the repository can use this endpoint.
  ///
  /// If the repository is private, OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/workflow-jobs#get-a-job-for-a-workflow-run](https://docs.github.com/rest/actions/workflow-jobs#get-a-job-for-a-workflow-run)
  pub fn get_job_for_workflow_run(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    job_id: impl Into<i64>,
  ) -> Request<(), (), get_job_for_workflow_run::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let job_id = job_id.into();
    let url = format!("/repos/{owner}/{repo}/actions/jobs/{job_id}");

    Request::<(), (), get_job_for_workflow_run::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Download job logs for a workflow run**
  ///
  /// Gets a redirect URL to download a plain text file of logs for a workflow job. This link expires after 1 minute. Look
  /// for `Location:` in the response header to find the URL for the download.
  ///
  /// Anyone with read access to the repository can use this endpoint.
  ///
  /// If the repository is private, OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/workflow-jobs#download-job-logs-for-a-workflow-run](https://docs.github.com/rest/actions/workflow-jobs#download-job-logs-for-a-workflow-run)
  pub fn download_job_logs_for_workflow_run(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    job_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let job_id = job_id.into();
    let url = format!("/repos/{owner}/{repo}/actions/jobs/{job_id}/logs");

    NoContentRequest::<(), ()>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Re-run a job from a workflow run**
  ///
  /// Re-run a job and its dependent jobs in a workflow run.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/workflow-runs#re-run-a-job-from-a-workflow-run](https://docs.github.com/rest/actions/workflow-runs#re-run-a-job-from-a-workflow-run)
  pub fn re_run_job_for_workflow_run(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    job_id: impl Into<i64>,
  ) -> Request<re_run_job_for_workflow_run::Request, (), re_run_job_for_workflow_run::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let job_id = job_id.into();
    let url = format!("/repos/{owner}/{repo}/actions/jobs/{job_id}/rerun");

    Request::<re_run_job_for_workflow_run::Request, (), re_run_job_for_workflow_run::Response>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Get the customization template for an OIDC subject claim for a repository**
  ///
  /// Gets the customization template for an OpenID Connect (OIDC) subject claim.
  ///
  /// OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/oidc#get-the-customization-template-for-an-oidc-subject-claim-for-a-repository](https://docs.github.com/rest/actions/oidc#get-the-customization-template-for-an-oidc-subject-claim-for-a-repository)
  pub fn get_custom_oidc_sub_claim_for_repo(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), (), get_custom_oidc_sub_claim_for_repo::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/actions/oidc/customization/sub");

    Request::<(), (), get_custom_oidc_sub_claim_for_repo::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Set the customization template for an OIDC subject claim for a repository**
  ///
  /// Sets the customization template and `opt-in` or `opt-out` flag for an OpenID Connect (OIDC) subject claim for a repository.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/oidc#set-the-customization-template-for-an-oidc-subject-claim-for-a-repository](https://docs.github.com/rest/actions/oidc#set-the-customization-template-for-an-oidc-subject-claim-for-a-repository)
  pub fn set_custom_oidc_sub_claim_for_repo(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<
    set_custom_oidc_sub_claim_for_repo::Request,
    (),
    set_custom_oidc_sub_claim_for_repo::Response,
  > {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/actions/oidc/customization/sub");

    Request::<
      set_custom_oidc_sub_claim_for_repo::Request,
      (),
      set_custom_oidc_sub_claim_for_repo::Response,
    >::builder(&self.config)
    .put(url)
    .build()
  }

  /// **List repository organization secrets**
  ///
  /// Lists all organization secrets shared with a repository without revealing their encrypted
  /// values.
  ///
  /// Authenticated users must have collaborator access to a repository to create, update, or read secrets.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/secrets#list-repository-organization-secrets](https://docs.github.com/rest/actions/secrets#list-repository-organization-secrets)
  pub fn list_repo_organization_secrets(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), list_repo_organization_secrets::Query, list_repo_organization_secrets::Response>
  {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/actions/organization-secrets");

    Request::<(), list_repo_organization_secrets::Query, list_repo_organization_secrets::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List repository organization variables**
  ///
  /// Lists all organization variables shared with a repository.
  ///
  /// Authenticated users must have collaborator access to a repository to create, update, or read variables.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/variables#list-repository-organization-variables](https://docs.github.com/rest/actions/variables#list-repository-organization-variables)
  pub fn list_repo_organization_variables(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<
    (),
    list_repo_organization_variables::Query,
    list_repo_organization_variables::Response,
  > {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/actions/organization-variables");

    Request::<(), list_repo_organization_variables::Query, list_repo_organization_variables::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get GitHub Actions permissions for a repository**
  ///
  /// Gets the GitHub Actions permissions policy for a repository, including whether GitHub Actions is enabled and the actions and reusable workflows allowed to run in the repository.
  ///
  /// OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/permissions#get-github-actions-permissions-for-a-repository](https://docs.github.com/rest/actions/permissions#get-github-actions-permissions-for-a-repository)
  pub fn get_github_actions_permissions_repository(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), (), get_github_actions_permissions_repository::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/actions/permissions");

    Request::<(), (), get_github_actions_permissions_repository::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Set GitHub Actions permissions for a repository**
  ///
  /// Sets the GitHub Actions permissions policy for enabling GitHub Actions and allowed actions and reusable workflows in the repository.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/permissions#set-github-actions-permissions-for-a-repository](https://docs.github.com/rest/actions/permissions#set-github-actions-permissions-for-a-repository)
  pub fn set_github_actions_permissions_repository(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> NoContentRequest<set_github_actions_permissions_repository::Request, ()> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/actions/permissions");

    NoContentRequest::<set_github_actions_permissions_repository::Request, ()>::builder(
      &self.config,
    )
    .put(url)
    .build()
  }

  /// **Get the level of access for workflows outside of the repository**
  ///
  /// Gets the level of access that workflows outside of the repository have to actions and reusable workflows in the repository.
  /// This endpoint only applies to private repositories.
  /// For more information, see "[Allowing access to components in a private repository](https://docs.github.com/repositories/managing-your-repositorys-settings-and-features/enabling-features-for-your-repository/managing-github-actions-settings-for-a-repository#allowing-access-to-components-in-a-private-repository)."
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/permissions#get-the-level-of-access-for-workflows-outside-of-the-repository](https://docs.github.com/rest/actions/permissions#get-the-level-of-access-for-workflows-outside-of-the-repository)
  pub fn get_workflow_access_to_repository(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), (), get_workflow_access_to_repository::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/actions/permissions/access");

    Request::<(), (), get_workflow_access_to_repository::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Set the level of access for workflows outside of the repository**
  ///
  /// Sets the level of access that workflows outside of the repository have to actions and reusable workflows in the repository.
  /// This endpoint only applies to private repositories.
  /// For more information, see "[Allowing access to components in a private repository](https://docs.github.com/repositories/managing-your-repositorys-settings-and-features/enabling-features-for-your-repository/managing-github-actions-settings-for-a-repository#allowing-access-to-components-in-a-private-repository)".
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/permissions#set-the-level-of-access-for-workflows-outside-of-the-repository](https://docs.github.com/rest/actions/permissions#set-the-level-of-access-for-workflows-outside-of-the-repository)
  pub fn set_workflow_access_to_repository(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> NoContentRequest<set_workflow_access_to_repository::Request, ()> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/actions/permissions/access");

    NoContentRequest::<set_workflow_access_to_repository::Request, ()>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **Get allowed actions and reusable workflows for a repository**
  ///
  /// Gets the settings for selected actions and reusable workflows that are allowed in a repository. To use this endpoint, the repository policy for `allowed_actions` must be configured to `selected`. For more information, see "[Set GitHub Actions permissions for a repository](#set-github-actions-permissions-for-a-repository)."
  ///
  /// OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/permissions#get-allowed-actions-and-reusable-workflows-for-a-repository](https://docs.github.com/rest/actions/permissions#get-allowed-actions-and-reusable-workflows-for-a-repository)
  pub fn get_allowed_actions_repository(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), (), get_allowed_actions_repository::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/actions/permissions/selected-actions");

    Request::<(), (), get_allowed_actions_repository::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Set allowed actions and reusable workflows for a repository**
  ///
  /// Sets the actions and reusable workflows that are allowed in a repository. To use this endpoint, the repository permission policy for `allowed_actions` must be configured to `selected`. For more information, see "[Set GitHub Actions permissions for a repository](#set-github-actions-permissions-for-a-repository)."
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/permissions#set-allowed-actions-and-reusable-workflows-for-a-repository](https://docs.github.com/rest/actions/permissions#set-allowed-actions-and-reusable-workflows-for-a-repository)
  pub fn set_allowed_actions_repository(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> NoContentRequest<set_allowed_actions_repository::Request, ()> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/actions/permissions/selected-actions");

    NoContentRequest::<set_allowed_actions_repository::Request, ()>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **Get default workflow permissions for a repository**
  ///
  /// Gets the default workflow permissions granted to the `GITHUB_TOKEN` when running workflows in a repository,
  /// as well as if GitHub Actions can submit approving pull request reviews.
  /// For more information, see "[Setting the permissions of the GITHUB_TOKEN for your repository](https://docs.github.com/repositories/managing-your-repositorys-settings-and-features/enabling-features-for-your-repository/managing-github-actions-settings-for-a-repository#setting-the-permissions-of-the-github_token-for-your-repository)."
  ///
  /// OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/permissions#get-default-workflow-permissions-for-a-repository](https://docs.github.com/rest/actions/permissions#get-default-workflow-permissions-for-a-repository)
  pub fn get_github_actions_default_workflow_permissions_repository(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), (), get_github_actions_default_workflow_permissions_repository::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/actions/permissions/workflow");

    Request::<(), (), get_github_actions_default_workflow_permissions_repository::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Set default workflow permissions for a repository**
  ///
  /// Sets the default workflow permissions granted to the `GITHUB_TOKEN` when running workflows in a repository, and sets if GitHub Actions
  /// can submit approving pull request reviews.
  /// For more information, see "[Setting the permissions of the GITHUB_TOKEN for your repository](https://docs.github.com/repositories/managing-your-repositorys-settings-and-features/enabling-features-for-your-repository/managing-github-actions-settings-for-a-repository#setting-the-permissions-of-the-github_token-for-your-repository)."
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/permissions#set-default-workflow-permissions-for-a-repository](https://docs.github.com/rest/actions/permissions#set-default-workflow-permissions-for-a-repository)
  pub fn set_github_actions_default_workflow_permissions_repository(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> NoContentRequest<set_github_actions_default_workflow_permissions_repository::Request, ()> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/actions/permissions/workflow");

    NoContentRequest::<set_github_actions_default_workflow_permissions_repository::Request, ()>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **List self-hosted runners for a repository**
  ///
  /// Lists all self-hosted runners configured in a repository.
  ///
  /// Authenticated users must have admin access to the repository to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/self-hosted-runners#list-self-hosted-runners-for-a-repository](https://docs.github.com/rest/actions/self-hosted-runners#list-self-hosted-runners-for-a-repository)
  pub fn list_self_hosted_runners_for_repo(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<
    (),
    list_self_hosted_runners_for_repo::Query,
    list_self_hosted_runners_for_repo::Response,
  > {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/actions/runners");

    Request::<
      (),
      list_self_hosted_runners_for_repo::Query,
      list_self_hosted_runners_for_repo::Response,
    >::builder(&self.config)
    .get(url)
    .build()
  }

  /// **List runner applications for a repository**
  ///
  /// Lists binaries for the runner application that you can download and run.
  ///
  /// Authenticated users must have admin access to the repository to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/self-hosted-runners#list-runner-applications-for-a-repository](https://docs.github.com/rest/actions/self-hosted-runners#list-runner-applications-for-a-repository)
  pub fn list_runner_applications_for_repo(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), (), list_runner_applications_for_repo::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/actions/runners/downloads");

    Request::<(), (), list_runner_applications_for_repo::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create configuration for a just-in-time runner for a repository**
  ///
  /// Generates a configuration that can be passed to the runner application at startup.
  ///
  /// The authenticated user must have admin access to the repository.
  ///
  /// OAuth tokens and personal access tokens (classic) need the`repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/self-hosted-runners#create-configuration-for-a-just-in-time-runner-for-a-repository](https://docs.github.com/rest/actions/self-hosted-runners#create-configuration-for-a-just-in-time-runner-for-a-repository)
  pub fn generate_runner_jitconfig_for_repo(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> NoContentRequest<generate_runner_jitconfig_for_repo::Request, ()> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/actions/runners/generate-jitconfig");

    NoContentRequest::<generate_runner_jitconfig_for_repo::Request, ()>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Create a registration token for a repository**
  ///
  /// Returns a token that you can pass to the `config` script. The token expires after one hour.
  ///
  /// For example, you can replace `TOKEN` in the following example with the registration token provided by this endpoint to configure your self-hosted runner:
  ///
  /// ```
  /// ./config.sh --url https://github.com/octo-org --token TOKEN
  /// ```
  ///
  /// Authenticated users must have admin access to the repository to use this endpoint.
  ///
  /// OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/self-hosted-runners#create-a-registration-token-for-a-repository](https://docs.github.com/rest/actions/self-hosted-runners#create-a-registration-token-for-a-repository)
  pub fn create_registration_token_for_repo(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), (), create_registration_token_for_repo::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/actions/runners/registration-token");

    Request::<(), (), create_registration_token_for_repo::Response>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Create a remove token for a repository**
  ///
  /// Returns a token that you can pass to the `config` script to remove a self-hosted runner from an repository. The token expires after one hour.
  ///
  /// For example, you can replace `TOKEN` in the following example with the registration token provided by this endpoint to remove your self-hosted runner from an organization:
  ///
  /// ```
  /// ./config.sh remove --token TOKEN
  /// ```
  ///
  /// Authenticated users must have admin access to the repository to use this endpoint.
  ///
  /// OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/self-hosted-runners#create-a-remove-token-for-a-repository](https://docs.github.com/rest/actions/self-hosted-runners#create-a-remove-token-for-a-repository)
  pub fn create_remove_token_for_repo(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), (), create_remove_token_for_repo::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/actions/runners/remove-token");

    Request::<(), (), create_remove_token_for_repo::Response>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Get a self-hosted runner for a repository**
  ///
  /// Gets a specific self-hosted runner configured in a repository.
  ///
  /// Authenticated users must have admin access to the repository to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/self-hosted-runners#get-a-self-hosted-runner-for-a-repository](https://docs.github.com/rest/actions/self-hosted-runners#get-a-self-hosted-runner-for-a-repository)
  pub fn get_self_hosted_runner_for_repo(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    runner_id: impl Into<i64>,
  ) -> Request<(), (), get_self_hosted_runner_for_repo::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let runner_id = runner_id.into();
    let url = format!("/repos/{owner}/{repo}/actions/runners/{runner_id}");

    Request::<(), (), get_self_hosted_runner_for_repo::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Delete a self-hosted runner from a repository**
  ///
  /// Forces the removal of a self-hosted runner from a repository. You can use this endpoint to completely remove the runner when the machine you were using no longer exists.
  ///
  /// Authenticated users must have admin access to the repository to use this endpoint.
  ///
  /// OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/self-hosted-runners#delete-a-self-hosted-runner-from-a-repository](https://docs.github.com/rest/actions/self-hosted-runners#delete-a-self-hosted-runner-from-a-repository)
  pub fn delete_self_hosted_runner_from_repo(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    runner_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let runner_id = runner_id.into();
    let url = format!("/repos/{owner}/{repo}/actions/runners/{runner_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List labels for a self-hosted runner for a repository**
  ///
  /// Lists all labels for a self-hosted runner configured in a repository.
  ///
  /// Authenticated users must have admin access to the repository to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/self-hosted-runners#list-labels-for-a-self-hosted-runner-for-a-repository](https://docs.github.com/rest/actions/self-hosted-runners#list-labels-for-a-self-hosted-runner-for-a-repository)
  pub fn list_labels_for_self_hosted_runner_for_repo(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    runner_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let runner_id = runner_id.into();
    let url = format!("/repos/{owner}/{repo}/actions/runners/{runner_id}/labels");

    NoContentRequest::<(), ()>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Add custom labels to a self-hosted runner for a repository**
  ///
  /// Adds custom labels to a self-hosted runner configured in a repository.
  ///
  /// Authenticated users must have admin access to the organization to use this endpoint.
  ///
  /// OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/self-hosted-runners#add-custom-labels-to-a-self-hosted-runner-for-a-repository](https://docs.github.com/rest/actions/self-hosted-runners#add-custom-labels-to-a-self-hosted-runner-for-a-repository)
  pub fn add_custom_labels_to_self_hosted_runner_for_repo(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    runner_id: impl Into<i64>,
  ) -> NoContentRequest<add_custom_labels_to_self_hosted_runner_for_repo::Request, ()> {
    let owner = owner.into();
    let repo = repo.into();
    let runner_id = runner_id.into();
    let url = format!("/repos/{owner}/{repo}/actions/runners/{runner_id}/labels");

    NoContentRequest::<add_custom_labels_to_self_hosted_runner_for_repo::Request, ()>::builder(
      &self.config,
    )
    .post(url)
    .build()
  }

  /// **Set custom labels for a self-hosted runner for a repository**
  ///
  /// Remove all previous custom labels and set the new custom labels for a specific
  /// self-hosted runner configured in a repository.
  ///
  /// Authenticated users must have admin access to the repository to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/self-hosted-runners#set-custom-labels-for-a-self-hosted-runner-for-a-repository](https://docs.github.com/rest/actions/self-hosted-runners#set-custom-labels-for-a-self-hosted-runner-for-a-repository)
  pub fn set_custom_labels_for_self_hosted_runner_for_repo(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    runner_id: impl Into<i64>,
  ) -> NoContentRequest<set_custom_labels_for_self_hosted_runner_for_repo::Request, ()> {
    let owner = owner.into();
    let repo = repo.into();
    let runner_id = runner_id.into();
    let url = format!("/repos/{owner}/{repo}/actions/runners/{runner_id}/labels");

    NoContentRequest::<set_custom_labels_for_self_hosted_runner_for_repo::Request, ()>::builder(
      &self.config,
    )
    .put(url)
    .build()
  }

  /// **Remove all custom labels from a self-hosted runner for a repository**
  ///
  /// Remove all custom labels from a self-hosted runner configured in a
  /// repository. Returns the remaining read-only labels from the runner.
  ///
  /// Authenticated users must have admin access to the repository to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/self-hosted-runners#remove-all-custom-labels-from-a-self-hosted-runner-for-a-repository](https://docs.github.com/rest/actions/self-hosted-runners#remove-all-custom-labels-from-a-self-hosted-runner-for-a-repository)
  pub fn remove_all_custom_labels_from_self_hosted_runner_for_repo(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    runner_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let runner_id = runner_id.into();
    let url = format!("/repos/{owner}/{repo}/actions/runners/{runner_id}/labels");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Remove a custom label from a self-hosted runner for a repository**
  ///
  /// Remove a custom label from a self-hosted runner configured
  /// in a repository. Returns the remaining labels from the runner.
  ///
  /// This endpoint returns a `404 Not Found` status if the custom label is not
  /// present on the runner.
  ///
  /// Authenticated users must have admin access to the repository to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/self-hosted-runners#remove-a-custom-label-from-a-self-hosted-runner-for-a-repository](https://docs.github.com/rest/actions/self-hosted-runners#remove-a-custom-label-from-a-self-hosted-runner-for-a-repository)
  pub fn remove_custom_label_from_self_hosted_runner_for_repo(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    runner_id: impl Into<i64>,
    name: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let runner_id = runner_id.into();
    let name = name.into();
    let url = format!("/repos/{owner}/{repo}/actions/runners/{runner_id}/labels/{name}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List workflow runs for a repository**
  ///
  /// Lists all workflow runs for a repository. You can use parameters to narrow the list of results. For more information about using parameters, see [Parameters](https://docs.github.com/rest/guides/getting-started-with-the-rest-api#parameters).
  ///
  /// Anyone with read access to the repository can use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with a private repository.
  ///
  /// This API will return up to 1,000 results for each search when using the following parameters: `actor`, `branch`, `check_suite_id`, `created`, `event`, `head_sha`, `status`.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/workflow-runs#list-workflow-runs-for-a-repository](https://docs.github.com/rest/actions/workflow-runs#list-workflow-runs-for-a-repository)
  pub fn list_workflow_runs_for_repo(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), list_workflow_runs_for_repo::Query, list_workflow_runs_for_repo::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/actions/runs");

    Request::<(), list_workflow_runs_for_repo::Query, list_workflow_runs_for_repo::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get a workflow run**
  ///
  /// Gets a specific workflow run.
  ///
  /// Anyone with read access to the repository can use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with a private repository.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/workflow-runs#get-a-workflow-run](https://docs.github.com/rest/actions/workflow-runs#get-a-workflow-run)
  pub fn get_workflow_run(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    run_id: impl Into<i64>,
  ) -> Request<(), get_workflow_run::Query, get_workflow_run::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let run_id = run_id.into();
    let url = format!("/repos/{owner}/{repo}/actions/runs/{run_id}");

    Request::<(), get_workflow_run::Query, get_workflow_run::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Delete a workflow run**
  ///
  /// Deletes a specific workflow run.
  ///
  /// Anyone with write access to the repository can use this endpoint.
  ///
  /// If the repository is private, OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/workflow-runs#delete-a-workflow-run](https://docs.github.com/rest/actions/workflow-runs#delete-a-workflow-run)
  pub fn delete_workflow_run(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    run_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let run_id = run_id.into();
    let url = format!("/repos/{owner}/{repo}/actions/runs/{run_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Get the review history for a workflow run**
  ///
  /// Anyone with read access to the repository can use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with a private repository.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/workflow-runs#get-the-review-history-for-a-workflow-run](https://docs.github.com/rest/actions/workflow-runs#get-the-review-history-for-a-workflow-run)
  pub fn get_reviews_for_run(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    run_id: impl Into<i64>,
  ) -> Request<(), (), get_reviews_for_run::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let run_id = run_id.into();
    let url = format!("/repos/{owner}/{repo}/actions/runs/{run_id}/approvals");

    Request::<(), (), get_reviews_for_run::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Approve a workflow run for a fork pull request**
  ///
  /// Approves a workflow run for a pull request from a public fork of a first time contributor. For more information, see ["Approving workflow runs from public forks](https://docs.github.com/actions/managing-workflow-runs/approving-workflow-runs-from-public-forks)."
  ///
  /// OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/workflow-runs#approve-a-workflow-run-for-a-fork-pull-request](https://docs.github.com/rest/actions/workflow-runs#approve-a-workflow-run-for-a-fork-pull-request)
  pub fn approve_workflow_run(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    run_id: impl Into<i64>,
  ) -> Request<(), (), approve_workflow_run::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let run_id = run_id.into();
    let url = format!("/repos/{owner}/{repo}/actions/runs/{run_id}/approve");

    Request::<(), (), approve_workflow_run::Response>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **List workflow run artifacts**
  ///
  /// Lists artifacts for a workflow run.
  ///
  /// Anyone with read access to the repository can use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with a private repository.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/artifacts#list-workflow-run-artifacts](https://docs.github.com/rest/actions/artifacts#list-workflow-run-artifacts)
  pub fn list_workflow_run_artifacts(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    run_id: impl Into<i64>,
  ) -> Request<(), list_workflow_run_artifacts::Query, list_workflow_run_artifacts::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let run_id = run_id.into();
    let url = format!("/repos/{owner}/{repo}/actions/runs/{run_id}/artifacts");

    Request::<(), list_workflow_run_artifacts::Query, list_workflow_run_artifacts::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get a workflow run attempt**
  ///
  /// Gets a specific workflow run attempt.
  ///
  /// Anyone with read access to the repository can use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with a private repository.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/workflow-runs#get-a-workflow-run-attempt](https://docs.github.com/rest/actions/workflow-runs#get-a-workflow-run-attempt)
  pub fn get_workflow_run_attempt(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    run_id: impl Into<i64>,
    attempt_number: impl Into<i64>,
  ) -> Request<(), get_workflow_run_attempt::Query, get_workflow_run_attempt::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let run_id = run_id.into();
    let attempt_number = attempt_number.into();
    let url = format!("/repos/{owner}/{repo}/actions/runs/{run_id}/attempts/{attempt_number}");

    Request::<(), get_workflow_run_attempt::Query, get_workflow_run_attempt::Response>::builder(
      &self.config,
    )
    .get(url)
    .build()
  }

  /// **List jobs for a workflow run attempt**
  ///
  /// Lists jobs for a specific workflow run attempt. You can use parameters to narrow the list of results. For more information
  /// about using parameters, see [Parameters](https://docs.github.com/rest/guides/getting-started-with-the-rest-api#parameters).
  ///
  /// Anyone with read access to the repository can use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint  with a private repository.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/workflow-jobs#list-jobs-for-a-workflow-run-attempt](https://docs.github.com/rest/actions/workflow-jobs#list-jobs-for-a-workflow-run-attempt)
  pub fn list_jobs_for_workflow_run_attempt(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    run_id: impl Into<i64>,
    attempt_number: impl Into<i64>,
  ) -> Request<
    (),
    list_jobs_for_workflow_run_attempt::Query,
    list_jobs_for_workflow_run_attempt::Response,
  > {
    let owner = owner.into();
    let repo = repo.into();
    let run_id = run_id.into();
    let attempt_number = attempt_number.into();
    let url = format!("/repos/{owner}/{repo}/actions/runs/{run_id}/attempts/{attempt_number}/jobs");

    Request::<
      (),
      list_jobs_for_workflow_run_attempt::Query,
      list_jobs_for_workflow_run_attempt::Response,
    >::builder(&self.config)
    .get(url)
    .build()
  }

  /// **Download workflow run attempt logs**
  ///
  /// Gets a redirect URL to download an archive of log files for a specific workflow run attempt. This link expires after
  /// 1 minute. Look for `Location:` in the response header to find the URL for the download.
  ///
  /// Anyone with read access to the repository can use this endpoint.
  ///
  /// If the repository is private, OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/workflow-runs#download-workflow-run-attempt-logs](https://docs.github.com/rest/actions/workflow-runs#download-workflow-run-attempt-logs)
  pub fn download_workflow_run_attempt_logs(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    run_id: impl Into<i64>,
    attempt_number: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let run_id = run_id.into();
    let attempt_number = attempt_number.into();
    let url = format!("/repos/{owner}/{repo}/actions/runs/{run_id}/attempts/{attempt_number}/logs");

    NoContentRequest::<(), ()>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Cancel a workflow run**
  ///
  /// Cancels a workflow run using its `id`.
  ///
  /// OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/workflow-runs#cancel-a-workflow-run](https://docs.github.com/rest/actions/workflow-runs#cancel-a-workflow-run)
  pub fn cancel_workflow_run(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    run_id: impl Into<i64>,
  ) -> Request<(), (), cancel_workflow_run::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let run_id = run_id.into();
    let url = format!("/repos/{owner}/{repo}/actions/runs/{run_id}/cancel");

    Request::<(), (), cancel_workflow_run::Response>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Review custom deployment protection rules for a workflow run**
  ///
  /// Approve or reject custom deployment protection rules provided by a GitHub App for a workflow run. For more information, see "[Using environments for deployment](https://docs.github.com/actions/deployment/targeting-different-environments/using-environments-for-deployment)."
  ///
  /// **Note:** GitHub Apps can only review their own custom deployment protection rules.
  /// To approve or reject pending deployments that are waiting for review from a specific person or team, see [`POST /repos/{owner}/{repo}/actions/runs/{run_id}/pending_deployments`](/rest/actions/workflow-runs#review-pending-deployments-for-a-workflow-run).
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with a private repository.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/workflow-runs#review-custom-deployment-protection-rules-for-a-workflow-run](https://docs.github.com/rest/actions/workflow-runs#review-custom-deployment-protection-rules-for-a-workflow-run)
  pub fn review_custom_gates_for_run(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    run_id: impl Into<i64>,
  ) -> NoContentRequest<review_custom_gates_for_run::Request, ()> {
    let owner = owner.into();
    let repo = repo.into();
    let run_id = run_id.into();
    let url = format!("/repos/{owner}/{repo}/actions/runs/{run_id}/deployment_protection_rule");

    NoContentRequest::<review_custom_gates_for_run::Request, ()>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Force cancel a workflow run**
  ///
  /// Cancels a workflow run and bypasses conditions that would otherwise cause a workflow execution to continue, such as an `always()` condition on a job.
  /// You should only use this endpoint to cancel a workflow run when the workflow run is not responding to [`POST /repos/{owner}/{repo}/actions/runs/{run_id}/cancel`](/rest/actions/workflow-runs#cancel-a-workflow-run).
  ///
  /// OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/workflow-runs#force-cancel-a-workflow-run](https://docs.github.com/rest/actions/workflow-runs#force-cancel-a-workflow-run)
  pub fn force_cancel_workflow_run(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    run_id: impl Into<i64>,
  ) -> Request<(), (), force_cancel_workflow_run::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let run_id = run_id.into();
    let url = format!("/repos/{owner}/{repo}/actions/runs/{run_id}/force-cancel");

    Request::<(), (), force_cancel_workflow_run::Response>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **List jobs for a workflow run**
  ///
  /// Lists jobs for a workflow run. You can use parameters to narrow the list of results. For more information
  /// about using parameters, see [Parameters](https://docs.github.com/rest/guides/getting-started-with-the-rest-api#parameters).
  ///
  /// Anyone with read access to the repository can use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with a private repository.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/workflow-jobs#list-jobs-for-a-workflow-run](https://docs.github.com/rest/actions/workflow-jobs#list-jobs-for-a-workflow-run)
  pub fn list_jobs_for_workflow_run(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    run_id: impl Into<i64>,
  ) -> Request<(), list_jobs_for_workflow_run::Query, list_jobs_for_workflow_run::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let run_id = run_id.into();
    let url = format!("/repos/{owner}/{repo}/actions/runs/{run_id}/jobs");

    Request::<(), list_jobs_for_workflow_run::Query, list_jobs_for_workflow_run::Response>::builder(
      &self.config,
    )
    .get(url)
    .build()
  }

  /// **Download workflow run logs**
  ///
  /// Gets a redirect URL to download an archive of log files for a workflow run. This link expires after 1 minute. Look for
  /// `Location:` in the response header to find the URL for the download.
  ///
  /// Anyone with read access to the repository can use this endpoint.
  ///
  /// If the repository is private, OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/workflow-runs#download-workflow-run-logs](https://docs.github.com/rest/actions/workflow-runs#download-workflow-run-logs)
  pub fn download_workflow_run_logs(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    run_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let run_id = run_id.into();
    let url = format!("/repos/{owner}/{repo}/actions/runs/{run_id}/logs");

    NoContentRequest::<(), ()>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Delete workflow run logs**
  ///
  /// Deletes all logs for a workflow run.
  ///
  /// OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/workflow-runs#delete-workflow-run-logs](https://docs.github.com/rest/actions/workflow-runs#delete-workflow-run-logs)
  pub fn delete_workflow_run_logs(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    run_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let run_id = run_id.into();
    let url = format!("/repos/{owner}/{repo}/actions/runs/{run_id}/logs");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Get pending deployments for a workflow run**
  ///
  /// Get all deployment environments for a workflow run that are waiting for protection rules to pass.
  ///
  /// Anyone with read access to the repository can use this endpoint.
  ///
  /// If the repository is private, OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/workflow-runs#get-pending-deployments-for-a-workflow-run](https://docs.github.com/rest/actions/workflow-runs#get-pending-deployments-for-a-workflow-run)
  pub fn get_pending_deployments_for_run(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    run_id: impl Into<i64>,
  ) -> Request<(), (), get_pending_deployments_for_run::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let run_id = run_id.into();
    let url = format!("/repos/{owner}/{repo}/actions/runs/{run_id}/pending_deployments");

    Request::<(), (), get_pending_deployments_for_run::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Review pending deployments for a workflow run**
  ///
  /// Approve or reject pending deployments that are waiting on approval by a required reviewer.
  ///
  /// Required reviewers with read access to the repository contents and deployments can use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/workflow-runs#review-pending-deployments-for-a-workflow-run](https://docs.github.com/rest/actions/workflow-runs#review-pending-deployments-for-a-workflow-run)
  pub fn review_pending_deployments_for_run(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    run_id: impl Into<i64>,
  ) -> Request<
    review_pending_deployments_for_run::Request,
    (),
    review_pending_deployments_for_run::Response,
  > {
    let owner = owner.into();
    let repo = repo.into();
    let run_id = run_id.into();
    let url = format!("/repos/{owner}/{repo}/actions/runs/{run_id}/pending_deployments");

    Request::<
      review_pending_deployments_for_run::Request,
      (),
      review_pending_deployments_for_run::Response,
    >::builder(&self.config)
    .post(url)
    .build()
  }

  /// **Re-run a workflow**
  ///
  /// Re-runs your workflow run using its `id`.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/workflow-runs#re-run-a-workflow](https://docs.github.com/rest/actions/workflow-runs#re-run-a-workflow)
  pub fn re_run_workflow(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    run_id: impl Into<i64>,
  ) -> Request<re_run_workflow::Request, (), re_run_workflow::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let run_id = run_id.into();
    let url = format!("/repos/{owner}/{repo}/actions/runs/{run_id}/rerun");

    Request::<re_run_workflow::Request, (), re_run_workflow::Response>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Re-run failed jobs from a workflow run**
  ///
  /// Re-run all of the failed jobs and their dependent jobs in a workflow run using the `id` of the workflow run.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/workflow-runs#re-run-failed-jobs-from-a-workflow-run](https://docs.github.com/rest/actions/workflow-runs#re-run-failed-jobs-from-a-workflow-run)
  pub fn re_run_workflow_failed_jobs(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    run_id: impl Into<i64>,
  ) -> Request<re_run_workflow_failed_jobs::Request, (), re_run_workflow_failed_jobs::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let run_id = run_id.into();
    let url = format!("/repos/{owner}/{repo}/actions/runs/{run_id}/rerun-failed-jobs");

    Request::<re_run_workflow_failed_jobs::Request, (), re_run_workflow_failed_jobs::Response>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Get workflow run usage**
  ///
  /// Gets the number of billable minutes and total run time for a specific workflow run. Billable minutes only apply to workflows in private repositories that use GitHub-hosted runners. Usage is listed for each GitHub-hosted runner operating system in milliseconds. Any job re-runs are also included in the usage. The usage does not include the multiplier for macOS and Windows runners and is not rounded up to the nearest whole minute. For more information, see "[Managing billing for GitHub Actions](https://docs.github.com/github/setting-up-and-managing-billing-and-payments-on-github/managing-billing-for-github-actions)".
  ///
  /// Anyone with read access to the repository can use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with a private repository.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/workflow-runs#get-workflow-run-usage](https://docs.github.com/rest/actions/workflow-runs#get-workflow-run-usage)
  pub fn get_workflow_run_usage(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    run_id: impl Into<i64>,
  ) -> Request<(), (), get_workflow_run_usage::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let run_id = run_id.into();
    let url = format!("/repos/{owner}/{repo}/actions/runs/{run_id}/timing");

    Request::<(), (), get_workflow_run_usage::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List repository secrets**
  ///
  /// Lists all secrets available in a repository without revealing their encrypted
  /// values.
  ///
  /// Authenticated users must have collaborator access to a repository to create, update, or read secrets.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/secrets#list-repository-secrets](https://docs.github.com/rest/actions/secrets#list-repository-secrets)
  pub fn list_repo_secrets(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), list_repo_secrets::Query, list_repo_secrets::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/actions/secrets");

    Request::<(), list_repo_secrets::Query, list_repo_secrets::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get a repository public key**
  ///
  /// Gets your public key, which you need to encrypt secrets. You need to
  /// encrypt a secret before you can create or update secrets.
  ///
  /// Anyone with read access to the repository can use this endpoint.
  ///
  /// If the repository is private, OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/secrets#get-a-repository-public-key](https://docs.github.com/rest/actions/secrets#get-a-repository-public-key)
  pub fn get_repo_public_key(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), (), get_repo_public_key::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/actions/secrets/public-key");

    Request::<(), (), get_repo_public_key::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get a repository secret**
  ///
  /// Gets a single repository secret without revealing its encrypted value.
  ///
  /// The authenticated user must have collaborator access to the repository to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/secrets#get-a-repository-secret](https://docs.github.com/rest/actions/secrets#get-a-repository-secret)
  pub fn get_repo_secret(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    secret_name: impl Into<String>,
  ) -> Request<(), (), get_repo_secret::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let secret_name = secret_name.into();
    let url = format!("/repos/{owner}/{repo}/actions/secrets/{secret_name}");

    Request::<(), (), get_repo_secret::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create or update a repository secret**
  ///
  /// Creates or updates a repository secret with an encrypted value. Encrypt your secret using
  /// [LibSodium](https://libsodium.gitbook.io/doc/bindings_for_other_languages). For more information, see "[Encrypting secrets for the REST API](https://docs.github.com/rest/guides/encrypting-secrets-for-the-rest-api)."
  ///
  /// Authenticated users must have collaborator access to a repository to create, update, or read secrets.
  ///
  /// OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/secrets#create-or-update-a-repository-secret](https://docs.github.com/rest/actions/secrets#create-or-update-a-repository-secret)
  pub fn create_or_update_repo_secret(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    secret_name: impl Into<String>,
  ) -> Request<create_or_update_repo_secret::Request, (), create_or_update_repo_secret::Response>
  {
    let owner = owner.into();
    let repo = repo.into();
    let secret_name = secret_name.into();
    let url = format!("/repos/{owner}/{repo}/actions/secrets/{secret_name}");

    Request::<create_or_update_repo_secret::Request, (), create_or_update_repo_secret::Response>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **Delete a repository secret**
  ///
  /// Deletes a secret in a repository using the secret name.
  ///
  /// Authenticated users must have collaborator access to a repository to create, update, or read secrets.
  ///
  /// OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/secrets#delete-a-repository-secret](https://docs.github.com/rest/actions/secrets#delete-a-repository-secret)
  pub fn delete_repo_secret(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    secret_name: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let secret_name = secret_name.into();
    let url = format!("/repos/{owner}/{repo}/actions/secrets/{secret_name}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List repository variables**
  ///
  /// Lists all repository variables.
  ///
  /// Authenticated users must have collaborator access to a repository to create, update, or read variables.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/variables#list-repository-variables](https://docs.github.com/rest/actions/variables#list-repository-variables)
  pub fn list_repo_variables(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), list_repo_variables::Query, list_repo_variables::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/actions/variables");

    Request::<(), list_repo_variables::Query, list_repo_variables::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create a repository variable**
  ///
  /// Creates a repository variable that you can reference in a GitHub Actions workflow.
  ///
  /// Authenticated users must have collaborator access to a repository to create, update, or read variables.
  ///
  /// OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/variables#create-a-repository-variable](https://docs.github.com/rest/actions/variables#create-a-repository-variable)
  pub fn create_repo_variable(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<create_repo_variable::Request, (), create_repo_variable::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/actions/variables");

    Request::<create_repo_variable::Request, (), create_repo_variable::Response>::builder(
      &self.config,
    )
    .post(url)
    .build()
  }

  /// **Get a repository variable**
  ///
  /// Gets a specific variable in a repository.
  ///
  /// The authenticated user must have collaborator access to the repository to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/variables#get-a-repository-variable](https://docs.github.com/rest/actions/variables#get-a-repository-variable)
  pub fn get_repo_variable(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    name: impl Into<String>,
  ) -> Request<(), (), get_repo_variable::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let name = name.into();
    let url = format!("/repos/{owner}/{repo}/actions/variables/{name}");

    Request::<(), (), get_repo_variable::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Update a repository variable**
  ///
  /// Updates a repository variable that you can reference in a GitHub Actions workflow.
  ///
  /// Authenticated users must have collaborator access to a repository to create, update, or read variables.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/variables#update-a-repository-variable](https://docs.github.com/rest/actions/variables#update-a-repository-variable)
  pub fn update_repo_variable(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    name: impl Into<String>,
  ) -> NoContentRequest<update_repo_variable::Request, ()> {
    let owner = owner.into();
    let repo = repo.into();
    let name = name.into();
    let url = format!("/repos/{owner}/{repo}/actions/variables/{name}");

    NoContentRequest::<update_repo_variable::Request, ()>::builder(&self.config)
      .patch(url)
      .build()
  }

  /// **Delete a repository variable**
  ///
  /// Deletes a repository variable using the variable name.
  ///
  /// Authenticated users must have collaborator access to a repository to create, update, or read variables.
  ///
  /// OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/variables#delete-a-repository-variable](https://docs.github.com/rest/actions/variables#delete-a-repository-variable)
  pub fn delete_repo_variable(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    name: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let name = name.into();
    let url = format!("/repos/{owner}/{repo}/actions/variables/{name}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List repository workflows**
  ///
  /// Lists the workflows in a repository.
  ///
  /// Anyone with read access to the repository can use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with a private repository.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/workflows#list-repository-workflows](https://docs.github.com/rest/actions/workflows#list-repository-workflows)
  pub fn list_repo_workflows(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), list_repo_workflows::Query, list_repo_workflows::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/actions/workflows");

    Request::<(), list_repo_workflows::Query, list_repo_workflows::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get a workflow**
  ///
  /// Gets a specific workflow. You can replace `workflow_id` with the workflow
  /// file name. For example, you could use `main.yaml`.
  ///
  /// Anyone with read access to the repository can use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with a private repository.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/workflows#get-a-workflow](https://docs.github.com/rest/actions/workflows#get-a-workflow)
  pub fn get_workflow(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    workflow_id: impl Into<StringOrInteger>,
  ) -> Request<(), (), get_workflow::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let workflow_id = workflow_id.into();
    let url = format!("/repos/{owner}/{repo}/actions/workflows/{workflow_id}");

    Request::<(), (), get_workflow::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Disable a workflow**
  ///
  /// Disables a workflow and sets the `state` of the workflow to `disabled_manually`. You can replace `workflow_id` with the workflow file name. For example, you could use `main.yaml`.
  ///
  /// OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/workflows#disable-a-workflow](https://docs.github.com/rest/actions/workflows#disable-a-workflow)
  pub fn disable_workflow(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    workflow_id: impl Into<StringOrInteger>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let workflow_id = workflow_id.into();
    let url = format!("/repos/{owner}/{repo}/actions/workflows/{workflow_id}/disable");

    NoContentRequest::<(), ()>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **Create a workflow dispatch event**
  ///
  /// You can use this endpoint to manually trigger a GitHub Actions workflow run. You can replace `workflow_id` with the workflow file name. For example, you could use `main.yaml`.
  ///
  /// You must configure your GitHub Actions workflow to run when the [`workflow_dispatch` webhook](/developers/webhooks-and-events/webhook-events-and-payloads#workflow_dispatch) event occurs. The `inputs` are configured in the workflow file. For more information about how to configure the `workflow_dispatch` event in the workflow file, see "[Events that trigger workflows](/actions/reference/events-that-trigger-workflows#workflow_dispatch)."
  ///
  /// OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/workflows#create-a-workflow-dispatch-event](https://docs.github.com/rest/actions/workflows#create-a-workflow-dispatch-event)
  pub fn create_workflow_dispatch(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    workflow_id: impl Into<StringOrInteger>,
  ) -> NoContentRequest<create_workflow_dispatch::Request, ()> {
    let owner = owner.into();
    let repo = repo.into();
    let workflow_id = workflow_id.into();
    let url = format!("/repos/{owner}/{repo}/actions/workflows/{workflow_id}/dispatches");

    NoContentRequest::<create_workflow_dispatch::Request, ()>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Enable a workflow**
  ///
  /// Enables a workflow and sets the `state` of the workflow to `active`. You can replace `workflow_id` with the workflow file name. For example, you could use `main.yaml`.
  ///
  /// OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/workflows#enable-a-workflow](https://docs.github.com/rest/actions/workflows#enable-a-workflow)
  pub fn enable_workflow(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    workflow_id: impl Into<StringOrInteger>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let workflow_id = workflow_id.into();
    let url = format!("/repos/{owner}/{repo}/actions/workflows/{workflow_id}/enable");

    NoContentRequest::<(), ()>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **List workflow runs for a workflow**
  ///
  /// List all workflow runs for a workflow. You can replace `workflow_id` with the workflow file name. For example, you could use `main.yaml`. You can use parameters to narrow the list of results. For more information about using parameters, see [Parameters](https://docs.github.com/rest/guides/getting-started-with-the-rest-api#parameters).
  ///
  /// Anyone with read access to the repository can use this endpoint
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with a private repository.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/workflow-runs#list-workflow-runs-for-a-workflow](https://docs.github.com/rest/actions/workflow-runs#list-workflow-runs-for-a-workflow)
  pub fn list_workflow_runs(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    workflow_id: impl Into<StringOrInteger>,
  ) -> Request<(), list_workflow_runs::Query, list_workflow_runs::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let workflow_id = workflow_id.into();
    let url = format!("/repos/{owner}/{repo}/actions/workflows/{workflow_id}/runs");

    Request::<(), list_workflow_runs::Query, list_workflow_runs::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get workflow usage**
  ///
  /// Gets the number of billable minutes used by a specific workflow during the current billing cycle. Billable minutes only apply to workflows in private repositories that use GitHub-hosted runners. Usage is listed for each GitHub-hosted runner operating system in milliseconds. Any job re-runs are also included in the usage. The usage does not include the multiplier for macOS and Windows runners and is not rounded up to the nearest whole minute. For more information, see "[Managing billing for GitHub Actions](https://docs.github.com/github/setting-up-and-managing-billing-and-payments-on-github/managing-billing-for-github-actions)".
  ///
  /// You can replace `workflow_id` with the workflow file name. For example, you could use `main.yaml`.
  ///
  /// Anyone with read access to the repository can use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with a private repository.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/workflows#get-workflow-usage](https://docs.github.com/rest/actions/workflows#get-workflow-usage)
  pub fn get_workflow_usage(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    workflow_id: impl Into<StringOrInteger>,
  ) -> Request<(), (), get_workflow_usage::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let workflow_id = workflow_id.into();
    let url = format!("/repos/{owner}/{repo}/actions/workflows/{workflow_id}/timing");

    Request::<(), (), get_workflow_usage::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List environment secrets**
  ///
  /// Lists all secrets available in an environment without revealing their
  /// encrypted values.
  ///
  /// Authenticated users must have collaborator access to a repository to create, update, or read secrets.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/secrets#list-environment-secrets](https://docs.github.com/rest/actions/secrets#list-environment-secrets)
  pub fn list_environment_secrets(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    environment_name: impl Into<String>,
  ) -> Request<(), list_environment_secrets::Query, list_environment_secrets::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let environment_name = environment_name.into();
    let url = format!("/repos/{owner}/{repo}/environments/{environment_name}/secrets");

    Request::<(), list_environment_secrets::Query, list_environment_secrets::Response>::builder(
      &self.config,
    )
    .get(url)
    .build()
  }

  /// **Get an environment public key**
  ///
  /// Get the public key for an environment, which you need to encrypt environment
  /// secrets. You need to encrypt a secret before you can create or update secrets.
  ///
  /// Anyone with read access to the repository can use this endpoint.
  ///
  /// If the repository is private, OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/secrets#get-an-environment-public-key](https://docs.github.com/rest/actions/secrets#get-an-environment-public-key)
  pub fn get_environment_public_key(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    environment_name: impl Into<String>,
  ) -> Request<(), (), get_environment_public_key::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let environment_name = environment_name.into();
    let url = format!("/repos/{owner}/{repo}/environments/{environment_name}/secrets/public-key");

    Request::<(), (), get_environment_public_key::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get an environment secret**
  ///
  /// Gets a single environment secret without revealing its encrypted value.
  ///
  /// Authenticated users must have collaborator access to a repository to create, update, or read secrets.
  ///
  /// OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/secrets#get-an-environment-secret](https://docs.github.com/rest/actions/secrets#get-an-environment-secret)
  pub fn get_environment_secret(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    environment_name: impl Into<String>,
    secret_name: impl Into<String>,
  ) -> Request<(), (), get_environment_secret::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let environment_name = environment_name.into();
    let secret_name = secret_name.into();
    let url =
      format!("/repos/{owner}/{repo}/environments/{environment_name}/secrets/{secret_name}");

    Request::<(), (), get_environment_secret::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create or update an environment secret**
  ///
  /// Creates or updates an environment secret with an encrypted value. Encrypt your secret using
  /// [LibSodium](https://libsodium.gitbook.io/doc/bindings_for_other_languages). For more information, see "[Encrypting secrets for the REST API](https://docs.github.com/rest/guides/encrypting-secrets-for-the-rest-api)."
  ///
  /// Authenticated users must have collaborator access to a repository to create, update, or read secrets.
  ///
  /// OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/secrets#create-or-update-an-environment-secret](https://docs.github.com/rest/actions/secrets#create-or-update-an-environment-secret)
  pub fn create_or_update_environment_secret(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    environment_name: impl Into<String>,
    secret_name: impl Into<String>,
  ) -> Request<
    create_or_update_environment_secret::Request,
    (),
    create_or_update_environment_secret::Response,
  > {
    let owner = owner.into();
    let repo = repo.into();
    let environment_name = environment_name.into();
    let secret_name = secret_name.into();
    let url =
      format!("/repos/{owner}/{repo}/environments/{environment_name}/secrets/{secret_name}");

    Request::<
      create_or_update_environment_secret::Request,
      (),
      create_or_update_environment_secret::Response,
    >::builder(&self.config)
    .put(url)
    .build()
  }

  /// **Delete an environment secret**
  ///
  /// Deletes a secret in an environment using the secret name.
  ///
  /// Authenticated users must have collaborator access to a repository to create, update, or read secrets.
  ///
  /// OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/secrets#delete-an-environment-secret](https://docs.github.com/rest/actions/secrets#delete-an-environment-secret)
  pub fn delete_environment_secret(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    environment_name: impl Into<String>,
    secret_name: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let environment_name = environment_name.into();
    let secret_name = secret_name.into();
    let url =
      format!("/repos/{owner}/{repo}/environments/{environment_name}/secrets/{secret_name}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List environment variables**
  ///
  /// Lists all environment variables.
  ///
  /// Authenticated users must have collaborator access to a repository to create, update, or read variables.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/variables#list-environment-variables](https://docs.github.com/rest/actions/variables#list-environment-variables)
  pub fn list_environment_variables(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    environment_name: impl Into<String>,
  ) -> Request<(), list_environment_variables::Query, list_environment_variables::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let environment_name = environment_name.into();
    let url = format!("/repos/{owner}/{repo}/environments/{environment_name}/variables");

    Request::<(), list_environment_variables::Query, list_environment_variables::Response>::builder(
      &self.config,
    )
    .get(url)
    .build()
  }

  /// **Create an environment variable**
  ///
  /// Create an environment variable that you can reference in a GitHub Actions workflow.
  ///
  /// Authenticated users must have collaborator access to a repository to create, update, or read variables.
  ///
  /// OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/variables#create-an-environment-variable](https://docs.github.com/rest/actions/variables#create-an-environment-variable)
  pub fn create_environment_variable(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    environment_name: impl Into<String>,
  ) -> Request<create_environment_variable::Request, (), create_environment_variable::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let environment_name = environment_name.into();
    let url = format!("/repos/{owner}/{repo}/environments/{environment_name}/variables");

    Request::<create_environment_variable::Request, (), create_environment_variable::Response>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Get an environment variable**
  ///
  /// Gets a specific variable in an environment.
  ///
  /// Authenticated users must have collaborator access to a repository to create, update, or read variables.
  ///
  /// OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/variables#get-an-environment-variable](https://docs.github.com/rest/actions/variables#get-an-environment-variable)
  pub fn get_environment_variable(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    environment_name: impl Into<String>,
    name: impl Into<String>,
  ) -> Request<(), (), get_environment_variable::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let environment_name = environment_name.into();
    let name = name.into();
    let url = format!("/repos/{owner}/{repo}/environments/{environment_name}/variables/{name}");

    Request::<(), (), get_environment_variable::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Update an environment variable**
  ///
  /// Updates an environment variable that you can reference in a GitHub Actions workflow.
  ///
  /// Authenticated users must have collaborator access to a repository to create, update, or read variables.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/variables#update-an-environment-variable](https://docs.github.com/rest/actions/variables#update-an-environment-variable)
  pub fn update_environment_variable(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    name: impl Into<String>,
    environment_name: impl Into<String>,
  ) -> NoContentRequest<update_environment_variable::Request, ()> {
    let owner = owner.into();
    let repo = repo.into();
    let name = name.into();
    let environment_name = environment_name.into();
    let url = format!("/repos/{owner}/{repo}/environments/{environment_name}/variables/{name}");

    NoContentRequest::<update_environment_variable::Request, ()>::builder(&self.config)
      .patch(url)
      .build()
  }

  /// **Delete an environment variable**
  ///
  /// Deletes an environment variable using the variable name.
  ///
  /// Authenticated users must have collaborator access to a repository to create, update, or read variables.
  ///
  /// OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/variables#delete-an-environment-variable](https://docs.github.com/rest/actions/variables#delete-an-environment-variable)
  pub fn delete_environment_variable(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    name: impl Into<String>,
    environment_name: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let name = name.into();
    let environment_name = environment_name.into();
    let url = format!("/repos/{owner}/{repo}/environments/{environment_name}/variables/{name}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }
}
