use octocrate_core::*;
#[allow(unused_imports)]
use octocrate_types::*;

pub struct GitHubSecretScanningAPI {
  config: SharedAPIConfig,
}

impl GitHubSecretScanningAPI {
  pub fn new(config: &SharedAPIConfig) -> Self {
    Self {
      config: config.clone(),
    }
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
  ) -> Request<(), SecretScanningListAlertsForOrgQuery, OrganizationSecretScanningAlertArray> {
    let org = org.into();
    let url = format!("/orgs/{org}/secret-scanning/alerts");

    Request::<(), SecretScanningListAlertsForOrgQuery, OrganizationSecretScanningAlertArray>::builder(&self.config)
      .get(url)
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
    alert_number: impl Into<serde_json::Value>,
  ) -> Request<(), SecretScanningListLocationsForAlertQuery, SecretScanningLocationArray> {
    let owner = owner.into();
    let repo = repo.into();
    let alert_number = alert_number.into();
    let url = format!("/repos/{owner}/{repo}/secret-scanning/alerts/{alert_number}/locations");

    Request::<(), SecretScanningListLocationsForAlertQuery, SecretScanningLocationArray>::builder(
      &self.config,
    )
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
  ) -> Request<(), SecretScanningListAlertsForRepoQuery, SecretScanningAlertArray> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/secret-scanning/alerts");

    Request::<(), SecretScanningListAlertsForRepoQuery, SecretScanningAlertArray>::builder(
      &self.config,
    )
    .get(url)
    .build()
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
  ) -> Request<(), SecretScanningListAlertsForEnterpriseQuery, OrganizationSecretScanningAlertArray>
  {
    let enterprise = enterprise.into();
    let url = format!("/enterprises/{enterprise}/secret-scanning/alerts");

    Request::<(), SecretScanningListAlertsForEnterpriseQuery, OrganizationSecretScanningAlertArray>::builder(&self.config)
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
    alert_number: impl Into<serde_json::Value>,
  ) -> Request<(), (), SecretScanningAlert> {
    let owner = owner.into();
    let repo = repo.into();
    let alert_number = alert_number.into();
    let url = format!("/repos/{owner}/{repo}/secret-scanning/alerts/{alert_number}");

    Request::<(), (), SecretScanningAlert>::builder(&self.config)
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
    alert_number: impl Into<serde_json::Value>,
  ) -> Request<SecretScanningUpdateAlertRequest, (), SecretScanningAlert> {
    let owner = owner.into();
    let repo = repo.into();
    let alert_number = alert_number.into();
    let url = format!("/repos/{owner}/{repo}/secret-scanning/alerts/{alert_number}");

    Request::<SecretScanningUpdateAlertRequest, (), SecretScanningAlert>::builder(&self.config)
      .patch(url)
      .build()
  }
}
