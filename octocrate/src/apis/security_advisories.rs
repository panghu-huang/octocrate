use octocrate_core::*;
#[allow(unused_imports)]
use octocrate_types::*;

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
  ) -> Request<(), SecurityAdvisoriesListGlobalAdvisoriesQuery, GlobalAdvisoryArray> {
    let url = format!("/advisories");

    Request::<(), SecurityAdvisoriesListGlobalAdvisoriesQuery, GlobalAdvisoryArray>::builder(
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
  pub fn get_global_advisory(&self, ghsa_id: impl Into<String>) -> Request<(), (), GlobalAdvisory> {
    let ghsa_id = ghsa_id.into();
    let url = format!("/advisories/{ghsa_id}");

    Request::<(), (), GlobalAdvisory>::builder(&self.config)
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
  ) -> Request<(), SecurityAdvisoriesListOrgRepositoryAdvisoriesQuery, RepositoryAdvisoryArray> {
    let org = org.into();
    let url = format!("/orgs/{org}/security-advisories");

    Request::<(), SecurityAdvisoriesListOrgRepositoryAdvisoriesQuery, RepositoryAdvisoryArray>::builder(&self.config)
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
  ) -> Request<(), SecurityAdvisoriesListRepositoryAdvisoriesQuery, RepositoryAdvisoryArray> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/security-advisories");

    Request::<(), SecurityAdvisoriesListRepositoryAdvisoriesQuery, RepositoryAdvisoryArray>::builder(&self.config)
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
  ) -> Request<RepositoryAdvisoryCreate, (), RepositoryAdvisory> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/security-advisories");

    Request::<RepositoryAdvisoryCreate, (), RepositoryAdvisory>::builder(&self.config)
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
  ) -> Request<PrivateVulnerabilityReportCreate, (), RepositoryAdvisory> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/security-advisories/reports");

    Request::<PrivateVulnerabilityReportCreate, (), RepositoryAdvisory>::builder(&self.config)
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
  ) -> Request<(), (), RepositoryAdvisory> {
    let owner = owner.into();
    let repo = repo.into();
    let ghsa_id = ghsa_id.into();
    let url = format!("/repos/{owner}/{repo}/security-advisories/{ghsa_id}");

    Request::<(), (), RepositoryAdvisory>::builder(&self.config)
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
  ) -> Request<RepositoryAdvisoryUpdate, (), RepositoryAdvisory> {
    let owner = owner.into();
    let repo = repo.into();
    let ghsa_id = ghsa_id.into();
    let url = format!("/repos/{owner}/{repo}/security-advisories/{ghsa_id}");

    Request::<RepositoryAdvisoryUpdate, (), RepositoryAdvisory>::builder(&self.config)
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
  ) -> Request<(), (), FullRepository> {
    let owner = owner.into();
    let repo = repo.into();
    let ghsa_id = ghsa_id.into();
    let url = format!("/repos/{owner}/{repo}/security-advisories/{ghsa_id}/forks");

    Request::<(), (), FullRepository>::builder(&self.config)
      .post(url)
      .build()
  }
}
