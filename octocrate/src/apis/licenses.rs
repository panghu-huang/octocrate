#[allow(unused_imports)]
use crate::types::*;
use octocrate_core::*;

/// View various OSS licenses.
pub struct GitHubLicensesAPI {
  config: SharedAPIConfig,
}

impl GitHubLicensesAPI {
  pub fn new(config: &SharedAPIConfig) -> Self {
    Self {
      config: config.clone(),
    }
  }

  /// **Get the license for a repository**
  ///
  /// This method returns the contents of the repository's license file, if one is detected.
  ///
  /// This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
  ///
  /// - **`application/vnd.github.raw+json`**: Returns the raw contents of the license.
  /// - **`application/vnd.github.html+json`**: Returns the license contents in HTML. Markup languages are rendered to HTML using GitHub's open-source [Markup library](https://github.com/github/markup).
  ///
  /// *Documentation*: [https://docs.github.com/rest/licenses/licenses#get-the-license-for-a-repository](https://docs.github.com/rest/licenses/licenses#get-the-license-for-a-repository)
  pub fn get_for_repo(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), LicensesGetForRepoQuery, LicenseContent> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/license");

    Request::<(), LicensesGetForRepoQuery, LicenseContent>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get a license**
  ///
  /// Gets information about a specific license. For more information, see "[Licensing a repository ](https://docs.github.com/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/licensing-a-repository)."
  ///
  /// *Documentation*: [https://docs.github.com/rest/licenses/licenses#get-a-license](https://docs.github.com/rest/licenses/licenses#get-a-license)
  pub fn get(&self, license: impl Into<String>) -> Request<(), (), License> {
    let license = license.into();
    let url = format!("/licenses/{license}");

    Request::<(), (), License>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get all commonly used licenses**
  ///
  /// Lists the most commonly used licenses on GitHub. For more information, see "[Licensing a repository ](https://docs.github.com/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/licensing-a-repository)."
  ///
  /// *Documentation*: [https://docs.github.com/rest/licenses/licenses#get-all-commonly-used-licenses](https://docs.github.com/rest/licenses/licenses#get-all-commonly-used-licenses)
  pub fn get_all_commonly_used(
    &self,
  ) -> Request<(), LicensesGetAllCommonlyUsedQuery, LicenseSimpleArray> {
    let url = format!("/licenses");

    Request::<(), LicensesGetAllCommonlyUsedQuery, LicenseSimpleArray>::builder(&self.config)
      .get(url)
      .build()
  }
}
