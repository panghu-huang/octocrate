use octocrate_core::*;
#[allow(unused_imports)]
use octocrate_types::*;

pub struct GitHubCodeScanningAPI {
  config: SharedAPIConfig,
}

impl GitHubCodeScanningAPI {
  pub fn new(config: &SharedAPIConfig) -> Self {
    Self {
      config: config.clone(),
    }
  }

  /// **List code scanning alerts for an organization**
  ///
  /// Lists code scanning alerts for the default branch for all eligible repositories in an organization. Eligible repositories are repositories that are owned by organizations that you own or for which you are a security manager. For more information, see "[Managing security managers in your organization](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/managing-security-managers-in-your-organization)."
  ///
  /// The authenticated user must be an owner or security manager for the organization to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `security_events` or `repo`s cope to use this endpoint with private or public repositories, or the `public_repo` scope to use this endpoint with only public repositories.
  ///
  /// *Documentation*: [https://docs.github.com/rest/code-scanning/code-scanning#list-code-scanning-alerts-for-an-organization](https://docs.github.com/rest/code-scanning/code-scanning#list-code-scanning-alerts-for-an-organization)
  pub fn list_alerts_for_org(
    &self,
    org: impl Into<String>,
  ) -> Request<(), CodeScanningListAlertsForOrgQuery, CodeScanningOrganizationAlertItemsArray> {
    let org = org.into();
    let url = format!("/orgs/{org}/code-scanning/alerts");

    Request::<(), CodeScanningListAlertsForOrgQuery, CodeScanningOrganizationAlertItemsArray>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List code scanning alerts for a repository**
  ///
  /// Lists code scanning alerts.
  ///
  /// The response includes a `most_recent_instance` object.
  /// This provides details of the most recent instance of this alert
  /// for the default branch (or for the specified Git reference if you used `ref` in the request).
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `security_events` scope to use this endpoint with private or public repositories, or the `public_repo` scope to use this endpoint with only public repositories.
  ///
  /// *Documentation*: [https://docs.github.com/rest/code-scanning/code-scanning#list-code-scanning-alerts-for-a-repository](https://docs.github.com/rest/code-scanning/code-scanning#list-code-scanning-alerts-for-a-repository)
  pub fn list_alerts_for_repo(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), CodeScanningListAlertsForRepoQuery, CodeScanningAlertItemsArray> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/code-scanning/alerts");

    Request::<(), CodeScanningListAlertsForRepoQuery, CodeScanningAlertItemsArray>::builder(
      &self.config,
    )
    .get(url)
    .build()
  }

  /// **Get a code scanning alert**
  ///
  /// Gets a single code scanning alert.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `security_events` scope to use this endpoint with private or public repositories, or the `public_repo` scope to use this endpoint with only public repositories.
  ///
  /// *Documentation*: [https://docs.github.com/rest/code-scanning/code-scanning#get-a-code-scanning-alert](https://docs.github.com/rest/code-scanning/code-scanning#get-a-code-scanning-alert)
  pub fn get_alert(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    alert_number: impl Into<serde_json::Value>,
  ) -> Request<(), (), CodeScanningAlert> {
    let owner = owner.into();
    let repo = repo.into();
    let alert_number = alert_number.into();
    let url = format!("/repos/{owner}/{repo}/code-scanning/alerts/{alert_number}");

    Request::<(), (), CodeScanningAlert>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Update a code scanning alert**
  ///
  /// Updates the status of a single code scanning alert.
  /// OAuth app tokens and personal access tokens (classic) need the `security_events` scope to use this endpoint with private or public repositories, or the `public_repo` scope to use this endpoint with only public repositories.
  ///
  /// *Documentation*: [https://docs.github.com/rest/code-scanning/code-scanning#update-a-code-scanning-alert](https://docs.github.com/rest/code-scanning/code-scanning#update-a-code-scanning-alert)
  pub fn update_alert(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    alert_number: impl Into<serde_json::Value>,
  ) -> Request<CodeScanningUpdateAlertRequest, (), CodeScanningAlert> {
    let owner = owner.into();
    let repo = repo.into();
    let alert_number = alert_number.into();
    let url = format!("/repos/{owner}/{repo}/code-scanning/alerts/{alert_number}");

    Request::<CodeScanningUpdateAlertRequest, (), CodeScanningAlert>::builder(&self.config)
      .patch(url)
      .build()
  }

  /// **List instances of a code scanning alert**
  ///
  /// Lists all instances of the specified code scanning alert.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `security_events` scope to use this endpoint with private or public repositories, or the `public_repo` scope to use this endpoint with only public repositories.
  ///
  /// *Documentation*: [https://docs.github.com/rest/code-scanning/code-scanning#list-instances-of-a-code-scanning-alert](https://docs.github.com/rest/code-scanning/code-scanning#list-instances-of-a-code-scanning-alert)
  pub fn list_alert_instances(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    alert_number: impl Into<serde_json::Value>,
  ) -> Request<(), CodeScanningListAlertInstancesQuery, CodeScanningAlertInstanceArray> {
    let owner = owner.into();
    let repo = repo.into();
    let alert_number = alert_number.into();
    let url = format!("/repos/{owner}/{repo}/code-scanning/alerts/{alert_number}/instances");

    Request::<(), CodeScanningListAlertInstancesQuery, CodeScanningAlertInstanceArray>::builder(
      &self.config,
    )
    .get(url)
    .build()
  }

  /// **List code scanning analyses for a repository**
  ///
  /// Lists the details of all code scanning analyses for a repository,
  /// starting with the most recent.
  /// The response is paginated and you can use the `page` and `per_page` parameters
  /// to list the analyses you're interested in.
  /// By default 30 analyses are listed per page.
  ///
  /// The `rules_count` field in the response give the number of rules
  /// that were run in the analysis.
  /// For very old analyses this data is not available,
  /// and `0` is returned in this field.
  ///
  /// **Deprecation notice**:
  /// The `tool_name` field is deprecated and will, in future, not be included in the response for this endpoint. The example response reflects this change. The tool name can now be found inside the `tool` field.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `security_events` scope to use this endpoint with private or public repositories, or the `public_repo` scope to use this endpoint with only public repositories.
  ///
  /// *Documentation*: [https://docs.github.com/rest/code-scanning/code-scanning#list-code-scanning-analyses-for-a-repository](https://docs.github.com/rest/code-scanning/code-scanning#list-code-scanning-analyses-for-a-repository)
  pub fn list_recent_analyses(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), CodeScanningListRecentAnalysesQuery, CodeScanningAnalysisArray> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/code-scanning/analyses");

    Request::<(), CodeScanningListRecentAnalysesQuery, CodeScanningAnalysisArray>::builder(
      &self.config,
    )
    .get(url)
    .build()
  }

  /// **Get a code scanning analysis for a repository**
  ///
  /// Gets a specified code scanning analysis for a repository.
  ///
  /// The default JSON response contains fields that describe the analysis.
  /// This includes the Git reference and commit SHA to which the analysis relates,
  /// the datetime of the analysis, the name of the code scanning tool,
  /// and the number of alerts.
  ///
  /// The `rules_count` field in the default response give the number of rules
  /// that were run in the analysis.
  /// For very old analyses this data is not available,
  /// and `0` is returned in this field.
  ///
  /// This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
  ///
  /// - **`application/sarif+json`**: Instead of returning a summary of the analysis, this endpoint returns a subset of the analysis data that was uploaded. The data is formatted as [SARIF version 2.1.0](https://docs.oasis-open.org/sarif/sarif/v2.1.0/cs01/sarif-v2.1.0-cs01.html). It also returns additional data such as the `github/alertNumber` and `github/alertUrl` properties.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `security_events` scope to use this endpoint with private or public repositories, or the `public_repo` scope to use this endpoint with only public repositories.
  ///
  /// *Documentation*: [https://docs.github.com/rest/code-scanning/code-scanning#get-a-code-scanning-analysis-for-a-repository](https://docs.github.com/rest/code-scanning/code-scanning#get-a-code-scanning-analysis-for-a-repository)
  pub fn get_analysis(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    analysis_id: impl Into<i64>,
  ) -> Request<(), (), CodeScanningAnalysis> {
    let owner = owner.into();
    let repo = repo.into();
    let analysis_id = analysis_id.into();
    let url = format!("/repos/{owner}/{repo}/code-scanning/analyses/{analysis_id}");

    Request::<(), (), CodeScanningAnalysis>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Delete a code scanning analysis from a repository**
  ///
  /// Deletes a specified code scanning analysis from a repository.
  ///
  /// You can delete one analysis at a time.
  /// To delete a series of analyses, start with the most recent analysis and work backwards.
  /// Conceptually, the process is similar to the undo function in a text editor.
  ///
  /// When you list the analyses for a repository,
  /// one or more will be identified as deletable in the response:
  ///
  /// ```
  /// "deletable": true
  /// ```
  ///
  /// An analysis is deletable when it's the most recent in a set of analyses.
  /// Typically, a repository will have multiple sets of analyses
  /// for each enabled code scanning tool,
  /// where a set is determined by a unique combination of analysis values:
  ///
  /// * `ref`
  /// * `tool`
  /// * `category`
  ///
  /// If you attempt to delete an analysis that is not the most recent in a set,
  /// you'll get a 400 response with the message:
  ///
  /// ```
  /// Analysis specified is not deletable.
  /// ```
  ///
  /// The response from a successful `DELETE` operation provides you with
  /// two alternative URLs for deleting the next analysis in the set:
  /// `next_analysis_url` and `confirm_delete_url`.
  /// Use the `next_analysis_url` URL if you want to avoid accidentally deleting the final analysis
  /// in a set. This is a useful option if you want to preserve at least one analysis
  /// for the specified tool in your repository.
  /// Use the `confirm_delete_url` URL if you are content to remove all analyses for a tool.
  /// When you delete the last analysis in a set, the value of `next_analysis_url` and `confirm_delete_url`
  /// in the 200 response is `null`.
  ///
  /// As an example of the deletion process,
  /// let's imagine that you added a workflow that configured a particular code scanning tool
  /// to analyze the code in a repository. This tool has added 15 analyses:
  /// 10 on the default branch, and another 5 on a topic branch.
  /// You therefore have two separate sets of analyses for this tool.
  /// You've now decided that you want to remove all of the analyses for the tool.
  /// To do this you must make 15 separate deletion requests.
  /// To start, you must find an analysis that's identified as deletable.
  /// Each set of analyses always has one that's identified as deletable.
  /// Having found the deletable analysis for one of the two sets,
  /// delete this analysis and then continue deleting the next analysis in the set until they're all deleted.
  /// Then repeat the process for the second set.
  /// The procedure therefore consists of a nested loop:
  ///
  /// **Outer loop**:
  /// * List the analyses for the repository, filtered by tool.
  /// * Parse this list to find a deletable analysis. If found:
  ///
  ///   **Inner loop**:
  ///   * Delete the identified analysis.
  ///   * Parse the response for the value of `confirm_delete_url` and, if found, use this in the next iteration.
  ///
  /// The above process assumes that you want to remove all trace of the tool's analyses from the GitHub user interface, for the specified repository, and it therefore uses the `confirm_delete_url` value. Alternatively, you could use the `next_analysis_url` value, which would leave the last analysis in each set undeleted to avoid removing a tool's analysis entirely.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with private or public repositories, or the `public_repo` scope to use this endpoint with only public repositories.
  ///
  /// *Documentation*: [https://docs.github.com/rest/code-scanning/code-scanning#delete-a-code-scanning-analysis-from-a-repository](https://docs.github.com/rest/code-scanning/code-scanning#delete-a-code-scanning-analysis-from-a-repository)
  pub fn delete_analysis(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    analysis_id: impl Into<i64>,
  ) -> Request<(), CodeScanningDeleteAnalysisQuery, CodeScanningAnalysisDeletion> {
    let owner = owner.into();
    let repo = repo.into();
    let analysis_id = analysis_id.into();
    let url = format!("/repos/{owner}/{repo}/code-scanning/analyses/{analysis_id}");

    Request::<(), CodeScanningDeleteAnalysisQuery, CodeScanningAnalysisDeletion>::builder(
      &self.config,
    )
    .delete(url)
    .build()
  }

  /// **List CodeQL databases for a repository**
  ///
  /// Lists the CodeQL databases that are available in a repository.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `security_events` scope to use this endpoint with private or public repositories, or the `public_repo` scope to use this endpoint with only public repositories.
  ///
  /// *Documentation*: [https://docs.github.com/rest/code-scanning/code-scanning#list-codeql-databases-for-a-repository](https://docs.github.com/rest/code-scanning/code-scanning#list-codeql-databases-for-a-repository)
  pub fn list_codeql_databases(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), (), CodeScanningCodeqlDatabaseArray> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/code-scanning/codeql/databases");

    Request::<(), (), CodeScanningCodeqlDatabaseArray>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get a CodeQL database for a repository**
  ///
  /// Gets a CodeQL database for a language in a repository.
  ///
  /// By default this endpoint returns JSON metadata about the CodeQL database. To
  /// download the CodeQL database binary content, set the `Accept` header of the request
  /// to [`application/zip`](https://docs.github.com/rest/overview/media-types), and make sure
  /// your HTTP client is configured to follow redirects or use the `Location` header
  /// to make a second request to get the redirect URL.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `security_events` scope to use this endpoint with private or public repositories, or the `public_repo` scope to use this endpoint with only public repositories.
  ///
  /// *Documentation*: [https://docs.github.com/rest/code-scanning/code-scanning#get-a-codeql-database-for-a-repository](https://docs.github.com/rest/code-scanning/code-scanning#get-a-codeql-database-for-a-repository)
  pub fn get_codeql_database(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    language: impl Into<String>,
  ) -> Request<(), (), CodeScanningCodeqlDatabase> {
    let owner = owner.into();
    let repo = repo.into();
    let language = language.into();
    let url = format!("/repos/{owner}/{repo}/code-scanning/codeql/databases/{language}");

    Request::<(), (), CodeScanningCodeqlDatabase>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get a code scanning default setup configuration**
  ///
  /// Gets a code scanning default setup configuration.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with private or public repositories, or the `public_repo` scope to use this endpoint with only public repositories.
  ///
  /// *Documentation*: [https://docs.github.com/rest/code-scanning/code-scanning#get-a-code-scanning-default-setup-configuration](https://docs.github.com/rest/code-scanning/code-scanning#get-a-code-scanning-default-setup-configuration)
  pub fn get_default_setup(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), (), CodeScanningDefaultSetup> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/code-scanning/default-setup");

    Request::<(), (), CodeScanningDefaultSetup>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Update a code scanning default setup configuration**
  ///
  /// Updates a code scanning default setup configuration.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with private or public repositories, or the `public_repo` scope to use this endpoint with only public repositories.
  ///
  /// *Documentation*: [https://docs.github.com/rest/code-scanning/code-scanning#update-a-code-scanning-default-setup-configuration](https://docs.github.com/rest/code-scanning/code-scanning#update-a-code-scanning-default-setup-configuration)
  pub fn update_default_setup(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<CodeScanningDefaultSetupUpdate, (), CodeScanningUpdateDefaultSetupResponse> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/code-scanning/default-setup");

    Request::<CodeScanningDefaultSetupUpdate, (), CodeScanningUpdateDefaultSetupResponse>::builder(
      &self.config,
    )
    .patch(url)
    .build()
  }

  /// **Upload an analysis as SARIF data**
  ///
  /// Uploads SARIF data containing the results of a code scanning analysis to make the results available in a repository. For troubleshooting information, see "[Troubleshooting SARIF uploads](https://docs.github.com/code-security/code-scanning/troubleshooting-sarif)."
  ///
  /// There are two places where you can upload code scanning results.
  ///  - If you upload to a pull request, for example `--ref refs/pull/42/merge` or `--ref refs/pull/42/head`, then the results appear as alerts in a pull request check. For more information, see "[Triaging code scanning alerts in pull requests](/code-security/secure-coding/triaging-code-scanning-alerts-in-pull-requests)."
  ///  - If you upload to a branch, for example `--ref refs/heads/my-branch`, then the results appear in the **Security** tab for your repository. For more information, see "[Managing code scanning alerts for your repository](/code-security/secure-coding/managing-code-scanning-alerts-for-your-repository#viewing-the-alerts-for-a-repository)."
  ///
  /// You must compress the SARIF-formatted analysis data that you want to upload, using `gzip`, and then encode it as a Base64 format string. For example:
  ///
  /// ```
  /// gzip -c analysis-data.sarif | base64 -w0
  /// ```
  ///
  /// SARIF upload supports a maximum number of entries per the following data objects, and an analysis will be rejected if any of these objects is above its maximum value. For some objects, there are additional values over which the entries will be ignored while keeping the most important entries whenever applicable.
  /// To get the most out of your analysis when it includes data above the supported limits, try to optimize the analysis configuration. For example, for the CodeQL tool, identify and remove the most noisy queries. For more information, see "[SARIF results exceed one or more limits](https://docs.github.com/code-security/code-scanning/troubleshooting-sarif/results-exceed-limit)."
  ///
  ///
  /// | **SARIF data**                   | **Maximum values** | **Additional limits**                                                            |
  /// |----------------------------------|:------------------:|----------------------------------------------------------------------------------|
  /// | Runs per file                    |         20         |                                                                                  |
  /// | Results per run                  |       25,000       | Only the top 5,000 results will be included, prioritized by severity.            |
  /// | Rules per run                    |       25,000       |                                                                                  |
  /// | Tool extensions per run          |        100         |                                                                                  |
  /// | Thread Flow Locations per result |       10,000       | Only the top 1,000 Thread Flow Locations will be included, using prioritization. |
  /// | Location per result	             |       1,000        | Only 100 locations will be included.                                             |
  /// | Tags per rule	                   |         20         | Only 10 tags will be included.                                                   |
  ///
  ///
  /// The `202 Accepted` response includes an `id` value.
  /// You can use this ID to check the status of the upload by using it in the `/sarifs/{sarif_id}` endpoint.
  /// For more information, see "[Get information about a SARIF upload](/rest/code-scanning/code-scanning#get-information-about-a-sarif-upload)."
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `security_events` scope to use this endpoint with private or public repositories, or the `public_repo` scope to use this endpoint with only public repositories.
  ///
  /// *Documentation*: [https://docs.github.com/rest/code-scanning/code-scanning#upload-an-analysis-as-sarif-data](https://docs.github.com/rest/code-scanning/code-scanning#upload-an-analysis-as-sarif-data)
  pub fn upload_sarif(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<CodeScanningUploadSarifRequest, (), CodeScanningSarifsReceipt> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/code-scanning/sarifs");

    Request::<CodeScanningUploadSarifRequest, (), CodeScanningSarifsReceipt>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Get information about a SARIF upload**
  ///
  /// Gets information about a SARIF upload, including the status and the URL of the analysis that was uploaded so that you can retrieve details of the analysis. For more information, see "[Get a code scanning analysis for a repository](/rest/code-scanning/code-scanning#get-a-code-scanning-analysis-for-a-repository)."
  /// OAuth app tokens and personal access tokens (classic) need the `security_events` scope to use this endpoint with private or public repositories, or the `public_repo` scope to use this endpoint with only public repositories.
  ///
  /// *Documentation*: [https://docs.github.com/rest/code-scanning/code-scanning#get-information-about-a-sarif-upload](https://docs.github.com/rest/code-scanning/code-scanning#get-information-about-a-sarif-upload)
  pub fn get_sarif(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    sarif_id: impl Into<String>,
  ) -> Request<(), (), CodeScanningSarifsStatus> {
    let owner = owner.into();
    let repo = repo.into();
    let sarif_id = sarif_id.into();
    let url = format!("/repos/{owner}/{repo}/code-scanning/sarifs/{sarif_id}");

    Request::<(), (), CodeScanningSarifsStatus>::builder(&self.config)
      .get(url)
      .build()
  }
}
