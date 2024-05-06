use octocrate_core::*;
#[allow(unused_imports)]
use octocrate_types::*;

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
  ) -> Request<(), DependabotListAlertsForEnterpriseQuery, DependabotAlertWithRepositoryArray> {
    let enterprise = enterprise.into();
    let url = format!("/enterprises/{enterprise}/dependabot/alerts");

    Request::<(), DependabotListAlertsForEnterpriseQuery, DependabotAlertWithRepositoryArray>::builder(&self.config)
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
  ) -> Request<(), DependabotListAlertsForOrgQuery, DependabotAlertWithRepositoryArray> {
    let org = org.into();
    let url = format!("/orgs/{org}/dependabot/alerts");

    Request::<(), DependabotListAlertsForOrgQuery, DependabotAlertWithRepositoryArray>::builder(
      &self.config,
    )
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
  ) -> Request<(), DependabotListOrgSecretsQuery, DependabotListOrgSecretsResponse> {
    let org = org.into();
    let url = format!("/orgs/{org}/dependabot/secrets");

    Request::<(), DependabotListOrgSecretsQuery, DependabotListOrgSecretsResponse>::builder(
      &self.config,
    )
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
  pub fn get_org_public_key(&self, org: impl Into<String>) -> Request<(), (), DependabotPublicKey> {
    let org = org.into();
    let url = format!("/orgs/{org}/dependabot/secrets/public-key");

    Request::<(), (), DependabotPublicKey>::builder(&self.config)
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
  ) -> Request<(), (), OrganizationDependabotSecret> {
    let org = org.into();
    let secret_name = secret_name.into();
    let url = format!("/orgs/{org}/dependabot/secrets/{secret_name}");

    Request::<(), (), OrganizationDependabotSecret>::builder(&self.config)
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
  ) -> Request<DependabotCreateOrUpdateOrgSecretRequest, (), EmptyObject> {
    let org = org.into();
    let secret_name = secret_name.into();
    let url = format!("/orgs/{org}/dependabot/secrets/{secret_name}");

    Request::<DependabotCreateOrUpdateOrgSecretRequest, (), EmptyObject>::builder(&self.config)
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
    DependabotListSelectedReposForOrgSecretQuery,
    DependabotListSelectedReposForOrgSecretResponse,
  > {
    let org = org.into();
    let secret_name = secret_name.into();
    let url = format!("/orgs/{org}/dependabot/secrets/{secret_name}/repositories");

    Request::<
      (),
      DependabotListSelectedReposForOrgSecretQuery,
      DependabotListSelectedReposForOrgSecretResponse,
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
  ) -> NoContentRequest<DependabotSetSelectedReposForOrgSecretRequest, ()> {
    let org = org.into();
    let secret_name = secret_name.into();
    let url = format!("/orgs/{org}/dependabot/secrets/{secret_name}/repositories");

    NoContentRequest::<DependabotSetSelectedReposForOrgSecretRequest, ()>::builder(&self.config)
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
  ) -> Request<(), DependabotListAlertsForRepoQuery, DependabotAlertArray> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/dependabot/alerts");

    Request::<(), DependabotListAlertsForRepoQuery, DependabotAlertArray>::builder(&self.config)
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
  ) -> Request<(), (), DependabotAlert> {
    let owner = owner.into();
    let repo = repo.into();
    let alert_number = alert_number.into();
    let url = format!("/repos/{owner}/{repo}/dependabot/alerts/{alert_number}");

    Request::<(), (), DependabotAlert>::builder(&self.config)
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
  ) -> Request<DependabotUpdateAlertRequest, (), DependabotAlert> {
    let owner = owner.into();
    let repo = repo.into();
    let alert_number = alert_number.into();
    let url = format!("/repos/{owner}/{repo}/dependabot/alerts/{alert_number}");

    Request::<DependabotUpdateAlertRequest, (), DependabotAlert>::builder(&self.config)
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
  ) -> Request<(), DependabotListRepoSecretsQuery, DependabotListRepoSecretsResponse> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/dependabot/secrets");

    Request::<(), DependabotListRepoSecretsQuery, DependabotListRepoSecretsResponse>::builder(
      &self.config,
    )
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
  ) -> Request<(), (), DependabotPublicKey> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/dependabot/secrets/public-key");

    Request::<(), (), DependabotPublicKey>::builder(&self.config)
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
  ) -> Request<(), (), DependabotSecret> {
    let owner = owner.into();
    let repo = repo.into();
    let secret_name = secret_name.into();
    let url = format!("/repos/{owner}/{repo}/dependabot/secrets/{secret_name}");

    Request::<(), (), DependabotSecret>::builder(&self.config)
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
  ) -> Request<DependabotCreateOrUpdateRepoSecretRequest, (), EmptyObject> {
    let owner = owner.into();
    let repo = repo.into();
    let secret_name = secret_name.into();
    let url = format!("/repos/{owner}/{repo}/dependabot/secrets/{secret_name}");

    Request::<DependabotCreateOrUpdateRepoSecretRequest, (), EmptyObject>::builder(&self.config)
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
