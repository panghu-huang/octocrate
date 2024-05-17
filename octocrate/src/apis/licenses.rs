use octocrate_core::*;
#[allow(unused_imports)]
use octocrate_types::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use typed_builder::TypedBuilder;

pub mod get_all_commonly_used {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<LicenseSimple>;

  /// Query for `Get all commonly used licenses`
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub featured: Option<bool>,
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
  }
}

pub mod get {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = License;
}

pub mod get_for_repo {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = LicenseContent;

  /// Query for `Get the license for a repository`
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The Git reference for the results you want to list. The `ref` for a branch can be formatted either as `refs/heads/<branch name>` or simply `<branch name>`. To reference a pull request use `refs/pull/<number>/merge`.
    #[serde(rename = "ref")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub ref_: Option<String>,
  }
}

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

  /// **Get all commonly used licenses**
  ///
  /// Lists the most commonly used licenses on GitHub. For more information, see "[Licensing a repository ](https://docs.github.com/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/licensing-a-repository)."
  ///
  /// *Documentation*: [https://docs.github.com/rest/licenses/licenses#get-all-commonly-used-licenses](https://docs.github.com/rest/licenses/licenses#get-all-commonly-used-licenses)
  pub fn get_all_commonly_used(
    &self,
  ) -> Request<(), get_all_commonly_used::Query, get_all_commonly_used::Response> {
    let url = format!("/licenses");

    Request::<(), get_all_commonly_used::Query, get_all_commonly_used::Response>::builder(
      &self.config,
    )
    .get(url)
    .build()
  }

  /// **Get a license**
  ///
  /// Gets information about a specific license. For more information, see "[Licensing a repository ](https://docs.github.com/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/licensing-a-repository)."
  ///
  /// *Documentation*: [https://docs.github.com/rest/licenses/licenses#get-a-license](https://docs.github.com/rest/licenses/licenses#get-a-license)
  pub fn get(&self, license: impl Into<String>) -> Request<(), (), get::Response> {
    let license = license.into();
    let url = format!("/licenses/{license}");

    Request::<(), (), get::Response>::builder(&self.config)
      .get(url)
      .build()
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
  ) -> Request<(), get_for_repo::Query, get_for_repo::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/license");

    Request::<(), get_for_repo::Query, get_for_repo::Response>::builder(&self.config)
      .get(url)
      .build()
  }
}
