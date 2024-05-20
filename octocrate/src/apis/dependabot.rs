use octocrate_core::*;
#[allow(unused_imports)]
use octocrate_types::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use typed_builder::TypedBuilder;

pub mod list_alerts_for_enterprise {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<DependabotAlertWithRepository>;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// A comma-separated list of states. If specified, only alerts with these states will be returned.
    ///
    /// Can be: `auto_dismissed`, `dismissed`, `fixed`, `open`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub state: Option<String>,
    /// A comma-separated list of severities. If specified, only alerts with these severities will be returned.
    ///
    /// Can be: `low`, `medium`, `high`, `critical`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub severity: Option<String>,
    /// A comma-separated list of ecosystems. If specified, only alerts for these ecosystems will be returned.
    ///
    /// Can be: `composer`, `go`, `maven`, `npm`, `nuget`, `pip`, `pub`, `rubygems`, `rust`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub ecosystem: Option<String>,
    /// A comma-separated list of package names. If specified, only alerts for these packages will be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub package: Option<String>,
    /// The scope of the vulnerable dependency. If specified, only alerts with this scope will be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub scope: Option<parameters::DependabotAlertScope>,
    /// The property by which to sort the results.
    /// `created` means when the alert was created.
    /// `updated` means when the alert's state last changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub sort: Option<parameters::DependabotAlertSort>,
    /// The direction to sort the results by.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub direction: Option<parameters::Direction>,
    /// A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results before this cursor. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub before: Option<String>,
    /// A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results after this cursor. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub after: Option<String>,
    /// **Deprecated**. The number of results per page (max 100), starting from the first matching result.
    /// This parameter must not be used in combination with `last`.
    /// Instead, use `per_page` in combination with `after` to fetch the first page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub first: Option<i64>,
    /// **Deprecated**. The number of results per page (max 100), starting from the last matching result.
    /// This parameter must not be used in combination with `first`.
    /// Instead, use `per_page` in combination with `before` to fetch the last page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub last: Option<i64>,
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
  }
}

pub mod list_alerts_for_org {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<DependabotAlertWithRepository>;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// A comma-separated list of states. If specified, only alerts with these states will be returned.
    ///
    /// Can be: `auto_dismissed`, `dismissed`, `fixed`, `open`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub state: Option<String>,
    /// A comma-separated list of severities. If specified, only alerts with these severities will be returned.
    ///
    /// Can be: `low`, `medium`, `high`, `critical`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub severity: Option<String>,
    /// A comma-separated list of ecosystems. If specified, only alerts for these ecosystems will be returned.
    ///
    /// Can be: `composer`, `go`, `maven`, `npm`, `nuget`, `pip`, `pub`, `rubygems`, `rust`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub ecosystem: Option<String>,
    /// A comma-separated list of package names. If specified, only alerts for these packages will be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub package: Option<String>,
    /// The scope of the vulnerable dependency. If specified, only alerts with this scope will be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub scope: Option<parameters::DependabotAlertScope>,
    /// The property by which to sort the results.
    /// `created` means when the alert was created.
    /// `updated` means when the alert's state last changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub sort: Option<parameters::DependabotAlertSort>,
    /// The direction to sort the results by.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub direction: Option<parameters::Direction>,
    /// A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results before this cursor. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub before: Option<String>,
    /// A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results after this cursor. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub after: Option<String>,
    /// **Deprecated**. The number of results per page (max 100), starting from the first matching result.
    /// This parameter must not be used in combination with `last`.
    /// Instead, use `per_page` in combination with `after` to fetch the first page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub first: Option<i64>,
    /// **Deprecated**. The number of results per page (max 100), starting from the last matching result.
    /// This parameter must not be used in combination with `first`.
    /// Instead, use `per_page` in combination with `before` to fetch the last page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub last: Option<i64>,
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
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
    pub secrets: Vec<OrganizationDependabotSecret>,
    pub total_count: i64,
  }
}

pub mod get_org_public_key {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = DependabotPublicKey;
}

pub mod get_org_secret {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = OrganizationDependabotSecret;
}

pub mod create_or_update_org_secret {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = EmptyObject;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// Value for your secret, encrypted with [LibSodium](https://libsodium.gitbook.io/doc/bindings_for_other_languages) using the public key retrieved from the [Get an organization public key](https://docs.github.com/rest/dependabot/secrets#get-an-organization-public-key) endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub encrypted_value: Option<String>,
    /// ID of the key you used to encrypt the secret.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub key_id: Option<String>,
    /// An array of repository ids that can access the organization secret. You can only provide a list of repository ids when the `visibility` is set to `selected`. You can manage the list of selected repositories using the [List selected repositories for an organization secret](https://docs.github.com/rest/dependabot/secrets#list-selected-repositories-for-an-organization-secret), [Set selected repositories for an organization secret](https://docs.github.com/rest/dependabot/secrets#set-selected-repositories-for-an-organization-secret), and [Remove selected repository from an organization secret](https://docs.github.com/rest/dependabot/secrets#remove-selected-repository-from-an-organization-secret) endpoints.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub selected_repository_ids: Option<Vec<String>>,
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
    /// An array of repository ids that can access the organization secret. You can only provide a list of repository ids when the `visibility` is set to `selected`. You can add and remove individual repositories using the [Set selected repositories for an organization secret](https://docs.github.com/rest/dependabot/secrets#set-selected-repositories-for-an-organization-secret) and [Remove selected repository from an organization secret](https://docs.github.com/rest/dependabot/secrets#remove-selected-repository-from-an-organization-secret) endpoints.
    pub selected_repository_ids: Vec<i64>,
  }
}

pub mod list_alerts_for_repo {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<DependabotAlert>;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// A comma-separated list of states. If specified, only alerts with these states will be returned.
    ///
    /// Can be: `auto_dismissed`, `dismissed`, `fixed`, `open`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub state: Option<String>,
    /// A comma-separated list of severities. If specified, only alerts with these severities will be returned.
    ///
    /// Can be: `low`, `medium`, `high`, `critical`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub severity: Option<String>,
    /// A comma-separated list of ecosystems. If specified, only alerts for these ecosystems will be returned.
    ///
    /// Can be: `composer`, `go`, `maven`, `npm`, `nuget`, `pip`, `pub`, `rubygems`, `rust`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub ecosystem: Option<String>,
    /// A comma-separated list of package names. If specified, only alerts for these packages will be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub package: Option<String>,
    /// A comma-separated list of full manifest paths. If specified, only alerts for these manifests will be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub manifest: Option<String>,
    /// The scope of the vulnerable dependency. If specified, only alerts with this scope will be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub scope: Option<parameters::DependabotAlertScope>,
    /// The property by which to sort the results.
    /// `created` means when the alert was created.
    /// `updated` means when the alert's state last changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub sort: Option<parameters::DependabotAlertSort>,
    /// The direction to sort the results by.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub direction: Option<parameters::Direction>,
    /// **Deprecated**. Page number of the results to fetch. Use cursor-based pagination with `before` or `after` instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
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
    /// **Deprecated**. The number of results per page (max 100), starting from the first matching result.
    /// This parameter must not be used in combination with `last`.
    /// Instead, use `per_page` in combination with `after` to fetch the first page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub first: Option<i64>,
    /// **Deprecated**. The number of results per page (max 100), starting from the last matching result.
    /// This parameter must not be used in combination with `first`.
    /// Instead, use `per_page` in combination with `before` to fetch the last page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub last: Option<i64>,
  }
}

pub mod get_alert {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = DependabotAlert;
}

pub mod update_alert {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = DependabotAlert;

  /// The state of the Dependabot alert.
  /// A `dismissed_reason` must be provided when setting the state to `dismissed`.
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum RequestState {
    #[serde(rename = "dismissed")]
    Dismissed,
    #[serde(rename = "open")]
    Open,
  }

  impl ToString for RequestState {
    fn to_string(&self) -> String {
      match self {
        RequestState::Dismissed => "dismissed".to_string(),
        RequestState::Open => "open".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// An optional comment associated with dismissing the alert.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub dismissed_comment: Option<String>,
    /// **Required when `state` is `dismissed`.** A reason for dismissing the alert.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub dismissed_reason: Option<DismissedReason>,
    /// The state of the Dependabot alert.
    /// A `dismissed_reason` must be provided when setting the state to `dismissed`.
    pub state: RequestState,
  }
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
    pub secrets: Vec<DependabotSecret>,
    pub total_count: i64,
  }
}

pub mod get_repo_public_key {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = DependabotPublicKey;
}

pub mod get_repo_secret {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = DependabotSecret;
}

pub mod create_or_update_repo_secret {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = EmptyObject;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// Value for your secret, encrypted with [LibSodium](https://libsodium.gitbook.io/doc/bindings_for_other_languages) using the public key retrieved from the [Get a repository public key](https://docs.github.com/rest/dependabot/secrets#get-a-repository-public-key) endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub encrypted_value: Option<String>,
    /// ID of the key you used to encrypt the secret.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub key_id: Option<String>,
  }
}

/// Endpoints to manage Dependabot.
pub struct GitHubDependabotAPI {
  config: SharedAPIConfig,
}

impl GitHubDependabotAPI {
  pub fn new(config: &SharedAPIConfig) -> Self {
    Self {
      config: config.clone(),
    }
  }

  /// **List Dependabot alerts for an enterprise**
  ///
  /// Lists Dependabot alerts for repositories that are owned by the specified enterprise.
  ///
  /// The authenticated user must be a member of the enterprise to use this endpoint.
  ///
  /// Alerts are only returned for organizations in the enterprise for which you are an organization owner or a security manager. For more information about security managers, see "[Managing security managers in your organization](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/managing-security-managers-in-your-organization)."
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` or `security_events` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/dependabot/alerts#list-dependabot-alerts-for-an-enterprise](https://docs.github.com/rest/dependabot/alerts#list-dependabot-alerts-for-an-enterprise)
  pub fn list_alerts_for_enterprise(
    &self,
    enterprise: impl Into<String>,
  ) -> Request<(), list_alerts_for_enterprise::Query, list_alerts_for_enterprise::Response> {
    let enterprise = enterprise.into();
    let url = format!("/enterprises/{enterprise}/dependabot/alerts");

    Request::<(), list_alerts_for_enterprise::Query, list_alerts_for_enterprise::Response>::builder(
      &self.config,
    )
    .get(url)
    .build()
  }

  /// **List Dependabot alerts for an organization**
  ///
  /// Lists Dependabot alerts for an organization.
  ///
  /// The authenticated user must be an owner or security manager for the organization to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `security_events` scope to use this endpoint. If this endpoint is only used with public repositories, the token can use the `public_repo` scope instead.
  ///
  /// *Documentation*: [https://docs.github.com/rest/dependabot/alerts#list-dependabot-alerts-for-an-organization](https://docs.github.com/rest/dependabot/alerts#list-dependabot-alerts-for-an-organization)
  pub fn list_alerts_for_org(
    &self,
    org: impl Into<String>,
  ) -> Request<(), list_alerts_for_org::Query, list_alerts_for_org::Response> {
    let org = org.into();
    let url = format!("/orgs/{org}/dependabot/alerts");

    Request::<(), list_alerts_for_org::Query, list_alerts_for_org::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List organization secrets**
  ///
  /// Lists all secrets available in an organization without revealing their
  /// encrypted values.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/dependabot/secrets#list-organization-secrets](https://docs.github.com/rest/dependabot/secrets#list-organization-secrets)
  pub fn list_org_secrets(
    &self,
    org: impl Into<String>,
  ) -> Request<(), list_org_secrets::Query, list_org_secrets::Response> {
    let org = org.into();
    let url = format!("/orgs/{org}/dependabot/secrets");

    Request::<(), list_org_secrets::Query, list_org_secrets::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get an organization public key**
  ///
  /// Gets your public key, which you need to encrypt secrets. You need to
  /// encrypt a secret before you can create or update secrets.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/dependabot/secrets#get-an-organization-public-key](https://docs.github.com/rest/dependabot/secrets#get-an-organization-public-key)
  pub fn get_org_public_key(
    &self,
    org: impl Into<String>,
  ) -> Request<(), (), get_org_public_key::Response> {
    let org = org.into();
    let url = format!("/orgs/{org}/dependabot/secrets/public-key");

    Request::<(), (), get_org_public_key::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get an organization secret**
  ///
  /// Gets a single organization secret without revealing its encrypted value.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/dependabot/secrets#get-an-organization-secret](https://docs.github.com/rest/dependabot/secrets#get-an-organization-secret)
  pub fn get_org_secret(
    &self,
    org: impl Into<String>,
    secret_name: impl Into<String>,
  ) -> Request<(), (), get_org_secret::Response> {
    let org = org.into();
    let secret_name = secret_name.into();
    let url = format!("/orgs/{org}/dependabot/secrets/{secret_name}");

    Request::<(), (), get_org_secret::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create or update an organization secret**
  ///
  /// Creates or updates an organization secret with an encrypted value. Encrypt your secret using
  /// [LibSodium](https://libsodium.gitbook.io/doc/bindings_for_other_languages). For more information, see "[Encrypting secrets for the REST API](https://docs.github.com/rest/guides/encrypting-secrets-for-the-rest-api)."
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/dependabot/secrets#create-or-update-an-organization-secret](https://docs.github.com/rest/dependabot/secrets#create-or-update-an-organization-secret)
  pub fn create_or_update_org_secret(
    &self,
    org: impl Into<String>,
    secret_name: impl Into<String>,
  ) -> Request<create_or_update_org_secret::Request, (), create_or_update_org_secret::Response> {
    let org = org.into();
    let secret_name = secret_name.into();
    let url = format!("/orgs/{org}/dependabot/secrets/{secret_name}");

    Request::<create_or_update_org_secret::Request, (), create_or_update_org_secret::Response>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **Delete an organization secret**
  ///
  /// Deletes a secret in an organization using the secret name.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/dependabot/secrets#delete-an-organization-secret](https://docs.github.com/rest/dependabot/secrets#delete-an-organization-secret)
  pub fn delete_org_secret(
    &self,
    org: impl Into<String>,
    secret_name: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let secret_name = secret_name.into();
    let url = format!("/orgs/{org}/dependabot/secrets/{secret_name}");

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
  /// *Documentation*: [https://docs.github.com/rest/dependabot/secrets#list-selected-repositories-for-an-organization-secret](https://docs.github.com/rest/dependabot/secrets#list-selected-repositories-for-an-organization-secret)
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
    let url = format!("/orgs/{org}/dependabot/secrets/{secret_name}/repositories");

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
  /// or update an organization secret](https://docs.github.com/rest/dependabot/secrets#create-or-update-an-organization-secret).
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/dependabot/secrets#set-selected-repositories-for-an-organization-secret](https://docs.github.com/rest/dependabot/secrets#set-selected-repositories-for-an-organization-secret)
  pub fn set_selected_repos_for_org_secret(
    &self,
    org: impl Into<String>,
    secret_name: impl Into<String>,
  ) -> NoContentRequest<set_selected_repos_for_org_secret::Request, ()> {
    let org = org.into();
    let secret_name = secret_name.into();
    let url = format!("/orgs/{org}/dependabot/secrets/{secret_name}/repositories");

    NoContentRequest::<set_selected_repos_for_org_secret::Request, ()>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **Add selected repository to an organization secret**
  ///
  /// Adds a repository to an organization secret when the `visibility` for
  /// repository access is set to `selected`. The visibility is set when you [Create or
  /// update an organization secret](https://docs.github.com/rest/dependabot/secrets#create-or-update-an-organization-secret).
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/dependabot/secrets#add-selected-repository-to-an-organization-secret](https://docs.github.com/rest/dependabot/secrets#add-selected-repository-to-an-organization-secret)
  pub fn add_selected_repo_to_org_secret(
    &self,
    org: impl Into<String>,
    secret_name: impl Into<String>,
    repository_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let secret_name = secret_name.into();
    let repository_id = repository_id.into();
    let url = format!("/orgs/{org}/dependabot/secrets/{secret_name}/repositories/{repository_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **Remove selected repository from an organization secret**
  ///
  /// Removes a repository from an organization secret when the `visibility`
  /// for repository access is set to `selected`. The visibility is set when you [Create
  /// or update an organization secret](https://docs.github.com/rest/dependabot/secrets#create-or-update-an-organization-secret).
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/dependabot/secrets#remove-selected-repository-from-an-organization-secret](https://docs.github.com/rest/dependabot/secrets#remove-selected-repository-from-an-organization-secret)
  pub fn remove_selected_repo_from_org_secret(
    &self,
    org: impl Into<String>,
    secret_name: impl Into<String>,
    repository_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let secret_name = secret_name.into();
    let repository_id = repository_id.into();
    let url = format!("/orgs/{org}/dependabot/secrets/{secret_name}/repositories/{repository_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List Dependabot alerts for a repository**
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `security_events` scope to use this endpoint. If this endpoint is only used with public repositories, the token can use the `public_repo` scope instead.
  ///
  /// *Documentation*: [https://docs.github.com/rest/dependabot/alerts#list-dependabot-alerts-for-a-repository](https://docs.github.com/rest/dependabot/alerts#list-dependabot-alerts-for-a-repository)
  pub fn list_alerts_for_repo(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), list_alerts_for_repo::Query, list_alerts_for_repo::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/dependabot/alerts");

    Request::<(), list_alerts_for_repo::Query, list_alerts_for_repo::Response>::builder(
      &self.config,
    )
    .get(url)
    .build()
  }

  /// **Get a Dependabot alert**
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `security_events` scope to use this endpoint. If this endpoint is only used with public repositories, the token can use the `public_repo` scope instead.
  ///
  /// *Documentation*: [https://docs.github.com/rest/dependabot/alerts#get-a-dependabot-alert](https://docs.github.com/rest/dependabot/alerts#get-a-dependabot-alert)
  pub fn get_alert(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    alert_number: impl Into<i64>,
  ) -> Request<(), (), get_alert::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let alert_number = alert_number.into();
    let url = format!("/repos/{owner}/{repo}/dependabot/alerts/{alert_number}");

    Request::<(), (), get_alert::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Update a Dependabot alert**
  ///
  /// The authenticated user must have access to security alerts for the repository to use this endpoint. For more information, see "[Granting access to security alerts](https://docs.github.com/repositories/managing-your-repositorys-settings-and-features/enabling-features-for-your-repository/managing-security-and-analysis-settings-for-your-repository#granting-access-to-security-alerts)."
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `security_events` scope to use this endpoint. If this endpoint is only used with public repositories, the token can use the `public_repo` scope instead.
  ///
  /// *Documentation*: [https://docs.github.com/rest/dependabot/alerts#update-a-dependabot-alert](https://docs.github.com/rest/dependabot/alerts#update-a-dependabot-alert)
  pub fn update_alert(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    alert_number: impl Into<i64>,
  ) -> Request<update_alert::Request, (), update_alert::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let alert_number = alert_number.into();
    let url = format!("/repos/{owner}/{repo}/dependabot/alerts/{alert_number}");

    Request::<update_alert::Request, (), update_alert::Response>::builder(&self.config)
      .patch(url)
      .build()
  }

  /// **List repository secrets**
  ///
  /// Lists all secrets available in a repository without revealing their encrypted
  /// values.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/dependabot/secrets#list-repository-secrets](https://docs.github.com/rest/dependabot/secrets#list-repository-secrets)
  pub fn list_repo_secrets(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), list_repo_secrets::Query, list_repo_secrets::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/dependabot/secrets");

    Request::<(), list_repo_secrets::Query, list_repo_secrets::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get a repository public key**
  ///
  /// Gets your public key, which you need to encrypt secrets. You need to
  /// encrypt a secret before you can create or update secrets. Anyone with read access
  /// to the repository can use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint if the repository is private.
  ///
  /// *Documentation*: [https://docs.github.com/rest/dependabot/secrets#get-a-repository-public-key](https://docs.github.com/rest/dependabot/secrets#get-a-repository-public-key)
  pub fn get_repo_public_key(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), (), get_repo_public_key::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/dependabot/secrets/public-key");

    Request::<(), (), get_repo_public_key::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get a repository secret**
  ///
  /// Gets a single repository secret without revealing its encrypted value.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/dependabot/secrets#get-a-repository-secret](https://docs.github.com/rest/dependabot/secrets#get-a-repository-secret)
  pub fn get_repo_secret(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    secret_name: impl Into<String>,
  ) -> Request<(), (), get_repo_secret::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let secret_name = secret_name.into();
    let url = format!("/repos/{owner}/{repo}/dependabot/secrets/{secret_name}");

    Request::<(), (), get_repo_secret::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create or update a repository secret**
  ///
  /// Creates or updates a repository secret with an encrypted value. Encrypt your secret using
  /// [LibSodium](https://libsodium.gitbook.io/doc/bindings_for_other_languages). For more information, see "[Encrypting secrets for the REST API](https://docs.github.com/rest/guides/encrypting-secrets-for-the-rest-api)."
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/dependabot/secrets#create-or-update-a-repository-secret](https://docs.github.com/rest/dependabot/secrets#create-or-update-a-repository-secret)
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
    let url = format!("/repos/{owner}/{repo}/dependabot/secrets/{secret_name}");

    Request::<create_or_update_repo_secret::Request, (), create_or_update_repo_secret::Response>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **Delete a repository secret**
  ///
  /// Deletes a secret in a repository using the secret name.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/dependabot/secrets#delete-a-repository-secret](https://docs.github.com/rest/dependabot/secrets#delete-a-repository-secret)
  pub fn delete_repo_secret(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    secret_name: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let secret_name = secret_name.into();
    let url = format!("/repos/{owner}/{repo}/dependabot/secrets/{secret_name}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }
}
