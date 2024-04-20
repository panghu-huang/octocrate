use octocrate_core::*;
#[allow(unused_imports)]
use octocrate_types::*;

/// Monitor charges and usage from Actions and Packages.
pub struct GitHubBillingAPI {
  config: SharedAPIConfig,
}

impl GitHubBillingAPI {
  pub fn new(config: &SharedAPIConfig) -> Self {
    Self {
      config: config.clone(),
    }
  }

  /// **Get shared storage billing for a user**
  ///
  /// Gets the estimated paid and estimated total storage used for GitHub Actions and GitHub Packages.
  ///
  /// Paid minutes only apply to packages stored for private repositories. For more information, see "[Managing billing for GitHub Packages](https://docs.github.com/github/setting-up-and-managing-billing-and-payments-on-github/managing-billing-for-github-packages)."
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `user` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/billing/billing#get-shared-storage-billing-for-a-user](https://docs.github.com/rest/billing/billing#get-shared-storage-billing-for-a-user)
  pub fn get_shared_storage_billing_user(
    &self,
    username: impl Into<String>,
  ) -> Request<(), (), CombinedBillingUsage> {
    let username = username.into();
    let url = format!("/users/{username}/settings/billing/shared-storage");

    Request::<(), (), CombinedBillingUsage>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get GitHub Actions billing for an organization**
  ///
  /// Gets the summary of the free and paid GitHub Actions minutes used.
  ///
  /// Paid minutes only apply to workflows in private repositories that use GitHub-hosted runners. Minutes used is listed for each GitHub-hosted runner operating system. Any job re-runs are also included in the usage. The usage returned includes any minute multipliers for macOS and Windows runners, and is rounded up to the nearest whole minute. For more information, see "[Managing billing for GitHub Actions](https://docs.github.com/github/setting-up-and-managing-billing-and-payments-on-github/managing-billing-for-github-actions)".
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` or `admin:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/billing/billing#get-github-actions-billing-for-an-organization](https://docs.github.com/rest/billing/billing#get-github-actions-billing-for-an-organization)
  pub fn get_github_actions_billing_org(
    &self,
    org: impl Into<String>,
  ) -> Request<(), (), ActionsBillingUsage> {
    let org = org.into();
    let url = format!("/orgs/{org}/settings/billing/actions");

    Request::<(), (), ActionsBillingUsage>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get shared storage billing for an organization**
  ///
  /// Gets the estimated paid and estimated total storage used for GitHub Actions and GitHub Packages.
  ///
  /// Paid minutes only apply to packages stored for private repositories. For more information, see "[Managing billing for GitHub Packages](https://docs.github.com/github/setting-up-and-managing-billing-and-payments-on-github/managing-billing-for-github-packages)."
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` or `admin:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/billing/billing#get-shared-storage-billing-for-an-organization](https://docs.github.com/rest/billing/billing#get-shared-storage-billing-for-an-organization)
  pub fn get_shared_storage_billing_org(
    &self,
    org: impl Into<String>,
  ) -> Request<(), (), CombinedBillingUsage> {
    let org = org.into();
    let url = format!("/orgs/{org}/settings/billing/shared-storage");

    Request::<(), (), CombinedBillingUsage>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get GitHub Packages billing for an organization**
  ///
  /// Gets the free and paid storage used for GitHub Packages in gigabytes.
  ///
  /// Paid minutes only apply to packages stored for private repositories. For more information, see "[Managing billing for GitHub Packages](https://docs.github.com/github/setting-up-and-managing-billing-and-payments-on-github/managing-billing-for-github-packages)."
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` or `admin:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/billing/billing#get-github-packages-billing-for-an-organization](https://docs.github.com/rest/billing/billing#get-github-packages-billing-for-an-organization)
  pub fn get_github_packages_billing_org(
    &self,
    org: impl Into<String>,
  ) -> Request<(), (), PackagesBillingUsage> {
    let org = org.into();
    let url = format!("/orgs/{org}/settings/billing/packages");

    Request::<(), (), PackagesBillingUsage>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get GitHub Packages billing for a user**
  ///
  /// Gets the free and paid storage used for GitHub Packages in gigabytes.
  ///
  /// Paid minutes only apply to packages stored for private repositories. For more information, see "[Managing billing for GitHub Packages](https://docs.github.com/github/setting-up-and-managing-billing-and-payments-on-github/managing-billing-for-github-packages)."
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `user` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/billing/billing#get-github-packages-billing-for-a-user](https://docs.github.com/rest/billing/billing#get-github-packages-billing-for-a-user)
  pub fn get_github_packages_billing_user(
    &self,
    username: impl Into<String>,
  ) -> Request<(), (), PackagesBillingUsage> {
    let username = username.into();
    let url = format!("/users/{username}/settings/billing/packages");

    Request::<(), (), PackagesBillingUsage>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get GitHub Actions billing for a user**
  ///
  /// Gets the summary of the free and paid GitHub Actions minutes used.
  ///
  /// Paid minutes only apply to workflows in private repositories that use GitHub-hosted runners. Minutes used is listed for each GitHub-hosted runner operating system. Any job re-runs are also included in the usage. The usage returned includes any minute multipliers for macOS and Windows runners, and is rounded up to the nearest whole minute. For more information, see "[Managing billing for GitHub Actions](https://docs.github.com/github/setting-up-and-managing-billing-and-payments-on-github/managing-billing-for-github-actions)".
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `user` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/billing/billing#get-github-actions-billing-for-a-user](https://docs.github.com/rest/billing/billing#get-github-actions-billing-for-a-user)
  pub fn get_github_actions_billing_user(
    &self,
    username: impl Into<String>,
  ) -> Request<(), (), ActionsBillingUsage> {
    let username = username.into();
    let url = format!("/users/{username}/settings/billing/actions");

    Request::<(), (), ActionsBillingUsage>::builder(&self.config)
      .get(url)
      .build()
  }
}
