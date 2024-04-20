#[allow(unused_imports)]
use crate::types::*;
use octocrate_core::*;

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
  ) -> Request<(), ReposGetCommitQuery, Commit> {
    let owner = owner.into();
    let repo = repo.into();
    let ref_ = ref_.into();
    let url = format!("/repos/{owner}/{repo}/commits/{ref_}");

    Request::<(), ReposGetCommitQuery, Commit>::builder(&self.config)
      .get(url)
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
  ) -> Request<ReposAddCollaboratorRequest, (), RepositoryInvitation> {
    let owner = owner.into();
    let repo = repo.into();
    let username = username.into();
    let url = format!("/repos/{owner}/{repo}/collaborators/{username}");

    Request::<ReposAddCollaboratorRequest, (), RepositoryInvitation>::builder(&self.config)
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
  ) -> Request<(), (), WebhookConfiguration> {
    let owner = owner.into();
    let repo = repo.into();
    let hook_id = hook_id.into();
    let url = format!("/repos/{owner}/{repo}/hooks/{hook_id}/config");

    Request::<(), (), WebhookConfiguration>::builder(&self.config)
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
  ) -> Request<ReposUpdateWebhookConfigForRepoRequest, (), WebhookConfiguration> {
    let owner = owner.into();
    let repo = repo.into();
    let hook_id = hook_id.into();
    let url = format!("/repos/{owner}/{repo}/hooks/{hook_id}/config");

    Request::<ReposUpdateWebhookConfigForRepoRequest, (), WebhookConfiguration>::builder(
      &self.config,
    )
    .patch(url)
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
  ) -> Request<(), (), Release> {
    let owner = owner.into();
    let repo = repo.into();
    let tag = tag.into();
    let url = format!("/repos/{owner}/{repo}/releases/tags/{tag}");

    Request::<(), (), Release>::builder(&self.config)
      .get(url)
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
  ) -> Request<(), (), DeployKey> {
    let owner = owner.into();
    let repo = repo.into();
    let key_id = key_id.into();
    let url = format!("/repos/{owner}/{repo}/keys/{key_id}");

    Request::<(), (), DeployKey>::builder(&self.config)
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
  ) -> Request<ReposRenameBranchRequest, (), BranchWithProtection> {
    let owner = owner.into();
    let repo = repo.into();
    let branch = branch.into();
    let url = format!("/repos/{owner}/{repo}/branches/{branch}/rename");

    Request::<ReposRenameBranchRequest, (), BranchWithProtection>::builder(&self.config)
      .post(url)
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
  ) -> Request<(), (), CheckAutomatedSecurityFixes> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/automated-security-fixes");

    Request::<(), (), CheckAutomatedSecurityFixes>::builder(&self.config)
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
  ) -> Request<(), ReposListPullRequestsAssociatedWithCommitQuery, PullRequestSimpleArray> {
    let owner = owner.into();
    let repo = repo.into();
    let commit_sha = commit_sha.into();
    let url = format!("/repos/{owner}/{repo}/commits/{commit_sha}/pulls");

    Request::<(), ReposListPullRequestsAssociatedWithCommitQuery, PullRequestSimpleArray>::builder(
      &self.config,
    )
    .get(url)
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
  ) -> Request<(), (), StatusCheckPolicy> {
    let owner = owner.into();
    let repo = repo.into();
    let branch = branch.into();
    let url = format!("/repos/{owner}/{repo}/branches/{branch}/protection/required_status_checks");

    Request::<(), (), StatusCheckPolicy>::builder(&self.config)
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
  ) -> Request<ReposUpdateStatusCheckProtectionRequest, (), StatusCheckPolicy> {
    let owner = owner.into();
    let repo = repo.into();
    let branch = branch.into();
    let url = format!("/repos/{owner}/{repo}/branches/{branch}/protection/required_status_checks");

    Request::<ReposUpdateStatusCheckProtectionRequest, (), StatusCheckPolicy>::builder(&self.config)
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
  ) -> Request<(), (), GitHubPages> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/pages");

    Request::<(), (), GitHubPages>::builder(&self.config)
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
  ) -> Request<serde_json::Value, (), GitHubPages> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/pages");

    Request::<serde_json::Value, (), GitHubPages>::builder(&self.config)
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
  ) -> NoContentRequest<serde_json::Value, ()> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/pages");

    NoContentRequest::<serde_json::Value, ()>::builder(&self.config)
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
  ) -> Request<(), (), CommitComment> {
    let owner = owner.into();
    let repo = repo.into();
    let comment_id = comment_id.into();
    let url = format!("/repos/{owner}/{repo}/comments/{comment_id}");

    Request::<(), (), CommitComment>::builder(&self.config)
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
  ) -> Request<ReposUpdateCommitCommentRequest, (), CommitComment> {
    let owner = owner.into();
    let repo = repo.into();
    let comment_id = comment_id.into();
    let url = format!("/repos/{owner}/{repo}/comments/{comment_id}");

    Request::<ReposUpdateCommitCommentRequest, (), CommitComment>::builder(&self.config)
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

  /// **List public repositories**
  ///
  /// Lists all public repositories in the order that they were created.
  ///
  /// Note:
  /// - For GitHub Enterprise Server, this endpoint will only list repositories available to all users on the enterprise.
  /// - Pagination is powered exclusively by the `since` parameter. Use the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers) to get the URL for the next page of repositories.
  ///
  /// *Documentation*: [https://docs.github.com/rest/repos/repos#list-public-repositories](https://docs.github.com/rest/repos/repos#list-public-repositories)
  pub fn list_public(&self) -> Request<(), ReposListPublicQuery, MinimalRepositoryArray> {
    let url = format!("/repositories");

    Request::<(), ReposListPublicQuery, MinimalRepositoryArray>::builder(&self.config)
      .get(url)
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
  ) -> Request<(), ReposListForksQuery, MinimalRepositoryArray> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/forks");

    Request::<(), ReposListForksQuery, MinimalRepositoryArray>::builder(&self.config)
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
  ) -> Request<ReposCreateForkRequest, (), FullRepository> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/forks");

    Request::<ReposCreateForkRequest, (), FullRepository>::builder(&self.config)
      .post(url)
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
  ) -> Request<(), (), BranchShortArray> {
    let owner = owner.into();
    let repo = repo.into();
    let commit_sha = commit_sha.into();
    let url = format!("/repos/{owner}/{repo}/commits/{commit_sha}/branches-where-head");

    Request::<(), (), BranchShortArray>::builder(&self.config)
      .get(url)
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
  ) -> Request<(), (), StringArray> {
    let owner = owner.into();
    let repo = repo.into();
    let branch = branch.into();
    let url =
      format!("/repos/{owner}/{repo}/branches/{branch}/protection/required_status_checks/contexts");

    Request::<(), (), StringArray>::builder(&self.config)
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
  ) -> Request<ReposAddStatusCheckContextsRequest, (), StringArray> {
    let owner = owner.into();
    let repo = repo.into();
    let branch = branch.into();
    let url =
      format!("/repos/{owner}/{repo}/branches/{branch}/protection/required_status_checks/contexts");

    Request::<ReposAddStatusCheckContextsRequest, (), StringArray>::builder(&self.config)
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
  ) -> Request<ReposSetStatusCheckContextsRequest, (), StringArray> {
    let owner = owner.into();
    let repo = repo.into();
    let branch = branch.into();
    let url =
      format!("/repos/{owner}/{repo}/branches/{branch}/protection/required_status_checks/contexts");

    Request::<ReposSetStatusCheckContextsRequest, (), StringArray>::builder(&self.config)
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
  ) -> Request<ReposRemoveStatusCheckContextsRequest, (), StringArray> {
    let owner = owner.into();
    let repo = repo.into();
    let branch = branch.into();
    let url =
      format!("/repos/{owner}/{repo}/branches/{branch}/protection/required_status_checks/contexts");

    Request::<ReposRemoveStatusCheckContextsRequest, (), StringArray>::builder(&self.config)
      .delete(url)
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
  ) -> Request<(), ReposCodeownersErrorsQuery, CodeownersErrors> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/codeowners/errors");

    Request::<(), ReposCodeownersErrorsQuery, CodeownersErrors>::builder(&self.config)
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
  ) -> Request<(), (), BranchProtection> {
    let owner = owner.into();
    let repo = repo.into();
    let branch = branch.into();
    let url = format!("/repos/{owner}/{repo}/branches/{branch}/protection");

    Request::<(), (), BranchProtection>::builder(&self.config)
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
  ) -> Request<ReposUpdateBranchProtectionRequest, (), ProtectedBranch> {
    let owner = owner.into();
    let repo = repo.into();
    let branch = branch.into();
    let url = format!("/repos/{owner}/{repo}/branches/{branch}/protection");

    Request::<ReposUpdateBranchProtectionRequest, (), ProtectedBranch>::builder(&self.config)
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
  ) -> Request<ReposCreateUsingTemplateRequest, (), FullRepository> {
    let template_owner = template_owner.into();
    let template_repo = template_repo.into();
    let url = format!("/repos/{template_owner}/{template_repo}/generate");

    Request::<ReposCreateUsingTemplateRequest, (), FullRepository>::builder(&self.config)
      .post(url)
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
  ) -> Request<(), (), ProtectedBranchAdminEnforced> {
    let owner = owner.into();
    let repo = repo.into();
    let branch = branch.into();
    let url = format!("/repos/{owner}/{repo}/branches/{branch}/protection/required_signatures");

    Request::<(), (), ProtectedBranchAdminEnforced>::builder(&self.config)
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
  ) -> Request<(), (), ProtectedBranchAdminEnforced> {
    let owner = owner.into();
    let repo = repo.into();
    let branch = branch.into();
    let url = format!("/repos/{owner}/{repo}/branches/{branch}/protection/required_signatures");

    Request::<(), (), ProtectedBranchAdminEnforced>::builder(&self.config)
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
  ) -> Request<(), ReposListForOrgQuery, MinimalRepositoryArray> {
    let org = org.into();
    let url = format!("/orgs/{org}/repos");

    Request::<(), ReposListForOrgQuery, MinimalRepositoryArray>::builder(&self.config)
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
  ) -> Request<ReposCreateInOrgRequest, (), FullRepository> {
    let org = org.into();
    let url = format!("/orgs/{org}/repos");

    Request::<ReposCreateInOrgRequest, (), FullRepository>::builder(&self.config)
      .post(url)
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
  ) -> Request<(), (), Release> {
    let owner = owner.into();
    let repo = repo.into();
    let release_id = release_id.into();
    let url = format!("/repos/{owner}/{repo}/releases/{release_id}");

    Request::<(), (), Release>::builder(&self.config)
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
  ) -> Request<ReposUpdateReleaseRequest, (), Release> {
    let owner = owner.into();
    let repo = repo.into();
    let release_id = release_id.into();
    let url = format!("/repos/{owner}/{repo}/releases/{release_id}");

    Request::<ReposUpdateReleaseRequest, (), Release>::builder(&self.config)
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
  ) -> Request<(), (), RepositoryCollaboratorPermission> {
    let owner = owner.into();
    let repo = repo.into();
    let username = username.into();
    let url = format!("/repos/{owner}/{repo}/collaborators/{username}/permission");

    Request::<(), (), RepositoryCollaboratorPermission>::builder(&self.config)
      .get(url)
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
  ) -> Request<(), ReposListReleaseAssetsQuery, ReleaseAssetArray> {
    let owner = owner.into();
    let repo = repo.into();
    let release_id = release_id.into();
    let url = format!("/repos/{owner}/{repo}/releases/{release_id}/assets");

    Request::<(), ReposListReleaseAssetsQuery, ReleaseAssetArray>::builder(&self.config)
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
  ) -> Request<(), ReposUploadReleaseAssetQuery, ReleaseAsset> {
    let owner = owner.into();
    let repo = repo.into();
    let release_id = release_id.into();
    let url = format!("/repos/{owner}/{repo}/releases/{release_id}/assets");

    Request::<(), ReposUploadReleaseAssetQuery, ReleaseAsset>::builder(&self.config)
      .post(url)
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
  ) -> Request<(), ReposListForAuthenticatedUserQuery, RepositoryArray> {
    let url = format!("/user/repos");

    Request::<(), ReposListForAuthenticatedUserQuery, RepositoryArray>::builder(&self.config)
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
  ) -> Request<ReposCreateForAuthenticatedUserRequest, (), FullRepository> {
    let url = format!("/user/repos");

    Request::<ReposCreateForAuthenticatedUserRequest, (), FullRepository>::builder(&self.config)
      .post(url)
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
  ) -> Request<(), ReposListForUserQuery, MinimalRepositoryArray> {
    let username = username.into();
    let url = format!("/users/{username}/repos");

    Request::<(), ReposListForUserQuery, MinimalRepositoryArray>::builder(&self.config)
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
  ) -> Request<(), (), Environment> {
    let owner = owner.into();
    let repo = repo.into();
    let environment_name = environment_name.into();
    let url = format!("/repos/{owner}/{repo}/environments/{environment_name}");

    Request::<(), (), Environment>::builder(&self.config)
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
  ) -> Request<ReposCreateOrUpdateEnvironmentRequest, (), Environment> {
    let owner = owner.into();
    let repo = repo.into();
    let environment_name = environment_name.into();
    let url = format!("/repos/{owner}/{repo}/environments/{environment_name}");

    Request::<ReposCreateOrUpdateEnvironmentRequest, (), Environment>::builder(&self.config)
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
  ) -> Request<(), (), GitHubPagesDeploymentStatus> {
    let owner = owner.into();
    let repo = repo.into();
    let pages_deployment_id = pages_deployment_id.into();
    let url = format!("/repos/{owner}/{repo}/pages/deployments/{pages_deployment_id}");

    Request::<(), (), GitHubPagesDeploymentStatus>::builder(&self.config)
      .get(url)
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
  ) -> Request<(), (), Vec<Vec<i64>>> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/stats/code_frequency");

    Request::<(), (), Vec<Vec<i64>>>::builder(&self.config)
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
  ) -> Request<(), ReposListTeamsQuery, TeamArray> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/teams");

    Request::<(), ReposListTeamsQuery, TeamArray>::builder(&self.config)
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
  ) -> Request<(), (), ReferrerTrafficArray> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/traffic/popular/referrers");

    Request::<(), (), ReferrerTrafficArray>::builder(&self.config)
      .get(url)
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
  ) -> Request<ReposMergeRequest, (), Commit> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/merges");

    Request::<ReposMergeRequest, (), Commit>::builder(&self.config)
      .post(url)
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
  ) -> Request<(), (), Vec<Vec<i64>>> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/stats/punch_card");

    Request::<(), (), Vec<Vec<i64>>>::builder(&self.config)
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
  ) -> Request<(), (), RuleSuite> {
    let org = org.into();
    let rule_suite_id = rule_suite_id.into();
    let url = format!("/orgs/{org}/rulesets/rule-suites/{rule_suite_id}");

    Request::<(), (), RuleSuite>::builder(&self.config)
      .get(url)
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
  ) -> Request<(), (), AutolinkReference> {
    let owner = owner.into();
    let repo = repo.into();
    let autolink_id = autolink_id.into();
    let url = format!("/repos/{owner}/{repo}/autolinks/{autolink_id}");

    Request::<(), (), AutolinkReference>::builder(&self.config)
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
  ) -> Request<(), ReposListCollaboratorsQuery, CollaboratorArray> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/collaborators");

    Request::<(), ReposListCollaboratorsQuery, CollaboratorArray>::builder(&self.config)
      .get(url)
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
  ) -> Request<(), ReposGetRepoRuleSuitesQuery, RuleSuites> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/rulesets/rule-suites");

    Request::<(), ReposGetRepoRuleSuitesQuery, RuleSuites>::builder(&self.config)
      .get(url)
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
  ) -> Request<(), ReposListWebhooksQuery, WebhookArray> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/hooks");

    Request::<(), ReposListWebhooksQuery, WebhookArray>::builder(&self.config)
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
  ) -> Request<ReposCreateWebhookRequest, (), Webhook> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/hooks");

    Request::<ReposCreateWebhookRequest, (), Webhook>::builder(&self.config)
      .post(url)
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

  /// **Transfer a repository**
  ///
  /// A transfer request will need to be accepted by the new owner when transferring a personal repository to another user. The response will contain the original `owner`, and the transfer will continue asynchronously. For more details on the requirements to transfer personal and organization-owned repositories, see [about repository transfers](https://docs.github.com/articles/about-repository-transfers/).
  ///
  /// *Documentation*: [https://docs.github.com/rest/repos/repos#transfer-a-repository](https://docs.github.com/rest/repos/repos#transfer-a-repository)
  pub fn transfer(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<ReposTransferRequest, (), MinimalRepository> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/transfer");

    Request::<ReposTransferRequest, (), MinimalRepository>::builder(&self.config)
      .post(url)
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
  ) -> Request<(), ReposListActivitiesQuery, ActivityArray> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/activity");

    Request::<(), ReposListActivitiesQuery, ActivityArray>::builder(&self.config)
      .get(url)
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
  ) -> Request<(), ReposGetAllEnvironmentsQuery, ReposGetAllEnvironmentsResponse> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/environments");

    Request::<(), ReposGetAllEnvironmentsQuery, ReposGetAllEnvironmentsResponse>::builder(
      &self.config,
    )
    .get(url)
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
  ) -> Request<(), ReposGetOrgRulesetsQuery, RepositoryRulesetArray> {
    let org = org.into();
    let url = format!("/orgs/{org}/rulesets");

    Request::<(), ReposGetOrgRulesetsQuery, RepositoryRulesetArray>::builder(&self.config)
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
  ) -> Request<ReposCreateOrgRulesetRequest, (), RepositoryRuleset> {
    let org = org.into();
    let url = format!("/orgs/{org}/rulesets");

    Request::<ReposCreateOrgRulesetRequest, (), RepositoryRuleset>::builder(&self.config)
      .post(url)
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
  ) -> Request<(), (), TeamArray> {
    let owner = owner.into();
    let repo = repo.into();
    let branch = branch.into();
    let url = format!("/repos/{owner}/{repo}/branches/{branch}/protection/restrictions/teams");

    Request::<(), (), TeamArray>::builder(&self.config)
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
  ) -> Request<ReposAddTeamAccessRestrictionsRequest, (), TeamArray> {
    let owner = owner.into();
    let repo = repo.into();
    let branch = branch.into();
    let url = format!("/repos/{owner}/{repo}/branches/{branch}/protection/restrictions/teams");

    Request::<ReposAddTeamAccessRestrictionsRequest, (), TeamArray>::builder(&self.config)
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
  ) -> Request<ReposSetTeamAccessRestrictionsRequest, (), TeamArray> {
    let owner = owner.into();
    let repo = repo.into();
    let branch = branch.into();
    let url = format!("/repos/{owner}/{repo}/branches/{branch}/protection/restrictions/teams");

    Request::<ReposSetTeamAccessRestrictionsRequest, (), TeamArray>::builder(&self.config)
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
  ) -> Request<ReposRemoveTeamAccessRestrictionsRequest, (), TeamArray> {
    let owner = owner.into();
    let repo = repo.into();
    let branch = branch.into();
    let url = format!("/repos/{owner}/{repo}/branches/{branch}/protection/restrictions/teams");

    Request::<ReposRemoveTeamAccessRestrictionsRequest, (), TeamArray>::builder(&self.config)
      .delete(url)
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
  ) -> Request<(), (), RuleSuite> {
    let owner = owner.into();
    let repo = repo.into();
    let rule_suite_id = rule_suite_id.into();
    let url = format!("/repos/{owner}/{repo}/rulesets/rule-suites/{rule_suite_id}");

    Request::<(), (), RuleSuite>::builder(&self.config)
      .get(url)
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
  ) -> Request<(), ReposListDeploymentBranchPoliciesQuery, ReposListDeploymentBranchPoliciesResponse>
  {
    let owner = owner.into();
    let repo = repo.into();
    let environment_name = environment_name.into();
    let url =
      format!("/repos/{owner}/{repo}/environments/{environment_name}/deployment-branch-policies");

    Request::<(), ReposListDeploymentBranchPoliciesQuery, ReposListDeploymentBranchPoliciesResponse>::builder(&self.config)
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
  ) -> Request<DeploymentBranchAndTagPolicyNamePattern, (), DeploymentBranchPolicy> {
    let owner = owner.into();
    let repo = repo.into();
    let environment_name = environment_name.into();
    let url =
      format!("/repos/{owner}/{repo}/environments/{environment_name}/deployment-branch-policies");

    Request::<DeploymentBranchAndTagPolicyNamePattern, (), DeploymentBranchPolicy>::builder(
      &self.config,
    )
    .post(url)
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
  ) -> Request<ReposUpdateInvitationRequest, (), RepositoryInvitation> {
    let owner = owner.into();
    let repo = repo.into();
    let invitation_id = invitation_id.into();
    let url = format!("/repos/{owner}/{repo}/invitations/{invitation_id}");

    Request::<ReposUpdateInvitationRequest, (), RepositoryInvitation>::builder(&self.config)
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
  ) -> Request<(), (), ProtectedBranchAdminEnforced> {
    let owner = owner.into();
    let repo = repo.into();
    let branch = branch.into();
    let url = format!("/repos/{owner}/{repo}/branches/{branch}/protection/enforce_admins");

    Request::<(), (), ProtectedBranchAdminEnforced>::builder(&self.config)
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
  ) -> Request<(), (), ProtectedBranchAdminEnforced> {
    let owner = owner.into();
    let repo = repo.into();
    let branch = branch.into();
    let url = format!("/repos/{owner}/{repo}/branches/{branch}/protection/enforce_admins");

    Request::<(), (), ProtectedBranchAdminEnforced>::builder(&self.config)
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

  /// **List deploy keys**
  ///
  ///
  /// *Documentation*: [https://docs.github.com/rest/deploy-keys/deploy-keys#list-deploy-keys](https://docs.github.com/rest/deploy-keys/deploy-keys#list-deploy-keys)
  pub fn list_deploy_keys(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), ReposListDeployKeysQuery, DeployKeyArray> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/keys");

    Request::<(), ReposListDeployKeysQuery, DeployKeyArray>::builder(&self.config)
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
  ) -> Request<ReposCreateDeployKeyRequest, (), DeployKey> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/keys");

    Request::<ReposCreateDeployKeyRequest, (), DeployKey>::builder(&self.config)
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
  ) -> Request<(), ReposListInvitationsForAuthenticatedUserQuery, RepositoryInvitationArray> {
    let url = format!("/user/repository_invitations");

    Request::<(), ReposListInvitationsForAuthenticatedUserQuery, RepositoryInvitationArray>::builder(&self.config)
      .get(url)
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
  ) -> Request<(), (), BranchRestrictionPolicy> {
    let owner = owner.into();
    let repo = repo.into();
    let branch = branch.into();
    let url = format!("/repos/{owner}/{repo}/branches/{branch}/protection/restrictions");

    Request::<(), (), BranchRestrictionPolicy>::builder(&self.config)
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

  /// **List deployments**
  ///
  /// Simple filtering of deployments is available via query parameters:
  ///
  /// *Documentation*: [https://docs.github.com/rest/deployments/deployments#list-deployments](https://docs.github.com/rest/deployments/deployments#list-deployments)
  pub fn list_deployments(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), ReposListDeploymentsQuery, DeploymentArray> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/deployments");

    Request::<(), ReposListDeploymentsQuery, DeploymentArray>::builder(&self.config)
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
  ) -> Request<ReposCreateDeploymentRequest, (), ReposCreateDeploymentResponse> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/deployments");

    Request::<ReposCreateDeploymentRequest, (), ReposCreateDeploymentResponse>::builder(
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
  ) -> Request<(), (), DeploymentStatus> {
    let owner = owner.into();
    let repo = repo.into();
    let deployment_id = deployment_id.into();
    let status_id = status_id.into();
    let url = format!("/repos/{owner}/{repo}/deployments/{deployment_id}/statuses/{status_id}");

    Request::<(), (), DeploymentStatus>::builder(&self.config)
      .get(url)
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
  ) -> Request<(), ReposListCommitsQuery, CommitArray> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/commits");

    Request::<(), ReposListCommitsQuery, CommitArray>::builder(&self.config)
      .get(url)
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
  ) -> Request<(), ReposGetOrgRuleSuitesQuery, RuleSuites> {
    let org = org.into();
    let url = format!("/orgs/{org}/rulesets/rule-suites");

    Request::<(), ReposGetOrgRuleSuitesQuery, RuleSuites>::builder(&self.config)
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
  ) -> Request<(), (), RepositoryRuleset> {
    let org = org.into();
    let ruleset_id = ruleset_id.into();
    let url = format!("/orgs/{org}/rulesets/{ruleset_id}");

    Request::<(), (), RepositoryRuleset>::builder(&self.config)
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
  ) -> Request<ReposUpdateOrgRulesetRequest, (), RepositoryRuleset> {
    let org = org.into();
    let ruleset_id = ruleset_id.into();
    let url = format!("/orgs/{org}/rulesets/{ruleset_id}");

    Request::<ReposUpdateOrgRulesetRequest, (), RepositoryRuleset>::builder(&self.config)
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

  /// **Get all repository rulesets**
  ///
  /// Get all the rulesets for a repository.
  ///
  /// *Documentation*: [https://docs.github.com/rest/repos/rules#get-all-repository-rulesets](https://docs.github.com/rest/repos/rules#get-all-repository-rulesets)
  pub fn get_repo_rulesets(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), ReposGetRepoRulesetsQuery, RepositoryRulesetArray> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/rulesets");

    Request::<(), ReposGetRepoRulesetsQuery, RepositoryRulesetArray>::builder(&self.config)
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
  ) -> Request<ReposCreateRepoRulesetRequest, (), RepositoryRuleset> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/rulesets");

    Request::<ReposCreateRepoRulesetRequest, (), RepositoryRuleset>::builder(&self.config)
      .post(url)
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
  ) -> Request<(), ReposCompareCommitsQuery, CommitComparison> {
    let owner = owner.into();
    let repo = repo.into();
    let basehead = basehead.into();
    let url = format!("/repos/{owner}/{repo}/compare/{basehead}");

    Request::<(), ReposCompareCommitsQuery, CommitComparison>::builder(&self.config)
      .get(url)
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
  ) -> Request<ReposGenerateReleaseNotesRequest, (), GeneratedReleaseNotesContent> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/releases/generate-notes");

    Request::<ReposGenerateReleaseNotesRequest, (), GeneratedReleaseNotesContent>::builder(
      &self.config,
    )
    .post(url)
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
  ) -> Request<(), ReposListBranchesQuery, ShortBranchArray> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/branches");

    Request::<(), ReposListBranchesQuery, ShortBranchArray>::builder(&self.config)
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
  ) -> NoContentRequest<ReposCreateDispatchEventRequest, ()> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/dispatches");

    NoContentRequest::<ReposCreateDispatchEventRequest, ()>::builder(&self.config)
      .post(url)
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
  ) -> Request<ReposCreatePagesDeploymentRequest, (), GitHubPages> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/pages/deployments");

    Request::<ReposCreatePagesDeploymentRequest, (), GitHubPages>::builder(&self.config)
      .post(url)
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
  ) -> Request<(), (), SimpleUserArray> {
    let owner = owner.into();
    let repo = repo.into();
    let branch = branch.into();
    let url = format!("/repos/{owner}/{repo}/branches/{branch}/protection/restrictions/users");

    Request::<(), (), SimpleUserArray>::builder(&self.config)
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
  ) -> Request<ReposAddUserAccessRestrictionsRequest, (), SimpleUserArray> {
    let owner = owner.into();
    let repo = repo.into();
    let branch = branch.into();
    let url = format!("/repos/{owner}/{repo}/branches/{branch}/protection/restrictions/users");

    Request::<ReposAddUserAccessRestrictionsRequest, (), SimpleUserArray>::builder(&self.config)
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
  ) -> Request<ReposSetUserAccessRestrictionsRequest, (), SimpleUserArray> {
    let owner = owner.into();
    let repo = repo.into();
    let branch = branch.into();
    let url = format!("/repos/{owner}/{repo}/branches/{branch}/protection/restrictions/users");

    Request::<ReposSetUserAccessRestrictionsRequest, (), SimpleUserArray>::builder(&self.config)
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
  ) -> Request<ReposRemoveUserAccessRestrictionsRequest, (), SimpleUserArray> {
    let owner = owner.into();
    let repo = repo.into();
    let branch = branch.into();
    let url = format!("/repos/{owner}/{repo}/branches/{branch}/protection/restrictions/users");

    Request::<ReposRemoveUserAccessRestrictionsRequest, (), SimpleUserArray>::builder(&self.config)
      .delete(url)
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
  ) -> Request<(), (), PageBuild> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/pages/builds/latest");

    Request::<(), (), PageBuild>::builder(&self.config)
      .get(url)
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
  ) -> Request<(), ReposListReleasesQuery, ReleaseArray> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/releases");

    Request::<(), ReposListReleasesQuery, ReleaseArray>::builder(&self.config)
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
  ) -> Request<ReposCreateReleaseRequest, (), Release> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/releases");

    Request::<ReposCreateReleaseRequest, (), Release>::builder(&self.config)
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
  ) -> Request<(), ReposListInvitationsQuery, RepositoryInvitationArray> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/invitations");

    Request::<(), ReposListInvitationsQuery, RepositoryInvitationArray>::builder(&self.config)
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
  ) -> Request<(), ReposListCommitCommentsForRepoQuery, CommitCommentArray> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/comments");

    Request::<(), ReposListCommitCommentsForRepoQuery, CommitCommentArray>::builder(&self.config)
      .get(url)
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
  ) -> Request<(), (), DeploymentBranchPolicy> {
    let owner = owner.into();
    let repo = repo.into();
    let environment_name = environment_name.into();
    let branch_policy_id = branch_policy_id.into();
    let url = format!("/repos/{owner}/{repo}/environments/{environment_name}/deployment-branch-policies/{branch_policy_id}");

    Request::<(), (), DeploymentBranchPolicy>::builder(&self.config)
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
  ) -> Request<DeploymentBranchPolicyNamePattern, (), DeploymentBranchPolicy> {
    let owner = owner.into();
    let repo = repo.into();
    let environment_name = environment_name.into();
    let branch_policy_id = branch_policy_id.into();
    let url = format!("/repos/{owner}/{repo}/environments/{environment_name}/deployment-branch-policies/{branch_policy_id}");

    Request::<DeploymentBranchPolicyNamePattern, (), DeploymentBranchPolicy>::builder(&self.config)
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
  ) -> Request<(), ReposGetRepoRulesetQuery, RepositoryRuleset> {
    let owner = owner.into();
    let repo = repo.into();
    let ruleset_id = ruleset_id.into();
    let url = format!("/repos/{owner}/{repo}/rulesets/{ruleset_id}");

    Request::<(), ReposGetRepoRulesetQuery, RepositoryRuleset>::builder(&self.config)
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
  ) -> Request<ReposUpdateRepoRulesetRequest, (), RepositoryRuleset> {
    let owner = owner.into();
    let repo = repo.into();
    let ruleset_id = ruleset_id.into();
    let url = format!("/repos/{owner}/{repo}/rulesets/{ruleset_id}");

    Request::<ReposUpdateRepoRulesetRequest, (), RepositoryRuleset>::builder(&self.config)
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
  ) -> Request<(), ReposListWebhookDeliveriesQuery, SimpleWebhookDeliveryArray> {
    let owner = owner.into();
    let repo = repo.into();
    let hook_id = hook_id.into();
    let url = format!("/repos/{owner}/{repo}/hooks/{hook_id}/deliveries");

    Request::<(), ReposListWebhookDeliveriesQuery, SimpleWebhookDeliveryArray>::builder(
      &self.config,
    )
    .get(url)
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
    ReposListCustomDeploymentRuleIntegrationsQuery,
    ReposListCustomDeploymentRuleIntegrationsResponse,
  > {
    let environment_name = environment_name.into();
    let repo = repo.into();
    let owner = owner.into();
    let url = format!(
      "/repos/{owner}/{repo}/environments/{environment_name}/deployment_protection_rules/apps"
    );

    Request::<
      (),
      ReposListCustomDeploymentRuleIntegrationsQuery,
      ReposListCustomDeploymentRuleIntegrationsResponse,
    >::builder(&self.config)
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
  ) -> Request<(), (), CommitActivityArray> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/stats/commit_activity");

    Request::<(), (), CommitActivityArray>::builder(&self.config)
      .get(url)
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
  ) -> Request<(), (), Webhook> {
    let owner = owner.into();
    let repo = repo.into();
    let hook_id = hook_id.into();
    let url = format!("/repos/{owner}/{repo}/hooks/{hook_id}");

    Request::<(), (), Webhook>::builder(&self.config)
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
  ) -> Request<ReposUpdateWebhookRequest, (), Webhook> {
    let owner = owner.into();
    let repo = repo.into();
    let hook_id = hook_id.into();
    let url = format!("/repos/{owner}/{repo}/hooks/{hook_id}");

    Request::<ReposUpdateWebhookRequest, (), Webhook>::builder(&self.config)
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
  ) -> Request<(), ReposListCommentsForCommitQuery, CommitCommentArray> {
    let owner = owner.into();
    let repo = repo.into();
    let commit_sha = commit_sha.into();
    let url = format!("/repos/{owner}/{repo}/commits/{commit_sha}/comments");

    Request::<(), ReposListCommentsForCommitQuery, CommitCommentArray>::builder(&self.config)
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
  ) -> Request<ReposCreateCommitCommentRequest, (), CommitComment> {
    let owner = owner.into();
    let repo = repo.into();
    let commit_sha = commit_sha.into();
    let url = format!("/repos/{owner}/{repo}/commits/{commit_sha}/comments");

    Request::<ReposCreateCommitCommentRequest, (), CommitComment>::builder(&self.config)
      .post(url)
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
  ) -> Request<(), (), WebhookDelivery> {
    let owner = owner.into();
    let repo = repo.into();
    let hook_id = hook_id.into();
    let delivery_id = delivery_id.into();
    let url = format!("/repos/{owner}/{repo}/hooks/{hook_id}/deliveries/{delivery_id}");

    Request::<(), (), WebhookDelivery>::builder(&self.config)
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
  ) -> Request<(), (), ParticipationStats> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/stats/participation");

    Request::<(), (), ParticipationStats>::builder(&self.config)
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
  ) -> Request<(), (), AutolinkReferenceArray> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/autolinks");

    Request::<(), (), AutolinkReferenceArray>::builder(&self.config)
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
  ) -> Request<ReposCreateAutolinkRequest, (), AutolinkReference> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/autolinks");

    Request::<ReposCreateAutolinkRequest, (), AutolinkReference>::builder(&self.config)
      .post(url)
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
  ) -> Request<(), (), CommunityProfile> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/community/profile");

    Request::<(), (), CommunityProfile>::builder(&self.config)
      .get(url)
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
  ) -> Request<(), ReposListPagesBuildsQuery, PageBuildArray> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/pages/builds");

    Request::<(), ReposListPagesBuildsQuery, PageBuildArray>::builder(&self.config)
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
  ) -> Request<(), (), PageBuildStatus> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/pages/builds");

    Request::<(), (), PageBuildStatus>::builder(&self.config)
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
  ) -> Request<(), (), Release> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/releases/latest");

    Request::<(), (), Release>::builder(&self.config)
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
  ) -> Request<ReposMergeUpstreamRequest, (), MergedUpstream> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/merge-upstream");

    Request::<ReposMergeUpstreamRequest, (), MergedUpstream>::builder(&self.config)
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
  ) -> Request<(), ReposListTagsQuery, TagArray> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/tags");

    Request::<(), ReposListTagsQuery, TagArray>::builder(&self.config)
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
  ) -> Request<(), (), ContentTrafficArray> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/traffic/popular/paths");

    Request::<(), (), ContentTrafficArray>::builder(&self.config)
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
  ) -> Request<(), ReposListCommitStatusesForRefQuery, StatusArray> {
    let owner = owner.into();
    let repo = repo.into();
    let ref_ = ref_.into();
    let url = format!("/repos/{owner}/{repo}/commits/{ref_}/statuses");

    Request::<(), ReposListCommitStatusesForRefQuery, StatusArray>::builder(&self.config)
      .get(url)
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
  ) -> Request<(), (), serde_json::Value> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/languages");

    Request::<(), (), serde_json::Value>::builder(&self.config)
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
  ) -> Request<(), ReposGetContentQuery, ReposGetContentResponse> {
    let owner = owner.into();
    let repo = repo.into();
    let path = path.into();
    let url = format!("/repos/{owner}/{repo}/contents/{path}");

    Request::<(), ReposGetContentQuery, ReposGetContentResponse>::builder(&self.config)
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
  ) -> Request<ReposCreateOrUpdateFileContentsRequest, (), FileCommit> {
    let owner = owner.into();
    let repo = repo.into();
    let path = path.into();
    let url = format!("/repos/{owner}/{repo}/contents/{path}");

    Request::<ReposCreateOrUpdateFileContentsRequest, (), FileCommit>::builder(&self.config)
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
  ) -> Request<ReposDeleteFileRequest, (), FileCommit> {
    let owner = owner.into();
    let repo = repo.into();
    let path = path.into();
    let url = format!("/repos/{owner}/{repo}/contents/{path}");

    Request::<ReposDeleteFileRequest, (), FileCommit>::builder(&self.config)
      .delete(url)
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
  ) -> Request<(), (), TagProtectionArray> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/tags/protection");

    Request::<(), (), TagProtectionArray>::builder(&self.config)
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
  ) -> Request<ReposCreateTagProtectionRequest, (), TagProtection> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/tags/protection");

    Request::<ReposCreateTagProtectionRequest, (), TagProtection>::builder(&self.config)
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
  ) -> Request<(), (), ReposGetPagesHealthCheckResponse> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/pages/health");

    Request::<(), (), ReposGetPagesHealthCheckResponse>::builder(&self.config)
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
  ) -> Request<(), ReposGetCombinedStatusForRefQuery, CombinedCommitStatus> {
    let owner = owner.into();
    let repo = repo.into();
    let ref_ = ref_.into();
    let url = format!("/repos/{owner}/{repo}/commits/{ref_}/status");

    Request::<(), ReposGetCombinedStatusForRefQuery, CombinedCommitStatus>::builder(&self.config)
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
  ) -> Request<(), (), PageBuild> {
    let owner = owner.into();
    let repo = repo.into();
    let build_id = build_id.into();
    let url = format!("/repos/{owner}/{repo}/pages/builds/{build_id}");

    Request::<(), (), PageBuild>::builder(&self.config)
      .get(url)
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
  ) -> Request<(), ReposGetBranchRulesQuery, RepositoryRuleArray> {
    let owner = owner.into();
    let repo = repo.into();
    let branch = branch.into();
    let url = format!("/repos/{owner}/{repo}/rules/branches/{branch}");

    Request::<(), ReposGetBranchRulesQuery, RepositoryRuleArray>::builder(&self.config)
      .get(url)
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
  ) -> Request<(), ReposListDeploymentStatusesQuery, DeploymentStatusArray> {
    let owner = owner.into();
    let repo = repo.into();
    let deployment_id = deployment_id.into();
    let url = format!("/repos/{owner}/{repo}/deployments/{deployment_id}/statuses");

    Request::<(), ReposListDeploymentStatusesQuery, DeploymentStatusArray>::builder(&self.config)
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
  ) -> Request<ReposCreateDeploymentStatusRequest, (), DeploymentStatus> {
    let owner = owner.into();
    let repo = repo.into();
    let deployment_id = deployment_id.into();
    let url = format!("/repos/{owner}/{repo}/deployments/{deployment_id}/statuses");

    Request::<ReposCreateDeploymentStatusRequest, (), DeploymentStatus>::builder(&self.config)
      .post(url)
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
  ) -> Request<(), ReposGetViewsQuery, ViewTraffic> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/traffic/views");

    Request::<(), ReposGetViewsQuery, ViewTraffic>::builder(&self.config)
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
  ) -> Request<(), ReposGetReadmeInDirectoryQuery, ContentFile> {
    let owner = owner.into();
    let repo = repo.into();
    let dir = dir.into();
    let url = format!("/repos/{owner}/{repo}/readme/{dir}");

    Request::<(), ReposGetReadmeInDirectoryQuery, ContentFile>::builder(&self.config)
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
  ) -> Request<(), (), DeploymentProtectionRule> {
    let owner = owner.into();
    let repo = repo.into();
    let environment_name = environment_name.into();
    let protection_rule_id = protection_rule_id.into();
    let url = format!("/repos/{owner}/{repo}/environments/{environment_name}/deployment_protection_rules/{protection_rule_id}");

    Request::<(), (), DeploymentProtectionRule>::builder(&self.config)
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
  ) -> Request<(), (), ProtectedBranchPullRequestReview> {
    let owner = owner.into();
    let repo = repo.into();
    let branch = branch.into();
    let url =
      format!("/repos/{owner}/{repo}/branches/{branch}/protection/required_pull_request_reviews");

    Request::<(), (), ProtectedBranchPullRequestReview>::builder(&self.config)
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
  ) -> Request<ReposUpdatePullRequestReviewProtectionRequest, (), ProtectedBranchPullRequestReview>
  {
    let owner = owner.into();
    let repo = repo.into();
    let branch = branch.into();
    let url =
      format!("/repos/{owner}/{repo}/branches/{branch}/protection/required_pull_request_reviews");

    Request::<ReposUpdatePullRequestReviewProtectionRequest, (), ProtectedBranchPullRequestReview>::builder(&self.config)
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

  /// **Get repository clones**
  ///
  /// Get the total number of clones and breakdown per day or week for the last 14 days. Timestamps are aligned to UTC midnight of the beginning of the day or week. Week begins on Monday.
  ///
  /// *Documentation*: [https://docs.github.com/rest/metrics/traffic#get-repository-clones](https://docs.github.com/rest/metrics/traffic#get-repository-clones)
  pub fn get_clones(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), ReposGetClonesQuery, CloneTraffic> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/traffic/clones");

    Request::<(), ReposGetClonesQuery, CloneTraffic>::builder(&self.config)
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
  ) -> Request<(), (), BranchWithProtection> {
    let owner = owner.into();
    let repo = repo.into();
    let branch = branch.into();
    let url = format!("/repos/{owner}/{repo}/branches/{branch}");

    Request::<(), (), BranchWithProtection>::builder(&self.config)
      .get(url)
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
  ) -> Request<(), ReposGetReadmeQuery, ContentFile> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/readme");

    Request::<(), ReposGetReadmeQuery, ContentFile>::builder(&self.config)
      .get(url)
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
  ) -> Request<(), (), FullRepository> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}");

    Request::<(), (), FullRepository>::builder(&self.config)
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
  ) -> Request<ReposUpdateRequest, (), FullRepository> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}");

    Request::<ReposUpdateRequest, (), FullRepository>::builder(&self.config)
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
  ) -> Request<(), (), ReleaseAsset> {
    let owner = owner.into();
    let repo = repo.into();
    let asset_id = asset_id.into();
    let url = format!("/repos/{owner}/{repo}/releases/assets/{asset_id}");

    Request::<(), (), ReleaseAsset>::builder(&self.config)
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
  ) -> Request<ReposUpdateReleaseAssetRequest, (), ReleaseAsset> {
    let owner = owner.into();
    let repo = repo.into();
    let asset_id = asset_id.into();
    let url = format!("/repos/{owner}/{repo}/releases/assets/{asset_id}");

    Request::<ReposUpdateReleaseAssetRequest, (), ReleaseAsset>::builder(&self.config)
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
  ) -> Request<(), (), ReposGetAllDeploymentProtectionRulesResponse> {
    let environment_name = environment_name.into();
    let repo = repo.into();
    let owner = owner.into();
    let url =
      format!("/repos/{owner}/{repo}/environments/{environment_name}/deployment_protection_rules");

    Request::<(), (), ReposGetAllDeploymentProtectionRulesResponse>::builder(&self.config)
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
  ) -> Request<ReposCreateDeploymentProtectionRuleRequest, (), DeploymentProtectionRule> {
    let environment_name = environment_name.into();
    let repo = repo.into();
    let owner = owner.into();
    let url =
      format!("/repos/{owner}/{repo}/environments/{environment_name}/deployment_protection_rules");

    Request::<ReposCreateDeploymentProtectionRuleRequest, (), DeploymentProtectionRule>::builder(
      &self.config,
    )
    .post(url)
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
  ) -> Request<(), (), ContributorActivityArray> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/stats/contributors");

    Request::<(), (), ContributorActivityArray>::builder(&self.config)
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
  ) -> Request<(), ReposGetAllTopicsQuery, Topic> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/topics");

    Request::<(), ReposGetAllTopicsQuery, Topic>::builder(&self.config)
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
  ) -> Request<ReposReplaceAllTopicsRequest, (), Topic> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/topics");

    Request::<ReposReplaceAllTopicsRequest, (), Topic>::builder(&self.config)
      .put(url)
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
  ) -> Request<(), (), Deployment> {
    let owner = owner.into();
    let repo = repo.into();
    let deployment_id = deployment_id.into();
    let url = format!("/repos/{owner}/{repo}/deployments/{deployment_id}");

    Request::<(), (), Deployment>::builder(&self.config)
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
  ) -> Request<(), (), GitHubAppArray> {
    let owner = owner.into();
    let repo = repo.into();
    let branch = branch.into();
    let url = format!("/repos/{owner}/{repo}/branches/{branch}/protection/restrictions/apps");

    Request::<(), (), GitHubAppArray>::builder(&self.config)
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
  ) -> Request<ReposAddAppAccessRestrictionsRequest, (), GitHubAppArray> {
    let owner = owner.into();
    let repo = repo.into();
    let branch = branch.into();
    let url = format!("/repos/{owner}/{repo}/branches/{branch}/protection/restrictions/apps");

    Request::<ReposAddAppAccessRestrictionsRequest, (), GitHubAppArray>::builder(&self.config)
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
  ) -> Request<ReposSetAppAccessRestrictionsRequest, (), GitHubAppArray> {
    let owner = owner.into();
    let repo = repo.into();
    let branch = branch.into();
    let url = format!("/repos/{owner}/{repo}/branches/{branch}/protection/restrictions/apps");

    Request::<ReposSetAppAccessRestrictionsRequest, (), GitHubAppArray>::builder(&self.config)
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
  ) -> Request<ReposRemoveAppAccessRestrictionsRequest, (), GitHubAppArray> {
    let owner = owner.into();
    let repo = repo.into();
    let branch = branch.into();
    let url = format!("/repos/{owner}/{repo}/branches/{branch}/protection/restrictions/apps");

    Request::<ReposRemoveAppAccessRestrictionsRequest, (), GitHubAppArray>::builder(&self.config)
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
  ) -> Request<(), (), CustomPropertyValueArray> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/properties/values");

    Request::<(), (), CustomPropertyValueArray>::builder(&self.config)
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
  ) -> NoContentRequest<ReposCreateOrUpdateCustomPropertiesValuesRequest, ()> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/properties/values");

    NoContentRequest::<ReposCreateOrUpdateCustomPropertiesValuesRequest, ()>::builder(&self.config)
      .patch(url)
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
  ) -> Request<(), (), ReposCheckPrivateVulnerabilityReportingResponse> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/private-vulnerability-reporting");

    Request::<(), (), ReposCheckPrivateVulnerabilityReportingResponse>::builder(&self.config)
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
  ) -> Request<(), ReposListContributorsQuery, ContributorArray> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/contributors");

    Request::<(), ReposListContributorsQuery, ContributorArray>::builder(&self.config)
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
  ) -> Request<ReposCreateCommitStatusRequest, (), Status> {
    let owner = owner.into();
    let repo = repo.into();
    let sha = sha.into();
    let url = format!("/repos/{owner}/{repo}/statuses/{sha}");

    Request::<ReposCreateCommitStatusRequest, (), Status>::builder(&self.config)
      .post(url)
      .build()
  }
}
