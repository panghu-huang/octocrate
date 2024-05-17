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

  pub type Response = Vec<OrganizationSecretScanningAlert>;

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QueryState {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "resolved")]
    Resolved,
  }

  impl ToString for QueryState {
    fn to_string(&self) -> String {
      match self {
        QueryState::Open => "open".to_string(),
        QueryState::Resolved => "resolved".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QuerySort {
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "updated")]
    Updated,
  }

  impl ToString for QuerySort {
    fn to_string(&self) -> String {
      match self {
        QuerySort::Created => "created".to_string(),
        QuerySort::Updated => "updated".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QueryDirection {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
  }

  impl ToString for QueryDirection {
    fn to_string(&self) -> String {
      match self {
        QueryDirection::Asc => "asc".to_string(),
        QueryDirection::Desc => "desc".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// Set to `open` or `resolved` to only list secret scanning alerts in a specific state.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub state: Option<QueryState>,
    /// A comma-separated list of secret types to return. By default all secret types are returned.
    /// See "[Secret scanning patterns](https://docs.github.com/code-security/secret-scanning/secret-scanning-patterns#supported-secrets-for-advanced-security)"
    /// for a complete list of secret types.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub secret_type: Option<String>,
    /// A comma-separated list of resolutions. Only secret scanning alerts with one of these resolutions are listed. Valid resolutions are `false_positive`, `wont_fix`, `revoked`, `pattern_edited`, `pattern_deleted` or `used_in_tests`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub resolution: Option<String>,
    /// The property to sort the results by. `created` means when the alert was created. `updated` means when the alert was updated or resolved.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub sort: Option<QuerySort>,
    /// The direction to sort the results by.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub direction: Option<QueryDirection>,
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
    /// A comma-separated list of validities that, when present, will return alerts that match the validities in this list. Valid options are `active`, `inactive`, and `unknown`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub validity: Option<String>,
  }
}

pub mod list_alerts_for_org {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<OrganizationSecretScanningAlert>;

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QueryState {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "resolved")]
    Resolved,
  }

  impl ToString for QueryState {
    fn to_string(&self) -> String {
      match self {
        QueryState::Open => "open".to_string(),
        QueryState::Resolved => "resolved".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QuerySort {
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "updated")]
    Updated,
  }

  impl ToString for QuerySort {
    fn to_string(&self) -> String {
      match self {
        QuerySort::Created => "created".to_string(),
        QuerySort::Updated => "updated".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QueryDirection {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
  }

  impl ToString for QueryDirection {
    fn to_string(&self) -> String {
      match self {
        QueryDirection::Asc => "asc".to_string(),
        QueryDirection::Desc => "desc".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// Set to `open` or `resolved` to only list secret scanning alerts in a specific state.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub state: Option<QueryState>,
    /// A comma-separated list of secret types to return. By default all secret types are returned.
    /// See "[Secret scanning patterns](https://docs.github.com/code-security/secret-scanning/secret-scanning-patterns#supported-secrets-for-advanced-security)"
    /// for a complete list of secret types.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub secret_type: Option<String>,
    /// A comma-separated list of resolutions. Only secret scanning alerts with one of these resolutions are listed. Valid resolutions are `false_positive`, `wont_fix`, `revoked`, `pattern_edited`, `pattern_deleted` or `used_in_tests`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub resolution: Option<String>,
    /// The property to sort the results by. `created` means when the alert was created. `updated` means when the alert was updated or resolved.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub sort: Option<QuerySort>,
    /// The direction to sort the results by.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub direction: Option<QueryDirection>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for events before this cursor. To receive an initial cursor on your first request, include an empty "before" query string.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub before: Option<String>,
    /// A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for events after this cursor.  To receive an initial cursor on your first request, include an empty "after" query string.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub after: Option<String>,
    /// A comma-separated list of validities that, when present, will return alerts that match the validities in this list. Valid options are `active`, `inactive`, and `unknown`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub validity: Option<String>,
  }
}

pub mod list_alerts_for_repo {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<SecretScanningAlert>;

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QueryState {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "resolved")]
    Resolved,
  }

  impl ToString for QueryState {
    fn to_string(&self) -> String {
      match self {
        QueryState::Open => "open".to_string(),
        QueryState::Resolved => "resolved".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QuerySort {
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "updated")]
    Updated,
  }

  impl ToString for QuerySort {
    fn to_string(&self) -> String {
      match self {
        QuerySort::Created => "created".to_string(),
        QuerySort::Updated => "updated".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QueryDirection {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
  }

  impl ToString for QueryDirection {
    fn to_string(&self) -> String {
      match self {
        QueryDirection::Asc => "asc".to_string(),
        QueryDirection::Desc => "desc".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// Set to `open` or `resolved` to only list secret scanning alerts in a specific state.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub state: Option<QueryState>,
    /// A comma-separated list of secret types to return. By default all secret types are returned.
    /// See "[Secret scanning patterns](https://docs.github.com/code-security/secret-scanning/secret-scanning-patterns#supported-secrets-for-advanced-security)"
    /// for a complete list of secret types.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub secret_type: Option<String>,
    /// A comma-separated list of resolutions. Only secret scanning alerts with one of these resolutions are listed. Valid resolutions are `false_positive`, `wont_fix`, `revoked`, `pattern_edited`, `pattern_deleted` or `used_in_tests`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub resolution: Option<String>,
    /// The property to sort the results by. `created` means when the alert was created. `updated` means when the alert was updated or resolved.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub sort: Option<QuerySort>,
    /// The direction to sort the results by.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub direction: Option<QueryDirection>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for events before this cursor. To receive an initial cursor on your first request, include an empty "before" query string.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub before: Option<String>,
    /// A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for events after this cursor.  To receive an initial cursor on your first request, include an empty "after" query string.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub after: Option<String>,
    /// A comma-separated list of validities that, when present, will return alerts that match the validities in this list. Valid options are `active`, `inactive`, and `unknown`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub validity: Option<String>,
  }
}

pub mod get_alert {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = SecretScanningAlert;
}

pub mod update_alert {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = SecretScanningAlert;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub resolution: Option<SecretScanningAlertResolution>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub resolution_comment: Option<String>,
    pub state: SecretScanningAlertState,
  }
}

pub mod list_locations_for_alert {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<SecretScanningLocation>;

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

pub struct GitHubSecretScanningAPI {
  config: SharedAPIConfig,
}

impl GitHubSecretScanningAPI {
  pub fn new(config: &SharedAPIConfig) -> Self {
    Self {
      config: config.clone(),
    }
  }

  /// **List secret scanning alerts for an enterprise**
  ///
  /// Lists secret scanning alerts for eligible repositories in an enterprise, from newest to oldest.
  ///
  /// Alerts are only returned for organizations in the enterprise for which the authenticated user is an organization owner or a [security manager](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/managing-security-managers-in-your-organization).
  ///
  /// The authenticated user must be a member of the enterprise in order to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope or `security_events` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/secret-scanning/secret-scanning#list-secret-scanning-alerts-for-an-enterprise](https://docs.github.com/rest/secret-scanning/secret-scanning#list-secret-scanning-alerts-for-an-enterprise)
  pub fn list_alerts_for_enterprise(
    &self,
    enterprise: impl Into<String>,
  ) -> Request<(), list_alerts_for_enterprise::Query, list_alerts_for_enterprise::Response> {
    let enterprise = enterprise.into();
    let url = format!("/enterprises/{enterprise}/secret-scanning/alerts");

    Request::<(), list_alerts_for_enterprise::Query, list_alerts_for_enterprise::Response>::builder(
      &self.config,
    )
    .get(url)
    .build()
  }

  /// **List secret scanning alerts for an organization**
  ///
  /// Lists secret scanning alerts for eligible repositories in an organization, from newest to oldest.
  ///
  /// The authenticated user must be an administrator or security manager for the organization to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` or `security_events` scope to use this endpoint. If this endpoint is only used with public repositories, the token can use the `public_repo` scope instead.
  ///
  /// *Documentation*: [https://docs.github.com/rest/secret-scanning/secret-scanning#list-secret-scanning-alerts-for-an-organization](https://docs.github.com/rest/secret-scanning/secret-scanning#list-secret-scanning-alerts-for-an-organization)
  pub fn list_alerts_for_org(
    &self,
    org: impl Into<String>,
  ) -> Request<(), list_alerts_for_org::Query, list_alerts_for_org::Response> {
    let org = org.into();
    let url = format!("/orgs/{org}/secret-scanning/alerts");

    Request::<(), list_alerts_for_org::Query, list_alerts_for_org::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List secret scanning alerts for a repository**
  ///
  /// Lists secret scanning alerts for an eligible repository, from newest to oldest.
  ///
  /// The authenticated user must be an administrator for the repository or for the organization that owns the repository to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` or `security_events` scope to use this endpoint. If this endpoint is only used with public repositories, the token can use the `public_repo` scope instead.
  ///
  /// *Documentation*: [https://docs.github.com/rest/secret-scanning/secret-scanning#list-secret-scanning-alerts-for-a-repository](https://docs.github.com/rest/secret-scanning/secret-scanning#list-secret-scanning-alerts-for-a-repository)
  pub fn list_alerts_for_repo(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), list_alerts_for_repo::Query, list_alerts_for_repo::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/secret-scanning/alerts");

    Request::<(), list_alerts_for_repo::Query, list_alerts_for_repo::Response>::builder(
      &self.config,
    )
    .get(url)
    .build()
  }

  /// **Get a secret scanning alert**
  ///
  /// Gets a single secret scanning alert detected in an eligible repository.
  ///
  /// The authenticated user must be an administrator for the repository or for the organization that owns the repository to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` or `security_events` scope to use this endpoint. If this endpoint is only used with public repositories, the token can use the `public_repo` scope instead.
  ///
  /// *Documentation*: [https://docs.github.com/rest/secret-scanning/secret-scanning#get-a-secret-scanning-alert](https://docs.github.com/rest/secret-scanning/secret-scanning#get-a-secret-scanning-alert)
  pub fn get_alert(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    alert_number: impl Into<i64>,
  ) -> Request<(), (), get_alert::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let alert_number = alert_number.into();
    let url = format!("/repos/{owner}/{repo}/secret-scanning/alerts/{alert_number}");

    Request::<(), (), get_alert::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Update a secret scanning alert**
  ///
  /// Updates the status of a secret scanning alert in an eligible repository.
  ///
  /// The authenticated user must be an administrator for the repository or for the organization that owns the repository to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` or `security_events` scope to use this endpoint. If this endpoint is only used with public repositories, the token can use the `public_repo` scope instead.
  ///
  /// *Documentation*: [https://docs.github.com/rest/secret-scanning/secret-scanning#update-a-secret-scanning-alert](https://docs.github.com/rest/secret-scanning/secret-scanning#update-a-secret-scanning-alert)
  pub fn update_alert(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    alert_number: impl Into<i64>,
  ) -> Request<update_alert::Request, (), update_alert::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let alert_number = alert_number.into();
    let url = format!("/repos/{owner}/{repo}/secret-scanning/alerts/{alert_number}");

    Request::<update_alert::Request, (), update_alert::Response>::builder(&self.config)
      .patch(url)
      .build()
  }

  /// **List locations for a secret scanning alert**
  ///
  /// Lists all locations for a given secret scanning alert for an eligible repository.
  ///
  /// The authenticated user must be an administrator for the repository or for the organization that owns the repository to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` or `security_events` scope to use this endpoint. If this endpoint is only used with public repositories, the token can use the `public_repo` scope instead.
  ///
  /// *Documentation*: [https://docs.github.com/rest/secret-scanning/secret-scanning#list-locations-for-a-secret-scanning-alert](https://docs.github.com/rest/secret-scanning/secret-scanning#list-locations-for-a-secret-scanning-alert)
  pub fn list_locations_for_alert(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    alert_number: impl Into<i64>,
  ) -> Request<(), list_locations_for_alert::Query, list_locations_for_alert::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let alert_number = alert_number.into();
    let url = format!("/repos/{owner}/{repo}/secret-scanning/alerts/{alert_number}/locations");

    Request::<(), list_locations_for_alert::Query, list_locations_for_alert::Response>::builder(
      &self.config,
    )
    .get(url)
    .build()
  }
}
