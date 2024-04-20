#[allow(unused_imports)]
use crate::types::*;
use octocrate_core::*;

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
  ) -> Request<(), (), EmptyObject> {
    let owner = owner.into();
    let repo = repo.into();
    let check_run_id = check_run_id.into();
    let url = format!("/repos/{owner}/{repo}/check-runs/{check_run_id}/rerequest");

    Request::<(), (), EmptyObject>::builder(&self.config)
      .post(url)
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
  ) -> Request<(), ChecksListAnnotationsQuery, CheckAnnotationArray> {
    let owner = owner.into();
    let repo = repo.into();
    let check_run_id = check_run_id.into();
    let url = format!("/repos/{owner}/{repo}/check-runs/{check_run_id}/annotations");

    Request::<(), ChecksListAnnotationsQuery, CheckAnnotationArray>::builder(&self.config)
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
  ) -> Request<(), (), EmptyObject> {
    let owner = owner.into();
    let repo = repo.into();
    let check_suite_id = check_suite_id.into();
    let url = format!("/repos/{owner}/{repo}/check-suites/{check_suite_id}/rerequest");

    Request::<(), (), EmptyObject>::builder(&self.config)
      .post(url)
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
  ) -> Request<(), ChecksListForSuiteQuery, ChecksListForSuiteResponse> {
    let owner = owner.into();
    let repo = repo.into();
    let check_suite_id = check_suite_id.into();
    let url = format!("/repos/{owner}/{repo}/check-suites/{check_suite_id}/check-runs");

    Request::<(), ChecksListForSuiteQuery, ChecksListForSuiteResponse>::builder(&self.config)
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
  ) -> Request<(), ChecksListSuitesForRefQuery, ChecksListSuitesForRefResponse> {
    let owner = owner.into();
    let repo = repo.into();
    let ref_ = ref_.into();
    let url = format!("/repos/{owner}/{repo}/commits/{ref_}/check-suites");

    Request::<(), ChecksListSuitesForRefQuery, ChecksListSuitesForRefResponse>::builder(
      &self.config,
    )
    .get(url)
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
  ) -> Request<(), ChecksListForRefQuery, ChecksListForRefResponse> {
    let owner = owner.into();
    let repo = repo.into();
    let ref_ = ref_.into();
    let url = format!("/repos/{owner}/{repo}/commits/{ref_}/check-runs");

    Request::<(), ChecksListForRefQuery, ChecksListForRefResponse>::builder(&self.config)
      .get(url)
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
  ) -> Request<ChecksSetSuitesPreferencesRequest, (), CheckSuitePreference> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/check-suites/preferences");

    Request::<ChecksSetSuitesPreferencesRequest, (), CheckSuitePreference>::builder(&self.config)
      .patch(url)
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
  ) -> Request<ChecksCreateSuiteRequest, (), CheckSuite> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/check-suites");

    Request::<ChecksCreateSuiteRequest, (), CheckSuite>::builder(&self.config)
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
  ) -> Request<(), (), CheckRun> {
    let owner = owner.into();
    let repo = repo.into();
    let check_run_id = check_run_id.into();
    let url = format!("/repos/{owner}/{repo}/check-runs/{check_run_id}");

    Request::<(), (), CheckRun>::builder(&self.config)
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
  ) -> Request<ChecksUpdateRequest, (), CheckRun> {
    let owner = owner.into();
    let repo = repo.into();
    let check_run_id = check_run_id.into();
    let url = format!("/repos/{owner}/{repo}/check-runs/{check_run_id}");

    Request::<ChecksUpdateRequest, (), CheckRun>::builder(&self.config)
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
  ) -> Request<(), (), CheckSuite> {
    let owner = owner.into();
    let repo = repo.into();
    let check_suite_id = check_suite_id.into();
    let url = format!("/repos/{owner}/{repo}/check-suites/{check_suite_id}");

    Request::<(), (), CheckSuite>::builder(&self.config)
      .get(url)
      .build()
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
  ) -> Request<ChecksCreateRequest, (), CheckRun> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/check-runs");

    Request::<ChecksCreateRequest, (), CheckRun>::builder(&self.config)
      .post(url)
      .build()
  }
}
