use octocrate_core::*;
#[allow(unused_imports)]
use octocrate_types::*;

/// Endpoints that give information about the API.
pub struct GitHubMetaAPI {
  config: SharedAPIConfig,
}

impl GitHubMetaAPI {
  pub fn new(config: &SharedAPIConfig) -> Self {
    Self {
      config: config.clone(),
    }
  }

  /// **GitHub API Root**
  ///
  /// Get Hypermedia links to resources accessible in GitHub's REST API
  ///
  /// *Documentation*: [https://docs.github.com/rest/meta/meta#github-api-root](https://docs.github.com/rest/meta/meta#github-api-root)
  pub fn root(&self) -> Request<(), (), Root> {
    let url = format!("/");

    Request::<(), (), Root>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get the Zen of GitHub**
  ///
  /// Get a random sentence from the Zen of GitHub
  ///
  /// *Documentation*: [https://docs.github.com/rest/meta/meta#get-the-zen-of-github](https://docs.github.com/rest/meta/meta#get-the-zen-of-github)
  pub fn get_zen(&self) -> Request<(), (), String> {
    let url = format!("/zen");

    Request::<(), (), String>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get Octocat**
  ///
  /// Get the octocat as ASCII art
  ///
  /// *Documentation*: [https://docs.github.com/rest/meta/meta#get-octocat](https://docs.github.com/rest/meta/meta#get-octocat)
  pub fn get_octocat(&self) -> NoContentRequest<(), MetaGetOctocatQuery> {
    let url = format!("/octocat");

    NoContentRequest::<(), MetaGetOctocatQuery>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get GitHub meta information**
  ///
  /// Returns meta information about GitHub, including a list of GitHub's IP addresses. For more information, see "[About GitHub's IP addresses](https://docs.github.com/articles/about-github-s-ip-addresses/)."
  ///
  /// The API's response also includes a list of GitHub's domain names.
  ///
  /// The values shown in the documentation's response are example values. You must always query the API directly to get the latest values.
  ///
  /// **Note:** This endpoint returns both IPv4 and IPv6 addresses. However, not all features support IPv6. You should refer to the specific documentation for each feature to determine if IPv6 is supported.
  ///
  /// *Documentation*: [https://docs.github.com/rest/meta/meta#get-apiname-meta-information](https://docs.github.com/rest/meta/meta#get-apiname-meta-information)
  pub fn get(&self) -> Request<(), (), ApiOverview> {
    let url = format!("/meta");

    Request::<(), (), ApiOverview>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get all API versions**
  ///
  /// Get all supported GitHub API versions.
  ///
  /// *Documentation*: [https://docs.github.com/rest/meta/meta#get-all-api-versions](https://docs.github.com/rest/meta/meta#get-all-api-versions)
  pub fn get_all_versions(&self) -> Request<(), (), StringArray> {
    let url = format!("/versions");

    Request::<(), (), StringArray>::builder(&self.config)
      .get(url)
      .build()
  }
}
