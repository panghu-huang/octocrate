#[allow(unused_imports)]
use crate::types::*;
use octocrate_core::*;

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
  ) -> Request<(), (), AuthenticationToken> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/actions/runners/registration-token");

    Request::<(), (), AuthenticationToken>::builder(&self.config)
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
  ) -> Request<(), (), AuthenticationToken> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/actions/runners/remove-token");

    Request::<(), (), AuthenticationToken>::builder(&self.config)
      .post(url)
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
  ) -> Request<(), ActionsListRepoWorkflowsQuery, ActionsListRepoWorkflowsResponse> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/actions/workflows");

    Request::<(), ActionsListRepoWorkflowsQuery, ActionsListRepoWorkflowsResponse>::builder(
      &self.config,
    )
    .get(url)
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
  ) -> Request<(), (), Job> {
    let owner = owner.into();
    let repo = repo.into();
    let job_id = job_id.into();
    let url = format!("/repos/{owner}/{repo}/actions/jobs/{job_id}");

    Request::<(), (), Job>::builder(&self.config)
      .get(url)
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
    ActionsListSelfHostedRunnersForOrgQuery,
    ActionsListSelfHostedRunnersForOrgResponse,
  > {
    let org = org.into();
    let url = format!("/orgs/{org}/actions/runners");

    Request::<(), ActionsListSelfHostedRunnersForOrgQuery, ActionsListSelfHostedRunnersForOrgResponse>::builder(&self.config)
      .get(url)
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
  ) -> Request<(), (), EnvironmentApprovalArray> {
    let owner = owner.into();
    let repo = repo.into();
    let run_id = run_id.into();
    let url = format!("/repos/{owner}/{repo}/actions/runs/{run_id}/approvals");

    Request::<(), (), EnvironmentApprovalArray>::builder(&self.config)
      .get(url)
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
  ) -> Request<(), (), ActionsVariable> {
    let owner = owner.into();
    let repo = repo.into();
    let environment_name = environment_name.into();
    let name = name.into();
    let url = format!("/repos/{owner}/{repo}/environments/{environment_name}/variables/{name}");

    Request::<(), (), ActionsVariable>::builder(&self.config)
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
  ) -> NoContentRequest<ActionsUpdateEnvironmentVariableRequest, ()> {
    let owner = owner.into();
    let repo = repo.into();
    let name = name.into();
    let environment_name = environment_name.into();
    let url = format!("/repos/{owner}/{repo}/environments/{environment_name}/variables/{name}");

    NoContentRequest::<ActionsUpdateEnvironmentVariableRequest, ()>::builder(&self.config)
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
  ) -> Request<(), ActionsListRepoVariablesQuery, ActionsListRepoVariablesResponse> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/actions/variables");

    Request::<(), ActionsListRepoVariablesQuery, ActionsListRepoVariablesResponse>::builder(
      &self.config,
    )
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
  ) -> Request<ActionsCreateRepoVariableRequest, (), EmptyObject> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/actions/variables");

    Request::<ActionsCreateRepoVariableRequest, (), EmptyObject>::builder(&self.config)
      .post(url)
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
  ) -> Request<(), (), ActionsWorkflowAccessToRepository> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/actions/permissions/access");

    Request::<(), (), ActionsWorkflowAccessToRepository>::builder(&self.config)
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
  ) -> NoContentRequest<ActionsWorkflowAccessToRepository, ()> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/actions/permissions/access");

    NoContentRequest::<ActionsWorkflowAccessToRepository, ()>::builder(&self.config)
      .put(url)
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
  ) -> NoContentRequest<ActionsAddCustomLabelsToSelfHostedRunnerForOrgRequest, ()> {
    let org = org.into();
    let runner_id = runner_id.into();
    let url = format!("/orgs/{org}/actions/runners/{runner_id}/labels");

    NoContentRequest::<ActionsAddCustomLabelsToSelfHostedRunnerForOrgRequest, ()>::builder(
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
  ) -> NoContentRequest<ActionsSetCustomLabelsForSelfHostedRunnerForOrgRequest, ()> {
    let org = org.into();
    let runner_id = runner_id.into();
    let url = format!("/orgs/{org}/actions/runners/{runner_id}/labels");

    NoContentRequest::<ActionsSetCustomLabelsForSelfHostedRunnerForOrgRequest, ()>::builder(
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
  ) -> Request<(), (), ActionsPublicKey> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/actions/secrets/public-key");

    Request::<(), (), ActionsPublicKey>::builder(&self.config)
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
  ) -> Request<(), (), ActionsCacheUsageByRepository> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/actions/cache/usage");

    Request::<(), (), ActionsCacheUsageByRepository>::builder(&self.config)
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
  ) -> Request<(), (), Artifact> {
    let owner = owner.into();
    let repo = repo.into();
    let artifact_id = artifact_id.into();
    let url = format!("/repos/{owner}/{repo}/actions/artifacts/{artifact_id}");

    Request::<(), (), Artifact>::builder(&self.config)
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
  ) -> Request<(), (), ActionsOrganizationPermissions> {
    let org = org.into();
    let url = format!("/orgs/{org}/actions/permissions");

    Request::<(), (), ActionsOrganizationPermissions>::builder(&self.config)
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
  ) -> NoContentRequest<ActionsSetGithubActionsPermissionsOrganizationRequest, ()> {
    let org = org.into();
    let url = format!("/orgs/{org}/actions/permissions");

    NoContentRequest::<ActionsSetGithubActionsPermissionsOrganizationRequest, ()>::builder(
      &self.config,
    )
    .put(url)
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
  ) -> Request<(), (), ActionsSecretForAnOrganization> {
    let org = org.into();
    let secret_name = secret_name.into();
    let url = format!("/orgs/{org}/actions/secrets/{secret_name}");

    Request::<(), (), ActionsSecretForAnOrganization>::builder(&self.config)
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
  ) -> Request<ActionsCreateOrUpdateOrgSecretRequest, (), EmptyObject> {
    let org = org.into();
    let secret_name = secret_name.into();
    let url = format!("/orgs/{org}/actions/secrets/{secret_name}");

    Request::<ActionsCreateOrUpdateOrgSecretRequest, (), EmptyObject>::builder(&self.config)
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
  ) -> Request<(), (), ActionsRepositoryPermissions> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/actions/permissions");

    Request::<(), (), ActionsRepositoryPermissions>::builder(&self.config)
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
  ) -> NoContentRequest<ActionsSetGithubActionsPermissionsRepositoryRequest, ()> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/actions/permissions");

    NoContentRequest::<ActionsSetGithubActionsPermissionsRepositoryRequest, ()>::builder(
      &self.config,
    )
    .put(url)
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
  ) -> Request<ActionsReRunJobForWorkflowRunRequest, (), EmptyObject> {
    let owner = owner.into();
    let repo = repo.into();
    let job_id = job_id.into();
    let url = format!("/repos/{owner}/{repo}/actions/jobs/{job_id}/rerun");

    Request::<ActionsReRunJobForWorkflowRunRequest, (), EmptyObject>::builder(&self.config)
      .post(url)
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
  ) -> Request<(), (), EmptyObject> {
    let owner = owner.into();
    let repo = repo.into();
    let run_id = run_id.into();
    let url = format!("/repos/{owner}/{repo}/actions/runs/{run_id}/approve");

    Request::<(), (), EmptyObject>::builder(&self.config)
      .post(url)
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
  ) -> Request<(), ActionsListOrgSecretsQuery, ActionsListOrgSecretsResponse> {
    let org = org.into();
    let url = format!("/orgs/{org}/actions/secrets");

    Request::<(), ActionsListOrgSecretsQuery, ActionsListOrgSecretsResponse>::builder(&self.config)
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
  ) -> Request<(), (), ActionsPublicKey> {
    let owner = owner.into();
    let repo = repo.into();
    let environment_name = environment_name.into();
    let url = format!("/repos/{owner}/{repo}/environments/{environment_name}/secrets/public-key");

    Request::<(), (), ActionsPublicKey>::builder(&self.config)
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
  ) -> Request<(), (), ActionsSecret> {
    let owner = owner.into();
    let repo = repo.into();
    let environment_name = environment_name.into();
    let secret_name = secret_name.into();
    let url =
      format!("/repos/{owner}/{repo}/environments/{environment_name}/secrets/{secret_name}");

    Request::<(), (), ActionsSecret>::builder(&self.config)
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
  ) -> Request<ActionsCreateOrUpdateEnvironmentSecretRequest, (), EmptyObject> {
    let owner = owner.into();
    let repo = repo.into();
    let environment_name = environment_name.into();
    let secret_name = secret_name.into();
    let url =
      format!("/repos/{owner}/{repo}/environments/{environment_name}/secrets/{secret_name}");

    Request::<ActionsCreateOrUpdateEnvironmentSecretRequest, (), EmptyObject>::builder(&self.config)
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
  ) -> Request<(), (), SelectedActions> {
    let org = org.into();
    let url = format!("/orgs/{org}/actions/permissions/selected-actions");

    Request::<(), (), SelectedActions>::builder(&self.config)
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
  ) -> NoContentRequest<SelectedActions, ()> {
    let org = org.into();
    let url = format!("/orgs/{org}/actions/permissions/selected-actions");

    NoContentRequest::<SelectedActions, ()>::builder(&self.config)
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
  ) -> Request<(), (), ActionsGetDefaultWorkflowPermissions> {
    let org = org.into();
    let url = format!("/orgs/{org}/actions/permissions/workflow");

    Request::<(), (), ActionsGetDefaultWorkflowPermissions>::builder(&self.config)
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
  ) -> NoContentRequest<ActionsSetDefaultWorkflowPermissions, ()> {
    let org = org.into();
    let url = format!("/orgs/{org}/actions/permissions/workflow");

    NoContentRequest::<ActionsSetDefaultWorkflowPermissions, ()>::builder(&self.config)
      .put(url)
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
  ) -> Request<(), ActionsListWorkflowRunArtifactsQuery, ActionsListWorkflowRunArtifactsResponse>
  {
    let owner = owner.into();
    let repo = repo.into();
    let run_id = run_id.into();
    let url = format!("/repos/{owner}/{repo}/actions/runs/{run_id}/artifacts");

    Request::<(), ActionsListWorkflowRunArtifactsQuery, ActionsListWorkflowRunArtifactsResponse>::builder(&self.config)
      .get(url)
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
  ) -> Request<(), ActionsListJobsForWorkflowRunQuery, ActionsListJobsForWorkflowRunResponse> {
    let owner = owner.into();
    let repo = repo.into();
    let run_id = run_id.into();
    let url = format!("/repos/{owner}/{repo}/actions/runs/{run_id}/jobs");

    Request::<(), ActionsListJobsForWorkflowRunQuery, ActionsListJobsForWorkflowRunResponse>::builder(&self.config)
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
  ) -> Request<(), ActionsListRepoSecretsQuery, ActionsListRepoSecretsResponse> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/actions/secrets");

    Request::<(), ActionsListRepoSecretsQuery, ActionsListRepoSecretsResponse>::builder(
      &self.config,
    )
    .get(url)
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
  ) -> Request<(), (), EmptyObject> {
    let owner = owner.into();
    let repo = repo.into();
    let run_id = run_id.into();
    let url = format!("/repos/{owner}/{repo}/actions/runs/{run_id}/force-cancel");

    Request::<(), (), EmptyObject>::builder(&self.config)
      .post(url)
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
  pub fn get_org_public_key(&self, org: impl Into<String>) -> Request<(), (), ActionsPublicKey> {
    let org = org.into();
    let url = format!("/orgs/{org}/actions/secrets/public-key");

    Request::<(), (), ActionsPublicKey>::builder(&self.config)
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
  ) -> Request<(), ActionsGetWorkflowRunQuery, WorkflowRun> {
    let owner = owner.into();
    let repo = repo.into();
    let run_id = run_id.into();
    let url = format!("/repos/{owner}/{repo}/actions/runs/{run_id}");

    Request::<(), ActionsGetWorkflowRunQuery, WorkflowRun>::builder(&self.config)
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
  ) -> Request<(), ActionsListArtifactsForRepoQuery, ActionsListArtifactsForRepoResponse> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/actions/artifacts");

    Request::<(), ActionsListArtifactsForRepoQuery, ActionsListArtifactsForRepoResponse>::builder(
      &self.config,
    )
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
  ) -> Request<(), ActionsGetActionsCacheListQuery, RepositoryActionsCaches> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/actions/caches");

    Request::<(), ActionsGetActionsCacheListQuery, RepositoryActionsCaches>::builder(&self.config)
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
  ) -> Request<(), ActionsDeleteActionsCacheByKeyQuery, RepositoryActionsCaches> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/actions/caches");

    Request::<(), ActionsDeleteActionsCacheByKeyQuery, RepositoryActionsCaches>::builder(
      &self.config,
    )
    .delete(url)
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
  ) -> Request<(), (), SelfHostedRunners> {
    let owner = owner.into();
    let repo = repo.into();
    let runner_id = runner_id.into();
    let url = format!("/repos/{owner}/{repo}/actions/runners/{runner_id}");

    Request::<(), (), SelfHostedRunners>::builder(&self.config)
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
  ) -> Request<ActionsReRunWorkflowRequest, (), EmptyObject> {
    let owner = owner.into();
    let repo = repo.into();
    let run_id = run_id.into();
    let url = format!("/repos/{owner}/{repo}/actions/runs/{run_id}/rerun");

    Request::<ActionsReRunWorkflowRequest, (), EmptyObject>::builder(&self.config)
      .post(url)
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
    ActionsListSelfHostedRunnersForRepoQuery,
    ActionsListSelfHostedRunnersForRepoResponse,
  > {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/actions/runners");

    Request::<
      (),
      ActionsListSelfHostedRunnersForRepoQuery,
      ActionsListSelfHostedRunnersForRepoResponse,
    >::builder(&self.config)
    .get(url)
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
  ) -> Request<(), (), ActionsVariable> {
    let owner = owner.into();
    let repo = repo.into();
    let name = name.into();
    let url = format!("/repos/{owner}/{repo}/actions/variables/{name}");

    Request::<(), (), ActionsVariable>::builder(&self.config)
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
  ) -> NoContentRequest<ActionsUpdateRepoVariableRequest, ()> {
    let owner = owner.into();
    let repo = repo.into();
    let name = name.into();
    let url = format!("/repos/{owner}/{repo}/actions/variables/{name}");

    NoContentRequest::<ActionsUpdateRepoVariableRequest, ()>::builder(&self.config)
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
    ActionsListJobsForWorkflowRunAttemptQuery,
    ActionsListJobsForWorkflowRunAttemptResponse,
  > {
    let owner = owner.into();
    let repo = repo.into();
    let run_id = run_id.into();
    let attempt_number = attempt_number.into();
    let url = format!("/repos/{owner}/{repo}/actions/runs/{run_id}/attempts/{attempt_number}/jobs");

    Request::<
      (),
      ActionsListJobsForWorkflowRunAttemptQuery,
      ActionsListJobsForWorkflowRunAttemptResponse,
    >::builder(&self.config)
    .get(url)
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
  ) -> Request<(), (), AuthenticationToken> {
    let org = org.into();
    let url = format!("/orgs/{org}/actions/runners/registration-token");

    Request::<(), (), AuthenticationToken>::builder(&self.config)
      .post(url)
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
    ActionsListSelectedReposForOrgVariableQuery,
    ActionsListSelectedReposForOrgVariableResponse,
  > {
    let org = org.into();
    let name = name.into();
    let url = format!("/orgs/{org}/actions/variables/{name}/repositories");

    Request::<
      (),
      ActionsListSelectedReposForOrgVariableQuery,
      ActionsListSelectedReposForOrgVariableResponse,
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
  ) -> NoContentRequest<ActionsSetSelectedReposForOrgVariableRequest, ()> {
    let org = org.into();
    let name = name.into();
    let url = format!("/orgs/{org}/actions/variables/{name}/repositories");

    NoContentRequest::<ActionsSetSelectedReposForOrgVariableRequest, ()>::builder(&self.config)
      .put(url)
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
  ) -> NoContentRequest<ActionsReviewCustomGatesForRunRequest, ()> {
    let owner = owner.into();
    let repo = repo.into();
    let run_id = run_id.into();
    let url = format!("/repos/{owner}/{repo}/actions/runs/{run_id}/deployment_protection_rule");

    NoContentRequest::<ActionsReviewCustomGatesForRunRequest, ()>::builder(&self.config)
      .post(url)
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
  ) -> Request<(), (), Workflow> {
    let owner = owner.into();
    let repo = repo.into();
    let workflow_id = workflow_id.into();
    let url = format!("/repos/{owner}/{repo}/actions/workflows/{workflow_id}");

    Request::<(), (), Workflow>::builder(&self.config)
      .get(url)
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
    ActionsListSelectedReposForOrgSecretQuery,
    ActionsListSelectedReposForOrgSecretResponse,
  > {
    let org = org.into();
    let secret_name = secret_name.into();
    let url = format!("/orgs/{org}/actions/secrets/{secret_name}/repositories");

    Request::<
      (),
      ActionsListSelectedReposForOrgSecretQuery,
      ActionsListSelectedReposForOrgSecretResponse,
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
  ) -> NoContentRequest<ActionsSetSelectedReposForOrgSecretRequest, ()> {
    let org = org.into();
    let secret_name = secret_name.into();
    let url = format!("/orgs/{org}/actions/secrets/{secret_name}/repositories");

    NoContentRequest::<ActionsSetSelectedReposForOrgSecretRequest, ()>::builder(&self.config)
      .put(url)
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
  ) -> Request<(), (), AuthenticationToken> {
    let org = org.into();
    let url = format!("/orgs/{org}/actions/runners/remove-token");

    Request::<(), (), AuthenticationToken>::builder(&self.config)
      .post(url)
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
    ActionsListSelectedRepositoriesEnabledGithubActionsOrganizationQuery,
    ActionsListSelectedRepositoriesEnabledGithubActionsOrganizationResponse,
  > {
    let org = org.into();
    let url = format!("/orgs/{org}/actions/permissions/repositories");

    Request::<
      (),
      ActionsListSelectedRepositoriesEnabledGithubActionsOrganizationQuery,
      ActionsListSelectedRepositoriesEnabledGithubActionsOrganizationResponse,
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
  ) -> NoContentRequest<ActionsSetSelectedRepositoriesEnabledGithubActionsOrganizationRequest, ()>
  {
    let org = org.into();
    let url = format!("/orgs/{org}/actions/permissions/repositories");

    NoContentRequest::<ActionsSetSelectedRepositoriesEnabledGithubActionsOrganizationRequest, ()>::builder(&self.config)
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
  ) -> Request<(), ActionsListWorkflowRunsQuery, ActionsListWorkflowRunsResponse> {
    let owner = owner.into();
    let repo = repo.into();
    let workflow_id = workflow_id.into();
    let url = format!("/repos/{owner}/{repo}/actions/workflows/{workflow_id}/runs");

    Request::<(), ActionsListWorkflowRunsQuery, ActionsListWorkflowRunsResponse>::builder(
      &self.config,
    )
    .get(url)
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
  ) -> Request<(), ActionsListEnvironmentVariablesQuery, ActionsListEnvironmentVariablesResponse>
  {
    let owner = owner.into();
    let repo = repo.into();
    let environment_name = environment_name.into();
    let url = format!("/repos/{owner}/{repo}/environments/{environment_name}/variables");

    Request::<(), ActionsListEnvironmentVariablesQuery, ActionsListEnvironmentVariablesResponse>::builder(&self.config)
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
  ) -> Request<ActionsCreateEnvironmentVariableRequest, (), EmptyObject> {
    let owner = owner.into();
    let repo = repo.into();
    let environment_name = environment_name.into();
    let url = format!("/repos/{owner}/{repo}/environments/{environment_name}/variables");

    Request::<ActionsCreateEnvironmentVariableRequest, (), EmptyObject>::builder(&self.config)
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
  ) -> Request<(), (), ActionsVariableForAnOrganization> {
    let org = org.into();
    let name = name.into();
    let url = format!("/orgs/{org}/actions/variables/{name}");

    Request::<(), (), ActionsVariableForAnOrganization>::builder(&self.config)
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
  ) -> NoContentRequest<ActionsUpdateOrgVariableRequest, ()> {
    let org = org.into();
    let name = name.into();
    let url = format!("/orgs/{org}/actions/variables/{name}");

    NoContentRequest::<ActionsUpdateOrgVariableRequest, ()>::builder(&self.config)
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
  ) -> NoContentRequest<ActionsGenerateRunnerJitconfigForOrgRequest, ()> {
    let org = org.into();
    let url = format!("/orgs/{org}/actions/runners/generate-jitconfig");

    NoContentRequest::<ActionsGenerateRunnerJitconfigForOrgRequest, ()>::builder(&self.config)
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
  ) -> Request<(), (), ActionsOidcSubjectCustomizationForARepository> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/actions/oidc/customization/sub");

    Request::<(), (), ActionsOidcSubjectCustomizationForARepository>::builder(&self.config)
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
  ) -> Request<ActionsOidcSubjectCustomizationForARepository, (), EmptyObject> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/actions/oidc/customization/sub");

    Request::<ActionsOidcSubjectCustomizationForARepository, (), EmptyObject>::builder(&self.config)
      .put(url)
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
  ) -> Request<ActionsReRunWorkflowFailedJobsRequest, (), EmptyObject> {
    let owner = owner.into();
    let repo = repo.into();
    let run_id = run_id.into();
    let url = format!("/repos/{owner}/{repo}/actions/runs/{run_id}/rerun-failed-jobs");

    Request::<ActionsReRunWorkflowFailedJobsRequest, (), EmptyObject>::builder(&self.config)
      .post(url)
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
  ) -> Request<
    (),
    ActionsListRepoOrganizationSecretsQuery,
    ActionsListRepoOrganizationSecretsResponse,
  > {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/actions/organization-secrets");

    Request::<(), ActionsListRepoOrganizationSecretsQuery, ActionsListRepoOrganizationSecretsResponse>::builder(&self.config)
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
  ) -> Request<(), (), ActionsSecret> {
    let owner = owner.into();
    let repo = repo.into();
    let secret_name = secret_name.into();
    let url = format!("/repos/{owner}/{repo}/actions/secrets/{secret_name}");

    Request::<(), (), ActionsSecret>::builder(&self.config)
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
  ) -> Request<ActionsCreateOrUpdateRepoSecretRequest, (), EmptyObject> {
    let owner = owner.into();
    let repo = repo.into();
    let secret_name = secret_name.into();
    let url = format!("/repos/{owner}/{repo}/actions/secrets/{secret_name}");

    Request::<ActionsCreateOrUpdateRepoSecretRequest, (), EmptyObject>::builder(&self.config)
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
  ) -> Request<(), (), ActionsGetDefaultWorkflowPermissions> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/actions/permissions/workflow");

    Request::<(), (), ActionsGetDefaultWorkflowPermissions>::builder(&self.config)
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
  ) -> NoContentRequest<ActionsSetDefaultWorkflowPermissions, ()> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/actions/permissions/workflow");

    NoContentRequest::<ActionsSetDefaultWorkflowPermissions, ()>::builder(&self.config)
      .put(url)
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
  ) -> Request<(), ActionsListEnvironmentSecretsQuery, ActionsListEnvironmentSecretsResponse> {
    let owner = owner.into();
    let repo = repo.into();
    let environment_name = environment_name.into();
    let url = format!("/repos/{owner}/{repo}/environments/{environment_name}/secrets");

    Request::<(), ActionsListEnvironmentSecretsQuery, ActionsListEnvironmentSecretsResponse>::builder(&self.config)
      .get(url)
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
  ) -> Request<(), (), SelectedActions> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/actions/permissions/selected-actions");

    Request::<(), (), SelectedActions>::builder(&self.config)
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
  ) -> NoContentRequest<SelectedActions, ()> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/actions/permissions/selected-actions");

    NoContentRequest::<SelectedActions, ()>::builder(&self.config)
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
  ) -> Request<(), (), RunnerApplicationArray> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/actions/runners/downloads");

    Request::<(), (), RunnerApplicationArray>::builder(&self.config)
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
  ) -> NoContentRequest<ActionsGenerateRunnerJitconfigForRepoRequest, ()> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/actions/runners/generate-jitconfig");

    NoContentRequest::<ActionsGenerateRunnerJitconfigForRepoRequest, ()>::builder(&self.config)
      .post(url)
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
  ) -> Request<(), (), WorkflowUsage> {
    let owner = owner.into();
    let repo = repo.into();
    let workflow_id = workflow_id.into();
    let url = format!("/repos/{owner}/{repo}/actions/workflows/{workflow_id}/timing");

    Request::<(), (), WorkflowUsage>::builder(&self.config)
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
    ActionsGetActionsCacheUsageByRepoForOrgQuery,
    ActionsGetActionsCacheUsageByRepoForOrgResponse,
  > {
    let org = org.into();
    let url = format!("/orgs/{org}/actions/cache/usage-by-repository");

    Request::<
      (),
      ActionsGetActionsCacheUsageByRepoForOrgQuery,
      ActionsGetActionsCacheUsageByRepoForOrgResponse,
    >::builder(&self.config)
    .get(url)
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
  ) -> Request<(), ActionsListWorkflowRunsForRepoQuery, ActionsListWorkflowRunsForRepoResponse> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/actions/runs");

    Request::<(), ActionsListWorkflowRunsForRepoQuery, ActionsListWorkflowRunsForRepoResponse>::builder(&self.config)
      .get(url)
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
  ) -> Request<(), (), PendingDeploymentArray> {
    let owner = owner.into();
    let repo = repo.into();
    let run_id = run_id.into();
    let url = format!("/repos/{owner}/{repo}/actions/runs/{run_id}/pending_deployments");

    Request::<(), (), PendingDeploymentArray>::builder(&self.config)
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
  ) -> Request<ActionsReviewPendingDeploymentsForRunRequest, (), DeploymentArray> {
    let owner = owner.into();
    let repo = repo.into();
    let run_id = run_id.into();
    let url = format!("/repos/{owner}/{repo}/actions/runs/{run_id}/pending_deployments");

    Request::<ActionsReviewPendingDeploymentsForRunRequest, (), DeploymentArray>::builder(
      &self.config,
    )
    .post(url)
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
  ) -> Request<(), ActionsGetWorkflowRunAttemptQuery, WorkflowRun> {
    let owner = owner.into();
    let repo = repo.into();
    let run_id = run_id.into();
    let attempt_number = attempt_number.into();
    let url = format!("/repos/{owner}/{repo}/actions/runs/{run_id}/attempts/{attempt_number}");

    Request::<(), ActionsGetWorkflowRunAttemptQuery, WorkflowRun>::builder(&self.config)
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
  ) -> Request<(), ActionsListOrgVariablesQuery, ActionsListOrgVariablesResponse> {
    let org = org.into();
    let url = format!("/orgs/{org}/actions/variables");

    Request::<(), ActionsListOrgVariablesQuery, ActionsListOrgVariablesResponse>::builder(
      &self.config,
    )
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
  ) -> Request<ActionsCreateOrgVariableRequest, (), EmptyObject> {
    let org = org.into();
    let url = format!("/orgs/{org}/actions/variables");

    Request::<ActionsCreateOrgVariableRequest, (), EmptyObject>::builder(&self.config)
      .post(url)
      .build()
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
  ) -> Request<(), (), ActionsCacheUsageOrgEnterprise> {
    let org = org.into();
    let url = format!("/orgs/{org}/actions/cache/usage");

    Request::<(), (), ActionsCacheUsageOrgEnterprise>::builder(&self.config)
      .get(url)
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
  ) -> NoContentRequest<ActionsCreateWorkflowDispatchRequest, ()> {
    let owner = owner.into();
    let repo = repo.into();
    let workflow_id = workflow_id.into();
    let url = format!("/repos/{owner}/{repo}/actions/workflows/{workflow_id}/dispatches");

    NoContentRequest::<ActionsCreateWorkflowDispatchRequest, ()>::builder(&self.config)
      .post(url)
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
  ) -> Request<(), (), EmptyObject> {
    let owner = owner.into();
    let repo = repo.into();
    let run_id = run_id.into();
    let url = format!("/repos/{owner}/{repo}/actions/runs/{run_id}/cancel");

    Request::<(), (), EmptyObject>::builder(&self.config)
      .post(url)
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
  ) -> Request<(), (), RunnerApplicationArray> {
    let org = org.into();
    let url = format!("/orgs/{org}/actions/runners/downloads");

    Request::<(), (), RunnerApplicationArray>::builder(&self.config)
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
    ActionsListRepoOrganizationVariablesQuery,
    ActionsListRepoOrganizationVariablesResponse,
  > {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/actions/organization-variables");

    Request::<
      (),
      ActionsListRepoOrganizationVariablesQuery,
      ActionsListRepoOrganizationVariablesResponse,
    >::builder(&self.config)
    .get(url)
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
  ) -> Request<(), (), WorkflowRunUsage> {
    let owner = owner.into();
    let repo = repo.into();
    let run_id = run_id.into();
    let url = format!("/repos/{owner}/{repo}/actions/runs/{run_id}/timing");

    Request::<(), (), WorkflowRunUsage>::builder(&self.config)
      .get(url)
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
  ) -> Request<(), (), SelfHostedRunners> {
    let org = org.into();
    let runner_id = runner_id.into();
    let url = format!("/orgs/{org}/actions/runners/{runner_id}");

    Request::<(), (), SelfHostedRunners>::builder(&self.config)
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
  ) -> NoContentRequest<ActionsAddCustomLabelsToSelfHostedRunnerForRepoRequest, ()> {
    let owner = owner.into();
    let repo = repo.into();
    let runner_id = runner_id.into();
    let url = format!("/repos/{owner}/{repo}/actions/runners/{runner_id}/labels");

    NoContentRequest::<ActionsAddCustomLabelsToSelfHostedRunnerForRepoRequest, ()>::builder(
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
  ) -> NoContentRequest<ActionsSetCustomLabelsForSelfHostedRunnerForRepoRequest, ()> {
    let owner = owner.into();
    let repo = repo.into();
    let runner_id = runner_id.into();
    let url = format!("/repos/{owner}/{repo}/actions/runners/{runner_id}/labels");

    NoContentRequest::<ActionsSetCustomLabelsForSelfHostedRunnerForRepoRequest, ()>::builder(
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
}
