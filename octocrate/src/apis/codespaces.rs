#[allow(unused_imports)]
use crate::types::*;
use octocrate_core::*;

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
  ) -> Request<(), (), Codespace> {
    let codespace_name = codespace_name.into();
    let url = format!("/user/codespaces/{codespace_name}/stop");

    Request::<(), (), Codespace>::builder(&self.config)
      .post(url)
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

  /// **Get public key for the authenticated user**
  ///
  /// Gets your public key, which you need to encrypt secrets. You need to encrypt a secret before you can create or update secrets.
  ///
  /// The authenticated user must have Codespaces access to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `codespace` or `codespace:secrets` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/codespaces/secrets#get-public-key-for-the-authenticated-user](https://docs.github.com/rest/codespaces/secrets#get-public-key-for-the-authenticated-user)
  pub fn get_public_key_for_authenticated_user(&self) -> Request<(), (), CodespacesUserPublicKey> {
    let url = format!("/user/codespaces/secrets/public-key");

    Request::<(), (), CodespacesUserPublicKey>::builder(&self.config)
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
    CodespacesRepoMachinesForAuthenticatedUserQuery,
    CodespacesRepoMachinesForAuthenticatedUserResponse,
  > {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/codespaces/machines");

    Request::<
      (),
      CodespacesRepoMachinesForAuthenticatedUserQuery,
      CodespacesRepoMachinesForAuthenticatedUserResponse,
    >::builder(&self.config)
    .get(url)
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
  ) -> Request<(), (), Codespace> {
    let org = org.into();
    let username = username.into();
    let codespace_name = codespace_name.into();
    let url = format!("/orgs/{org}/members/{username}/codespaces/{codespace_name}/stop");

    Request::<(), (), Codespace>::builder(&self.config)
      .post(url)
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
    CodespacesListSelectedReposForOrgSecretQuery,
    CodespacesListSelectedReposForOrgSecretResponse,
  > {
    let org = org.into();
    let secret_name = secret_name.into();
    let url = format!("/orgs/{org}/codespaces/secrets/{secret_name}/repositories");

    Request::<
      (),
      CodespacesListSelectedReposForOrgSecretQuery,
      CodespacesListSelectedReposForOrgSecretResponse,
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
  ) -> NoContentRequest<CodespacesSetSelectedReposForOrgSecretRequest, ()> {
    let org = org.into();
    let secret_name = secret_name.into();
    let url = format!("/orgs/{org}/codespaces/secrets/{secret_name}/repositories");

    NoContentRequest::<CodespacesSetSelectedReposForOrgSecretRequest, ()>::builder(&self.config)
      .put(url)
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
  ) -> Request<(), (), CodespacesSecret> {
    let secret_name = secret_name.into();
    let url = format!("/user/codespaces/secrets/{secret_name}");

    Request::<(), (), CodespacesSecret>::builder(&self.config)
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
  ) -> Request<CodespacesCreateOrUpdateSecretForAuthenticatedUserRequest, (), EmptyObject> {
    let secret_name = secret_name.into();
    let url = format!("/user/codespaces/secrets/{secret_name}");

    Request::<CodespacesCreateOrUpdateSecretForAuthenticatedUserRequest, (), EmptyObject>::builder(
      &self.config,
    )
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
  ) -> Request<(), (), Codespace> {
    let codespace_name = codespace_name.into();
    let url = format!("/user/codespaces/{codespace_name}/start");

    Request::<(), (), Codespace>::builder(&self.config)
      .post(url)
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
  ) -> Request<(), (), CodespacesSecret> {
    let org = org.into();
    let secret_name = secret_name.into();
    let url = format!("/orgs/{org}/codespaces/secrets/{secret_name}");

    Request::<(), (), CodespacesSecret>::builder(&self.config)
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
  ) -> Request<CodespacesCreateOrUpdateOrgSecretRequest, (), EmptyObject> {
    let org = org.into();
    let secret_name = secret_name.into();
    let url = format!("/orgs/{org}/codespaces/secrets/{secret_name}");

    Request::<CodespacesCreateOrUpdateOrgSecretRequest, (), EmptyObject>::builder(&self.config)
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
  ) -> Request<(), CodespacesListInOrganizationQuery, CodespacesListInOrganizationResponse> {
    let org = org.into();
    let url = format!("/orgs/{org}/codespaces");

    Request::<(), CodespacesListInOrganizationQuery, CodespacesListInOrganizationResponse>::builder(
      &self.config,
    )
    .get(url)
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
    CodespacesListSecretsForAuthenticatedUserQuery,
    CodespacesListSecretsForAuthenticatedUserResponse,
  > {
    let url = format!("/user/codespaces/secrets");

    Request::<
      (),
      CodespacesListSecretsForAuthenticatedUserQuery,
      CodespacesListSecretsForAuthenticatedUserResponse,
    >::builder(&self.config)
    .get(url)
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
  ) -> Request<(), (), FetchesInformationAboutAnExportOfACodespace> {
    let codespace_name = codespace_name.into();
    let url = format!("/user/codespaces/{codespace_name}/exports");

    Request::<(), (), FetchesInformationAboutAnExportOfACodespace>::builder(&self.config)
      .post(url)
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
  ) -> Request<(), (), Codespace> {
    let codespace_name = codespace_name.into();
    let url = format!("/user/codespaces/{codespace_name}");

    Request::<(), (), Codespace>::builder(&self.config)
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
  ) -> Request<CodespacesUpdateForAuthenticatedUserRequest, (), Codespace> {
    let codespace_name = codespace_name.into();
    let url = format!("/user/codespaces/{codespace_name}");

    Request::<CodespacesUpdateForAuthenticatedUserRequest, (), Codespace>::builder(&self.config)
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
  ) -> Request<CodespacesCreateWithPrForAuthenticatedUserRequest, (), Codespace> {
    let owner = owner.into();
    let repo = repo.into();
    let pull_number = pull_number.into();
    let url = format!("/repos/{owner}/{repo}/pulls/{pull_number}/codespaces");

    Request::<CodespacesCreateWithPrForAuthenticatedUserRequest, (), Codespace>::builder(
      &self.config,
    )
    .post(url)
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
  ) -> Request<(), CodespacesListOrgSecretsQuery, CodespacesListOrgSecretsResponse> {
    let org = org.into();
    let url = format!("/orgs/{org}/codespaces/secrets");

    Request::<(), CodespacesListOrgSecretsQuery, CodespacesListOrgSecretsResponse>::builder(
      &self.config,
    )
    .get(url)
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
    CodespacesListInRepositoryForAuthenticatedUserQuery,
    CodespacesListInRepositoryForAuthenticatedUserResponse,
  > {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/codespaces");

    Request::<
      (),
      CodespacesListInRepositoryForAuthenticatedUserQuery,
      CodespacesListInRepositoryForAuthenticatedUserResponse,
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
  ) -> Request<CodespacesCreateWithRepoForAuthenticatedUserRequest, (), Codespace> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/codespaces");

    Request::<CodespacesCreateWithRepoForAuthenticatedUserRequest, (), Codespace>::builder(
      &self.config,
    )
    .post(url)
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
  ) -> Request<(), (), CodespacesListRepositoriesForSecretForAuthenticatedUserResponse> {
    let secret_name = secret_name.into();
    let url = format!("/user/codespaces/secrets/{secret_name}/repositories");

    Request::<(), (), CodespacesListRepositoriesForSecretForAuthenticatedUserResponse>::builder(
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
  ) -> NoContentRequest<CodespacesSetRepositoriesForSecretForAuthenticatedUserRequest, ()> {
    let secret_name = secret_name.into();
    let url = format!("/user/codespaces/secrets/{secret_name}/repositories");

    NoContentRequest::<CodespacesSetRepositoriesForSecretForAuthenticatedUserRequest, ()>::builder(
      &self.config,
    )
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
  ) -> NoContentRequest<CodespacesSetCodespacesAccessUsersRequest, ()> {
    let org = org.into();
    let url = format!("/orgs/{org}/codespaces/access/selected_users");

    NoContentRequest::<CodespacesSetCodespacesAccessUsersRequest, ()>::builder(&self.config)
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
  ) -> NoContentRequest<CodespacesDeleteCodespacesAccessUsersRequest, ()> {
    let org = org.into();
    let url = format!("/orgs/{org}/codespaces/access/selected_users");

    NoContentRequest::<CodespacesDeleteCodespacesAccessUsersRequest, ()>::builder(&self.config)
      .delete(url)
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
  ) -> Request<(), (), CodespacesPublicKey> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/codespaces/secrets/public-key");

    Request::<(), (), CodespacesPublicKey>::builder(&self.config)
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
  ) -> Request<(), CodespacesListRepoSecretsQuery, CodespacesListRepoSecretsResponse> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/codespaces/secrets");

    Request::<(), CodespacesListRepoSecretsQuery, CodespacesListRepoSecretsResponse>::builder(
      &self.config,
    )
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
    CodespacesPreFlightWithRepoForAuthenticatedUserQuery,
    CodespacesPreFlightWithRepoForAuthenticatedUserResponse,
  > {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/codespaces/new");

    Request::<
      (),
      CodespacesPreFlightWithRepoForAuthenticatedUserQuery,
      CodespacesPreFlightWithRepoForAuthenticatedUserResponse,
    >::builder(&self.config)
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
  ) -> Request<(), (), CodespacesSecret> {
    let owner = owner.into();
    let repo = repo.into();
    let secret_name = secret_name.into();
    let url = format!("/repos/{owner}/{repo}/codespaces/secrets/{secret_name}");

    Request::<(), (), CodespacesSecret>::builder(&self.config)
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
  ) -> Request<CodespacesCreateOrUpdateRepoSecretRequest, (), EmptyObject> {
    let owner = owner.into();
    let repo = repo.into();
    let secret_name = secret_name.into();
    let url = format!("/repos/{owner}/{repo}/codespaces/secrets/{secret_name}");

    Request::<CodespacesCreateOrUpdateRepoSecretRequest, (), EmptyObject>::builder(&self.config)
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

  /// **Get an organization public key**
  ///
  /// Gets a public key for an organization, which is required in order to encrypt secrets. You need to encrypt the value of a secret before you can create or update secrets.
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/codespaces/organization-secrets#get-an-organization-public-key](https://docs.github.com/rest/codespaces/organization-secrets#get-an-organization-public-key)
  pub fn get_org_public_key(&self, org: impl Into<String>) -> Request<(), (), CodespacesPublicKey> {
    let org = org.into();
    let url = format!("/orgs/{org}/codespaces/secrets/public-key");

    Request::<(), (), CodespacesPublicKey>::builder(&self.config)
      .get(url)
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
  ) -> Request<
    (),
    CodespacesListForAuthenticatedUserQuery,
    CodespacesListForAuthenticatedUserResponse,
  > {
    let url = format!("/user/codespaces");

    Request::<(), CodespacesListForAuthenticatedUserQuery, CodespacesListForAuthenticatedUserResponse>::builder(&self.config)
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
  ) -> Request<CodespacesCreateForAuthenticatedUserRequest, (), Codespace> {
    let url = format!("/user/codespaces");

    Request::<CodespacesCreateForAuthenticatedUserRequest, (), Codespace>::builder(&self.config)
      .post(url)
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
  ) -> Request<(), CodespacesCheckPermissionsForDevcontainerQuery, CodespacesPermissionsCheck> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/codespaces/permissions_check");

    Request::<(), CodespacesCheckPermissionsForDevcontainerQuery, CodespacesPermissionsCheck>::builder(&self.config)
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
  ) -> Request<CodespacesPublishForAuthenticatedUserRequest, (), Codespace> {
    let codespace_name = codespace_name.into();
    let url = format!("/user/codespaces/{codespace_name}/publish");

    Request::<CodespacesPublishForAuthenticatedUserRequest, (), Codespace>::builder(&self.config)
      .post(url)
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
  ) -> Request<(), (), CodespacesCodespaceMachinesForAuthenticatedUserResponse> {
    let codespace_name = codespace_name.into();
    let url = format!("/user/codespaces/{codespace_name}/machines");

    Request::<(), (), CodespacesCodespaceMachinesForAuthenticatedUserResponse>::builder(
      &self.config,
    )
    .get(url)
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
  ) -> Request<(), (), FetchesInformationAboutAnExportOfACodespace> {
    let codespace_name = codespace_name.into();
    let export_id = export_id.into();
    let url = format!("/user/codespaces/{codespace_name}/exports/{export_id}");

    Request::<(), (), FetchesInformationAboutAnExportOfACodespace>::builder(&self.config)
      .get(url)
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
    CodespacesListDevcontainersInRepositoryForAuthenticatedUserQuery,
    CodespacesListDevcontainersInRepositoryForAuthenticatedUserResponse,
  > {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/codespaces/devcontainers");

    Request::<
      (),
      CodespacesListDevcontainersInRepositoryForAuthenticatedUserQuery,
      CodespacesListDevcontainersInRepositoryForAuthenticatedUserResponse,
    >::builder(&self.config)
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
  ) -> NoContentRequest<CodespacesSetCodespacesAccessRequest, ()> {
    let org = org.into();
    let url = format!("/orgs/{org}/codespaces/access");

    NoContentRequest::<CodespacesSetCodespacesAccessRequest, ()>::builder(&self.config)
      .put(url)
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
  ) -> Request<
    (),
    CodespacesGetCodespacesForUserInOrgQuery,
    CodespacesGetCodespacesForUserInOrgResponse,
  > {
    let org = org.into();
    let username = username.into();
    let url = format!("/orgs/{org}/members/{username}/codespaces");

    Request::<
      (),
      CodespacesGetCodespacesForUserInOrgQuery,
      CodespacesGetCodespacesForUserInOrgResponse,
    >::builder(&self.config)
    .get(url)
    .build()
  }
}
