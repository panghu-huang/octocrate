use octocrate_core::*;
#[allow(unused_imports)]
use octocrate_types::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use typed_builder::TypedBuilder;

pub mod list_global_advisories {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<GlobalAdvisory>;

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QueryType {
    #[serde(rename = "reviewed")]
    Reviewed,
    #[serde(rename = "malware")]
    Malware,
    #[serde(rename = "unreviewed")]
    Unreviewed,
  }

  impl ToString for QueryType {
    fn to_string(&self) -> String {
      match self {
        QueryType::Reviewed => "reviewed".to_string(),
        QueryType::Malware => "malware".to_string(),
        QueryType::Unreviewed => "unreviewed".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QuerySeverity {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "critical")]
    Critical,
  }

  impl ToString for QuerySeverity {
    fn to_string(&self) -> String {
      match self {
        QuerySeverity::Unknown => "unknown".to_string(),
        QuerySeverity::Low => "low".to_string(),
        QuerySeverity::Medium => "medium".to_string(),
        QuerySeverity::High => "high".to_string(),
        QuerySeverity::Critical => "critical".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QueryDirection {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
  }

  impl ToString for QueryDirection {
    fn to_string(&self) -> String {
      match self {
        QueryDirection::Asc => "asc".to_string(),
        QueryDirection::Desc => "desc".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QuerySort {
    #[serde(rename = "updated")]
    Updated,
    #[serde(rename = "published")]
    Published,
  }

  impl ToString for QuerySort {
    fn to_string(&self) -> String {
      match self {
        QuerySort::Updated => "updated".to_string(),
        QuerySort::Published => "published".to_string(),
      }
    }
  }

  /// Query for `List global security advisories`
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// If specified, only advisories with this GHSA (GitHub Security Advisory) identifier will be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub ghsa_id: Option<String>,
    /// If specified, only advisories of this type will be returned. By default, a request with no other parameters defined will only return reviewed advisories that are not malware.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub type_: Option<QueryType>,
    /// If specified, only advisories with this CVE (Common Vulnerabilities and Exposures) identifier will be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub cve_id: Option<String>,
    /// If specified, only advisories for these ecosystems will be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub ecosystem: Option<SecurityAdvisoryEcosystems>,
    /// If specified, only advisories with these severities will be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub severity: Option<QuerySeverity>,
    /// If specified, only advisories with these Common Weakness Enumerations (CWEs) will be returned.
    ///
    /// Example: `cwes=79,284,22` or `cwes[]=79&cwes[]=284&cwes[]=22`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub cwes: Option<Vec<String>>,
    /// Whether to only return advisories that have been withdrawn.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub is_withdrawn: Option<bool>,
    /// If specified, only return advisories that affect any of `package` or `package@version`. A maximum of 1000 packages can be specified.
    /// If the query parameter causes the URL to exceed the maximum URL length supported by your client, you must specify fewer packages.
    ///
    /// Example: `affects=package1,package2@1.0.0,package3@^2.0.0` or `affects[]=package1&affects[]=package2@1.0.0`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub affects: Option<Vec<String>>,
    /// If specified, only return advisories that were published on a date or date range.
    ///
    /// For more information on the syntax of the date range, see "[Understanding the search syntax](https://docs.github.com/search-github/getting-started-with-searching-on-github/understanding-the-search-syntax#query-for-dates)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub published: Option<String>,
    /// If specified, only return advisories that were updated on a date or date range.
    ///
    /// For more information on the syntax of the date range, see "[Understanding the search syntax](https://docs.github.com/search-github/getting-started-with-searching-on-github/understanding-the-search-syntax#query-for-dates)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub updated: Option<String>,
    /// If specified, only show advisories that were updated or published on a date or date range.
    ///
    /// For more information on the syntax of the date range, see "[Understanding the search syntax](https://docs.github.com/search-github/getting-started-with-searching-on-github/understanding-the-search-syntax#query-for-dates)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub modified: Option<String>,
    /// A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results before this cursor. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub before: Option<String>,
    /// A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results after this cursor. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub after: Option<String>,
    /// The direction to sort the results by.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub direction: Option<QueryDirection>,
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// The property to sort the results by.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub sort: Option<QuerySort>,
  }
}

pub mod get_global_advisory {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = GlobalAdvisory;
}

pub mod list_org_repository_advisories {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<RepositoryAdvisory>;

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QueryDirection {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
  }

  impl ToString for QueryDirection {
    fn to_string(&self) -> String {
      match self {
        QueryDirection::Asc => "asc".to_string(),
        QueryDirection::Desc => "desc".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QuerySort {
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "updated")]
    Updated,
    #[serde(rename = "published")]
    Published,
  }

  impl ToString for QuerySort {
    fn to_string(&self) -> String {
      match self {
        QuerySort::Created => "created".to_string(),
        QuerySort::Updated => "updated".to_string(),
        QuerySort::Published => "published".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QueryState {
    #[serde(rename = "triage")]
    Triage,
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "published")]
    Published,
    #[serde(rename = "closed")]
    Closed,
  }

  impl ToString for QueryState {
    fn to_string(&self) -> String {
      match self {
        QueryState::Triage => "triage".to_string(),
        QueryState::Draft => "draft".to_string(),
        QueryState::Published => "published".to_string(),
        QueryState::Closed => "closed".to_string(),
      }
    }
  }

  /// Query for `List repository security advisories for an organization`
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The direction to sort the results by.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub direction: Option<QueryDirection>,
    /// The property to sort the results by.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub sort: Option<QuerySort>,
    /// A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results before this cursor. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub before: Option<String>,
    /// A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results after this cursor. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub after: Option<String>,
    /// The number of advisories to return per page. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// Filter by the state of the repository advisories. Only advisories of this state will be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub state: Option<QueryState>,
  }
}

pub mod list_repository_advisories {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<RepositoryAdvisory>;

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QueryDirection {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
  }

  impl ToString for QueryDirection {
    fn to_string(&self) -> String {
      match self {
        QueryDirection::Asc => "asc".to_string(),
        QueryDirection::Desc => "desc".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QuerySort {
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "updated")]
    Updated,
    #[serde(rename = "published")]
    Published,
  }

  impl ToString for QuerySort {
    fn to_string(&self) -> String {
      match self {
        QuerySort::Created => "created".to_string(),
        QuerySort::Updated => "updated".to_string(),
        QuerySort::Published => "published".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QueryState {
    #[serde(rename = "triage")]
    Triage,
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "published")]
    Published,
    #[serde(rename = "closed")]
    Closed,
  }

  impl ToString for QueryState {
    fn to_string(&self) -> String {
      match self {
        QueryState::Triage => "triage".to_string(),
        QueryState::Draft => "draft".to_string(),
        QueryState::Published => "published".to_string(),
        QueryState::Closed => "closed".to_string(),
      }
    }
  }

  /// Query for `List repository security advisories`
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The direction to sort the results by.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub direction: Option<QueryDirection>,
    /// The property to sort the results by.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub sort: Option<QuerySort>,
    /// A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results before this cursor. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub before: Option<String>,
    /// A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results after this cursor. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub after: Option<String>,
    /// The number of advisories to return per page. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// Filter by state of the repository advisories. Only advisories of this state will be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub state: Option<QueryState>,
  }
}

pub mod create_repository_advisory {
  #[allow(unused_imports)]
  use super::*;

  pub type Request = RepositoryAdvisoryCreate;
  pub type Response = RepositoryAdvisory;
}

pub mod create_private_vulnerability_report {
  #[allow(unused_imports)]
  use super::*;

  pub type Request = PrivateVulnerabilityReportCreate;
  pub type Response = RepositoryAdvisory;
}

pub mod get_repository_advisory {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = RepositoryAdvisory;
}

pub mod update_repository_advisory {
  #[allow(unused_imports)]
  use super::*;

  pub type Request = RepositoryAdvisoryUpdate;
  pub type Response = RepositoryAdvisory;
}

pub mod create_repository_advisory_cve_request {
  #[allow(unused_imports)]
  use super::*;
}

pub mod create_fork {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = FullRepository;
}

pub struct GitHubSecurityAdvisoriesAPI {
  config: SharedAPIConfig,
}

impl GitHubSecurityAdvisoriesAPI {
  pub fn new(config: &SharedAPIConfig) -> Self {
    Self {
      config: config.clone(),
    }
  }

  /// **List global security advisories**
  ///
  /// Lists all global security advisories that match the specified parameters. If no other parameters are defined, the request will return only GitHub-reviewed advisories that are not malware.
  ///
  /// By default, all responses will exclude advisories for malware, because malware are not standard vulnerabilities. To list advisories for malware, you must include the `type` parameter in your request, with the value `malware`. For more information about the different types of security advisories, see "[About the GitHub Advisory database](https://docs.github.com/code-security/security-advisories/global-security-advisories/about-the-github-advisory-database#about-types-of-security-advisories)."
  ///
  /// *Documentation*: [https://docs.github.com/rest/security-advisories/global-advisories#list-global-security-advisories](https://docs.github.com/rest/security-advisories/global-advisories#list-global-security-advisories)
  pub fn list_global_advisories(
    &self,
  ) -> Request<(), list_global_advisories::Query, list_global_advisories::Response> {
    let url = format!("/advisories");

    Request::<(), list_global_advisories::Query, list_global_advisories::Response>::builder(
      &self.config,
    )
    .get(url)
    .build()
  }

  /// **Get a global security advisory**
  ///
  /// Gets a global security advisory using its GitHub Security Advisory (GHSA) identifier.
  ///
  /// *Documentation*: [https://docs.github.com/rest/security-advisories/global-advisories#get-a-global-security-advisory](https://docs.github.com/rest/security-advisories/global-advisories#get-a-global-security-advisory)
  pub fn get_global_advisory(
    &self,
    ghsa_id: impl Into<String>,
  ) -> Request<(), (), get_global_advisory::Response> {
    let ghsa_id = ghsa_id.into();
    let url = format!("/advisories/{ghsa_id}");

    Request::<(), (), get_global_advisory::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List repository security advisories for an organization**
  ///
  /// Lists repository security advisories for an organization.
  ///
  /// The authenticated user must be an owner or security manager for the organization to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` or `repository_advisories:write` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/security-advisories/repository-advisories#list-repository-security-advisories-for-an-organization](https://docs.github.com/rest/security-advisories/repository-advisories#list-repository-security-advisories-for-an-organization)
  pub fn list_org_repository_advisories(
    &self,
    org: impl Into<String>,
  ) -> Request<(), list_org_repository_advisories::Query, list_org_repository_advisories::Response>
  {
    let org = org.into();
    let url = format!("/orgs/{org}/security-advisories");

    Request::<(), list_org_repository_advisories::Query, list_org_repository_advisories::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List repository security advisories**
  ///
  /// Lists security advisories in a repository.
  ///
  /// The authenticated user can access unpublished security advisories from a repository if they are a security manager or administrator of that repository, or if they are a collaborator on any security advisory.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` or `repository_advisories:read` scope to to get a published security advisory in a private repository, or any unpublished security advisory that the authenticated user has access to.
  ///
  /// *Documentation*: [https://docs.github.com/rest/security-advisories/repository-advisories#list-repository-security-advisories](https://docs.github.com/rest/security-advisories/repository-advisories#list-repository-security-advisories)
  pub fn list_repository_advisories(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), list_repository_advisories::Query, list_repository_advisories::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/security-advisories");

    Request::<(), list_repository_advisories::Query, list_repository_advisories::Response>::builder(
      &self.config,
    )
    .get(url)
    .build()
  }

  /// **Create a repository security advisory**
  ///
  /// Creates a new repository security advisory.
  ///
  /// In order to create a draft repository security advisory, the authenticated user must be a security manager or administrator of that repository.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` or `repository_advisories:write` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/security-advisories/repository-advisories#create-a-repository-security-advisory](https://docs.github.com/rest/security-advisories/repository-advisories#create-a-repository-security-advisory)
  pub fn create_repository_advisory(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<create_repository_advisory::Request, (), create_repository_advisory::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/security-advisories");

    Request::<create_repository_advisory::Request, (), create_repository_advisory::Response>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Privately report a security vulnerability**
  ///
  /// Report a security vulnerability to the maintainers of the repository.
  /// See "[Privately reporting a security vulnerability](https://docs.github.com/code-security/security-advisories/guidance-on-reporting-and-writing/privately-reporting-a-security-vulnerability)" for more information about private vulnerability reporting.
  ///
  /// *Documentation*: [https://docs.github.com/rest/security-advisories/repository-advisories#privately-report-a-security-vulnerability](https://docs.github.com/rest/security-advisories/repository-advisories#privately-report-a-security-vulnerability)
  pub fn create_private_vulnerability_report(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<
    create_private_vulnerability_report::Request,
    (),
    create_private_vulnerability_report::Response,
  > {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/security-advisories/reports");

    Request::<
      create_private_vulnerability_report::Request,
      (),
      create_private_vulnerability_report::Response,
    >::builder(&self.config)
    .post(url)
    .build()
  }

  /// **Get a repository security advisory**
  ///
  /// Get a repository security advisory using its GitHub Security Advisory (GHSA) identifier.
  ///
  /// Anyone can access any published security advisory on a public repository.
  ///
  /// The authenticated user can access an unpublished security advisory from a repository if they are a security manager or administrator of that repository, or if they are a
  /// collaborator on the security advisory.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` or `repository_advisories:read` scope to to get a published security advisory in a private repository, or any unpublished security advisory that the authenticated user has access to.
  ///
  /// *Documentation*: [https://docs.github.com/rest/security-advisories/repository-advisories#get-a-repository-security-advisory](https://docs.github.com/rest/security-advisories/repository-advisories#get-a-repository-security-advisory)
  pub fn get_repository_advisory(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    ghsa_id: impl Into<String>,
  ) -> Request<(), (), get_repository_advisory::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let ghsa_id = ghsa_id.into();
    let url = format!("/repos/{owner}/{repo}/security-advisories/{ghsa_id}");

    Request::<(), (), get_repository_advisory::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Update a repository security advisory**
  ///
  /// Update a repository security advisory using its GitHub Security Advisory (GHSA) identifier.
  ///
  /// In order to update any security advisory, the authenticated user must be a security manager or administrator of that repository,
  /// or a collaborator on the repository security advisory.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` or `repository_advisories:write` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/security-advisories/repository-advisories#update-a-repository-security-advisory](https://docs.github.com/rest/security-advisories/repository-advisories#update-a-repository-security-advisory)
  pub fn update_repository_advisory(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    ghsa_id: impl Into<String>,
  ) -> Request<update_repository_advisory::Request, (), update_repository_advisory::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let ghsa_id = ghsa_id.into();
    let url = format!("/repos/{owner}/{repo}/security-advisories/{ghsa_id}");

    Request::<update_repository_advisory::Request, (), update_repository_advisory::Response>::builder(&self.config)
      .patch(url)
      .build()
  }

  /// **Request a CVE for a repository security advisory**
  ///
  /// If you want a CVE identification number for the security vulnerability in your project, and don't already have one, you can request a CVE identification number from GitHub. For more information see "[Requesting a CVE identification number](https://docs.github.com/code-security/security-advisories/repository-security-advisories/publishing-a-repository-security-advisory#requesting-a-cve-identification-number-optional)."
  ///
  /// You may request a CVE for public repositories, but cannot do so for private repositories.
  ///
  /// In order to request a CVE for a repository security advisory, the authenticated user must be a security manager or administrator of that repository.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` or `repository_advisories:write` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/security-advisories/repository-advisories#request-a-cve-for-a-repository-security-advisory](https://docs.github.com/rest/security-advisories/repository-advisories#request-a-cve-for-a-repository-security-advisory)
  pub fn create_repository_advisory_cve_request(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    ghsa_id: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let ghsa_id = ghsa_id.into();
    let url = format!("/repos/{owner}/{repo}/security-advisories/{ghsa_id}/cve");

    NoContentRequest::<(), ()>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Create a temporary private fork**
  ///
  /// Create a temporary private fork to collaborate on fixing a security vulnerability in your repository.
  ///
  /// **Note**: Forking a repository happens asynchronously. You may have to wait up to 5 minutes before you can access the fork.
  ///
  /// *Documentation*: [https://docs.github.com/rest/security-advisories/repository-advisories#create-a-temporary-private-fork](https://docs.github.com/rest/security-advisories/repository-advisories#create-a-temporary-private-fork)
  pub fn create_fork(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    ghsa_id: impl Into<String>,
  ) -> Request<(), (), create_fork::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let ghsa_id = ghsa_id.into();
    let url = format!("/repos/{owner}/{repo}/security-advisories/{ghsa_id}/forks");

    Request::<(), (), create_fork::Response>::builder(&self.config)
      .post(url)
      .build()
  }
}
