use octocrate_core::*;
#[allow(unused_imports)]
use octocrate_types::*;

/// Information for integrations and installations.
pub struct GitHubAppsAPI {
  config: SharedAPIConfig,
}

impl GitHubAppsAPI {
  pub fn new(config: &SharedAPIConfig) -> Self {
    Self {
      config: config.clone(),
    }
  }

  /// **Get the authenticated app**
  ///
  /// Returns the GitHub App associated with the authentication credentials used. To see how many app installations are associated with this GitHub App, see the `installations_count` in the response. For more details about your app's installations, see the "[List installations for the authenticated app](https://docs.github.com/rest/apps/apps#list-installations-for-the-authenticated-app)" endpoint.
  ///
  /// You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/apps/apps#get-the-authenticated-app](https://docs.github.com/rest/apps/apps#get-the-authenticated-app)
  pub fn get_authenticated(&self) -> Request<(), (), Integration> {
    let url = format!("/app");

    Request::<(), (), Integration>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create a GitHub App from a manifest**
  ///
  /// Use this endpoint to complete the handshake necessary when implementing the [GitHub App Manifest flow](https://docs.github.com/apps/building-github-apps/creating-github-apps-from-a-manifest/). When you create a GitHub App with the manifest flow, you receive a temporary `code` used to retrieve the GitHub App's `id`, `pem` (private key), and `webhook_secret`.
  ///
  /// *Documentation*: [https://docs.github.com/rest/apps/apps#create-a-github-app-from-a-manifest](https://docs.github.com/rest/apps/apps#create-a-github-app-from-a-manifest)
  pub fn create_from_manifest(
    &self,
    code: impl Into<String>,
  ) -> Request<(), (), AppsCreateFromManifestResponse> {
    let code = code.into();
    let url = format!("/app-manifests/{code}/conversions");

    Request::<(), (), AppsCreateFromManifestResponse>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Get a webhook configuration for an app**
  ///
  /// Returns the webhook configuration for a GitHub App. For more information about configuring a webhook for your app, see "[Creating a GitHub App](/developers/apps/creating-a-github-app)."
  ///
  /// You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/apps/webhooks#get-a-webhook-configuration-for-an-app](https://docs.github.com/rest/apps/webhooks#get-a-webhook-configuration-for-an-app)
  pub fn get_webhook_config_for_app(&self) -> Request<(), (), WebhookConfig> {
    let url = format!("/app/hook/config");

    Request::<(), (), WebhookConfig>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Update a webhook configuration for an app**
  ///
  /// Updates the webhook configuration for a GitHub App. For more information about configuring a webhook for your app, see "[Creating a GitHub App](/developers/apps/creating-a-github-app)."
  ///
  /// You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/apps/webhooks#update-a-webhook-configuration-for-an-app](https://docs.github.com/rest/apps/webhooks#update-a-webhook-configuration-for-an-app)
  pub fn update_webhook_config_for_app(
    &self,
  ) -> Request<AppsUpdateWebhookConfigForAppRequest, (), WebhookConfig> {
    let url = format!("/app/hook/config");

    Request::<AppsUpdateWebhookConfigForAppRequest, (), WebhookConfig>::builder(&self.config)
      .patch(url)
      .build()
  }

  /// **List deliveries for an app webhook**
  ///
  /// Returns a list of webhook deliveries for the webhook configured for a GitHub App.
  ///
  /// You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/apps/webhooks#list-deliveries-for-an-app-webhook](https://docs.github.com/rest/apps/webhooks#list-deliveries-for-an-app-webhook)
  pub fn list_webhook_deliveries(
    &self,
  ) -> Request<(), AppsListWebhookDeliveriesQuery, Vec<HookDeliveryItem>> {
    let url = format!("/app/hook/deliveries");

    Request::<(), AppsListWebhookDeliveriesQuery, Vec<HookDeliveryItem>>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get a delivery for an app webhook**
  ///
  /// Returns a delivery for the webhook configured for a GitHub App.
  ///
  /// You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/apps/webhooks#get-a-delivery-for-an-app-webhook](https://docs.github.com/rest/apps/webhooks#get-a-delivery-for-an-app-webhook)
  pub fn get_webhook_delivery(&self, delivery_id: impl Into<i64>) -> Request<(), (), HookDelivery> {
    let delivery_id = delivery_id.into();
    let url = format!("/app/hook/deliveries/{delivery_id}");

    Request::<(), (), HookDelivery>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Redeliver a delivery for an app webhook**
  ///
  /// Redeliver a delivery for the webhook configured for a GitHub App.
  ///
  /// You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/apps/webhooks#redeliver-a-delivery-for-an-app-webhook](https://docs.github.com/rest/apps/webhooks#redeliver-a-delivery-for-an-app-webhook)
  pub fn redeliver_webhook_delivery(
    &self,
    delivery_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let delivery_id = delivery_id.into();
    let url = format!("/app/hook/deliveries/{delivery_id}/attempts");

    NoContentRequest::<(), ()>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **List installation requests for the authenticated app**
  ///
  /// Lists all the pending installation requests for the authenticated GitHub App.
  ///
  /// *Documentation*: [https://docs.github.com/rest/apps/apps#list-installation-requests-for-the-authenticated-app](https://docs.github.com/rest/apps/apps#list-installation-requests-for-the-authenticated-app)
  pub fn list_installation_requests_for_authenticated_app(
    &self,
  ) -> Request<
    (),
    AppsListInstallationRequestsForAuthenticatedAppQuery,
    Vec<IntegrationInstallationRequest>,
  > {
    let url = format!("/app/installation-requests");

    Request::<
      (),
      AppsListInstallationRequestsForAuthenticatedAppQuery,
      Vec<IntegrationInstallationRequest>,
    >::builder(&self.config)
    .get(url)
    .build()
  }

  /// **List installations for the authenticated app**
  ///
  /// The permissions the installation has are included under the `permissions` key.
  ///
  /// You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/apps/apps#list-installations-for-the-authenticated-app](https://docs.github.com/rest/apps/apps#list-installations-for-the-authenticated-app)
  pub fn list_installations(&self) -> Request<(), AppsListInstallationsQuery, Vec<Installation>> {
    let url = format!("/app/installations");

    Request::<(), AppsListInstallationsQuery, Vec<Installation>>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get an installation for the authenticated app**
  ///
  /// Enables an authenticated GitHub App to find an installation's information using the installation id.
  ///
  /// You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/apps/apps#get-an-installation-for-the-authenticated-app](https://docs.github.com/rest/apps/apps#get-an-installation-for-the-authenticated-app)
  pub fn get_installation(&self, installation_id: impl Into<i64>) -> Request<(), (), Installation> {
    let installation_id = installation_id.into();
    let url = format!("/app/installations/{installation_id}");

    Request::<(), (), Installation>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Delete an installation for the authenticated app**
  ///
  /// Uninstalls a GitHub App on a user, organization, or business account. If you prefer to temporarily suspend an app's access to your account's resources, then we recommend the "[Suspend an app installation](https://docs.github.com/rest/apps/apps#suspend-an-app-installation)" endpoint.
  ///
  /// You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/apps/apps#delete-an-installation-for-the-authenticated-app](https://docs.github.com/rest/apps/apps#delete-an-installation-for-the-authenticated-app)
  pub fn delete_installation(&self, installation_id: impl Into<i64>) -> NoContentRequest<(), ()> {
    let installation_id = installation_id.into();
    let url = format!("/app/installations/{installation_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Create an installation access token for an app**
  ///
  /// Creates an installation access token that enables a GitHub App to make authenticated API requests for the app's installation on an organization or individual account. Installation tokens expire one hour from the time you create them. Using an expired token produces a status code of `401 - Unauthorized`, and requires creating a new installation token. By default the installation token has access to all repositories that the installation can access.
  ///
  /// Optionally, you can use the `repositories` or `repository_ids` body parameters to specify individual repositories that the installation access token can access. If you don't use `repositories` or `repository_ids` to grant access to specific repositories, the installation access token will have access to all repositories that the installation was granted access to. The installation access token cannot be granted access to repositories that the installation was not granted access to. Up to 500 repositories can be listed in this manner.
  ///
  /// Optionally, use the `permissions` body parameter to specify the permissions that the installation access token should have. If `permissions` is not specified, the installation access token will have all of the permissions that were granted to the app. The installation access token cannot be granted permissions that the app was not granted.
  ///
  /// When using the repository or permission parameters to reduce the access of the token, the complexity of the token is increased due to both the number of permissions in the request and the number of repositories the token will have access to. If the complexity is too large, the token will fail to be issued. If this occurs, the error message will indicate the maximum number of repositories that should be requested. For the average application requesting 8 permissions, this limit is around 5000 repositories. With fewer permissions requested, more repositories are supported.
  ///
  /// You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/apps/apps#create-an-installation-access-token-for-an-app](https://docs.github.com/rest/apps/apps#create-an-installation-access-token-for-an-app)
  pub fn create_installation_access_token(
    &self,
    installation_id: impl Into<i64>,
  ) -> Request<AppsCreateInstallationAccessTokenRequest, (), InstallationToken> {
    let installation_id = installation_id.into();
    let url = format!("/app/installations/{installation_id}/access_tokens");

    Request::<AppsCreateInstallationAccessTokenRequest, (), InstallationToken>::builder(
      &self.config,
    )
    .post(url)
    .build()
  }

  /// **Suspend an app installation**
  ///
  /// Suspends a GitHub App on a user, organization, or business account, which blocks the app from accessing the account's resources. When a GitHub App is suspended, the app's access to the GitHub API or webhook events is blocked for that account.
  ///
  /// You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/apps/apps#suspend-an-app-installation](https://docs.github.com/rest/apps/apps#suspend-an-app-installation)
  pub fn suspend_installation(&self, installation_id: impl Into<i64>) -> NoContentRequest<(), ()> {
    let installation_id = installation_id.into();
    let url = format!("/app/installations/{installation_id}/suspended");

    NoContentRequest::<(), ()>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **Unsuspend an app installation**
  ///
  /// Removes a GitHub App installation suspension.
  ///
  /// You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/apps/apps#unsuspend-an-app-installation](https://docs.github.com/rest/apps/apps#unsuspend-an-app-installation)
  pub fn unsuspend_installation(
    &self,
    installation_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let installation_id = installation_id.into();
    let url = format!("/app/installations/{installation_id}/suspended");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Delete an app authorization**
  ///
  /// OAuth and GitHub application owners can revoke a grant for their application and a specific user. You must use [Basic Authentication](https://docs.github.com/rest/overview/other-authentication-methods#basic-authentication) when accessing this endpoint, using the OAuth application's `client_id` and `client_secret` as the username and password. You must also provide a valid OAuth `access_token` as an input parameter and the grant for the token's owner will be deleted.
  /// Deleting an application's grant will also delete all OAuth tokens associated with the application for the user. Once deleted, the application will have no access to the user's account and will no longer be listed on [the application authorizations settings screen within GitHub](https://github.com/settings/applications#authorized).
  ///
  /// *Documentation*: [https://docs.github.com/rest/apps/oauth-applications#delete-an-app-authorization](https://docs.github.com/rest/apps/oauth-applications#delete-an-app-authorization)
  pub fn delete_authorization(
    &self,
    client_id: impl Into<String>,
  ) -> NoContentRequest<AppsDeleteAuthorizationRequest, ()> {
    let client_id = client_id.into();
    let url = format!("/applications/{client_id}/grant");

    NoContentRequest::<AppsDeleteAuthorizationRequest, ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Check a token**
  ///
  /// OAuth applications and GitHub applications with OAuth authorizations can use this API method for checking OAuth token validity without exceeding the normal rate limits for failed login attempts. Authentication works differently with this particular endpoint. You must use [Basic Authentication](https://docs.github.com/rest/overview/other-authentication-methods#basic-authentication) to use this endpoint, where the username is the application `client_id` and the password is its `client_secret`. Invalid tokens will return `404 NOT FOUND`.
  ///
  /// *Documentation*: [https://docs.github.com/rest/apps/oauth-applications#check-a-token](https://docs.github.com/rest/apps/oauth-applications#check-a-token)
  pub fn check_token(
    &self,
    client_id: impl Into<String>,
  ) -> Request<AppsCheckTokenRequest, (), Authorization> {
    let client_id = client_id.into();
    let url = format!("/applications/{client_id}/token");

    Request::<AppsCheckTokenRequest, (), Authorization>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Reset a token**
  ///
  /// OAuth applications and GitHub applications with OAuth authorizations can use this API method to reset a valid OAuth token without end-user involvement. Applications must save the "token" property in the response because changes take effect immediately. You must use [Basic Authentication](https://docs.github.com/rest/overview/other-authentication-methods#basic-authentication) when accessing this endpoint, using the application's `client_id` and `client_secret` as the username and password. Invalid tokens will return `404 NOT FOUND`.
  ///
  /// *Documentation*: [https://docs.github.com/rest/apps/oauth-applications#reset-a-token](https://docs.github.com/rest/apps/oauth-applications#reset-a-token)
  pub fn reset_token(
    &self,
    client_id: impl Into<String>,
  ) -> Request<AppsResetTokenRequest, (), Authorization> {
    let client_id = client_id.into();
    let url = format!("/applications/{client_id}/token");

    Request::<AppsResetTokenRequest, (), Authorization>::builder(&self.config)
      .patch(url)
      .build()
  }

  /// **Delete an app token**
  ///
  /// OAuth  or GitHub application owners can revoke a single token for an OAuth application or a GitHub application with an OAuth authorization. You must use [Basic Authentication](https://docs.github.com/rest/overview/other-authentication-methods#basic-authentication) when accessing this endpoint, using the application's `client_id` and `client_secret` as the username and password.
  ///
  /// *Documentation*: [https://docs.github.com/rest/apps/oauth-applications#delete-an-app-token](https://docs.github.com/rest/apps/oauth-applications#delete-an-app-token)
  pub fn delete_token(
    &self,
    client_id: impl Into<String>,
  ) -> NoContentRequest<AppsDeleteTokenRequest, ()> {
    let client_id = client_id.into();
    let url = format!("/applications/{client_id}/token");

    NoContentRequest::<AppsDeleteTokenRequest, ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Create a scoped access token**
  ///
  /// Use a non-scoped user access token to create a repository-scoped and/or permission-scoped user access token. You can specify
  /// which repositories the token can access and which permissions are granted to the
  /// token.
  ///
  /// Invalid tokens will return `404 NOT FOUND`.
  ///
  /// You must use [Basic Authentication](https://docs.github.com/rest/overview/other-authentication-methods#basic-authentication)
  /// when accessing this endpoint, using the `client_id` and `client_secret` of the GitHub App
  /// as the username and password.
  ///
  /// *Documentation*: [https://docs.github.com/rest/apps/apps#create-a-scoped-access-token](https://docs.github.com/rest/apps/apps#create-a-scoped-access-token)
  pub fn scope_token(
    &self,
    client_id: impl Into<String>,
  ) -> Request<AppsScopeTokenRequest, (), Authorization> {
    let client_id = client_id.into();
    let url = format!("/applications/{client_id}/token/scoped");

    Request::<AppsScopeTokenRequest, (), Authorization>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Get an app**
  ///
  /// **Note**: The `:app_slug` is just the URL-friendly name of your GitHub App. You can find this on the settings page for your GitHub App (e.g., `https://github.com/settings/apps/:app_slug`).
  ///
  /// *Documentation*: [https://docs.github.com/rest/apps/apps#get-an-app](https://docs.github.com/rest/apps/apps#get-an-app)
  pub fn get_by_slug(&self, app_slug: impl Into<String>) -> Request<(), (), Integration> {
    let app_slug = app_slug.into();
    let url = format!("/apps/{app_slug}");

    Request::<(), (), Integration>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List repositories accessible to the app installation**
  ///
  /// List repositories that an app installation can access.
  ///
  /// *Documentation*: [https://docs.github.com/rest/apps/installations#list-repositories-accessible-to-the-app-installation](https://docs.github.com/rest/apps/installations#list-repositories-accessible-to-the-app-installation)
  pub fn list_repos_accessible_to_installation(
    &self,
  ) -> Request<
    (),
    AppsListReposAccessibleToInstallationQuery,
    AppsListReposAccessibleToInstallationResponse,
  > {
    let url = format!("/installation/repositories");

    Request::<
      (),
      AppsListReposAccessibleToInstallationQuery,
      AppsListReposAccessibleToInstallationResponse,
    >::builder(&self.config)
    .get(url)
    .build()
  }

  /// **Revoke an installation access token**
  ///
  /// Revokes the installation token you're using to authenticate as an installation and access this endpoint.
  ///
  /// Once an installation token is revoked, the token is invalidated and cannot be used. Other endpoints that require the revoked installation token must have a new installation token to work. You can create a new token using the "[Create an installation access token for an app](https://docs.github.com/rest/apps/apps#create-an-installation-access-token-for-an-app)" endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/apps/installations#revoke-an-installation-access-token](https://docs.github.com/rest/apps/installations#revoke-an-installation-access-token)
  pub fn revoke_installation_access_token(&self) -> NoContentRequest<(), ()> {
    let url = format!("/installation/token");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Get a subscription plan for an account**
  ///
  /// Shows whether the user or organization account actively subscribes to a plan listed by the authenticated GitHub App. When someone submits a plan change that won't be processed until the end of their billing cycle, you will also see the upcoming pending change.
  ///
  /// GitHub Apps must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint. OAuth apps must use [basic authentication](https://docs.github.com/rest/overview/other-authentication-methods#basic-authentication) with their client ID and client secret to access this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/apps/marketplace#get-a-subscription-plan-for-an-account](https://docs.github.com/rest/apps/marketplace#get-a-subscription-plan-for-an-account)
  pub fn get_subscription_plan_for_account(
    &self,
    account_id: impl Into<i64>,
  ) -> Request<(), (), MarketplacePurchase> {
    let account_id = account_id.into();
    let url = format!("/marketplace_listing/accounts/{account_id}");

    Request::<(), (), MarketplacePurchase>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List plans**
  ///
  /// Lists all plans that are part of your GitHub Marketplace listing.
  ///
  /// GitHub Apps must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint. OAuth apps must use [basic authentication](https://docs.github.com/rest/overview/other-authentication-methods#basic-authentication) with their client ID and client secret to access this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/apps/marketplace#list-plans](https://docs.github.com/rest/apps/marketplace#list-plans)
  pub fn list_plans(&self) -> Request<(), AppsListPlansQuery, Vec<MarketplaceListingPlan>> {
    let url = format!("/marketplace_listing/plans");

    Request::<(), AppsListPlansQuery, Vec<MarketplaceListingPlan>>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List accounts for a plan**
  ///
  /// Returns user and organization accounts associated with the specified plan, including free plans. For per-seat pricing, you see the list of accounts that have purchased the plan, including the number of seats purchased. When someone submits a plan change that won't be processed until the end of their billing cycle, you will also see the upcoming pending change.
  ///
  /// GitHub Apps must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint. OAuth apps must use [basic authentication](https://docs.github.com/rest/overview/other-authentication-methods#basic-authentication) with their client ID and client secret to access this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/apps/marketplace#list-accounts-for-a-plan](https://docs.github.com/rest/apps/marketplace#list-accounts-for-a-plan)
  pub fn list_accounts_for_plan(
    &self,
    plan_id: impl Into<i64>,
  ) -> Request<(), AppsListAccountsForPlanQuery, Vec<MarketplacePurchase>> {
    let plan_id = plan_id.into();
    let url = format!("/marketplace_listing/plans/{plan_id}/accounts");

    Request::<(), AppsListAccountsForPlanQuery, Vec<MarketplacePurchase>>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get a subscription plan for an account (stubbed)**
  ///
  /// Shows whether the user or organization account actively subscribes to a plan listed by the authenticated GitHub App. When someone submits a plan change that won't be processed until the end of their billing cycle, you will also see the upcoming pending change.
  ///
  /// GitHub Apps must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint. OAuth apps must use [basic authentication](https://docs.github.com/rest/overview/other-authentication-methods#basic-authentication) with their client ID and client secret to access this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/apps/marketplace#get-a-subscription-plan-for-an-account-stubbed](https://docs.github.com/rest/apps/marketplace#get-a-subscription-plan-for-an-account-stubbed)
  pub fn get_subscription_plan_for_account_stubbed(
    &self,
    account_id: impl Into<i64>,
  ) -> Request<(), (), MarketplacePurchase> {
    let account_id = account_id.into();
    let url = format!("/marketplace_listing/stubbed/accounts/{account_id}");

    Request::<(), (), MarketplacePurchase>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List plans (stubbed)**
  ///
  /// Lists all plans that are part of your GitHub Marketplace listing.
  ///
  /// GitHub Apps must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint. OAuth apps must use [basic authentication](https://docs.github.com/rest/overview/other-authentication-methods#basic-authentication) with their client ID and client secret to access this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/apps/marketplace#list-plans-stubbed](https://docs.github.com/rest/apps/marketplace#list-plans-stubbed)
  pub fn list_plans_stubbed(
    &self,
  ) -> Request<(), AppsListPlansStubbedQuery, Vec<MarketplaceListingPlan>> {
    let url = format!("/marketplace_listing/stubbed/plans");

    Request::<(), AppsListPlansStubbedQuery, Vec<MarketplaceListingPlan>>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List accounts for a plan (stubbed)**
  ///
  /// Returns repository and organization accounts associated with the specified plan, including free plans. For per-seat pricing, you see the list of accounts that have purchased the plan, including the number of seats purchased. When someone submits a plan change that won't be processed until the end of their billing cycle, you will also see the upcoming pending change.
  ///
  /// GitHub Apps must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint. OAuth apps must use [basic authentication](https://docs.github.com/rest/overview/other-authentication-methods#basic-authentication) with their client ID and client secret to access this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/apps/marketplace#list-accounts-for-a-plan-stubbed](https://docs.github.com/rest/apps/marketplace#list-accounts-for-a-plan-stubbed)
  pub fn list_accounts_for_plan_stubbed(
    &self,
    plan_id: impl Into<i64>,
  ) -> Request<(), AppsListAccountsForPlanStubbedQuery, Vec<MarketplacePurchase>> {
    let plan_id = plan_id.into();
    let url = format!("/marketplace_listing/stubbed/plans/{plan_id}/accounts");

    Request::<(), AppsListAccountsForPlanStubbedQuery, Vec<MarketplacePurchase>>::builder(
      &self.config,
    )
    .get(url)
    .build()
  }

  /// **Get an organization installation for the authenticated app**
  ///
  /// Enables an authenticated GitHub App to find the organization's installation information.
  ///
  /// You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/apps/apps#get-an-organization-installation-for-the-authenticated-app](https://docs.github.com/rest/apps/apps#get-an-organization-installation-for-the-authenticated-app)
  pub fn get_org_installation(&self, org: impl Into<String>) -> Request<(), (), Installation> {
    let org = org.into();
    let url = format!("/orgs/{org}/installation");

    Request::<(), (), Installation>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get a repository installation for the authenticated app**
  ///
  /// Enables an authenticated GitHub App to find the repository's installation information. The installation's account type will be either an organization or a user account, depending which account the repository belongs to.
  ///
  /// You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/apps/apps#get-a-repository-installation-for-the-authenticated-app](https://docs.github.com/rest/apps/apps#get-a-repository-installation-for-the-authenticated-app)
  pub fn get_repo_installation(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), (), Installation> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/installation");

    Request::<(), (), Installation>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List app installations accessible to the user access token**
  ///
  /// Lists installations of your GitHub App that the authenticated user has explicit permission (`:read`, `:write`, or `:admin`) to access.
  ///
  /// The authenticated user has explicit permission to access repositories they own, repositories where they are a collaborator, and repositories that they can access through an organization membership.
  ///
  /// You can find the permissions for the installation under the `permissions` key.
  ///
  /// *Documentation*: [https://docs.github.com/rest/apps/installations#list-app-installations-accessible-to-the-user-access-token](https://docs.github.com/rest/apps/installations#list-app-installations-accessible-to-the-user-access-token)
  pub fn list_installations_for_authenticated_user(
    &self,
  ) -> Request<
    (),
    AppsListInstallationsForAuthenticatedUserQuery,
    AppsListInstallationsForAuthenticatedUserResponse,
  > {
    let url = format!("/user/installations");

    Request::<
      (),
      AppsListInstallationsForAuthenticatedUserQuery,
      AppsListInstallationsForAuthenticatedUserResponse,
    >::builder(&self.config)
    .get(url)
    .build()
  }

  /// **List repositories accessible to the user access token**
  ///
  /// List repositories that the authenticated user has explicit permission (`:read`, `:write`, or `:admin`) to access for an installation.
  ///
  /// The authenticated user has explicit permission to access repositories they own, repositories where they are a collaborator, and repositories that they can access through an organization membership.
  ///
  /// The access the user has to each repository is included in the hash under the `permissions` key.
  ///
  /// *Documentation*: [https://docs.github.com/rest/apps/installations#list-repositories-accessible-to-the-user-access-token](https://docs.github.com/rest/apps/installations#list-repositories-accessible-to-the-user-access-token)
  pub fn list_installation_repos_for_authenticated_user(
    &self,
    installation_id: impl Into<i64>,
  ) -> Request<
    (),
    AppsListInstallationReposForAuthenticatedUserQuery,
    AppsListInstallationReposForAuthenticatedUserResponse,
  > {
    let installation_id = installation_id.into();
    let url = format!("/user/installations/{installation_id}/repositories");

    Request::<
      (),
      AppsListInstallationReposForAuthenticatedUserQuery,
      AppsListInstallationReposForAuthenticatedUserResponse,
    >::builder(&self.config)
    .get(url)
    .build()
  }

  /// **Add a repository to an app installation**
  ///
  /// Add a single repository to an installation. The authenticated user must have admin access to the repository.
  ///
  /// *Documentation*: [https://docs.github.com/rest/apps/installations#add-a-repository-to-an-app-installation](https://docs.github.com/rest/apps/installations#add-a-repository-to-an-app-installation)
  pub fn add_repo_to_installation_for_authenticated_user(
    &self,
    installation_id: impl Into<i64>,
    repository_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let installation_id = installation_id.into();
    let repository_id = repository_id.into();
    let url = format!("/user/installations/{installation_id}/repositories/{repository_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **Remove a repository from an app installation**
  ///
  /// Remove a single repository from an installation. The authenticated user must have admin access to the repository. The installation must have the `repository_selection` of `selected`.
  ///
  /// *Documentation*: [https://docs.github.com/rest/apps/installations#remove-a-repository-from-an-app-installation](https://docs.github.com/rest/apps/installations#remove-a-repository-from-an-app-installation)
  pub fn remove_repo_from_installation_for_authenticated_user(
    &self,
    installation_id: impl Into<i64>,
    repository_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let installation_id = installation_id.into();
    let repository_id = repository_id.into();
    let url = format!("/user/installations/{installation_id}/repositories/{repository_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List subscriptions for the authenticated user**
  ///
  /// Lists the active subscriptions for the authenticated user.
  ///
  /// *Documentation*: [https://docs.github.com/rest/apps/marketplace#list-subscriptions-for-the-authenticated-user](https://docs.github.com/rest/apps/marketplace#list-subscriptions-for-the-authenticated-user)
  pub fn list_subscriptions_for_authenticated_user(
    &self,
  ) -> Request<(), AppsListSubscriptionsForAuthenticatedUserQuery, Vec<UserMarketplacePurchase>> {
    let url = format!("/user/marketplace_purchases");

    Request::<(), AppsListSubscriptionsForAuthenticatedUserQuery, Vec<UserMarketplacePurchase>>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List subscriptions for the authenticated user (stubbed)**
  ///
  /// Lists the active subscriptions for the authenticated user.
  ///
  /// *Documentation*: [https://docs.github.com/rest/apps/marketplace#list-subscriptions-for-the-authenticated-user-stubbed](https://docs.github.com/rest/apps/marketplace#list-subscriptions-for-the-authenticated-user-stubbed)
  pub fn list_subscriptions_for_authenticated_user_stubbed(
    &self,
  ) -> Request<
    (),
    AppsListSubscriptionsForAuthenticatedUserStubbedQuery,
    Vec<UserMarketplacePurchase>,
  > {
    let url = format!("/user/marketplace_purchases/stubbed");

    Request::<(), AppsListSubscriptionsForAuthenticatedUserStubbedQuery, Vec<UserMarketplacePurchase>>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get a user installation for the authenticated app**
  ///
  /// Enables an authenticated GitHub App to find the user’s installation information.
  ///
  /// You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/apps/apps#get-a-user-installation-for-the-authenticated-app](https://docs.github.com/rest/apps/apps#get-a-user-installation-for-the-authenticated-app)
  pub fn get_user_installation(
    &self,
    username: impl Into<String>,
  ) -> Request<(), (), Installation> {
    let username = username.into();
    let url = format!("/users/{username}/installation");

    Request::<(), (), Installation>::builder(&self.config)
      .get(url)
      .build()
  }
}
