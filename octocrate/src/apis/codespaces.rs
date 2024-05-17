use octocrate_core::*;
#[allow(unused_imports)]
use octocrate_types::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use typed_builder::TypedBuilder;

pub mod list_in_organization {
  #[allow(unused_imports)]
  use super::*;

  /// Query for `List codespaces for the organization`
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
    pub codespaces: Vec<Codespace>,
    pub total_count: i64,
  }
}

pub mod set_codespaces_access {
  #[allow(unused_imports)]
  use super::*;

  /// Which users can access codespaces in the organization. `disabled` means that no users can access codespaces in the organization.
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum RequestVisibility {
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "selected_members")]
    SelectedMembers,
    #[serde(rename = "all_members")]
    AllMembers,
    #[serde(rename = "all_members_and_outside_collaborators")]
    AllMembersAndOutsideCollaborators,
  }

  impl ToString for RequestVisibility {
    fn to_string(&self) -> String {
      match self {
        RequestVisibility::Disabled => "disabled".to_string(),
        RequestVisibility::SelectedMembers => "selected_members".to_string(),
        RequestVisibility::AllMembers => "all_members".to_string(),
        RequestVisibility::AllMembersAndOutsideCollaborators => {
          "all_members_and_outside_collaborators".to_string()
        }
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The usernames of the organization members who should have access to codespaces in the organization. Required when `visibility` is `selected_members`. The provided list of usernames will replace any existing value.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub selected_usernames: Option<Vec<String>>,
    /// Which users can access codespaces in the organization. `disabled` means that no users can access codespaces in the organization.
    pub visibility: RequestVisibility,
  }
}

pub mod set_codespaces_access_users {
  #[allow(unused_imports)]
  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The usernames of the organization members whose codespaces be billed to the organization.
    pub selected_usernames: Vec<String>,
  }
}

pub mod delete_codespaces_access_users {
  #[allow(unused_imports)]
  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The usernames of the organization members whose codespaces should not be billed to the organization.
    pub selected_usernames: Vec<String>,
  }
}

pub mod list_org_secrets {
  #[allow(unused_imports)]
  use super::*;

  /// Query for `List organization secrets`
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
    pub secrets: Vec<CodespacesOrgSecret>,
    pub total_count: i64,
  }
}

pub mod get_org_public_key {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = CodespacesPublicKey;
}

pub mod get_org_secret {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = CodespacesOrgSecret;
}

pub mod create_or_update_org_secret {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = EmptyObject;

  /// Which type of organization repositories have access to the organization secret. `selected` means only the repositories specified by `selected_repository_ids` can access the secret.
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum RequestVisibility {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "selected")]
    Selected,
  }

  impl ToString for RequestVisibility {
    fn to_string(&self) -> String {
      match self {
        RequestVisibility::All => "all".to_string(),
        RequestVisibility::Private => "private".to_string(),
        RequestVisibility::Selected => "selected".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The value for your secret, encrypted with [LibSodium](https://libsodium.gitbook.io/doc/bindings_for_other_languages) using the public key retrieved from the [Get an organization public key](https://docs.github.com/rest/codespaces/organization-secrets#get-an-organization-public-key) endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub encrypted_value: Option<String>,
    /// The ID of the key you used to encrypt the secret.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub key_id: Option<String>,
    /// An array of repository IDs that can access the organization secret. You can only provide a list of repository IDs when the `visibility` is set to `selected`. You can manage the list of selected repositories using the [List selected repositories for an organization secret](https://docs.github.com/rest/codespaces/organization-secrets#list-selected-repositories-for-an-organization-secret), [Set selected repositories for an organization secret](https://docs.github.com/rest/codespaces/organization-secrets#set-selected-repositories-for-an-organization-secret), and [Remove selected repository from an organization secret](https://docs.github.com/rest/codespaces/organization-secrets#remove-selected-repository-from-an-organization-secret) endpoints.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub selected_repository_ids: Option<Vec<i64>>,
    /// Which type of organization repositories have access to the organization secret. `selected` means only the repositories specified by `selected_repository_ids` can access the secret.
    pub visibility: RequestVisibility,
  }
}

pub mod delete_org_secret {
  #[allow(unused_imports)]
  use super::*;
}

pub mod list_selected_repos_for_org_secret {
  #[allow(unused_imports)]
  use super::*;

  /// Query for `List selected repositories for an organization secret`
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
    /// An array of repository ids that can access the organization secret. You can only provide a list of repository ids when the `visibility` is set to `selected`. You can add and remove individual repositories using the [Set selected repositories for an organization secret](https://docs.github.com/rest/codespaces/organization-secrets#set-selected-repositories-for-an-organization-secret) and [Remove selected repository from an organization secret](https://docs.github.com/rest/codespaces/organization-secrets#remove-selected-repository-from-an-organization-secret) endpoints.
    pub selected_repository_ids: Vec<i64>,
  }
}

pub mod add_selected_repo_to_org_secret {
  #[allow(unused_imports)]
  use super::*;
}

pub mod remove_selected_repo_from_org_secret {
  #[allow(unused_imports)]
  use super::*;
}

pub mod get_codespaces_for_user_in_org {
  #[allow(unused_imports)]
  use super::*;

  /// Query for `List codespaces for a user in organization`
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
    pub codespaces: Vec<Codespace>,
    pub total_count: i64,
  }
}

pub mod delete_from_organization {
  #[allow(unused_imports)]
  use super::*;
}

pub mod stop_in_organization {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Codespace;
}

pub mod list_in_repository_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;

  /// Query for `List codespaces in a repository for the authenticated user`
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
    pub codespaces: Vec<Codespace>,
    pub total_count: i64,
  }
}

pub mod create_with_repo_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Codespace;

  /// The geographic area for this codespace. If not specified, the value is assigned by IP. This property replaces `location`, which is being deprecated.
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum RequestGeo {
    EuropeWest,
    SoutheastAsia,
    UsEast,
    UsWest,
  }

  impl ToString for RequestGeo {
    fn to_string(&self) -> String {
      match self {
        RequestGeo::EuropeWest => "EuropeWest".to_string(),
        RequestGeo::SoutheastAsia => "SoutheastAsia".to_string(),
        RequestGeo::UsEast => "UsEast".to_string(),
        RequestGeo::UsWest => "UsWest".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// IP for location auto-detection when proxying a request
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub client_ip: Option<String>,
    /// Path to devcontainer.json config to use for this codespace
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub devcontainer_path: Option<String>,
    /// Display name for this codespace
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub display_name: Option<String>,
    /// The geographic area for this codespace. If not specified, the value is assigned by IP. This property replaces `location`, which is being deprecated.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub geo: Option<RequestGeo>,
    /// Time in minutes before codespace stops from inactivity
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub idle_timeout_minutes: Option<i64>,
    /// The requested location for a new codespace. Best efforts are made to respect this upon creation. Assigned by IP if not provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub location: Option<String>,
    /// Machine type to use for this codespace
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub machine: Option<String>,
    /// Whether to authorize requested permissions from devcontainer.json
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub multi_repo_permissions_opt_out: Option<bool>,
    /// Git ref (typically a branch name) for this codespace
    #[serde(rename = "ref")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub ref_: Option<String>,
    /// Duration in minutes after codespace has gone idle in which it will be deleted. Must be integer minutes between 0 and 43200 (30 days).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub retention_period_minutes: Option<i64>,
    /// Working directory for this codespace
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub working_directory: Option<String>,
  }
}

pub mod list_devcontainers_in_repository_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;

  /// Query for `List devcontainer configurations in a repository for the authenticated user`
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
  pub struct ResponseDevcontainers {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub name: Option<String>,
    pub path: String,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Response {
    pub devcontainers: Vec<ResponseDevcontainers>,
    pub total_count: i64,
  }
}

pub mod repo_machines_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;

  /// Query for `List available machine types for a repository`
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The location to check for available machines. Assigned by IP if not provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub location: Option<String>,
    /// IP for location auto-detection when proxying a request
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub client_ip: Option<String>,
    /// The branch or commit to check for prebuild availability and devcontainer restrictions.
    #[serde(rename = "ref")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub ref_: Option<String>,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Response {
    pub machines: Vec<CodespaceMachine>,
    pub total_count: i64,
  }
}

pub mod pre_flight_with_repo_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;

  /// Query for `Get default attributes for a codespace`
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The branch or commit to check for a default devcontainer path. If not specified, the default branch will be checked.
    #[serde(rename = "ref")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub ref_: Option<String>,
    /// An alternative IP for default location auto-detection, such as when proxying a request.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub client_ip: Option<String>,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct ResponseDefaults {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub devcontainer_path: Option<String>,
    pub location: String,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub billable_owner: Option<SimpleUser>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub defaults: Option<ResponseDefaults>,
  }
}

pub mod check_permissions_for_devcontainer {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = CodespacesPermissionsCheckForDevcontainer;

  /// Query for `Check if permissions defined by a devcontainer have been accepted by the authenticated user`
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The git reference that points to the location of the devcontainer configuration to use for the permission check. The value of `ref` will typically be a branch name (`heads/BRANCH_NAME`). For more information, see "[Git References](https://git-scm.com/book/en/v2/Git-Internals-Git-References)" in the Git documentation.
    #[serde(rename = "ref")]
    pub ref_: String,
    /// Path to the devcontainer.json configuration to use for the permission check.
    pub devcontainer_path: String,
  }
}

pub mod list_repo_secrets {
  #[allow(unused_imports)]
  use super::*;

  /// Query for `List repository secrets`
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
    pub secrets: Vec<RepoCodespacesSecret>,
    pub total_count: i64,
  }
}

pub mod get_repo_public_key {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = CodespacesPublicKey;
}

pub mod get_repo_secret {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = RepoCodespacesSecret;
}

pub mod create_or_update_repo_secret {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = EmptyObject;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// Value for your secret, encrypted with [LibSodium](https://libsodium.gitbook.io/doc/bindings_for_other_languages) using the public key retrieved from the [Get a repository public key](https://docs.github.com/rest/codespaces/repository-secrets#get-a-repository-public-key) endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub encrypted_value: Option<String>,
    /// ID of the key you used to encrypt the secret.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub key_id: Option<String>,
  }
}

pub mod delete_repo_secret {
  #[allow(unused_imports)]
  use super::*;
}

pub mod create_with_pr_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Codespace;

  /// The geographic area for this codespace. If not specified, the value is assigned by IP. This property replaces `location`, which is being deprecated.
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum RequestGeo {
    EuropeWest,
    SoutheastAsia,
    UsEast,
    UsWest,
  }

  impl ToString for RequestGeo {
    fn to_string(&self) -> String {
      match self {
        RequestGeo::EuropeWest => "EuropeWest".to_string(),
        RequestGeo::SoutheastAsia => "SoutheastAsia".to_string(),
        RequestGeo::UsEast => "UsEast".to_string(),
        RequestGeo::UsWest => "UsWest".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// IP for location auto-detection when proxying a request
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub client_ip: Option<String>,
    /// Path to devcontainer.json config to use for this codespace
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub devcontainer_path: Option<String>,
    /// Display name for this codespace
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub display_name: Option<String>,
    /// The geographic area for this codespace. If not specified, the value is assigned by IP. This property replaces `location`, which is being deprecated.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub geo: Option<RequestGeo>,
    /// Time in minutes before codespace stops from inactivity
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub idle_timeout_minutes: Option<i64>,
    /// The requested location for a new codespace. Best efforts are made to respect this upon creation. Assigned by IP if not provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub location: Option<String>,
    /// Machine type to use for this codespace
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub machine: Option<String>,
    /// Whether to authorize requested permissions from devcontainer.json
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub multi_repo_permissions_opt_out: Option<bool>,
    /// Duration in minutes after codespace has gone idle in which it will be deleted. Must be integer minutes between 0 and 43200 (30 days).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub retention_period_minutes: Option<i64>,
    /// Working directory for this codespace
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub working_directory: Option<String>,
  }
}

pub mod list_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;

  /// Query for `List codespaces for the authenticated user`
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
    /// ID of the Repository to filter on
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub repository_id: Option<i64>,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Response {
    pub codespaces: Vec<Codespace>,
    pub total_count: i64,
  }
}

pub mod create_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Codespace;

  /// The geographic area for this codespace. If not specified, the value is assigned by IP. This property replaces `location`, which is being deprecated.
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum RequestItem1Geo {
    EuropeWest,
    SoutheastAsia,
    UsEast,
    UsWest,
  }

  impl ToString for RequestItem1Geo {
    fn to_string(&self) -> String {
      match self {
        RequestItem1Geo::EuropeWest => "EuropeWest".to_string(),
        RequestItem1Geo::SoutheastAsia => "SoutheastAsia".to_string(),
        RequestItem1Geo::UsEast => "UsEast".to_string(),
        RequestItem1Geo::UsWest => "UsWest".to_string(),
      }
    }
  }

  /// The geographic area for this codespace. If not specified, the value is assigned by IP. This property replaces `location`, which is being deprecated.
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum RequestItem2Geo {
    EuropeWest,
    SoutheastAsia,
    UsEast,
    UsWest,
  }

  impl ToString for RequestItem2Geo {
    fn to_string(&self) -> String {
      match self {
        RequestItem2Geo::EuropeWest => "EuropeWest".to_string(),
        RequestItem2Geo::SoutheastAsia => "SoutheastAsia".to_string(),
        RequestItem2Geo::UsEast => "UsEast".to_string(),
        RequestItem2Geo::UsWest => "UsWest".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize)]
  #[serde(untagged)]
  pub enum Request {
    RequestItem1(RequestItem1),
    RequestItem2(RequestItem2),
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct RequestItem1 {
    /// IP for location auto-detection when proxying a request
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub client_ip: Option<String>,
    /// Path to devcontainer.json config to use for this codespace
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub devcontainer_path: Option<String>,
    /// Display name for this codespace
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub display_name: Option<String>,
    /// The geographic area for this codespace. If not specified, the value is assigned by IP. This property replaces `location`, which is being deprecated.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub geo: Option<RequestItem1Geo>,
    /// Time in minutes before codespace stops from inactivity
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub idle_timeout_minutes: Option<i64>,
    /// The requested location for a new codespace. Best efforts are made to respect this upon creation. Assigned by IP if not provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub location: Option<String>,
    /// Machine type to use for this codespace
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub machine: Option<String>,
    /// Whether to authorize requested permissions from devcontainer.json
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub multi_repo_permissions_opt_out: Option<bool>,
    /// Git ref (typically a branch name) for this codespace
    #[serde(rename = "ref")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub ref_: Option<String>,
    /// Repository id for this codespace
    pub repository_id: i64,
    /// Duration in minutes after codespace has gone idle in which it will be deleted. Must be integer minutes between 0 and 43200 (30 days).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub retention_period_minutes: Option<i64>,
    /// Working directory for this codespace
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub working_directory: Option<String>,
  }

  /// Pull request number for this codespace
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct RequestItem2PullRequest {
    /// Pull request number
    pub pull_request_number: i64,
    /// Repository id for this codespace
    pub repository_id: i64,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct RequestItem2 {
    /// Path to devcontainer.json config to use for this codespace
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub devcontainer_path: Option<String>,
    /// The geographic area for this codespace. If not specified, the value is assigned by IP. This property replaces `location`, which is being deprecated.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub geo: Option<RequestItem2Geo>,
    /// Time in minutes before codespace stops from inactivity
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub idle_timeout_minutes: Option<i64>,
    /// The requested location for a new codespace. Best efforts are made to respect this upon creation. Assigned by IP if not provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub location: Option<String>,
    /// Machine type to use for this codespace
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub machine: Option<String>,
    /// Pull request number for this codespace
    pub pull_request: RequestItem2PullRequest,
    /// Working directory for this codespace
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub working_directory: Option<String>,
  }
}

pub mod list_secrets_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;

  /// Query for `List secrets for the authenticated user`
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
    pub secrets: Vec<CodespacesSecret>,
    pub total_count: i64,
  }
}

pub mod get_public_key_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = CodespacesUserPublicKey;
}

pub mod get_secret_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = CodespacesSecret;
}

pub mod create_or_update_secret_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = EmptyObject;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// Value for your secret, encrypted with [LibSodium](https://libsodium.gitbook.io/doc/bindings_for_other_languages) using the public key retrieved from the [Get the public key for the authenticated user](https://docs.github.com/rest/codespaces/secrets#get-public-key-for-the-authenticated-user) endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub encrypted_value: Option<String>,
    /// ID of the key you used to encrypt the secret.
    pub key_id: String,
    /// An array of repository ids that can access the user secret. You can manage the list of selected repositories using the [List selected repositories for a user secret](https://docs.github.com/rest/codespaces/secrets#list-selected-repositories-for-a-user-secret), [Set selected repositories for a user secret](https://docs.github.com/rest/codespaces/secrets#set-selected-repositories-for-a-user-secret), and [Remove a selected repository from a user secret](https://docs.github.com/rest/codespaces/secrets#remove-a-selected-repository-from-a-user-secret) endpoints.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub selected_repository_ids: Option<Vec<StringOrInteger>>,
  }
}

pub mod delete_secret_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;
}

pub mod list_repositories_for_secret_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Response {
    pub repositories: Vec<MinimalRepository>,
    pub total_count: i64,
  }
}

pub mod set_repositories_for_secret_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// An array of repository ids for which a codespace can access the secret. You can manage the list of selected repositories using the [List selected repositories for a user secret](https://docs.github.com/rest/codespaces/secrets#list-selected-repositories-for-a-user-secret), [Add a selected repository to a user secret](https://docs.github.com/rest/codespaces/secrets#add-a-selected-repository-to-a-user-secret), and [Remove a selected repository from a user secret](https://docs.github.com/rest/codespaces/secrets#remove-a-selected-repository-from-a-user-secret) endpoints.
    pub selected_repository_ids: Vec<i64>,
  }
}

pub mod add_repository_for_secret_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;
}

pub mod remove_repository_for_secret_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;
}

pub mod get_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Codespace;
}

pub mod update_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Codespace;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// Display name for this codespace
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub display_name: Option<String>,
    /// A valid machine to transition this codespace to.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub machine: Option<String>,
    /// Recently opened folders inside the codespace. It is currently used by the clients to determine the folder path to load the codespace in.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub recent_folders: Option<Vec<String>>,
  }
}

pub mod delete_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;
}

pub mod export_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = CodespaceExportDetails;
}

pub mod get_export_details_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = CodespaceExportDetails;
}

pub mod codespace_machines_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Response {
    pub machines: Vec<CodespaceMachine>,
    pub total_count: i64,
  }
}

pub mod publish_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = CodespaceWithFullRepository;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// A name for the new repository.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub name: Option<String>,
    /// Whether the new repository should be private.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub private: Option<bool>,
  }
}

pub mod start_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Codespace;
}

pub mod stop_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Codespace;
}

/// Endpoints to manage Codespaces using the REST API.
pub struct GitHubCodespacesAPI {
  config: SharedAPIConfig,
}

impl GitHubCodespacesAPI {
  pub fn new(config: &SharedAPIConfig) -> Self {
    Self {
      config: config.clone(),
    }
  }

  /// **List codespaces for the organization**
  ///
  /// Lists the codespaces associated to a specified organization.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/codespaces/organizations#list-codespaces-for-the-organization](https://docs.github.com/rest/codespaces/organizations#list-codespaces-for-the-organization)
  pub fn list_in_organization(
    &self,
    org: impl Into<String>,
  ) -> Request<(), list_in_organization::Query, list_in_organization::Response> {
    let org = org.into();
    let url = format!("/orgs/{org}/codespaces");

    Request::<(), list_in_organization::Query, list_in_organization::Response>::builder(
      &self.config,
    )
    .get(url)
    .build()
  }

  /// **Manage access control for organization codespaces**
  ///
  /// Sets which users can access codespaces in an organization. This is synonymous with granting or revoking codespaces access permissions for users according to the visibility.
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/codespaces/organizations#manage-access-control-for-organization-codespaces](https://docs.github.com/rest/codespaces/organizations#manage-access-control-for-organization-codespaces)
  pub fn set_codespaces_access(
    &self,
    org: impl Into<String>,
  ) -> NoContentRequest<set_codespaces_access::Request, ()> {
    let org = org.into();
    let url = format!("/orgs/{org}/codespaces/access");

    NoContentRequest::<set_codespaces_access::Request, ()>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **Add users to Codespaces access for an organization**
  ///
  /// Codespaces for the specified users will be billed to the organization.
  ///
  /// To use this endpoint, the access settings for the organization must be set to `selected_members`.
  /// For information on how to change this setting, see "[Manage access control for organization codespaces](https://docs.github.com/rest/codespaces/organizations#manage-access-control-for-organization-codespaces)."
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/codespaces/organizations#add-users-to-codespaces-access-for-an-organization](https://docs.github.com/rest/codespaces/organizations#add-users-to-codespaces-access-for-an-organization)
  pub fn set_codespaces_access_users(
    &self,
    org: impl Into<String>,
  ) -> NoContentRequest<set_codespaces_access_users::Request, ()> {
    let org = org.into();
    let url = format!("/orgs/{org}/codespaces/access/selected_users");

    NoContentRequest::<set_codespaces_access_users::Request, ()>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Remove users from Codespaces access for an organization**
  ///
  /// Codespaces for the specified users will no longer be billed to the organization.
  ///
  /// To use this endpoint, the access settings for the organization must be set to `selected_members`.
  /// For information on how to change this setting, see "[Manage access control for organization codespaces](https://docs.github.com/rest/codespaces/organizations#manage-access-control-for-organization-codespaces)."
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/codespaces/organizations#remove-users-from-codespaces-access-for-an-organization](https://docs.github.com/rest/codespaces/organizations#remove-users-from-codespaces-access-for-an-organization)
  pub fn delete_codespaces_access_users(
    &self,
    org: impl Into<String>,
  ) -> NoContentRequest<delete_codespaces_access_users::Request, ()> {
    let org = org.into();
    let url = format!("/orgs/{org}/codespaces/access/selected_users");

    NoContentRequest::<delete_codespaces_access_users::Request, ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List organization secrets**
  ///
  /// Lists all Codespaces development environment secrets available at the organization-level without revealing their encrypted
  /// values.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/codespaces/organization-secrets#list-organization-secrets](https://docs.github.com/rest/codespaces/organization-secrets#list-organization-secrets)
  pub fn list_org_secrets(
    &self,
    org: impl Into<String>,
  ) -> Request<(), list_org_secrets::Query, list_org_secrets::Response> {
    let org = org.into();
    let url = format!("/orgs/{org}/codespaces/secrets");

    Request::<(), list_org_secrets::Query, list_org_secrets::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get an organization public key**
  ///
  /// Gets a public key for an organization, which is required in order to encrypt secrets. You need to encrypt the value of a secret before you can create or update secrets.
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/codespaces/organization-secrets#get-an-organization-public-key](https://docs.github.com/rest/codespaces/organization-secrets#get-an-organization-public-key)
  pub fn get_org_public_key(
    &self,
    org: impl Into<String>,
  ) -> Request<(), (), get_org_public_key::Response> {
    let org = org.into();
    let url = format!("/orgs/{org}/codespaces/secrets/public-key");

    Request::<(), (), get_org_public_key::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get an organization secret**
  ///
  /// Gets an organization development environment secret without revealing its encrypted value.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/codespaces/organization-secrets#get-an-organization-secret](https://docs.github.com/rest/codespaces/organization-secrets#get-an-organization-secret)
  pub fn get_org_secret(
    &self,
    org: impl Into<String>,
    secret_name: impl Into<String>,
  ) -> Request<(), (), get_org_secret::Response> {
    let org = org.into();
    let secret_name = secret_name.into();
    let url = format!("/orgs/{org}/codespaces/secrets/{secret_name}");

    Request::<(), (), get_org_secret::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create or update an organization secret**
  ///
  /// Creates or updates an organization development environment secret with an encrypted value. Encrypt your secret using
  /// [LibSodium](https://libsodium.gitbook.io/doc/bindings_for_other_languages). For more information, see "[Encrypting secrets for the REST API](https://docs.github.com/rest/guides/encrypting-secrets-for-the-rest-api)."
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/codespaces/organization-secrets#create-or-update-an-organization-secret](https://docs.github.com/rest/codespaces/organization-secrets#create-or-update-an-organization-secret)
  pub fn create_or_update_org_secret(
    &self,
    org: impl Into<String>,
    secret_name: impl Into<String>,
  ) -> Request<create_or_update_org_secret::Request, (), create_or_update_org_secret::Response> {
    let org = org.into();
    let secret_name = secret_name.into();
    let url = format!("/orgs/{org}/codespaces/secrets/{secret_name}");

    Request::<create_or_update_org_secret::Request, (), create_or_update_org_secret::Response>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **Delete an organization secret**
  ///
  /// Deletes an organization development environment secret using the secret name.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/codespaces/organization-secrets#delete-an-organization-secret](https://docs.github.com/rest/codespaces/organization-secrets#delete-an-organization-secret)
  pub fn delete_org_secret(
    &self,
    org: impl Into<String>,
    secret_name: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let secret_name = secret_name.into();
    let url = format!("/orgs/{org}/codespaces/secrets/{secret_name}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List selected repositories for an organization secret**
  ///
  /// Lists all repositories that have been selected when the `visibility`
  /// for repository access to a secret is set to `selected`.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/codespaces/organization-secrets#list-selected-repositories-for-an-organization-secret](https://docs.github.com/rest/codespaces/organization-secrets#list-selected-repositories-for-an-organization-secret)
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
    let url = format!("/orgs/{org}/codespaces/secrets/{secret_name}/repositories");

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
  /// Replaces all repositories for an organization development environment secret when the `visibility`
  /// for repository access is set to `selected`. The visibility is set when you [Create
  /// or update an organization secret](https://docs.github.com/rest/codespaces/organization-secrets#create-or-update-an-organization-secret).
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/codespaces/organization-secrets#set-selected-repositories-for-an-organization-secret](https://docs.github.com/rest/codespaces/organization-secrets#set-selected-repositories-for-an-organization-secret)
  pub fn set_selected_repos_for_org_secret(
    &self,
    org: impl Into<String>,
    secret_name: impl Into<String>,
  ) -> NoContentRequest<set_selected_repos_for_org_secret::Request, ()> {
    let org = org.into();
    let secret_name = secret_name.into();
    let url = format!("/orgs/{org}/codespaces/secrets/{secret_name}/repositories");

    NoContentRequest::<set_selected_repos_for_org_secret::Request, ()>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **Add selected repository to an organization secret**
  ///
  /// Adds a repository to an organization development environment secret when the `visibility` for repository access is set to `selected`. The visibility is set when you [Create or update an organization secret](https://docs.github.com/rest/codespaces/organization-secrets#create-or-update-an-organization-secret).
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/codespaces/organization-secrets#add-selected-repository-to-an-organization-secret](https://docs.github.com/rest/codespaces/organization-secrets#add-selected-repository-to-an-organization-secret)
  pub fn add_selected_repo_to_org_secret(
    &self,
    org: impl Into<String>,
    secret_name: impl Into<String>,
    repository_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let secret_name = secret_name.into();
    let repository_id = repository_id.into();
    let url = format!("/orgs/{org}/codespaces/secrets/{secret_name}/repositories/{repository_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **Remove selected repository from an organization secret**
  ///
  /// Removes a repository from an organization development environment secret when the `visibility`
  /// for repository access is set to `selected`. The visibility is set when you [Create
  /// or update an organization secret](https://docs.github.com/rest/codespaces/organization-secrets#create-or-update-an-organization-secret).
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/codespaces/organization-secrets#remove-selected-repository-from-an-organization-secret](https://docs.github.com/rest/codespaces/organization-secrets#remove-selected-repository-from-an-organization-secret)
  pub fn remove_selected_repo_from_org_secret(
    &self,
    org: impl Into<String>,
    secret_name: impl Into<String>,
    repository_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let secret_name = secret_name.into();
    let repository_id = repository_id.into();
    let url = format!("/orgs/{org}/codespaces/secrets/{secret_name}/repositories/{repository_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List codespaces for a user in organization**
  ///
  /// Lists the codespaces that a member of an organization has for repositories in that organization.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/codespaces/organizations#list-codespaces-for-a-user-in-organization](https://docs.github.com/rest/codespaces/organizations#list-codespaces-for-a-user-in-organization)
  pub fn get_codespaces_for_user_in_org(
    &self,
    org: impl Into<String>,
    username: impl Into<String>,
  ) -> Request<(), get_codespaces_for_user_in_org::Query, get_codespaces_for_user_in_org::Response>
  {
    let org = org.into();
    let username = username.into();
    let url = format!("/orgs/{org}/members/{username}/codespaces");

    Request::<(), get_codespaces_for_user_in_org::Query, get_codespaces_for_user_in_org::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Delete a codespace from the organization**
  ///
  /// Deletes a user's codespace.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/codespaces/organizations#delete-a-codespace-from-the-organization](https://docs.github.com/rest/codespaces/organizations#delete-a-codespace-from-the-organization)
  pub fn delete_from_organization(
    &self,
    org: impl Into<String>,
    username: impl Into<String>,
    codespace_name: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let username = username.into();
    let codespace_name = codespace_name.into();
    let url = format!("/orgs/{org}/members/{username}/codespaces/{codespace_name}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Stop a codespace for an organization user**
  ///
  /// Stops a user's codespace.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/codespaces/organizations#stop-a-codespace-for-an-organization-user](https://docs.github.com/rest/codespaces/organizations#stop-a-codespace-for-an-organization-user)
  pub fn stop_in_organization(
    &self,
    org: impl Into<String>,
    username: impl Into<String>,
    codespace_name: impl Into<String>,
  ) -> Request<(), (), stop_in_organization::Response> {
    let org = org.into();
    let username = username.into();
    let codespace_name = codespace_name.into();
    let url = format!("/orgs/{org}/members/{username}/codespaces/{codespace_name}/stop");

    Request::<(), (), stop_in_organization::Response>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **List codespaces in a repository for the authenticated user**
  ///
  /// Lists the codespaces associated to a specified repository and the authenticated user.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `codespace` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/codespaces/codespaces#list-codespaces-in-a-repository-for-the-authenticated-user](https://docs.github.com/rest/codespaces/codespaces#list-codespaces-in-a-repository-for-the-authenticated-user)
  pub fn list_in_repository_for_authenticated_user(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<
    (),
    list_in_repository_for_authenticated_user::Query,
    list_in_repository_for_authenticated_user::Response,
  > {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/codespaces");

    Request::<
      (),
      list_in_repository_for_authenticated_user::Query,
      list_in_repository_for_authenticated_user::Response,
    >::builder(&self.config)
    .get(url)
    .build()
  }

  /// **Create a codespace in a repository**
  ///
  /// Creates a codespace owned by the authenticated user in the specified repository.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `codespace` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/codespaces/codespaces#create-a-codespace-in-a-repository](https://docs.github.com/rest/codespaces/codespaces#create-a-codespace-in-a-repository)
  pub fn create_with_repo_for_authenticated_user(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<
    create_with_repo_for_authenticated_user::Request,
    (),
    create_with_repo_for_authenticated_user::Response,
  > {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/codespaces");

    Request::<
      create_with_repo_for_authenticated_user::Request,
      (),
      create_with_repo_for_authenticated_user::Response,
    >::builder(&self.config)
    .post(url)
    .build()
  }

  /// **List devcontainer configurations in a repository for the authenticated user**
  ///
  /// Lists the devcontainer.json files associated with a specified repository and the authenticated user. These files
  /// specify launchpoint configurations for codespaces created within the repository.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `codespace` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/codespaces/codespaces#list-devcontainer-configurations-in-a-repository-for-the-authenticated-user](https://docs.github.com/rest/codespaces/codespaces#list-devcontainer-configurations-in-a-repository-for-the-authenticated-user)
  pub fn list_devcontainers_in_repository_for_authenticated_user(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<
    (),
    list_devcontainers_in_repository_for_authenticated_user::Query,
    list_devcontainers_in_repository_for_authenticated_user::Response,
  > {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/codespaces/devcontainers");

    Request::<
      (),
      list_devcontainers_in_repository_for_authenticated_user::Query,
      list_devcontainers_in_repository_for_authenticated_user::Response,
    >::builder(&self.config)
    .get(url)
    .build()
  }

  /// **List available machine types for a repository**
  ///
  /// List the machine types available for a given repository based on its configuration.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `codespace` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/codespaces/machines#list-available-machine-types-for-a-repository](https://docs.github.com/rest/codespaces/machines#list-available-machine-types-for-a-repository)
  pub fn repo_machines_for_authenticated_user(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<
    (),
    repo_machines_for_authenticated_user::Query,
    repo_machines_for_authenticated_user::Response,
  > {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/codespaces/machines");

    Request::<
      (),
      repo_machines_for_authenticated_user::Query,
      repo_machines_for_authenticated_user::Response,
    >::builder(&self.config)
    .get(url)
    .build()
  }

  /// **Get default attributes for a codespace**
  ///
  /// Gets the default attributes for codespaces created by the user with the repository.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `codespace` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/codespaces/codespaces#get-default-attributes-for-a-codespace](https://docs.github.com/rest/codespaces/codespaces#get-default-attributes-for-a-codespace)
  pub fn pre_flight_with_repo_for_authenticated_user(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<
    (),
    pre_flight_with_repo_for_authenticated_user::Query,
    pre_flight_with_repo_for_authenticated_user::Response,
  > {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/codespaces/new");

    Request::<
      (),
      pre_flight_with_repo_for_authenticated_user::Query,
      pre_flight_with_repo_for_authenticated_user::Response,
    >::builder(&self.config)
    .get(url)
    .build()
  }

  /// **Check if permissions defined by a devcontainer have been accepted by the authenticated user**
  ///
  /// Checks whether the permissions defined by a given devcontainer configuration have been accepted by the authenticated user.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `codespace` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/codespaces/codespaces#check-if-permissions-defined-by-a-devcontainer-have-been-accepted-by-the-authenticated-user](https://docs.github.com/rest/codespaces/codespaces#check-if-permissions-defined-by-a-devcontainer-have-been-accepted-by-the-authenticated-user)
  pub fn check_permissions_for_devcontainer(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<
    (),
    check_permissions_for_devcontainer::Query,
    check_permissions_for_devcontainer::Response,
  > {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/codespaces/permissions_check");

    Request::<
      (),
      check_permissions_for_devcontainer::Query,
      check_permissions_for_devcontainer::Response,
    >::builder(&self.config)
    .get(url)
    .build()
  }

  /// **List repository secrets**
  ///
  /// Lists all development environment secrets available in a repository without revealing their encrypted
  /// values.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/codespaces/repository-secrets#list-repository-secrets](https://docs.github.com/rest/codespaces/repository-secrets#list-repository-secrets)
  pub fn list_repo_secrets(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), list_repo_secrets::Query, list_repo_secrets::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/codespaces/secrets");

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
  /// If the repository is private, OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/codespaces/repository-secrets#get-a-repository-public-key](https://docs.github.com/rest/codespaces/repository-secrets#get-a-repository-public-key)
  pub fn get_repo_public_key(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), (), get_repo_public_key::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/codespaces/secrets/public-key");

    Request::<(), (), get_repo_public_key::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get a repository secret**
  ///
  /// Gets a single repository development environment secret without revealing its encrypted value.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/codespaces/repository-secrets#get-a-repository-secret](https://docs.github.com/rest/codespaces/repository-secrets#get-a-repository-secret)
  pub fn get_repo_secret(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    secret_name: impl Into<String>,
  ) -> Request<(), (), get_repo_secret::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let secret_name = secret_name.into();
    let url = format!("/repos/{owner}/{repo}/codespaces/secrets/{secret_name}");

    Request::<(), (), get_repo_secret::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create or update a repository secret**
  ///
  /// Creates or updates a repository development environment secret with an encrypted value. Encrypt your secret using
  /// [LibSodium](https://libsodium.gitbook.io/doc/bindings_for_other_languages). For more information, see "[Encrypting secrets for the REST API](https://docs.github.com/rest/guides/encrypting-secrets-for-the-rest-api)."
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/codespaces/repository-secrets#create-or-update-a-repository-secret](https://docs.github.com/rest/codespaces/repository-secrets#create-or-update-a-repository-secret)
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
    let url = format!("/repos/{owner}/{repo}/codespaces/secrets/{secret_name}");

    Request::<create_or_update_repo_secret::Request, (), create_or_update_repo_secret::Response>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **Delete a repository secret**
  ///
  /// Deletes a development environment secret in a repository using the secret name.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/codespaces/repository-secrets#delete-a-repository-secret](https://docs.github.com/rest/codespaces/repository-secrets#delete-a-repository-secret)
  pub fn delete_repo_secret(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    secret_name: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let secret_name = secret_name.into();
    let url = format!("/repos/{owner}/{repo}/codespaces/secrets/{secret_name}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Create a codespace from a pull request**
  ///
  /// Creates a codespace owned by the authenticated user for the specified pull request.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `codespace` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/codespaces/codespaces#create-a-codespace-from-a-pull-request](https://docs.github.com/rest/codespaces/codespaces#create-a-codespace-from-a-pull-request)
  pub fn create_with_pr_for_authenticated_user(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    pull_number: impl Into<i64>,
  ) -> Request<
    create_with_pr_for_authenticated_user::Request,
    (),
    create_with_pr_for_authenticated_user::Response,
  > {
    let owner = owner.into();
    let repo = repo.into();
    let pull_number = pull_number.into();
    let url = format!("/repos/{owner}/{repo}/pulls/{pull_number}/codespaces");

    Request::<
      create_with_pr_for_authenticated_user::Request,
      (),
      create_with_pr_for_authenticated_user::Response,
    >::builder(&self.config)
    .post(url)
    .build()
  }

  /// **List codespaces for the authenticated user**
  ///
  /// Lists the authenticated user's codespaces.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `codespace` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/codespaces/codespaces#list-codespaces-for-the-authenticated-user](https://docs.github.com/rest/codespaces/codespaces#list-codespaces-for-the-authenticated-user)
  pub fn list_for_authenticated_user(
    &self,
  ) -> Request<(), list_for_authenticated_user::Query, list_for_authenticated_user::Response> {
    let url = format!("/user/codespaces");

    Request::<(), list_for_authenticated_user::Query, list_for_authenticated_user::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create a codespace for the authenticated user**
  ///
  /// Creates a new codespace, owned by the authenticated user.
  ///
  /// This endpoint requires either a `repository_id` OR a `pull_request` but not both.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `codespace` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/codespaces/codespaces#create-a-codespace-for-the-authenticated-user](https://docs.github.com/rest/codespaces/codespaces#create-a-codespace-for-the-authenticated-user)
  pub fn create_for_authenticated_user(
    &self,
  ) -> Request<create_for_authenticated_user::Request, (), create_for_authenticated_user::Response>
  {
    let url = format!("/user/codespaces");

    Request::<create_for_authenticated_user::Request, (), create_for_authenticated_user::Response>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **List secrets for the authenticated user**
  ///
  /// Lists all development environment secrets available for a user's codespaces without revealing their
  /// encrypted values.
  ///
  /// The authenticated user must have Codespaces access to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `codespace` or `codespace:secrets` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/codespaces/secrets#list-secrets-for-the-authenticated-user](https://docs.github.com/rest/codespaces/secrets#list-secrets-for-the-authenticated-user)
  pub fn list_secrets_for_authenticated_user(
    &self,
  ) -> Request<
    (),
    list_secrets_for_authenticated_user::Query,
    list_secrets_for_authenticated_user::Response,
  > {
    let url = format!("/user/codespaces/secrets");

    Request::<
      (),
      list_secrets_for_authenticated_user::Query,
      list_secrets_for_authenticated_user::Response,
    >::builder(&self.config)
    .get(url)
    .build()
  }

  /// **Get public key for the authenticated user**
  ///
  /// Gets your public key, which you need to encrypt secrets. You need to encrypt a secret before you can create or update secrets.
  ///
  /// The authenticated user must have Codespaces access to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `codespace` or `codespace:secrets` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/codespaces/secrets#get-public-key-for-the-authenticated-user](https://docs.github.com/rest/codespaces/secrets#get-public-key-for-the-authenticated-user)
  pub fn get_public_key_for_authenticated_user(
    &self,
  ) -> Request<(), (), get_public_key_for_authenticated_user::Response> {
    let url = format!("/user/codespaces/secrets/public-key");

    Request::<(), (), get_public_key_for_authenticated_user::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get a secret for the authenticated user**
  ///
  /// Gets a development environment secret available to a user's codespaces without revealing its encrypted value.
  ///
  /// The authenticated user must have Codespaces access to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `codespace` or `codespace:secrets` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/codespaces/secrets#get-a-secret-for-the-authenticated-user](https://docs.github.com/rest/codespaces/secrets#get-a-secret-for-the-authenticated-user)
  pub fn get_secret_for_authenticated_user(
    &self,
    secret_name: impl Into<String>,
  ) -> Request<(), (), get_secret_for_authenticated_user::Response> {
    let secret_name = secret_name.into();
    let url = format!("/user/codespaces/secrets/{secret_name}");

    Request::<(), (), get_secret_for_authenticated_user::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create or update a secret for the authenticated user**
  ///
  /// Creates or updates a development environment secret for a user's codespace with an encrypted value. Encrypt your secret using
  /// [LibSodium](https://libsodium.gitbook.io/doc/bindings_for_other_languages). For more information, see "[Encrypting secrets for the REST API](https://docs.github.com/rest/guides/encrypting-secrets-for-the-rest-api)."
  ///
  /// The authenticated user must have Codespaces access to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `codespace` or `codespace:secrets` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/codespaces/secrets#create-or-update-a-secret-for-the-authenticated-user](https://docs.github.com/rest/codespaces/secrets#create-or-update-a-secret-for-the-authenticated-user)
  pub fn create_or_update_secret_for_authenticated_user(
    &self,
    secret_name: impl Into<String>,
  ) -> Request<
    create_or_update_secret_for_authenticated_user::Request,
    (),
    create_or_update_secret_for_authenticated_user::Response,
  > {
    let secret_name = secret_name.into();
    let url = format!("/user/codespaces/secrets/{secret_name}");

    Request::<
      create_or_update_secret_for_authenticated_user::Request,
      (),
      create_or_update_secret_for_authenticated_user::Response,
    >::builder(&self.config)
    .put(url)
    .build()
  }

  /// **Delete a secret for the authenticated user**
  ///
  /// Deletes a development environment secret from a user's codespaces using the secret name. Deleting the secret will remove access from all codespaces that were allowed to access the secret.
  ///
  /// The authenticated user must have Codespaces access to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `codespace` or `codespace:secrets` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/codespaces/secrets#delete-a-secret-for-the-authenticated-user](https://docs.github.com/rest/codespaces/secrets#delete-a-secret-for-the-authenticated-user)
  pub fn delete_secret_for_authenticated_user(
    &self,
    secret_name: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let secret_name = secret_name.into();
    let url = format!("/user/codespaces/secrets/{secret_name}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List selected repositories for a user secret**
  ///
  /// List the repositories that have been granted the ability to use a user's development environment secret.
  ///
  /// The authenticated user must have Codespaces access to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `codespace` or `codespace:secrets` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/codespaces/secrets#list-selected-repositories-for-a-user-secret](https://docs.github.com/rest/codespaces/secrets#list-selected-repositories-for-a-user-secret)
  pub fn list_repositories_for_secret_for_authenticated_user(
    &self,
    secret_name: impl Into<String>,
  ) -> Request<(), (), list_repositories_for_secret_for_authenticated_user::Response> {
    let secret_name = secret_name.into();
    let url = format!("/user/codespaces/secrets/{secret_name}/repositories");

    Request::<(), (), list_repositories_for_secret_for_authenticated_user::Response>::builder(
      &self.config,
    )
    .get(url)
    .build()
  }

  /// **Set selected repositories for a user secret**
  ///
  /// Select the repositories that will use a user's development environment secret.
  ///
  /// The authenticated user must have Codespaces access to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `codespace` or `codespace:secrets` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/codespaces/secrets#set-selected-repositories-for-a-user-secret](https://docs.github.com/rest/codespaces/secrets#set-selected-repositories-for-a-user-secret)
  pub fn set_repositories_for_secret_for_authenticated_user(
    &self,
    secret_name: impl Into<String>,
  ) -> NoContentRequest<set_repositories_for_secret_for_authenticated_user::Request, ()> {
    let secret_name = secret_name.into();
    let url = format!("/user/codespaces/secrets/{secret_name}/repositories");

    NoContentRequest::<set_repositories_for_secret_for_authenticated_user::Request, ()>::builder(
      &self.config,
    )
    .put(url)
    .build()
  }

  /// **Add a selected repository to a user secret**
  ///
  /// Adds a repository to the selected repositories for a user's development environment secret.
  ///
  /// The authenticated user must have Codespaces access to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `codespace` or `codespace:secrets` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/codespaces/secrets#add-a-selected-repository-to-a-user-secret](https://docs.github.com/rest/codespaces/secrets#add-a-selected-repository-to-a-user-secret)
  pub fn add_repository_for_secret_for_authenticated_user(
    &self,
    secret_name: impl Into<String>,
    repository_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let secret_name = secret_name.into();
    let repository_id = repository_id.into();
    let url = format!("/user/codespaces/secrets/{secret_name}/repositories/{repository_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **Remove a selected repository from a user secret**
  ///
  /// Removes a repository from the selected repositories for a user's development environment secret.
  ///
  /// The authenticated user must have Codespaces access to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `codespace` or `codespace:secrets` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/codespaces/secrets#remove-a-selected-repository-from-a-user-secret](https://docs.github.com/rest/codespaces/secrets#remove-a-selected-repository-from-a-user-secret)
  pub fn remove_repository_for_secret_for_authenticated_user(
    &self,
    secret_name: impl Into<String>,
    repository_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let secret_name = secret_name.into();
    let repository_id = repository_id.into();
    let url = format!("/user/codespaces/secrets/{secret_name}/repositories/{repository_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Get a codespace for the authenticated user**
  ///
  /// Gets information about a user's codespace.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `codespace` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/codespaces/codespaces#get-a-codespace-for-the-authenticated-user](https://docs.github.com/rest/codespaces/codespaces#get-a-codespace-for-the-authenticated-user)
  pub fn get_for_authenticated_user(
    &self,
    codespace_name: impl Into<String>,
  ) -> Request<(), (), get_for_authenticated_user::Response> {
    let codespace_name = codespace_name.into();
    let url = format!("/user/codespaces/{codespace_name}");

    Request::<(), (), get_for_authenticated_user::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Update a codespace for the authenticated user**
  ///
  /// Updates a codespace owned by the authenticated user. Currently only the codespace's machine type and recent folders can be modified using this endpoint.
  ///
  /// If you specify a new machine type it will be applied the next time your codespace is started.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `codespace` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/codespaces/codespaces#update-a-codespace-for-the-authenticated-user](https://docs.github.com/rest/codespaces/codespaces#update-a-codespace-for-the-authenticated-user)
  pub fn update_for_authenticated_user(
    &self,
    codespace_name: impl Into<String>,
  ) -> Request<update_for_authenticated_user::Request, (), update_for_authenticated_user::Response>
  {
    let codespace_name = codespace_name.into();
    let url = format!("/user/codespaces/{codespace_name}");

    Request::<update_for_authenticated_user::Request, (), update_for_authenticated_user::Response>::builder(&self.config)
      .patch(url)
      .build()
  }

  /// **Delete a codespace for the authenticated user**
  ///
  /// Deletes a user's codespace.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `codespace` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/codespaces/codespaces#delete-a-codespace-for-the-authenticated-user](https://docs.github.com/rest/codespaces/codespaces#delete-a-codespace-for-the-authenticated-user)
  pub fn delete_for_authenticated_user(
    &self,
    codespace_name: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let codespace_name = codespace_name.into();
    let url = format!("/user/codespaces/{codespace_name}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Export a codespace for the authenticated user**
  ///
  /// Triggers an export of the specified codespace and returns a URL and ID where the status of the export can be monitored.
  ///
  /// If changes cannot be pushed to the codespace's repository, they will be pushed to a new or previously-existing fork instead.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `codespace` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/codespaces/codespaces#export-a-codespace-for-the-authenticated-user](https://docs.github.com/rest/codespaces/codespaces#export-a-codespace-for-the-authenticated-user)
  pub fn export_for_authenticated_user(
    &self,
    codespace_name: impl Into<String>,
  ) -> Request<(), (), export_for_authenticated_user::Response> {
    let codespace_name = codespace_name.into();
    let url = format!("/user/codespaces/{codespace_name}/exports");

    Request::<(), (), export_for_authenticated_user::Response>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Get details about a codespace export**
  ///
  /// Gets information about an export of a codespace.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `codespace` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/codespaces/codespaces#get-details-about-a-codespace-export](https://docs.github.com/rest/codespaces/codespaces#get-details-about-a-codespace-export)
  pub fn get_export_details_for_authenticated_user(
    &self,
    codespace_name: impl Into<String>,
    export_id: impl Into<String>,
  ) -> Request<(), (), get_export_details_for_authenticated_user::Response> {
    let codespace_name = codespace_name.into();
    let export_id = export_id.into();
    let url = format!("/user/codespaces/{codespace_name}/exports/{export_id}");

    Request::<(), (), get_export_details_for_authenticated_user::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List machine types for a codespace**
  ///
  /// List the machine types a codespace can transition to use.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `codespace` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/codespaces/machines#list-machine-types-for-a-codespace](https://docs.github.com/rest/codespaces/machines#list-machine-types-for-a-codespace)
  pub fn codespace_machines_for_authenticated_user(
    &self,
    codespace_name: impl Into<String>,
  ) -> Request<(), (), codespace_machines_for_authenticated_user::Response> {
    let codespace_name = codespace_name.into();
    let url = format!("/user/codespaces/{codespace_name}/machines");

    Request::<(), (), codespace_machines_for_authenticated_user::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create a repository from an unpublished codespace**
  ///
  /// Publishes an unpublished codespace, creating a new repository and assigning it to the codespace.
  ///
  /// The codespace's token is granted write permissions to the repository, allowing the user to push their changes.
  ///
  /// This will fail for a codespace that is already published, meaning it has an associated repository.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `codespace` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/codespaces/codespaces#create-a-repository-from-an-unpublished-codespace](https://docs.github.com/rest/codespaces/codespaces#create-a-repository-from-an-unpublished-codespace)
  pub fn publish_for_authenticated_user(
    &self,
    codespace_name: impl Into<String>,
  ) -> Request<publish_for_authenticated_user::Request, (), publish_for_authenticated_user::Response>
  {
    let codespace_name = codespace_name.into();
    let url = format!("/user/codespaces/{codespace_name}/publish");

    Request::<publish_for_authenticated_user::Request, (), publish_for_authenticated_user::Response>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Start a codespace for the authenticated user**
  ///
  /// Starts a user's codespace.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `codespace` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/codespaces/codespaces#start-a-codespace-for-the-authenticated-user](https://docs.github.com/rest/codespaces/codespaces#start-a-codespace-for-the-authenticated-user)
  pub fn start_for_authenticated_user(
    &self,
    codespace_name: impl Into<String>,
  ) -> Request<(), (), start_for_authenticated_user::Response> {
    let codespace_name = codespace_name.into();
    let url = format!("/user/codespaces/{codespace_name}/start");

    Request::<(), (), start_for_authenticated_user::Response>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Stop a codespace for the authenticated user**
  ///
  /// Stops a user's codespace.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `codespace` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/codespaces/codespaces#stop-a-codespace-for-the-authenticated-user](https://docs.github.com/rest/codespaces/codespaces#stop-a-codespace-for-the-authenticated-user)
  pub fn stop_for_authenticated_user(
    &self,
    codespace_name: impl Into<String>,
  ) -> Request<(), (), stop_for_authenticated_user::Response> {
    let codespace_name = codespace_name.into();
    let url = format!("/user/codespaces/{codespace_name}/stop");

    Request::<(), (), stop_for_authenticated_user::Response>::builder(&self.config)
      .post(url)
      .build()
  }
}
