use octocrate_core::*;
#[allow(unused_imports)]
use octocrate_types::*;

/// Interact with GitHub Orgs.
pub struct GitHubOrgsAPI {
  config: SharedAPIConfig,
}

impl GitHubOrgsAPI {
  pub fn new(config: &SharedAPIConfig) -> Self {
    Self {
      config: config.clone(),
    }
  }

  /// **List organizations**
  ///
  /// Lists all organizations, in the order that they were created.
  ///
  /// **Note:** Pagination is powered exclusively by the `since` parameter. Use the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers) to get the URL for the next page of organizations.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/orgs#list-organizations](https://docs.github.com/rest/orgs/orgs#list-organizations)
  pub fn list(&self) -> Request<(), OrgsListQuery, Vec<OrganizationSimple>> {
    let url = format!("/organizations");

    Request::<(), OrgsListQuery, Vec<OrganizationSimple>>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get an organization**
  ///
  /// Gets information about an organization.
  ///
  /// When the value of `two_factor_requirement_enabled` is `true`, the organization requires all members, billing managers, and outside collaborators to enable [two-factor authentication](https://docs.github.com/articles/securing-your-account-with-two-factor-authentication-2fa/).
  ///
  /// To see the full details about an organization, the authenticated user must be an organization owner.
  ///
  /// The values returned by this endpoint are set by the "Update an organization" endpoint. If your organization set a default security configuration (beta), the following values retrieved from the "Update an organization" endpoint have been overwritten by that configuration:
  ///
  /// - advanced_security_enabled_for_new_repositories
  /// - dependabot_alerts_enabled_for_new_repositories
  /// - dependabot_security_updates_enabled_for_new_repositories
  /// - dependency_graph_enabled_for_new_repositories
  /// - secret_scanning_enabled_for_new_repositories
  /// - secret_scanning_push_protection_enabled_for_new_repositories
  ///
  /// For more information on security configurations, see "[Enabling security features at scale](https://docs.github.com/code-security/securing-your-organization/introduction-to-securing-your-organization-at-scale/about-enabling-security-features-at-scale)."
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to see the full details about an organization.
  ///
  /// To see information about an organization's GitHub plan, GitHub Apps need the `Organization plan` permission.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/orgs#get-an-organization](https://docs.github.com/rest/orgs/orgs#get-an-organization)
  pub fn get(&self, org: impl Into<String>) -> Request<(), (), OrganizationFull> {
    let org = org.into();
    let url = format!("/orgs/{org}");

    Request::<(), (), OrganizationFull>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Update an organization**
  ///
  /// **Parameter Deprecation Notice:** GitHub will replace and discontinue `members_allowed_repository_creation_type` in favor of more granular permissions. The new input parameters are `members_can_create_public_repositories`, `members_can_create_private_repositories` for all organizations and `members_can_create_internal_repositories` for organizations associated with an enterprise account using GitHub Enterprise Cloud or GitHub Enterprise Server 2.20+. For more information, see the [blog post](https://developer.github.com/changes/2019-12-03-internal-visibility-changes).
  ///
  /// Updates the organization's profile and member privileges.
  ///
  /// With security configurations (beta), your organization can choose a default security configuration which will automatically apply a set of security enablement settings to new repositories in your organization based on their visibility. For targeted repositories, the following attributes will be overridden by the default security configuration:
  ///
  /// - advanced_security_enabled_for_new_repositories
  /// - dependabot_alerts_enabled_for_new_repositories
  /// - dependabot_security_updates_enabled_for_new_repositories
  /// - dependency_graph_enabled_for_new_repositories
  /// - secret_scanning_enabled_for_new_repositories
  /// - secret_scanning_push_protection_enabled_for_new_repositories
  ///
  /// For more information on setting a default security configuration, see "[Enabling security features at scale](https://docs.github.com/code-security/securing-your-organization/introduction-to-securing-your-organization-at-scale/about-enabling-security-features-at-scale)."
  ///
  /// The authenticated user must be an organization owner to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` or `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/orgs#update-an-organization](https://docs.github.com/rest/orgs/orgs#update-an-organization)
  pub fn update(&self, org: impl Into<String>) -> Request<OrgsUpdateRequest, (), OrganizationFull> {
    let org = org.into();
    let url = format!("/orgs/{org}");

    Request::<OrgsUpdateRequest, (), OrganizationFull>::builder(&self.config)
      .patch(url)
      .build()
  }

  /// **Delete an organization**
  ///
  /// Deletes an organization and all its repositories.
  ///
  /// The organization login will be unavailable for 90 days after deletion.
  ///
  /// Please review the Terms of Service regarding account deletion before using this endpoint:
  ///
  /// https://docs.github.com/site-policy/github-terms/github-terms-of-service
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/orgs#delete-an-organization](https://docs.github.com/rest/orgs/orgs#delete-an-organization)
  pub fn delete(&self, org: impl Into<String>) -> NoContentRequest<(), ()> {
    let org = org.into();
    let url = format!("/orgs/{org}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List users blocked by an organization**
  ///
  /// List the users blocked by an organization.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/blocking#list-users-blocked-by-an-organization](https://docs.github.com/rest/orgs/blocking#list-users-blocked-by-an-organization)
  pub fn list_blocked_users(
    &self,
    org: impl Into<String>,
  ) -> Request<(), OrgsListBlockedUsersQuery, Vec<SimpleUser>> {
    let org = org.into();
    let url = format!("/orgs/{org}/blocks");

    Request::<(), OrgsListBlockedUsersQuery, Vec<SimpleUser>>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Check if a user is blocked by an organization**
  ///
  /// Returns a 204 if the given user is blocked by the given organization. Returns a 404 if the organization is not blocking the user, or if the user account has been identified as spam by GitHub.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/blocking#check-if-a-user-is-blocked-by-an-organization](https://docs.github.com/rest/orgs/blocking#check-if-a-user-is-blocked-by-an-organization)
  pub fn check_blocked_user(
    &self,
    org: impl Into<String>,
    username: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let username = username.into();
    let url = format!("/orgs/{org}/blocks/{username}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Block a user from an organization**
  ///
  /// Blocks the given user on behalf of the specified organization and returns a 204. If the organization cannot block the given user a 422 is returned.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/blocking#block-a-user-from-an-organization](https://docs.github.com/rest/orgs/blocking#block-a-user-from-an-organization)
  pub fn block_user(
    &self,
    org: impl Into<String>,
    username: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let username = username.into();
    let url = format!("/orgs/{org}/blocks/{username}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **Unblock a user from an organization**
  ///
  /// Unblocks the given user on behalf of the specified organization.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/blocking#unblock-a-user-from-an-organization](https://docs.github.com/rest/orgs/blocking#unblock-a-user-from-an-organization)
  pub fn unblock_user(
    &self,
    org: impl Into<String>,
    username: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let username = username.into();
    let url = format!("/orgs/{org}/blocks/{username}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List failed organization invitations**
  ///
  /// The return hash contains `failed_at` and `failed_reason` fields which represent the time at which the invitation failed and the reason for the failure.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/members#list-failed-organization-invitations](https://docs.github.com/rest/orgs/members#list-failed-organization-invitations)
  pub fn list_failed_invitations(
    &self,
    org: impl Into<String>,
  ) -> Request<(), OrgsListFailedInvitationsQuery, Vec<OrganizationInvitation>> {
    let org = org.into();
    let url = format!("/orgs/{org}/failed_invitations");

    Request::<(), OrgsListFailedInvitationsQuery, Vec<OrganizationInvitation>>::builder(
      &self.config,
    )
    .get(url)
    .build()
  }

  /// **List organization webhooks**
  ///
  /// You must be an organization owner to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need `admin:org_hook` scope. OAuth apps cannot list, view, or edit
  /// webhooks that they did not create and users cannot list, view, or edit webhooks that were created by OAuth apps.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/webhooks#list-organization-webhooks](https://docs.github.com/rest/orgs/webhooks#list-organization-webhooks)
  pub fn list_webhooks(
    &self,
    org: impl Into<String>,
  ) -> Request<(), OrgsListWebhooksQuery, Vec<OrgHook>> {
    let org = org.into();
    let url = format!("/orgs/{org}/hooks");

    Request::<(), OrgsListWebhooksQuery, Vec<OrgHook>>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create an organization webhook**
  ///
  /// Create a hook that posts payloads in JSON format.
  ///
  /// You must be an organization owner to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need `admin:org_hook` scope. OAuth apps cannot list, view, or
  /// edit webhooks that they did not create and users cannot list, view, or edit webhooks that were created by OAuth apps.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/webhooks#create-an-organization-webhook](https://docs.github.com/rest/orgs/webhooks#create-an-organization-webhook)
  pub fn create_webhook(
    &self,
    org: impl Into<String>,
  ) -> Request<OrgsCreateWebhookRequest, (), OrgHook> {
    let org = org.into();
    let url = format!("/orgs/{org}/hooks");

    Request::<OrgsCreateWebhookRequest, (), OrgHook>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Get an organization webhook**
  ///
  /// Returns a webhook configured in an organization. To get only the webhook
  /// `config` properties, see "[Get a webhook configuration for an organization](/rest/orgs/webhooks#get-a-webhook-configuration-for-an-organization).
  ///
  /// You must be an organization owner to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need `admin:org_hook` scope. OAuth apps cannot list, view, or edit
  /// webhooks that they did not create and users cannot list, view, or edit webhooks that were created by OAuth apps.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/webhooks#get-an-organization-webhook](https://docs.github.com/rest/orgs/webhooks#get-an-organization-webhook)
  pub fn get_webhook(
    &self,
    org: impl Into<String>,
    hook_id: impl Into<i64>,
  ) -> Request<(), (), OrgHook> {
    let org = org.into();
    let hook_id = hook_id.into();
    let url = format!("/orgs/{org}/hooks/{hook_id}");

    Request::<(), (), OrgHook>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Update an organization webhook**
  ///
  /// Updates a webhook configured in an organization. When you update a webhook,
  /// the `secret` will be overwritten. If you previously had a `secret` set, you must
  /// provide the same `secret` or set a new `secret` or the secret will be removed. If
  /// you are only updating individual webhook `config` properties, use "[Update a webhook
  /// configuration for an organization](/rest/orgs/webhooks#update-a-webhook-configuration-for-an-organization)".
  ///
  /// You must be an organization owner to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need `admin:org_hook` scope. OAuth apps cannot list, view, or edit
  /// webhooks that they did not create and users cannot list, view, or edit webhooks that were created by OAuth apps.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/webhooks#update-an-organization-webhook](https://docs.github.com/rest/orgs/webhooks#update-an-organization-webhook)
  pub fn update_webhook(
    &self,
    org: impl Into<String>,
    hook_id: impl Into<i64>,
  ) -> Request<OrgsUpdateWebhookRequest, (), OrgHook> {
    let org = org.into();
    let hook_id = hook_id.into();
    let url = format!("/orgs/{org}/hooks/{hook_id}");

    Request::<OrgsUpdateWebhookRequest, (), OrgHook>::builder(&self.config)
      .patch(url)
      .build()
  }

  /// **Delete an organization webhook**
  ///
  /// You must be an organization owner to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need `admin:org_hook` scope. OAuth apps cannot list, view, or edit
  /// webhooks that they did not create and users cannot list, view, or edit webhooks that were created by OAuth apps.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/webhooks#delete-an-organization-webhook](https://docs.github.com/rest/orgs/webhooks#delete-an-organization-webhook)
  pub fn delete_webhook(
    &self,
    org: impl Into<String>,
    hook_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let hook_id = hook_id.into();
    let url = format!("/orgs/{org}/hooks/{hook_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Get a webhook configuration for an organization**
  ///
  /// Returns the webhook configuration for an organization. To get more information about the webhook, including the `active` state and `events`, use "[Get an organization webhook ](/rest/orgs/webhooks#get-an-organization-webhook)."
  ///
  /// You must be an organization owner to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need `admin:org_hook` scope. OAuth apps cannot list, view, or edit
  /// webhooks that they did not create and users cannot list, view, or edit webhooks that were created by OAuth apps.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/webhooks#get-a-webhook-configuration-for-an-organization](https://docs.github.com/rest/orgs/webhooks#get-a-webhook-configuration-for-an-organization)
  pub fn get_webhook_config_for_org(
    &self,
    org: impl Into<String>,
    hook_id: impl Into<i64>,
  ) -> Request<(), (), WebhookConfig> {
    let org = org.into();
    let hook_id = hook_id.into();
    let url = format!("/orgs/{org}/hooks/{hook_id}/config");

    Request::<(), (), WebhookConfig>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Update a webhook configuration for an organization**
  ///
  /// Updates the webhook configuration for an organization. To update more information about the webhook, including the `active` state and `events`, use "[Update an organization webhook ](/rest/orgs/webhooks#update-an-organization-webhook)."
  ///
  /// You must be an organization owner to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need `admin:org_hook` scope. OAuth apps cannot list, view, or edit
  /// webhooks that they did not create and users cannot list, view, or edit webhooks that were created by OAuth apps.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/webhooks#update-a-webhook-configuration-for-an-organization](https://docs.github.com/rest/orgs/webhooks#update-a-webhook-configuration-for-an-organization)
  pub fn update_webhook_config_for_org(
    &self,
    org: impl Into<String>,
    hook_id: impl Into<i64>,
  ) -> Request<OrgsUpdateWebhookConfigForOrgRequest, (), WebhookConfig> {
    let org = org.into();
    let hook_id = hook_id.into();
    let url = format!("/orgs/{org}/hooks/{hook_id}/config");

    Request::<OrgsUpdateWebhookConfigForOrgRequest, (), WebhookConfig>::builder(&self.config)
      .patch(url)
      .build()
  }

  /// **List deliveries for an organization webhook**
  ///
  /// Returns a list of webhook deliveries for a webhook configured in an organization.
  ///
  /// You must be an organization owner to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need `admin:org_hook` scope. OAuth apps cannot list, view, or edit
  /// webhooks that they did not create and users cannot list, view, or edit webhooks that were created by OAuth apps.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/webhooks#list-deliveries-for-an-organization-webhook](https://docs.github.com/rest/orgs/webhooks#list-deliveries-for-an-organization-webhook)
  pub fn list_webhook_deliveries(
    &self,
    org: impl Into<String>,
    hook_id: impl Into<i64>,
  ) -> Request<(), OrgsListWebhookDeliveriesQuery, Vec<HookDeliveryItem>> {
    let org = org.into();
    let hook_id = hook_id.into();
    let url = format!("/orgs/{org}/hooks/{hook_id}/deliveries");

    Request::<(), OrgsListWebhookDeliveriesQuery, Vec<HookDeliveryItem>>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get a webhook delivery for an organization webhook**
  ///
  /// Returns a delivery for a webhook configured in an organization.
  ///
  /// You must be an organization owner to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need `admin:org_hook` scope. OAuth apps cannot list, view, or edit
  /// webhooks that they did not create and users cannot list, view, or edit webhooks that were created by OAuth apps.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/webhooks#get-a-webhook-delivery-for-an-organization-webhook](https://docs.github.com/rest/orgs/webhooks#get-a-webhook-delivery-for-an-organization-webhook)
  pub fn get_webhook_delivery(
    &self,
    org: impl Into<String>,
    hook_id: impl Into<i64>,
    delivery_id: impl Into<i64>,
  ) -> Request<(), (), HookDelivery> {
    let org = org.into();
    let hook_id = hook_id.into();
    let delivery_id = delivery_id.into();
    let url = format!("/orgs/{org}/hooks/{hook_id}/deliveries/{delivery_id}");

    Request::<(), (), HookDelivery>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Redeliver a delivery for an organization webhook**
  ///
  /// Redeliver a delivery for a webhook configured in an organization.
  ///
  /// You must be an organization owner to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need `admin:org_hook` scope. OAuth apps cannot list, view, or edit
  /// webhooks that they did not create and users cannot list, view, or edit webhooks that were created by OAuth apps.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/webhooks#redeliver-a-delivery-for-an-organization-webhook](https://docs.github.com/rest/orgs/webhooks#redeliver-a-delivery-for-an-organization-webhook)
  pub fn redeliver_webhook_delivery(
    &self,
    org: impl Into<String>,
    hook_id: impl Into<i64>,
    delivery_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let hook_id = hook_id.into();
    let delivery_id = delivery_id.into();
    let url = format!("/orgs/{org}/hooks/{hook_id}/deliveries/{delivery_id}/attempts");

    NoContentRequest::<(), ()>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Ping an organization webhook**
  ///
  /// This will trigger a [ping event](https://docs.github.com/webhooks/#ping-event)
  /// to be sent to the hook.
  ///
  /// You must be an organization owner to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need `admin:org_hook` scope. OAuth apps cannot list, view, or edit
  /// webhooks that they did not create and users cannot list, view, or edit webhooks that were created by OAuth apps.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/webhooks#ping-an-organization-webhook](https://docs.github.com/rest/orgs/webhooks#ping-an-organization-webhook)
  pub fn ping_webhook(
    &self,
    org: impl Into<String>,
    hook_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let hook_id = hook_id.into();
    let url = format!("/orgs/{org}/hooks/{hook_id}/pings");

    NoContentRequest::<(), ()>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **List app installations for an organization**
  ///
  /// Lists all GitHub Apps in an organization. The installation count includes
  /// all GitHub Apps installed on repositories in the organization.
  ///
  /// The authenticated user must be an organization owner to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:read` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/orgs#list-app-installations-for-an-organization](https://docs.github.com/rest/orgs/orgs#list-app-installations-for-an-organization)
  pub fn list_app_installations(
    &self,
    org: impl Into<String>,
  ) -> Request<(), OrgsListAppInstallationsQuery, OrgsListAppInstallationsResponse> {
    let org = org.into();
    let url = format!("/orgs/{org}/installations");

    Request::<(), OrgsListAppInstallationsQuery, OrgsListAppInstallationsResponse>::builder(
      &self.config,
    )
    .get(url)
    .build()
  }

  /// **List pending organization invitations**
  ///
  /// The return hash contains a `role` field which refers to the Organization Invitation role and will be one of the following values: `direct_member`, `admin`, `billing_manager`, or `hiring_manager`. If the invitee is not a GitHub member, the `login` field in the return hash will be `null`.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/members#list-pending-organization-invitations](https://docs.github.com/rest/orgs/members#list-pending-organization-invitations)
  pub fn list_pending_invitations(
    &self,
    org: impl Into<String>,
  ) -> Request<(), OrgsListPendingInvitationsQuery, Vec<OrganizationInvitation>> {
    let org = org.into();
    let url = format!("/orgs/{org}/invitations");

    Request::<(), OrgsListPendingInvitationsQuery, Vec<OrganizationInvitation>>::builder(
      &self.config,
    )
    .get(url)
    .build()
  }

  /// **Create an organization invitation**
  ///
  /// Invite people to an organization by using their GitHub user ID or their email address. In order to create invitations in an organization, the authenticated user must be an organization owner.
  ///
  /// This endpoint triggers [notifications](https://docs.github.com/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. For more information, see "[Rate limits for the API](https://docs.github.com/rest/overview/rate-limits-for-the-rest-api#about-secondary-rate-limits)"
  /// and "[Best practices for using the REST API](https://docs.github.com/rest/guides/best-practices-for-using-the-rest-api)."
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/members#create-an-organization-invitation](https://docs.github.com/rest/orgs/members#create-an-organization-invitation)
  pub fn create_invitation(
    &self,
    org: impl Into<String>,
  ) -> Request<OrgsCreateInvitationRequest, (), OrganizationInvitation> {
    let org = org.into();
    let url = format!("/orgs/{org}/invitations");

    Request::<OrgsCreateInvitationRequest, (), OrganizationInvitation>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Cancel an organization invitation**
  ///
  /// Cancel an organization invitation. In order to cancel an organization invitation, the authenticated user must be an organization owner.
  ///
  /// This endpoint triggers [notifications](https://docs.github.com/github/managing-subscriptions-and-notifications-on-github/about-notifications).
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/members#cancel-an-organization-invitation](https://docs.github.com/rest/orgs/members#cancel-an-organization-invitation)
  pub fn cancel_invitation(
    &self,
    org: impl Into<String>,
    invitation_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let invitation_id = invitation_id.into();
    let url = format!("/orgs/{org}/invitations/{invitation_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List organization invitation teams**
  ///
  /// List all teams associated with an invitation. In order to see invitations in an organization, the authenticated user must be an organization owner.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/members#list-organization-invitation-teams](https://docs.github.com/rest/orgs/members#list-organization-invitation-teams)
  pub fn list_invitation_teams(
    &self,
    org: impl Into<String>,
    invitation_id: impl Into<i64>,
  ) -> Request<(), OrgsListInvitationTeamsQuery, Vec<Team>> {
    let org = org.into();
    let invitation_id = invitation_id.into();
    let url = format!("/orgs/{org}/invitations/{invitation_id}/teams");

    Request::<(), OrgsListInvitationTeamsQuery, Vec<Team>>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List organization members**
  ///
  /// List all users who are members of an organization. If the authenticated user is also a member of this organization then both concealed and public members will be returned.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/members#list-organization-members](https://docs.github.com/rest/orgs/members#list-organization-members)
  pub fn list_members(
    &self,
    org: impl Into<String>,
  ) -> Request<(), OrgsListMembersQuery, Vec<SimpleUser>> {
    let org = org.into();
    let url = format!("/orgs/{org}/members");

    Request::<(), OrgsListMembersQuery, Vec<SimpleUser>>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Check organization membership for a user**
  ///
  /// Check if a user is, publicly or privately, a member of the organization.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/members#check-organization-membership-for-a-user](https://docs.github.com/rest/orgs/members#check-organization-membership-for-a-user)
  pub fn check_membership_for_user(
    &self,
    org: impl Into<String>,
    username: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let username = username.into();
    let url = format!("/orgs/{org}/members/{username}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Remove an organization member**
  ///
  /// Removing a user from this list will remove them from all teams and they will no longer have any access to the organization's repositories.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/members#remove-an-organization-member](https://docs.github.com/rest/orgs/members#remove-an-organization-member)
  pub fn remove_member(
    &self,
    org: impl Into<String>,
    username: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let username = username.into();
    let url = format!("/orgs/{org}/members/{username}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Get organization membership for a user**
  ///
  /// In order to get a user's membership with an organization, the authenticated user must be an organization member. The `state` parameter in the response can be used to identify the user's membership status.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/members#get-organization-membership-for-a-user](https://docs.github.com/rest/orgs/members#get-organization-membership-for-a-user)
  pub fn get_membership_for_user(
    &self,
    org: impl Into<String>,
    username: impl Into<String>,
  ) -> Request<(), (), OrgMembership> {
    let org = org.into();
    let username = username.into();
    let url = format!("/orgs/{org}/memberships/{username}");

    Request::<(), (), OrgMembership>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Set organization membership for a user**
  ///
  /// Only authenticated organization owners can add a member to the organization or update the member's role.
  ///
  /// *   If the authenticated user is _adding_ a member to the organization, the invited user will receive an email inviting them to the organization. The user's [membership status](https://docs.github.com/rest/orgs/members#get-organization-membership-for-a-user) will be `pending` until they accept the invitation.
  ///     
  /// *   Authenticated users can _update_ a user's membership by passing the `role` parameter. If the authenticated user changes a member's role to `admin`, the affected user will receive an email notifying them that they've been made an organization owner. If the authenticated user changes an owner's role to `member`, no email will be sent.
  ///
  /// **Rate limits**
  ///
  /// To prevent abuse, the authenticated user is limited to 50 organization invitations per 24 hour period. If the organization is more than one month old or on a paid plan, the limit is 500 invitations per 24 hour period.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/members#set-organization-membership-for-a-user](https://docs.github.com/rest/orgs/members#set-organization-membership-for-a-user)
  pub fn set_membership_for_user(
    &self,
    org: impl Into<String>,
    username: impl Into<String>,
  ) -> Request<OrgsSetMembershipForUserRequest, (), OrgMembership> {
    let org = org.into();
    let username = username.into();
    let url = format!("/orgs/{org}/memberships/{username}");

    Request::<OrgsSetMembershipForUserRequest, (), OrgMembership>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **Remove organization membership for a user**
  ///
  /// In order to remove a user's membership with an organization, the authenticated user must be an organization owner.
  ///
  /// If the specified user is an active member of the organization, this will remove them from the organization. If the specified user has been invited to the organization, this will cancel their invitation. The specified user will receive an email notification in both cases.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/members#remove-organization-membership-for-a-user](https://docs.github.com/rest/orgs/members#remove-organization-membership-for-a-user)
  pub fn remove_membership_for_user(
    &self,
    org: impl Into<String>,
    username: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let username = username.into();
    let url = format!("/orgs/{org}/memberships/{username}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List organization fine-grained permissions for an organization**
  ///
  /// Lists the fine-grained permissions that can be used in custom organization roles for an organization. For more information, see "[Managing people's access to your organization with roles](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/about-custom-organization-roles)."
  ///
  /// To list the fine-grained permissions that can be used in custom repository roles for an organization, see "[List repository fine-grained permissions for an organization](https://docs.github.com/rest/orgs/organization-roles#list-repository-fine-grained-permissions-for-an-organization)."
  ///
  /// To use this endpoint, the authenticated user must be one of:
  ///
  /// - An administrator for the organization.
  /// - A user, or a user on a team, with the fine-grained permissions of `read_organization_custom_org_role` in the organization.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/organization-roles#list-organization-fine-grained-permissions-for-an-organization](https://docs.github.com/rest/orgs/organization-roles#list-organization-fine-grained-permissions-for-an-organization)
  pub fn list_organization_fine_grained_permissions(
    &self,
    org: impl Into<String>,
  ) -> Request<(), (), Vec<OrganizationFineGrainedPermission>> {
    let org = org.into();
    let url = format!("/orgs/{org}/organization-fine-grained-permissions");

    Request::<(), (), Vec<OrganizationFineGrainedPermission>>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get all organization roles for an organization**
  ///
  /// Lists the organization roles available in this organization. For more information on organization roles, see "[Managing people's access to your organization with roles](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/about-custom-organization-roles)."
  ///
  /// To use this endpoint, the authenticated user must be one of:
  ///
  /// - An administrator for the organization.
  /// - A user, or a user on a team, with the fine-grained permissions of `read_organization_custom_org_role` in the organization.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/organization-roles#get-all-organization-roles-for-an-organization](https://docs.github.com/rest/orgs/organization-roles#get-all-organization-roles-for-an-organization)
  pub fn list_org_roles(
    &self,
    org: impl Into<String>,
  ) -> Request<(), (), OrgsListOrgRolesResponse> {
    let org = org.into();
    let url = format!("/orgs/{org}/organization-roles");

    Request::<(), (), OrgsListOrgRolesResponse>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create a custom organization role**
  ///
  /// Creates a custom organization role that can be assigned to users and teams, granting them specific permissions over the organization. For more information on custom organization roles, see "[Managing people's access to your organization with roles](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/about-custom-organization-roles)."
  ///
  /// To use this endpoint, the authenticated user must be one of:
  ///
  /// - An administrator for the organization.
  /// - A user, or a user on a team, with the fine-grained permissions of `write_organization_custom_org_role` in the organization.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/organization-roles#create-a-custom-organization-role](https://docs.github.com/rest/orgs/organization-roles#create-a-custom-organization-role)
  pub fn create_custom_organization_role(
    &self,
    org: impl Into<String>,
  ) -> Request<OrgsCreateCustomOrganizationRoleRequest, (), OrganizationRole> {
    let org = org.into();
    let url = format!("/orgs/{org}/organization-roles");

    Request::<OrgsCreateCustomOrganizationRoleRequest, (), OrganizationRole>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Remove all organization roles for a team**
  ///
  /// Removes all assigned organization roles from a team. For more information on organization roles, see "[Managing people's access to your organization with roles](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/about-custom-organization-roles)."
  ///
  /// The authenticated user must be an administrator for the organization to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/organization-roles#remove-all-organization-roles-for-a-team](https://docs.github.com/rest/orgs/organization-roles#remove-all-organization-roles-for-a-team)
  pub fn revoke_all_org_roles_team(
    &self,
    org: impl Into<String>,
    team_slug: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let team_slug = team_slug.into();
    let url = format!("/orgs/{org}/organization-roles/teams/{team_slug}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Assign an organization role to a team**
  ///
  /// Assigns an organization role to a team in an organization. For more information on organization roles, see "[Managing people's access to your organization with roles](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/about-custom-organization-roles)."
  ///
  /// The authenticated user must be an administrator for the organization to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/organization-roles#assign-an-organization-role-to-a-team](https://docs.github.com/rest/orgs/organization-roles#assign-an-organization-role-to-a-team)
  pub fn assign_team_to_org_role(
    &self,
    org: impl Into<String>,
    team_slug: impl Into<String>,
    role_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let team_slug = team_slug.into();
    let role_id = role_id.into();
    let url = format!("/orgs/{org}/organization-roles/teams/{team_slug}/{role_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **Remove an organization role from a team**
  ///
  /// Removes an organization role from a team. For more information on organization roles, see "[Managing people's access to your organization with roles](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/about-custom-organization-roles)."
  ///
  /// The authenticated user must be an administrator for the organization to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/organization-roles#remove-an-organization-role-from-a-team](https://docs.github.com/rest/orgs/organization-roles#remove-an-organization-role-from-a-team)
  pub fn revoke_org_role_team(
    &self,
    org: impl Into<String>,
    team_slug: impl Into<String>,
    role_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let team_slug = team_slug.into();
    let role_id = role_id.into();
    let url = format!("/orgs/{org}/organization-roles/teams/{team_slug}/{role_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Remove all organization roles for a user**
  ///
  /// Revokes all assigned organization roles from a user. For more information on organization roles, see "[Managing people's access to your organization with roles](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/about-custom-organization-roles)."
  ///
  /// The authenticated user must be an administrator for the organization to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/organization-roles#remove-all-organization-roles-for-a-user](https://docs.github.com/rest/orgs/organization-roles#remove-all-organization-roles-for-a-user)
  pub fn revoke_all_org_roles_user(
    &self,
    org: impl Into<String>,
    username: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let username = username.into();
    let url = format!("/orgs/{org}/organization-roles/users/{username}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Assign an organization role to a user**
  ///
  /// Assigns an organization role to a member of an organization. For more information on organization roles, see "[Managing people's access to your organization with roles](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/about-custom-organization-roles)."
  ///
  /// The authenticated user must be an administrator for the organization to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/organization-roles#assign-an-organization-role-to-a-user](https://docs.github.com/rest/orgs/organization-roles#assign-an-organization-role-to-a-user)
  pub fn assign_user_to_org_role(
    &self,
    org: impl Into<String>,
    username: impl Into<String>,
    role_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let username = username.into();
    let role_id = role_id.into();
    let url = format!("/orgs/{org}/organization-roles/users/{username}/{role_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **Remove an organization role from a user**
  ///
  /// Remove an organization role from a user. For more information on organization roles, see "[Managing people's access to your organization with roles](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/about-custom-organization-roles)."
  ///
  /// The authenticated user must be an administrator for the organization to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/organization-roles#remove-an-organization-role-from-a-user](https://docs.github.com/rest/orgs/organization-roles#remove-an-organization-role-from-a-user)
  pub fn revoke_org_role_user(
    &self,
    org: impl Into<String>,
    username: impl Into<String>,
    role_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let username = username.into();
    let role_id = role_id.into();
    let url = format!("/orgs/{org}/organization-roles/users/{username}/{role_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Get an organization role**
  ///
  /// Gets an organization role that is available to this organization. For more information on organization roles, see "[Managing people's access to your organization with roles](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/about-custom-organization-roles)."
  ///
  /// To use this endpoint, the authenticated user must be one of:
  ///
  /// - An administrator for the organization.
  /// - A user, or a user on a team, with the fine-grained permissions of `read_organization_custom_org_role` in the organization.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/organization-roles#get-an-organization-role](https://docs.github.com/rest/orgs/organization-roles#get-an-organization-role)
  pub fn get_org_role(
    &self,
    org: impl Into<String>,
    role_id: impl Into<i64>,
  ) -> Request<(), (), OrganizationRole> {
    let org = org.into();
    let role_id = role_id.into();
    let url = format!("/orgs/{org}/organization-roles/{role_id}");

    Request::<(), (), OrganizationRole>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Update a custom organization role**
  ///
  /// Updates an existing custom organization role. Permission changes will apply to all assignees. For more information on custom organization roles, see "[Managing people's access to your organization with roles](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/about-custom-organization-roles)."
  ///
  ///
  /// To use this endpoint, the authenticated user must be one of:
  ///
  /// - An administrator for the organization.
  /// - A user, or a user on a team, with the fine-grained permissions of `write_organization_custom_org_role` in the organization.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/organization-roles#update-a-custom-organization-role](https://docs.github.com/rest/orgs/organization-roles#update-a-custom-organization-role)
  pub fn patch_custom_organization_role(
    &self,
    org: impl Into<String>,
    role_id: impl Into<i64>,
  ) -> Request<OrgsPatchCustomOrganizationRoleRequest, (), OrganizationRole> {
    let org = org.into();
    let role_id = role_id.into();
    let url = format!("/orgs/{org}/organization-roles/{role_id}");

    Request::<OrgsPatchCustomOrganizationRoleRequest, (), OrganizationRole>::builder(&self.config)
      .patch(url)
      .build()
  }

  /// **Delete a custom organization role.**
  ///
  /// Deletes a custom organization role. For more information on custom organization roles, see "[Managing people's access to your organization with roles](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/about-custom-organization-roles)."
  ///
  /// To use this endpoint, the authenticated user must be one of:
  ///
  /// - An administrator for the organization.
  /// - A user, or a user on a team, with the fine-grained permissions of `write_organization_custom_org_role` in the organization.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/organization-roles#delete-a-custom-organization-role](https://docs.github.com/rest/orgs/organization-roles#delete-a-custom-organization-role)
  pub fn delete_custom_organization_role(
    &self,
    org: impl Into<String>,
    role_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let role_id = role_id.into();
    let url = format!("/orgs/{org}/organization-roles/{role_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List teams that are assigned to an organization role**
  ///
  /// Lists the teams that are assigned to an organization role. For more information on organization roles, see "[Managing people's access to your organization with roles](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/about-custom-organization-roles)."
  ///
  /// To use this endpoint, you must be an administrator for the organization.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/organization-roles#list-teams-that-are-assigned-to-an-organization-role](https://docs.github.com/rest/orgs/organization-roles#list-teams-that-are-assigned-to-an-organization-role)
  pub fn list_org_role_teams(
    &self,
    org: impl Into<String>,
    role_id: impl Into<i64>,
  ) -> Request<(), OrgsListOrgRoleTeamsQuery, Vec<Team>> {
    let org = org.into();
    let role_id = role_id.into();
    let url = format!("/orgs/{org}/organization-roles/{role_id}/teams");

    Request::<(), OrgsListOrgRoleTeamsQuery, Vec<Team>>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List users that are assigned to an organization role**
  ///
  /// Lists organization members that are assigned to an organization role. For more information on organization roles, see "[Managing people's access to your organization with roles](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/about-custom-organization-roles)."
  ///
  /// To use this endpoint, you must be an administrator for the organization.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/organization-roles#list-users-that-are-assigned-to-an-organization-role](https://docs.github.com/rest/orgs/organization-roles#list-users-that-are-assigned-to-an-organization-role)
  pub fn list_org_role_users(
    &self,
    org: impl Into<String>,
    role_id: impl Into<i64>,
  ) -> Request<(), OrgsListOrgRoleUsersQuery, Vec<SimpleUser>> {
    let org = org.into();
    let role_id = role_id.into();
    let url = format!("/orgs/{org}/organization-roles/{role_id}/users");

    Request::<(), OrgsListOrgRoleUsersQuery, Vec<SimpleUser>>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List outside collaborators for an organization**
  ///
  /// List all users who are outside collaborators of an organization.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/outside-collaborators#list-outside-collaborators-for-an-organization](https://docs.github.com/rest/orgs/outside-collaborators#list-outside-collaborators-for-an-organization)
  pub fn list_outside_collaborators(
    &self,
    org: impl Into<String>,
  ) -> Request<(), OrgsListOutsideCollaboratorsQuery, Vec<SimpleUser>> {
    let org = org.into();
    let url = format!("/orgs/{org}/outside_collaborators");

    Request::<(), OrgsListOutsideCollaboratorsQuery, Vec<SimpleUser>>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Convert an organization member to outside collaborator**
  ///
  /// When an organization member is converted to an outside collaborator, they'll only have access to the repositories that their current team membership allows. The user will no longer be a member of the organization. For more information, see "[Converting an organization member to an outside collaborator](https://docs.github.com/articles/converting-an-organization-member-to-an-outside-collaborator/)". Converting an organization member to an outside collaborator may be restricted by enterprise administrators. For more information, see "[Enforcing repository management policies in your enterprise](https://docs.github.com/admin/policies/enforcing-policies-for-your-enterprise/enforcing-repository-management-policies-in-your-enterprise#enforcing-a-policy-for-inviting-outside-collaborators-to-repositories)."
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/outside-collaborators#convert-an-organization-member-to-outside-collaborator](https://docs.github.com/rest/orgs/outside-collaborators#convert-an-organization-member-to-outside-collaborator)
  pub fn convert_member_to_outside_collaborator(
    &self,
    org: impl Into<String>,
    username: impl Into<String>,
  ) -> Request<
    OrgsConvertMemberToOutsideCollaboratorRequest,
    (),
    OrgsConvertMemberToOutsideCollaboratorResponse,
  > {
    let org = org.into();
    let username = username.into();
    let url = format!("/orgs/{org}/outside_collaborators/{username}");

    Request::<
      OrgsConvertMemberToOutsideCollaboratorRequest,
      (),
      OrgsConvertMemberToOutsideCollaboratorResponse,
    >::builder(&self.config)
    .put(url)
    .build()
  }

  /// **Remove outside collaborator from an organization**
  ///
  /// Removing a user from this list will remove them from all the organization's repositories.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/outside-collaborators#remove-outside-collaborator-from-an-organization](https://docs.github.com/rest/orgs/outside-collaborators#remove-outside-collaborator-from-an-organization)
  pub fn remove_outside_collaborator(
    &self,
    org: impl Into<String>,
    username: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let username = username.into();
    let url = format!("/orgs/{org}/outside_collaborators/{username}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List requests to access organization resources with fine-grained personal access tokens**
  ///
  /// Lists requests from organization members to access organization resources with a fine-grained personal access token.
  ///
  /// Only GitHub Apps can use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/personal-access-tokens#list-requests-to-access-organization-resources-with-fine-grained-personal-access-tokens](https://docs.github.com/rest/orgs/personal-access-tokens#list-requests-to-access-organization-resources-with-fine-grained-personal-access-tokens)
  pub fn list_pat_grant_requests(
    &self,
    org: impl Into<String>,
  ) -> Request<(), OrgsListPatGrantRequestsQuery, Vec<OrganizationProgrammaticAccessGrantRequest>>
  {
    let org = org.into();
    let url = format!("/orgs/{org}/personal-access-token-requests");

    Request::<(), OrgsListPatGrantRequestsQuery, Vec<OrganizationProgrammaticAccessGrantRequest>>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Review requests to access organization resources with fine-grained personal access tokens**
  ///
  /// Approves or denies multiple pending requests to access organization resources via a fine-grained personal access token.
  ///
  /// Only GitHub Apps can use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/personal-access-tokens#review-requests-to-access-organization-resources-with-fine-grained-personal-access-tokens](https://docs.github.com/rest/orgs/personal-access-tokens#review-requests-to-access-organization-resources-with-fine-grained-personal-access-tokens)
  pub fn review_pat_grant_requests_in_bulk(
    &self,
    org: impl Into<String>,
  ) -> NoContentRequest<OrgsReviewPatGrantRequestsInBulkRequest, ()> {
    let org = org.into();
    let url = format!("/orgs/{org}/personal-access-token-requests");

    NoContentRequest::<OrgsReviewPatGrantRequestsInBulkRequest, ()>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Review a request to access organization resources with a fine-grained personal access token**
  ///
  /// Approves or denies a pending request to access organization resources via a fine-grained personal access token.
  ///
  /// Only GitHub Apps can use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/personal-access-tokens#review-a-request-to-access-organization-resources-with-a-fine-grained-personal-access-token](https://docs.github.com/rest/orgs/personal-access-tokens#review-a-request-to-access-organization-resources-with-a-fine-grained-personal-access-token)
  pub fn review_pat_grant_request(
    &self,
    org: impl Into<String>,
    pat_request_id: impl Into<i64>,
  ) -> NoContentRequest<OrgsReviewPatGrantRequestRequest, ()> {
    let org = org.into();
    let pat_request_id = pat_request_id.into();
    let url = format!("/orgs/{org}/personal-access-token-requests/{pat_request_id}");

    NoContentRequest::<OrgsReviewPatGrantRequestRequest, ()>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **List repositories requested to be accessed by a fine-grained personal access token**
  ///
  /// Lists the repositories a fine-grained personal access token request is requesting access to.
  ///
  /// Only GitHub Apps can use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/personal-access-tokens#list-repositories-requested-to-be-accessed-by-a-fine-grained-personal-access-token](https://docs.github.com/rest/orgs/personal-access-tokens#list-repositories-requested-to-be-accessed-by-a-fine-grained-personal-access-token)
  pub fn list_pat_grant_request_repositories(
    &self,
    org: impl Into<String>,
    pat_request_id: impl Into<i64>,
  ) -> Request<(), OrgsListPatGrantRequestRepositoriesQuery, Vec<MinimalRepository>> {
    let org = org.into();
    let pat_request_id = pat_request_id.into();
    let url = format!("/orgs/{org}/personal-access-token-requests/{pat_request_id}/repositories");

    Request::<(), OrgsListPatGrantRequestRepositoriesQuery, Vec<MinimalRepository>>::builder(
      &self.config,
    )
    .get(url)
    .build()
  }

  /// **List fine-grained personal access tokens with access to organization resources**
  ///
  /// Lists approved fine-grained personal access tokens owned by organization members that can access organization resources.
  ///
  /// Only GitHub Apps can use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/personal-access-tokens#list-fine-grained-personal-access-tokens-with-access-to-organization-resources](https://docs.github.com/rest/orgs/personal-access-tokens#list-fine-grained-personal-access-tokens-with-access-to-organization-resources)
  pub fn list_pat_grants(
    &self,
    org: impl Into<String>,
  ) -> Request<(), OrgsListPatGrantsQuery, Vec<OrganizationProgrammaticAccessGrant>> {
    let org = org.into();
    let url = format!("/orgs/{org}/personal-access-tokens");

    Request::<(), OrgsListPatGrantsQuery, Vec<OrganizationProgrammaticAccessGrant>>::builder(
      &self.config,
    )
    .get(url)
    .build()
  }

  /// **Update the access to organization resources via fine-grained personal access tokens**
  ///
  /// Updates the access organization members have to organization resources via fine-grained personal access tokens. Limited to revoking a token's existing access.
  ///
  /// Only GitHub Apps can use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/personal-access-tokens#update-the-access-to-organization-resources-via-fine-grained-personal-access-tokens](https://docs.github.com/rest/orgs/personal-access-tokens#update-the-access-to-organization-resources-via-fine-grained-personal-access-tokens)
  pub fn update_pat_accesses(
    &self,
    org: impl Into<String>,
  ) -> NoContentRequest<OrgsUpdatePatAccessesRequest, ()> {
    let org = org.into();
    let url = format!("/orgs/{org}/personal-access-tokens");

    NoContentRequest::<OrgsUpdatePatAccessesRequest, ()>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Update the access a fine-grained personal access token has to organization resources**
  ///
  /// Updates the access an organization member has to organization resources via a fine-grained personal access token. Limited to revoking the token's existing access. Limited to revoking a token's existing access.
  ///
  /// Only GitHub Apps can use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/personal-access-tokens#update-the-access-a-fine-grained-personal-access-token-has-to-organization-resources](https://docs.github.com/rest/orgs/personal-access-tokens#update-the-access-a-fine-grained-personal-access-token-has-to-organization-resources)
  pub fn update_pat_access(
    &self,
    org: impl Into<String>,
    pat_id: impl Into<i64>,
  ) -> NoContentRequest<OrgsUpdatePatAccessRequest, ()> {
    let org = org.into();
    let pat_id = pat_id.into();
    let url = format!("/orgs/{org}/personal-access-tokens/{pat_id}");

    NoContentRequest::<OrgsUpdatePatAccessRequest, ()>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **List repositories a fine-grained personal access token has access to**
  ///
  /// Lists the repositories a fine-grained personal access token has access to.
  ///
  /// Only GitHub Apps can use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/personal-access-tokens#list-repositories-a-fine-grained-personal-access-token-has-access-to](https://docs.github.com/rest/orgs/personal-access-tokens#list-repositories-a-fine-grained-personal-access-token-has-access-to)
  pub fn list_pat_grant_repositories(
    &self,
    org: impl Into<String>,
    pat_id: impl Into<i64>,
  ) -> Request<(), OrgsListPatGrantRepositoriesQuery, Vec<MinimalRepository>> {
    let org = org.into();
    let pat_id = pat_id.into();
    let url = format!("/orgs/{org}/personal-access-tokens/{pat_id}/repositories");

    Request::<(), OrgsListPatGrantRepositoriesQuery, Vec<MinimalRepository>>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get all custom properties for an organization**
  ///
  /// Gets all custom properties defined for an organization.
  /// Organization members can read these properties.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/custom-properties#get-all-custom-properties-for-an-organization](https://docs.github.com/rest/orgs/custom-properties#get-all-custom-properties-for-an-organization)
  pub fn get_all_custom_properties(
    &self,
    org: impl Into<String>,
  ) -> Request<(), (), Vec<OrgCustomProperty>> {
    let org = org.into();
    let url = format!("/orgs/{org}/properties/schema");

    Request::<(), (), Vec<OrgCustomProperty>>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create or update custom properties for an organization**
  ///
  /// Creates new or updates existing custom properties defined for an organization in a batch.
  ///
  /// To use this endpoint, the authenticated user must be one of:
  ///   - An administrator for the organization.
  ///   - A user, or a user on a team, with the fine-grained permission of `custom_properties_org_definitions_manager` in the organization.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/custom-properties#create-or-update-custom-properties-for-an-organization](https://docs.github.com/rest/orgs/custom-properties#create-or-update-custom-properties-for-an-organization)
  pub fn create_or_update_custom_properties(
    &self,
    org: impl Into<String>,
  ) -> Request<OrgsCreateOrUpdateCustomPropertiesRequest, (), Vec<OrgCustomProperty>> {
    let org = org.into();
    let url = format!("/orgs/{org}/properties/schema");

    Request::<OrgsCreateOrUpdateCustomPropertiesRequest, (), Vec<OrgCustomProperty>>::builder(
      &self.config,
    )
    .patch(url)
    .build()
  }

  /// **Get a custom property for an organization**
  ///
  /// Gets a custom property that is defined for an organization.
  /// Organization members can read these properties.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/custom-properties#get-a-custom-property-for-an-organization](https://docs.github.com/rest/orgs/custom-properties#get-a-custom-property-for-an-organization)
  pub fn get_custom_property(
    &self,
    org: impl Into<String>,
    custom_property_name: impl Into<String>,
  ) -> Request<(), (), OrgCustomProperty> {
    let org = org.into();
    let custom_property_name = custom_property_name.into();
    let url = format!("/orgs/{org}/properties/schema/{custom_property_name}");

    Request::<(), (), OrgCustomProperty>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create or update a custom property for an organization**
  ///
  /// Creates a new or updates an existing custom property that is defined for an organization.
  ///
  /// To use this endpoint, the authenticated user must be one of:
  /// - An administrator for the organization.
  /// - A user, or a user on a team, with the fine-grained permission of `custom_properties_org_definitions_manager` in the organization.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/custom-properties#create-or-update-a-custom-property-for-an-organization](https://docs.github.com/rest/orgs/custom-properties#create-or-update-a-custom-property-for-an-organization)
  pub fn create_or_update_custom_property(
    &self,
    org: impl Into<String>,
    custom_property_name: impl Into<String>,
  ) -> Request<OrgsCreateOrUpdateCustomPropertyRequest, (), OrgCustomProperty> {
    let org = org.into();
    let custom_property_name = custom_property_name.into();
    let url = format!("/orgs/{org}/properties/schema/{custom_property_name}");

    Request::<OrgsCreateOrUpdateCustomPropertyRequest, (), OrgCustomProperty>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **Remove a custom property for an organization**
  ///
  /// Removes a custom property that is defined for an organization.
  ///
  /// To use this endpoint, the authenticated user must be one of:
  ///   - An administrator for the organization.
  ///   - A user, or a user on a team, with the fine-grained permission of `custom_properties_org_definitions_manager` in the organization.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/custom-properties#remove-a-custom-property-for-an-organization](https://docs.github.com/rest/orgs/custom-properties#remove-a-custom-property-for-an-organization)
  pub fn remove_custom_property(
    &self,
    org: impl Into<String>,
    custom_property_name: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let custom_property_name = custom_property_name.into();
    let url = format!("/orgs/{org}/properties/schema/{custom_property_name}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List custom property values for organization repositories**
  ///
  /// Lists organization repositories with all of their custom property values.
  /// Organization members can read these properties.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/custom-properties#list-custom-property-values-for-organization-repositories](https://docs.github.com/rest/orgs/custom-properties#list-custom-property-values-for-organization-repositories)
  pub fn list_custom_properties_values_for_repos(
    &self,
    org: impl Into<String>,
  ) -> Request<(), OrgsListCustomPropertiesValuesForReposQuery, Vec<OrgRepoCustomPropertyValues>>
  {
    let org = org.into();
    let url = format!("/orgs/{org}/properties/values");

    Request::<(), OrgsListCustomPropertiesValuesForReposQuery, Vec<OrgRepoCustomPropertyValues>>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create or update custom property values for organization repositories**
  ///
  /// Create new or update existing custom property values for repositories in a batch that belong to an organization.
  /// Each target repository will have its custom property values updated to match the values provided in the request.
  ///
  /// A maximum of 30 repositories can be updated in a single request.
  ///
  /// Using a value of `null` for a custom property will remove or 'unset' the property value from the repository.
  ///
  /// To use this endpoint, the authenticated user must be one of:
  ///   - An administrator for the organization.
  ///   - A user, or a user on a team, with the fine-grained permission of `custom_properties_org_values_editor` in the organization.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/custom-properties#create-or-update-custom-property-values-for-organization-repositories](https://docs.github.com/rest/orgs/custom-properties#create-or-update-custom-property-values-for-organization-repositories)
  pub fn create_or_update_custom_properties_values_for_repos(
    &self,
    org: impl Into<String>,
  ) -> NoContentRequest<OrgsCreateOrUpdateCustomPropertiesValuesForReposRequest, ()> {
    let org = org.into();
    let url = format!("/orgs/{org}/properties/values");

    NoContentRequest::<OrgsCreateOrUpdateCustomPropertiesValuesForReposRequest, ()>::builder(
      &self.config,
    )
    .patch(url)
    .build()
  }

  /// **List public organization members**
  ///
  /// Members of an organization can choose to have their membership publicized or not.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/members#list-public-organization-members](https://docs.github.com/rest/orgs/members#list-public-organization-members)
  pub fn list_public_members(
    &self,
    org: impl Into<String>,
  ) -> Request<(), OrgsListPublicMembersQuery, Vec<SimpleUser>> {
    let org = org.into();
    let url = format!("/orgs/{org}/public_members");

    Request::<(), OrgsListPublicMembersQuery, Vec<SimpleUser>>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Check public organization membership for a user**
  ///
  /// Check if the provided user is a public member of the organization.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/members#check-public-organization-membership-for-a-user](https://docs.github.com/rest/orgs/members#check-public-organization-membership-for-a-user)
  pub fn check_public_membership_for_user(
    &self,
    org: impl Into<String>,
    username: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let username = username.into();
    let url = format!("/orgs/{org}/public_members/{username}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Set public organization membership for the authenticated user**
  ///
  /// The user can publicize their own membership. (A user cannot publicize the membership for another user.)
  ///
  /// Note that you'll need to set `Content-Length` to zero when calling out to this endpoint. For more information, see "[HTTP method](https://docs.github.com/rest/guides/getting-started-with-the-rest-api#http-method)."
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/members#set-public-organization-membership-for-the-authenticated-user](https://docs.github.com/rest/orgs/members#set-public-organization-membership-for-the-authenticated-user)
  pub fn set_public_membership_for_authenticated_user(
    &self,
    org: impl Into<String>,
    username: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let username = username.into();
    let url = format!("/orgs/{org}/public_members/{username}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **Remove public organization membership for the authenticated user**
  ///
  /// Removes the public membership for the authenticated user from the specified organization, unless public visibility is enforced by default.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/members#remove-public-organization-membership-for-the-authenticated-user](https://docs.github.com/rest/orgs/members#remove-public-organization-membership-for-the-authenticated-user)
  pub fn remove_public_membership_for_authenticated_user(
    &self,
    org: impl Into<String>,
    username: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let username = username.into();
    let url = format!("/orgs/{org}/public_members/{username}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List security manager teams**
  ///
  /// Lists teams that are security managers for an organization. For more information, see "[Managing security managers in your organization](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/managing-security-managers-in-your-organization)."
  ///
  /// The authenticated user must be an administrator or security manager for the organization to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `read:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/security-managers#list-security-manager-teams](https://docs.github.com/rest/orgs/security-managers#list-security-manager-teams)
  pub fn list_security_manager_teams(
    &self,
    org: impl Into<String>,
  ) -> Request<(), (), Vec<TeamSimple>> {
    let org = org.into();
    let url = format!("/orgs/{org}/security-managers");

    Request::<(), (), Vec<TeamSimple>>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Add a security manager team**
  ///
  /// Adds a team as a security manager for an organization. For more information, see "[Managing security for an organization](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/managing-security-managers-in-your-organization) for an organization."
  ///
  /// The authenticated user must be an administrator for the organization to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `write:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/security-managers#add-a-security-manager-team](https://docs.github.com/rest/orgs/security-managers#add-a-security-manager-team)
  pub fn add_security_manager_team(
    &self,
    org: impl Into<String>,
    team_slug: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let team_slug = team_slug.into();
    let url = format!("/orgs/{org}/security-managers/teams/{team_slug}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **Remove a security manager team**
  ///
  /// Removes the security manager role from a team for an organization. For more information, see "[Managing security managers in your organization](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/managing-security-managers-in-your-organization) team from an organization."
  ///
  /// The authenticated user must be an administrator for the organization to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/security-managers#remove-a-security-manager-team](https://docs.github.com/rest/orgs/security-managers#remove-a-security-manager-team)
  pub fn remove_security_manager_team(
    &self,
    org: impl Into<String>,
    team_slug: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let team_slug = team_slug.into();
    let url = format!("/orgs/{org}/security-managers/teams/{team_slug}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Enable or disable a security feature for an organization**
  ///
  /// Enables or disables the specified security feature for all eligible repositories in an organization. For more information, see "[Managing security managers in your organization](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/managing-security-managers-in-your-organization)."
  ///
  /// The authenticated user must be an organization owner or be member of a team with the security manager role to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `write:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/orgs#enable-or-disable-a-security-feature-for-an-organization](https://docs.github.com/rest/orgs/orgs#enable-or-disable-a-security-feature-for-an-organization)
  pub fn enable_or_disable_security_product_on_all_org_repos(
    &self,
    org: impl Into<String>,
    security_product: impl Into<
      OrgsEnableOrDisableSecurityProductOnAllOrgReposParametersSecurityProduct,
    >,
    enablement: impl Into<OrgsEnableOrDisableSecurityProductOnAllOrgReposParametersEnablement>,
  ) -> NoContentRequest<OrgsEnableOrDisableSecurityProductOnAllOrgReposRequest, ()> {
    let org = org.into();
    let security_product = security_product.into();
    let enablement = enablement.into();
    let security_product = security_product.to_string();
    let enablement = enablement.to_string();
    let url = format!("/orgs/{org}/{security_product}/{enablement}");

    NoContentRequest::<OrgsEnableOrDisableSecurityProductOnAllOrgReposRequest, ()>::builder(
      &self.config,
    )
    .post(url)
    .build()
  }

  /// **List organization memberships for the authenticated user**
  ///
  /// Lists all of the authenticated user's organization memberships.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/members#list-organization-memberships-for-the-authenticated-user](https://docs.github.com/rest/orgs/members#list-organization-memberships-for-the-authenticated-user)
  pub fn list_memberships_for_authenticated_user(
    &self,
  ) -> Request<(), OrgsListMembershipsForAuthenticatedUserQuery, Vec<OrgMembership>> {
    let url = format!("/user/memberships/orgs");

    Request::<(), OrgsListMembershipsForAuthenticatedUserQuery, Vec<OrgMembership>>::builder(
      &self.config,
    )
    .get(url)
    .build()
  }

  /// **Get an organization membership for the authenticated user**
  ///
  /// If the authenticated user is an active or pending member of the organization, this endpoint will return the user's membership. If the authenticated user is not affiliated with the organization, a `404` is returned. This endpoint will return a `403` if the request is made by a GitHub App that is blocked by the organization.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/members#get-an-organization-membership-for-the-authenticated-user](https://docs.github.com/rest/orgs/members#get-an-organization-membership-for-the-authenticated-user)
  pub fn get_membership_for_authenticated_user(
    &self,
    org: impl Into<String>,
  ) -> Request<(), (), OrgMembership> {
    let org = org.into();
    let url = format!("/user/memberships/orgs/{org}");

    Request::<(), (), OrgMembership>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Update an organization membership for the authenticated user**
  ///
  /// Converts the authenticated user to an active member of the organization, if that user has a pending invitation from the organization.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/members#update-an-organization-membership-for-the-authenticated-user](https://docs.github.com/rest/orgs/members#update-an-organization-membership-for-the-authenticated-user)
  pub fn update_membership_for_authenticated_user(
    &self,
    org: impl Into<String>,
  ) -> Request<OrgsUpdateMembershipForAuthenticatedUserRequest, (), OrgMembership> {
    let org = org.into();
    let url = format!("/user/memberships/orgs/{org}");

    Request::<OrgsUpdateMembershipForAuthenticatedUserRequest, (), OrgMembership>::builder(
      &self.config,
    )
    .patch(url)
    .build()
  }

  /// **List organizations for the authenticated user**
  ///
  /// List organizations for the authenticated user.
  ///
  /// For OAuth app tokens and personal access tokens (classic), this endpoint only lists organizations that your authorization allows you to operate on in some way (e.g., you can list teams with `read:org` scope, you can publicize your organization membership with `user` scope, etc.). Therefore, this API requires at least `user` or `read:org` scope for OAuth app tokens and personal access tokens (classic). Requests with insufficient scope will receive a `403 Forbidden` response.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/orgs#list-organizations-for-the-authenticated-user](https://docs.github.com/rest/orgs/orgs#list-organizations-for-the-authenticated-user)
  pub fn list_for_authenticated_user(
    &self,
  ) -> Request<(), OrgsListForAuthenticatedUserQuery, Vec<OrganizationSimple>> {
    let url = format!("/user/orgs");

    Request::<(), OrgsListForAuthenticatedUserQuery, Vec<OrganizationSimple>>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List organizations for a user**
  ///
  /// List [public organization memberships](https://docs.github.com/articles/publicizing-or-concealing-organization-membership) for the specified user.
  ///
  /// This method only lists _public_ memberships, regardless of authentication. If you need to fetch all of the organization memberships (public and private) for the authenticated user, use the [List organizations for the authenticated user](https://docs.github.com/rest/orgs/orgs#list-organizations-for-the-authenticated-user) API instead.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/orgs#list-organizations-for-a-user](https://docs.github.com/rest/orgs/orgs#list-organizations-for-a-user)
  pub fn list_for_user(
    &self,
    username: impl Into<String>,
  ) -> Request<(), OrgsListForUserQuery, Vec<OrganizationSimple>> {
    let username = username.into();
    let url = format!("/users/{username}/orgs");

    Request::<(), OrgsListForUserQuery, Vec<OrganizationSimple>>::builder(&self.config)
      .get(url)
      .build()
  }
}
