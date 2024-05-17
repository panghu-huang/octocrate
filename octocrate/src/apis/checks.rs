use octocrate_core::*;
#[allow(unused_imports)]
use octocrate_types::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use typed_builder::TypedBuilder;

pub mod create {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = CheckRun;

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum RequestItem1Status {
    #[serde(rename = "completed")]
    Completed,
  }

  impl ToString for RequestItem1Status {
    fn to_string(&self) -> String {
      match self {
        RequestItem1Status::Completed => "completed".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum RequestItem2Status {
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "in_progress")]
    InProgress,
  }

  impl ToString for RequestItem2Status {
    fn to_string(&self) -> String {
      match self {
        RequestItem2Status::Queued => "queued".to_string(),
        RequestItem2Status::InProgress => "in_progress".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize)]
  #[serde(untagged)]
  pub enum Request {
    RequestItem1(RequestItem1),
    RequestItem2(RequestItem2),
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct RequestItem1 {
    pub status: RequestItem1Status,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct RequestItem2 {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub status: Option<RequestItem2Status>,
  }
}

pub mod get {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = CheckRun;
}

pub mod update {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = CheckRun;

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum RequestItem1Status {
    #[serde(rename = "completed")]
    Completed,
  }

  impl ToString for RequestItem1Status {
    fn to_string(&self) -> String {
      match self {
        RequestItem1Status::Completed => "completed".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum RequestItem2Status {
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "in_progress")]
    InProgress,
  }

  impl ToString for RequestItem2Status {
    fn to_string(&self) -> String {
      match self {
        RequestItem2Status::Queued => "queued".to_string(),
        RequestItem2Status::InProgress => "in_progress".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize)]
  #[serde(untagged)]
  pub enum Request {
    RequestItem1(RequestItem1),
    RequestItem2(RequestItem2),
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct RequestItem1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub status: Option<RequestItem1Status>,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct RequestItem2 {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub status: Option<RequestItem2Status>,
  }
}

pub mod list_annotations {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<CheckAnnotation>;

  /// Query for `List check run annotations`
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
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

pub mod rerequest_run {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = EmptyObject;
}

pub mod create_suite {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = CheckSuite;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The sha of the head commit.
    pub head_sha: String,
  }
}

pub mod set_suites_preferences {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = CheckSuitePreference;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct RequestAutoTriggerChecks {
    /// The `id` of the GitHub App.
    pub app_id: i64,
    /// Set to `true` to enable automatic creation of CheckSuite events upon pushes to the repository, or `false` to disable them.
    pub setting: bool,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// Enables or disables automatic creation of CheckSuite events upon pushes to the repository. Enabled by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub auto_trigger_checks: Option<Vec<RequestAutoTriggerChecks>>,
  }
}

pub mod get_suite {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = CheckSuite;
}

pub mod list_for_suite {
  #[allow(unused_imports)]
  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QueryStatus {
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
  }

  impl ToString for QueryStatus {
    fn to_string(&self) -> String {
      match self {
        QueryStatus::Queued => "queued".to_string(),
        QueryStatus::InProgress => "in_progress".to_string(),
        QueryStatus::Completed => "completed".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QueryFilter {
    #[serde(rename = "latest")]
    Latest,
    #[serde(rename = "all")]
    All,
  }

  impl ToString for QueryFilter {
    fn to_string(&self) -> String {
      match self {
        QueryFilter::Latest => "latest".to_string(),
        QueryFilter::All => "all".to_string(),
      }
    }
  }

  /// Query for `List check runs in a check suite`
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// Returns check runs with the specified `name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub check_name: Option<String>,
    /// Returns check runs with the specified `status`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub status: Option<QueryStatus>,
    /// Filters check runs by their `completed_at` timestamp. `latest` returns the most recent check runs.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub filter: Option<QueryFilter>,
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Response {
    pub check_runs: Vec<CheckRun>,
    pub total_count: i64,
  }
}

pub mod rerequest_suite {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = EmptyObject;
}

pub mod list_for_ref {
  #[allow(unused_imports)]
  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QueryStatus {
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
  }

  impl ToString for QueryStatus {
    fn to_string(&self) -> String {
      match self {
        QueryStatus::Queued => "queued".to_string(),
        QueryStatus::InProgress => "in_progress".to_string(),
        QueryStatus::Completed => "completed".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QueryFilter {
    #[serde(rename = "latest")]
    Latest,
    #[serde(rename = "all")]
    All,
  }

  impl ToString for QueryFilter {
    fn to_string(&self) -> String {
      match self {
        QueryFilter::Latest => "latest".to_string(),
        QueryFilter::All => "all".to_string(),
      }
    }
  }

  /// Query for `List check runs for a Git reference`
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// Returns check runs with the specified `name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub check_name: Option<String>,
    /// Returns check runs with the specified `status`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub status: Option<QueryStatus>,
    /// Filters check runs by their `completed_at` timestamp. `latest` returns the most recent check runs.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub filter: Option<QueryFilter>,
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub app_id: Option<i64>,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Response {
    pub check_runs: Vec<CheckRun>,
    pub total_count: i64,
  }
}

pub mod list_suites_for_ref {
  #[allow(unused_imports)]
  use super::*;

  /// Query for `List check suites for a Git reference`
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// Filters check suites by GitHub App `id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub app_id: Option<i64>,
    /// Returns check runs with the specified `name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub check_name: Option<String>,
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Response {
    pub check_suites: Vec<CheckSuite>,
    pub total_count: i64,
  }
}

/// Rich interactions with checks run by your integrations.
pub struct GitHubChecksAPI {
  config: SharedAPIConfig,
}

impl GitHubChecksAPI {
  pub fn new(config: &SharedAPIConfig) -> Self {
    Self {
      config: config.clone(),
    }
  }

  /// **Create a check run**
  ///
  /// Creates a new check run for a specific commit in a repository.
  ///
  /// To create a check run, you must use a GitHub App. OAuth apps and authenticated users are not able to create a check suite.
  ///
  /// In a check suite, GitHub limits the number of check runs with the same name to 1000. Once these check runs exceed 1000, GitHub will start to automatically delete older check runs.
  ///
  /// **Note:** The Checks API only looks for pushes in the repository where the check suite or check run were created. Pushes to a branch in a forked repository are not detected and return an empty `pull_requests` array.
  ///
  /// *Documentation*: [https://docs.github.com/rest/checks/runs#create-a-check-run](https://docs.github.com/rest/checks/runs#create-a-check-run)
  pub fn create(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<create::Request, (), create::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/check-runs");

    Request::<create::Request, (), create::Response>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Get a check run**
  ///
  /// Gets a single check run using its `id`.
  ///
  /// **Note:** The Checks API only looks for pushes in the repository where the check suite or check run were created. Pushes to a branch in a forked repository are not detected and return an empty `pull_requests` array.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint on a private repository.
  ///
  /// *Documentation*: [https://docs.github.com/rest/checks/runs#get-a-check-run](https://docs.github.com/rest/checks/runs#get-a-check-run)
  pub fn get(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    check_run_id: impl Into<i64>,
  ) -> Request<(), (), get::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let check_run_id = check_run_id.into();
    let url = format!("/repos/{owner}/{repo}/check-runs/{check_run_id}");

    Request::<(), (), get::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Update a check run**
  ///
  /// Updates a check run for a specific commit in a repository.
  ///
  /// **Note:** The endpoints to manage checks only look for pushes in the repository where the check suite or check run were created. Pushes to a branch in a forked repository are not detected and return an empty `pull_requests` array.
  ///
  /// OAuth apps and personal access tokens (classic) cannot use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/checks/runs#update-a-check-run](https://docs.github.com/rest/checks/runs#update-a-check-run)
  pub fn update(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    check_run_id: impl Into<i64>,
  ) -> Request<update::Request, (), update::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let check_run_id = check_run_id.into();
    let url = format!("/repos/{owner}/{repo}/check-runs/{check_run_id}");

    Request::<update::Request, (), update::Response>::builder(&self.config)
      .patch(url)
      .build()
  }

  /// **List check run annotations**
  ///
  /// Lists annotations for a check run using the annotation `id`.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint on a private repository.
  ///
  /// *Documentation*: [https://docs.github.com/rest/checks/runs#list-check-run-annotations](https://docs.github.com/rest/checks/runs#list-check-run-annotations)
  pub fn list_annotations(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    check_run_id: impl Into<i64>,
  ) -> Request<(), list_annotations::Query, list_annotations::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let check_run_id = check_run_id.into();
    let url = format!("/repos/{owner}/{repo}/check-runs/{check_run_id}/annotations");

    Request::<(), list_annotations::Query, list_annotations::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Rerequest a check run**
  ///
  /// Triggers GitHub to rerequest an existing check run, without pushing new code to a repository. This endpoint will trigger the [`check_run` webhook](https://docs.github.com/webhooks/event-payloads/#check_run) event with the action `rerequested`. When a check run is `rerequested`, its `status` is reset to `queued` and the `conclusion` is cleared.
  ///
  /// For more information about how to re-run GitHub Actions jobs, see "[Re-run a job from a workflow run](https://docs.github.com/rest/actions/workflow-runs#re-run-a-job-from-a-workflow-run)".
  ///
  /// OAuth apps and personal access tokens (classic) cannot use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/checks/runs#rerequest-a-check-run](https://docs.github.com/rest/checks/runs#rerequest-a-check-run)
  pub fn rerequest_run(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    check_run_id: impl Into<i64>,
  ) -> Request<(), (), rerequest_run::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let check_run_id = check_run_id.into();
    let url = format!("/repos/{owner}/{repo}/check-runs/{check_run_id}/rerequest");

    Request::<(), (), rerequest_run::Response>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Create a check suite**
  ///
  /// Creates a check suite manually. By default, check suites are automatically created when you create a [check run](https://docs.github.com/rest/checks/runs). You only need to use this endpoint for manually creating check suites when you've disabled automatic creation using "[Update repository preferences for check suites](https://docs.github.com/rest/checks/suites#update-repository-preferences-for-check-suites)".
  ///
  /// **Note:** The Checks API only looks for pushes in the repository where the check suite or check run were created. Pushes to a branch in a forked repository are not detected and return an empty `pull_requests` array and a `null` value for `head_branch`.
  ///
  /// OAuth apps and personal access tokens (classic) cannot use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/checks/suites#create-a-check-suite](https://docs.github.com/rest/checks/suites#create-a-check-suite)
  pub fn create_suite(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<create_suite::Request, (), create_suite::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/check-suites");

    Request::<create_suite::Request, (), create_suite::Response>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Update repository preferences for check suites**
  ///
  /// Changes the default automatic flow when creating check suites. By default, a check suite is automatically created each time code is pushed to a repository. When you disable the automatic creation of check suites, you can manually [Create a check suite](https://docs.github.com/rest/checks/suites#create-a-check-suite).
  /// You must have admin permissions in the repository to set preferences for check suites.
  ///
  /// *Documentation*: [https://docs.github.com/rest/checks/suites#update-repository-preferences-for-check-suites](https://docs.github.com/rest/checks/suites#update-repository-preferences-for-check-suites)
  pub fn set_suites_preferences(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<set_suites_preferences::Request, (), set_suites_preferences::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/check-suites/preferences");

    Request::<set_suites_preferences::Request, (), set_suites_preferences::Response>::builder(
      &self.config,
    )
    .patch(url)
    .build()
  }

  /// **Get a check suite**
  ///
  /// Gets a single check suite using its `id`.
  ///
  /// **Note:** The Checks API only looks for pushes in the repository where the check suite or check run were created. Pushes to a branch in a forked repository are not detected and return an empty `pull_requests` array and a `null` value for `head_branch`.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint on a private repository.
  ///
  /// *Documentation*: [https://docs.github.com/rest/checks/suites#get-a-check-suite](https://docs.github.com/rest/checks/suites#get-a-check-suite)
  pub fn get_suite(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    check_suite_id: impl Into<i64>,
  ) -> Request<(), (), get_suite::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let check_suite_id = check_suite_id.into();
    let url = format!("/repos/{owner}/{repo}/check-suites/{check_suite_id}");

    Request::<(), (), get_suite::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List check runs in a check suite**
  ///
  /// Lists check runs for a check suite using its `id`.
  ///
  /// **Note:** The endpoints to manage checks only look for pushes in the repository where the check suite or check run were created. Pushes to a branch in a forked repository are not detected and return an empty `pull_requests` array.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint on a private repository.
  ///
  /// *Documentation*: [https://docs.github.com/rest/checks/runs#list-check-runs-in-a-check-suite](https://docs.github.com/rest/checks/runs#list-check-runs-in-a-check-suite)
  pub fn list_for_suite(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    check_suite_id: impl Into<i64>,
  ) -> Request<(), list_for_suite::Query, list_for_suite::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let check_suite_id = check_suite_id.into();
    let url = format!("/repos/{owner}/{repo}/check-suites/{check_suite_id}/check-runs");

    Request::<(), list_for_suite::Query, list_for_suite::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Rerequest a check suite**
  ///
  /// Triggers GitHub to rerequest an existing check suite, without pushing new code to a repository. This endpoint will trigger the [`check_suite` webhook](https://docs.github.com/webhooks/event-payloads/#check_suite) event with the action `rerequested`. When a check suite is `rerequested`, its `status` is reset to `queued` and the `conclusion` is cleared.
  ///
  /// OAuth apps and personal access tokens (classic) cannot use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/checks/suites#rerequest-a-check-suite](https://docs.github.com/rest/checks/suites#rerequest-a-check-suite)
  pub fn rerequest_suite(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    check_suite_id: impl Into<i64>,
  ) -> Request<(), (), rerequest_suite::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let check_suite_id = check_suite_id.into();
    let url = format!("/repos/{owner}/{repo}/check-suites/{check_suite_id}/rerequest");

    Request::<(), (), rerequest_suite::Response>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **List check runs for a Git reference**
  ///
  /// Lists check runs for a commit ref. The `ref` can be a SHA, branch name, or a tag name.
  ///
  /// **Note:** The endpoints to manage checks only look for pushes in the repository where the check suite or check run were created. Pushes to a branch in a forked repository are not detected and return an empty `pull_requests` array.
  ///
  /// If there are more than 1000 check suites on a single git reference, this endpoint will limit check runs to the 1000 most recent check suites. To iterate over all possible check runs, use the [List check suites for a Git reference](https://docs.github.com/rest/reference/checks#list-check-suites-for-a-git-reference) endpoint and provide the `check_suite_id` parameter to the [List check runs in a check suite](https://docs.github.com/rest/reference/checks#list-check-runs-in-a-check-suite) endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint on a private repository.
  ///
  /// *Documentation*: [https://docs.github.com/rest/checks/runs#list-check-runs-for-a-git-reference](https://docs.github.com/rest/checks/runs#list-check-runs-for-a-git-reference)
  pub fn list_for_ref(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    ref_: impl Into<String>,
  ) -> Request<(), list_for_ref::Query, list_for_ref::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let ref_ = ref_.into();
    let url = format!("/repos/{owner}/{repo}/commits/{ref_}/check-runs");

    Request::<(), list_for_ref::Query, list_for_ref::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List check suites for a Git reference**
  ///
  /// Lists check suites for a commit `ref`. The `ref` can be a SHA, branch name, or a tag name.
  ///
  /// **Note:** The endpoints to manage checks only look for pushes in the repository where the check suite or check run were created. Pushes to a branch in a forked repository are not detected and return an empty `pull_requests` array and a `null` value for `head_branch`.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint on a private repository.
  ///
  /// *Documentation*: [https://docs.github.com/rest/checks/suites#list-check-suites-for-a-git-reference](https://docs.github.com/rest/checks/suites#list-check-suites-for-a-git-reference)
  pub fn list_suites_for_ref(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    ref_: impl Into<String>,
  ) -> Request<(), list_suites_for_ref::Query, list_suites_for_ref::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let ref_ = ref_.into();
    let url = format!("/repos/{owner}/{repo}/commits/{ref_}/check-suites");

    Request::<(), list_suites_for_ref::Query, list_suites_for_ref::Response>::builder(&self.config)
      .get(url)
      .build()
  }
}
