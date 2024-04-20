use octocrate_core::*;
#[allow(unused_imports)]
use octocrate_types::*;

/// Interact with GitHub Issues.
pub struct GitHubIssuesAPI {
  config: SharedAPIConfig,
}

impl GitHubIssuesAPI {
  pub fn new(config: &SharedAPIConfig) -> Self {
    Self {
      config: config.clone(),
    }
  }

  /// **List issues assigned to the authenticated user**
  ///
  /// List issues assigned to the authenticated user across all visible repositories including owned repositories, member
  /// repositories, and organization repositories. You can use the `filter` query parameter to fetch issues that are not
  /// necessarily assigned to you.
  ///
  /// **Note**: GitHub's REST API considers every pull request an issue, but not every issue is a pull request. For this
  /// reason, "Issues" endpoints may return both issues and pull requests in the response. You can identify pull requests by
  /// the `pull_request` key. Be aware that the `id` of a pull request returned from "Issues" endpoints will be an _issue id_. To find out the pull
  /// request id, use the "[List pull requests](https://docs.github.com/rest/pulls/pulls#list-pull-requests)" endpoint.
  ///
  /// This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
  ///
  /// - **`application/vnd.github.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
  /// - **`application/vnd.github.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
  /// - **`application/vnd.github.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
  /// - **`application/vnd.github.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
  ///
  /// *Documentation*: [https://docs.github.com/rest/issues/issues#list-issues-assigned-to-the-authenticated-user](https://docs.github.com/rest/issues/issues#list-issues-assigned-to-the-authenticated-user)
  pub fn list(&self) -> Request<(), IssuesListQuery, IssueArray> {
    let url = format!("/issues");

    Request::<(), IssuesListQuery, IssueArray>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List organization issues assigned to the authenticated user**
  ///
  /// List issues in an organization assigned to the authenticated user.
  ///
  /// **Note**: GitHub's REST API considers every pull request an issue, but not every issue is a pull request. For this
  /// reason, "Issues" endpoints may return both issues and pull requests in the response. You can identify pull requests by
  /// the `pull_request` key. Be aware that the `id` of a pull request returned from "Issues" endpoints will be an _issue id_. To find out the pull
  /// request id, use the "[List pull requests](https://docs.github.com/rest/pulls/pulls#list-pull-requests)" endpoint.
  ///
  /// This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
  ///
  /// - **`application/vnd.github.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
  /// - **`application/vnd.github.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
  /// - **`application/vnd.github.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
  /// - **`application/vnd.github.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
  ///
  /// *Documentation*: [https://docs.github.com/rest/issues/issues#list-organization-issues-assigned-to-the-authenticated-user](https://docs.github.com/rest/issues/issues#list-organization-issues-assigned-to-the-authenticated-user)
  pub fn list_for_org(
    &self,
    org: impl Into<String>,
  ) -> Request<(), IssuesListForOrgQuery, IssueArray> {
    let org = org.into();
    let url = format!("/orgs/{org}/issues");

    Request::<(), IssuesListForOrgQuery, IssueArray>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List assignees**
  ///
  /// Lists the [available assignees](https://docs.github.com/articles/assigning-issues-and-pull-requests-to-other-github-users/) for issues in a repository.
  ///
  /// *Documentation*: [https://docs.github.com/rest/issues/assignees#list-assignees](https://docs.github.com/rest/issues/assignees#list-assignees)
  pub fn list_assignees(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), IssuesListAssigneesQuery, SimpleUserArray> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/assignees");

    Request::<(), IssuesListAssigneesQuery, SimpleUserArray>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Check if a user can be assigned**
  ///
  /// Checks if a user has permission to be assigned to an issue in this repository.
  ///
  /// If the `assignee` can be assigned to issues in the repository, a `204` header with no content is returned.
  ///
  /// Otherwise a `404` status code is returned.
  ///
  /// *Documentation*: [https://docs.github.com/rest/issues/assignees#check-if-a-user-can-be-assigned](https://docs.github.com/rest/issues/assignees#check-if-a-user-can-be-assigned)
  pub fn check_user_can_be_assigned(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    assignee: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let assignee = assignee.into();
    let url = format!("/repos/{owner}/{repo}/assignees/{assignee}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List repository issues**
  ///
  /// List issues in a repository. Only open issues will be listed.
  ///
  /// **Note**: GitHub's REST API considers every pull request an issue, but not every issue is a pull request. For this
  /// reason, "Issues" endpoints may return both issues and pull requests in the response. You can identify pull requests by
  /// the `pull_request` key. Be aware that the `id` of a pull request returned from "Issues" endpoints will be an _issue id_. To find out the pull
  /// request id, use the "[List pull requests](https://docs.github.com/rest/pulls/pulls#list-pull-requests)" endpoint.
  ///
  /// This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
  ///
  /// - **`application/vnd.github.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
  /// - **`application/vnd.github.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
  /// - **`application/vnd.github.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
  /// - **`application/vnd.github.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
  ///
  /// *Documentation*: [https://docs.github.com/rest/issues/issues#list-repository-issues](https://docs.github.com/rest/issues/issues#list-repository-issues)
  pub fn list_for_repo(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), IssuesListForRepoQuery, IssueArray> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/issues");

    Request::<(), IssuesListForRepoQuery, IssueArray>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create an issue**
  ///
  /// Any user with pull access to a repository can create an issue. If [issues are disabled in the repository](https://docs.github.com/articles/disabling-issues/), the API returns a `410 Gone` status.
  ///
  /// This endpoint triggers [notifications](https://docs.github.com/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. For more information, see "[Rate limits for the API](https://docs.github.com/rest/overview/rate-limits-for-the-rest-api#about-secondary-rate-limits)"
  /// and "[Best practices for using the REST API](https://docs.github.com/rest/guides/best-practices-for-using-the-rest-api)."
  ///
  /// This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
  ///
  /// - **`application/vnd.github.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
  /// - **`application/vnd.github.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
  /// - **`application/vnd.github.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
  /// - **`application/vnd.github.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
  ///
  /// *Documentation*: [https://docs.github.com/rest/issues/issues#create-an-issue](https://docs.github.com/rest/issues/issues#create-an-issue)
  pub fn create(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<IssuesCreateRequest, (), Issue> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/issues");

    Request::<IssuesCreateRequest, (), Issue>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **List issue comments for a repository**
  ///
  /// You can use the REST API to list comments on issues and pull requests for a repository. Every pull request is an issue, but not every issue is a pull request.
  ///
  /// By default, issue comments are ordered by ascending ID.
  ///
  /// This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
  ///
  /// - **`application/vnd.github.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
  /// - **`application/vnd.github.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
  /// - **`application/vnd.github.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
  /// - **`application/vnd.github.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
  ///
  /// *Documentation*: [https://docs.github.com/rest/issues/comments#list-issue-comments-for-a-repository](https://docs.github.com/rest/issues/comments#list-issue-comments-for-a-repository)
  pub fn list_comments_for_repo(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), IssuesListCommentsForRepoQuery, IssueCommentArray> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/issues/comments");

    Request::<(), IssuesListCommentsForRepoQuery, IssueCommentArray>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get an issue comment**
  ///
  /// You can use the REST API to get comments on issues and pull requests. Every pull request is an issue, but not every issue is a pull request.
  ///
  /// This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
  ///
  /// - **`application/vnd.github.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
  /// - **`application/vnd.github.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
  /// - **`application/vnd.github.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
  /// - **`application/vnd.github.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
  ///
  /// *Documentation*: [https://docs.github.com/rest/issues/comments#get-an-issue-comment](https://docs.github.com/rest/issues/comments#get-an-issue-comment)
  pub fn get_comment(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    comment_id: impl Into<i64>,
  ) -> Request<(), (), IssueComment> {
    let owner = owner.into();
    let repo = repo.into();
    let comment_id = comment_id.into();
    let url = format!("/repos/{owner}/{repo}/issues/comments/{comment_id}");

    Request::<(), (), IssueComment>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Update an issue comment**
  ///
  /// You can use the REST API to update comments on issues and pull requests. Every pull request is an issue, but not every issue is a pull request.
  ///
  /// This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
  ///
  /// - **`application/vnd.github.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
  /// - **`application/vnd.github.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
  /// - **`application/vnd.github.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
  /// - **`application/vnd.github.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
  ///
  /// *Documentation*: [https://docs.github.com/rest/issues/comments#update-an-issue-comment](https://docs.github.com/rest/issues/comments#update-an-issue-comment)
  pub fn update_comment(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    comment_id: impl Into<i64>,
  ) -> Request<IssuesUpdateCommentRequest, (), IssueComment> {
    let owner = owner.into();
    let repo = repo.into();
    let comment_id = comment_id.into();
    let url = format!("/repos/{owner}/{repo}/issues/comments/{comment_id}");

    Request::<IssuesUpdateCommentRequest, (), IssueComment>::builder(&self.config)
      .patch(url)
      .build()
  }

  /// **Delete an issue comment**
  ///
  /// You can use the REST API to delete comments on issues and pull requests. Every pull request is an issue, but not every issue is a pull request.
  ///
  /// *Documentation*: [https://docs.github.com/rest/issues/comments#delete-an-issue-comment](https://docs.github.com/rest/issues/comments#delete-an-issue-comment)
  pub fn delete_comment(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    comment_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let comment_id = comment_id.into();
    let url = format!("/repos/{owner}/{repo}/issues/comments/{comment_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List issue events for a repository**
  ///
  /// Lists events for a repository.
  ///
  /// *Documentation*: [https://docs.github.com/rest/issues/events#list-issue-events-for-a-repository](https://docs.github.com/rest/issues/events#list-issue-events-for-a-repository)
  pub fn list_events_for_repo(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), IssuesListEventsForRepoQuery, IssueEventArray> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/issues/events");

    Request::<(), IssuesListEventsForRepoQuery, IssueEventArray>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get an issue event**
  ///
  /// Gets a single event by the event id.
  ///
  /// *Documentation*: [https://docs.github.com/rest/issues/events#get-an-issue-event](https://docs.github.com/rest/issues/events#get-an-issue-event)
  pub fn get_event(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    event_id: impl Into<i64>,
  ) -> Request<(), (), IssueEvent> {
    let owner = owner.into();
    let repo = repo.into();
    let event_id = event_id.into();
    let url = format!("/repos/{owner}/{repo}/issues/events/{event_id}");

    Request::<(), (), IssueEvent>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get an issue**
  ///
  /// The API returns a [`301 Moved Permanently` status](https://docs.github.com/rest/guides/best-practices-for-using-the-rest-api#follow-redirects) if the issue was
  /// [transferred](https://docs.github.com/articles/transferring-an-issue-to-another-repository/) to another repository. If
  /// the issue was transferred to or deleted from a repository where the authenticated user lacks read access, the API
  /// returns a `404 Not Found` status. If the issue was deleted from a repository where the authenticated user has read
  /// access, the API returns a `410 Gone` status. To receive webhook events for transferred and deleted issues, subscribe
  /// to the [`issues`](https://docs.github.com/webhooks/event-payloads/#issues) webhook.
  ///
  /// **Note**: GitHub's REST API considers every pull request an issue, but not every issue is a pull request. For this
  /// reason, "Issues" endpoints may return both issues and pull requests in the response. You can identify pull requests by
  /// the `pull_request` key. Be aware that the `id` of a pull request returned from "Issues" endpoints will be an _issue id_. To find out the pull
  /// request id, use the "[List pull requests](https://docs.github.com/rest/pulls/pulls#list-pull-requests)" endpoint.
  ///
  /// This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
  ///
  /// - **`application/vnd.github.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
  /// - **`application/vnd.github.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
  /// - **`application/vnd.github.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
  /// - **`application/vnd.github.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
  ///
  /// *Documentation*: [https://docs.github.com/rest/issues/issues#get-an-issue](https://docs.github.com/rest/issues/issues#get-an-issue)
  pub fn get(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    issue_number: impl Into<i64>,
  ) -> Request<(), (), Issue> {
    let owner = owner.into();
    let repo = repo.into();
    let issue_number = issue_number.into();
    let url = format!("/repos/{owner}/{repo}/issues/{issue_number}");

    Request::<(), (), Issue>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Update an issue**
  ///
  /// Issue owners and users with push access can edit an issue.
  ///
  /// This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
  ///
  /// - **`application/vnd.github.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
  /// - **`application/vnd.github.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
  /// - **`application/vnd.github.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
  /// - **`application/vnd.github.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
  ///
  /// *Documentation*: [https://docs.github.com/rest/issues/issues#update-an-issue](https://docs.github.com/rest/issues/issues#update-an-issue)
  pub fn update(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    issue_number: impl Into<i64>,
  ) -> Request<IssuesUpdateRequest, (), Issue> {
    let owner = owner.into();
    let repo = repo.into();
    let issue_number = issue_number.into();
    let url = format!("/repos/{owner}/{repo}/issues/{issue_number}");

    Request::<IssuesUpdateRequest, (), Issue>::builder(&self.config)
      .patch(url)
      .build()
  }

  /// **Add assignees to an issue**
  ///
  /// Adds up to 10 assignees to an issue. Users already assigned to an issue are not replaced.
  ///
  /// *Documentation*: [https://docs.github.com/rest/issues/assignees#add-assignees-to-an-issue](https://docs.github.com/rest/issues/assignees#add-assignees-to-an-issue)
  pub fn add_assignees(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    issue_number: impl Into<i64>,
  ) -> Request<IssuesAddAssigneesRequest, (), Issue> {
    let owner = owner.into();
    let repo = repo.into();
    let issue_number = issue_number.into();
    let url = format!("/repos/{owner}/{repo}/issues/{issue_number}/assignees");

    Request::<IssuesAddAssigneesRequest, (), Issue>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Remove assignees from an issue**
  ///
  /// Removes one or more assignees from an issue.
  ///
  /// *Documentation*: [https://docs.github.com/rest/issues/assignees#remove-assignees-from-an-issue](https://docs.github.com/rest/issues/assignees#remove-assignees-from-an-issue)
  pub fn remove_assignees(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    issue_number: impl Into<i64>,
  ) -> Request<IssuesRemoveAssigneesRequest, (), Issue> {
    let owner = owner.into();
    let repo = repo.into();
    let issue_number = issue_number.into();
    let url = format!("/repos/{owner}/{repo}/issues/{issue_number}/assignees");

    Request::<IssuesRemoveAssigneesRequest, (), Issue>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Check if a user can be assigned to a issue**
  ///
  /// Checks if a user has permission to be assigned to a specific issue.
  ///
  /// If the `assignee` can be assigned to this issue, a `204` status code with no content is returned.
  ///
  /// Otherwise a `404` status code is returned.
  ///
  /// *Documentation*: [https://docs.github.com/rest/issues/assignees#check-if-a-user-can-be-assigned-to-a-issue](https://docs.github.com/rest/issues/assignees#check-if-a-user-can-be-assigned-to-a-issue)
  pub fn check_user_can_be_assigned_to_issue(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    issue_number: impl Into<i64>,
    assignee: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let issue_number = issue_number.into();
    let assignee = assignee.into();
    let url = format!("/repos/{owner}/{repo}/issues/{issue_number}/assignees/{assignee}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List issue comments**
  ///
  /// You can use the REST API to list comments on issues and pull requests. Every pull request is an issue, but not every issue is a pull request.
  ///
  /// Issue comments are ordered by ascending ID.
  ///
  /// This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
  ///
  /// - **`application/vnd.github.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
  /// - **`application/vnd.github.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
  /// - **`application/vnd.github.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
  /// - **`application/vnd.github.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
  ///
  /// *Documentation*: [https://docs.github.com/rest/issues/comments#list-issue-comments](https://docs.github.com/rest/issues/comments#list-issue-comments)
  pub fn list_comments(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    issue_number: impl Into<i64>,
  ) -> Request<(), IssuesListCommentsQuery, IssueCommentArray> {
    let owner = owner.into();
    let repo = repo.into();
    let issue_number = issue_number.into();
    let url = format!("/repos/{owner}/{repo}/issues/{issue_number}/comments");

    Request::<(), IssuesListCommentsQuery, IssueCommentArray>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create an issue comment**
  ///
  /// You can use the REST API to create comments on issues and pull requests. Every pull request is an issue, but not every issue is a pull request.
  ///
  /// This endpoint triggers [notifications](https://docs.github.com/github/managing-subscriptions-and-notifications-on-github/about-notifications).
  /// Creating content too quickly using this endpoint may result in secondary rate limiting.
  /// For more information, see "[Rate limits for the API](https://docs.github.com/rest/overview/rate-limits-for-the-rest-api#about-secondary-rate-limits)"
  /// and "[Best practices for using the REST API](https://docs.github.com/rest/guides/best-practices-for-using-the-rest-api)."
  ///
  /// This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
  ///
  /// - **`application/vnd.github.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
  /// - **`application/vnd.github.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
  /// - **`application/vnd.github.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
  /// - **`application/vnd.github.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
  ///
  /// *Documentation*: [https://docs.github.com/rest/issues/comments#create-an-issue-comment](https://docs.github.com/rest/issues/comments#create-an-issue-comment)
  pub fn create_comment(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    issue_number: impl Into<i64>,
  ) -> Request<IssuesCreateCommentRequest, (), IssueComment> {
    let owner = owner.into();
    let repo = repo.into();
    let issue_number = issue_number.into();
    let url = format!("/repos/{owner}/{repo}/issues/{issue_number}/comments");

    Request::<IssuesCreateCommentRequest, (), IssueComment>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **List issue events**
  ///
  /// Lists all events for an issue.
  ///
  /// *Documentation*: [https://docs.github.com/rest/issues/events#list-issue-events](https://docs.github.com/rest/issues/events#list-issue-events)
  pub fn list_events(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    issue_number: impl Into<i64>,
  ) -> Request<(), IssuesListEventsQuery, IssueEventForIssueArray> {
    let owner = owner.into();
    let repo = repo.into();
    let issue_number = issue_number.into();
    let url = format!("/repos/{owner}/{repo}/issues/{issue_number}/events");

    Request::<(), IssuesListEventsQuery, IssueEventForIssueArray>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List labels for an issue**
  ///
  /// Lists all labels for an issue.
  ///
  /// *Documentation*: [https://docs.github.com/rest/issues/labels#list-labels-for-an-issue](https://docs.github.com/rest/issues/labels#list-labels-for-an-issue)
  pub fn list_labels_on_issue(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    issue_number: impl Into<i64>,
  ) -> Request<(), IssuesListLabelsOnIssueQuery, LabelArray> {
    let owner = owner.into();
    let repo = repo.into();
    let issue_number = issue_number.into();
    let url = format!("/repos/{owner}/{repo}/issues/{issue_number}/labels");

    Request::<(), IssuesListLabelsOnIssueQuery, LabelArray>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Add labels to an issue**
  ///
  /// Adds labels to an issue. If you provide an empty array of labels, all labels are removed from the issue.
  ///
  /// *Documentation*: [https://docs.github.com/rest/issues/labels#add-labels-to-an-issue](https://docs.github.com/rest/issues/labels#add-labels-to-an-issue)
  pub fn add_labels(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    issue_number: impl Into<i64>,
  ) -> Request<IssuesAddLabelsRequest, (), LabelArray> {
    let owner = owner.into();
    let repo = repo.into();
    let issue_number = issue_number.into();
    let url = format!("/repos/{owner}/{repo}/issues/{issue_number}/labels");

    Request::<IssuesAddLabelsRequest, (), LabelArray>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Set labels for an issue**
  ///
  /// Removes any previous labels and sets the new labels for an issue.
  ///
  /// *Documentation*: [https://docs.github.com/rest/issues/labels#set-labels-for-an-issue](https://docs.github.com/rest/issues/labels#set-labels-for-an-issue)
  pub fn set_labels(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    issue_number: impl Into<i64>,
  ) -> Request<IssuesSetLabelsRequest, (), LabelArray> {
    let owner = owner.into();
    let repo = repo.into();
    let issue_number = issue_number.into();
    let url = format!("/repos/{owner}/{repo}/issues/{issue_number}/labels");

    Request::<IssuesSetLabelsRequest, (), LabelArray>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **Remove all labels from an issue**
  ///
  /// Removes all labels from an issue.
  ///
  /// *Documentation*: [https://docs.github.com/rest/issues/labels#remove-all-labels-from-an-issue](https://docs.github.com/rest/issues/labels#remove-all-labels-from-an-issue)
  pub fn remove_all_labels(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    issue_number: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let issue_number = issue_number.into();
    let url = format!("/repos/{owner}/{repo}/issues/{issue_number}/labels");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Remove a label from an issue**
  ///
  /// Removes the specified label from the issue, and returns the remaining labels on the issue. This endpoint returns a `404 Not Found` status if the label does not exist.
  ///
  /// *Documentation*: [https://docs.github.com/rest/issues/labels#remove-a-label-from-an-issue](https://docs.github.com/rest/issues/labels#remove-a-label-from-an-issue)
  pub fn remove_label(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    issue_number: impl Into<i64>,
    name: impl Into<String>,
  ) -> Request<(), (), LabelArray> {
    let owner = owner.into();
    let repo = repo.into();
    let issue_number = issue_number.into();
    let name = name.into();
    let url = format!("/repos/{owner}/{repo}/issues/{issue_number}/labels/{name}");

    Request::<(), (), LabelArray>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Lock an issue**
  ///
  /// Users with push access can lock an issue or pull request's conversation.
  ///
  /// Note that, if you choose not to pass any parameters, you'll need to set `Content-Length` to zero when calling out to this endpoint. For more information, see "[HTTP method](https://docs.github.com/rest/guides/getting-started-with-the-rest-api#http-method)."
  ///
  /// *Documentation*: [https://docs.github.com/rest/issues/issues#lock-an-issue](https://docs.github.com/rest/issues/issues#lock-an-issue)
  pub fn lock(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    issue_number: impl Into<i64>,
  ) -> NoContentRequest<IssuesLockRequest, ()> {
    let owner = owner.into();
    let repo = repo.into();
    let issue_number = issue_number.into();
    let url = format!("/repos/{owner}/{repo}/issues/{issue_number}/lock");

    NoContentRequest::<IssuesLockRequest, ()>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **Unlock an issue**
  ///
  /// Users with push access can unlock an issue's conversation.
  ///
  /// *Documentation*: [https://docs.github.com/rest/issues/issues#unlock-an-issue](https://docs.github.com/rest/issues/issues#unlock-an-issue)
  pub fn unlock(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    issue_number: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let issue_number = issue_number.into();
    let url = format!("/repos/{owner}/{repo}/issues/{issue_number}/lock");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List timeline events for an issue**
  ///
  /// List all timeline events for an issue.
  ///
  /// *Documentation*: [https://docs.github.com/rest/issues/timeline#list-timeline-events-for-an-issue](https://docs.github.com/rest/issues/timeline#list-timeline-events-for-an-issue)
  pub fn list_events_for_timeline(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    issue_number: impl Into<i64>,
  ) -> Request<(), IssuesListEventsForTimelineQuery, TimelineEventArray> {
    let owner = owner.into();
    let repo = repo.into();
    let issue_number = issue_number.into();
    let url = format!("/repos/{owner}/{repo}/issues/{issue_number}/timeline");

    Request::<(), IssuesListEventsForTimelineQuery, TimelineEventArray>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List labels for a repository**
  ///
  /// Lists all labels for a repository.
  ///
  /// *Documentation*: [https://docs.github.com/rest/issues/labels#list-labels-for-a-repository](https://docs.github.com/rest/issues/labels#list-labels-for-a-repository)
  pub fn list_labels_for_repo(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), IssuesListLabelsForRepoQuery, LabelArray> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/labels");

    Request::<(), IssuesListLabelsForRepoQuery, LabelArray>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create a label**
  ///
  /// Creates a label for the specified repository with the given name and color. The name and color parameters are required. The color must be a valid [hexadecimal color code](http://www.color-hex.com/).
  ///
  /// *Documentation*: [https://docs.github.com/rest/issues/labels#create-a-label](https://docs.github.com/rest/issues/labels#create-a-label)
  pub fn create_label(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<IssuesCreateLabelRequest, (), Label> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/labels");

    Request::<IssuesCreateLabelRequest, (), Label>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Get a label**
  ///
  /// Gets a label using the given name.
  ///
  /// *Documentation*: [https://docs.github.com/rest/issues/labels#get-a-label](https://docs.github.com/rest/issues/labels#get-a-label)
  pub fn get_label(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    name: impl Into<String>,
  ) -> Request<(), (), Label> {
    let owner = owner.into();
    let repo = repo.into();
    let name = name.into();
    let url = format!("/repos/{owner}/{repo}/labels/{name}");

    Request::<(), (), Label>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Update a label**
  ///
  /// Updates a label using the given label name.
  ///
  /// *Documentation*: [https://docs.github.com/rest/issues/labels#update-a-label](https://docs.github.com/rest/issues/labels#update-a-label)
  pub fn update_label(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    name: impl Into<String>,
  ) -> Request<IssuesUpdateLabelRequest, (), Label> {
    let owner = owner.into();
    let repo = repo.into();
    let name = name.into();
    let url = format!("/repos/{owner}/{repo}/labels/{name}");

    Request::<IssuesUpdateLabelRequest, (), Label>::builder(&self.config)
      .patch(url)
      .build()
  }

  /// **Delete a label**
  ///
  /// Deletes a label using the given label name.
  ///
  /// *Documentation*: [https://docs.github.com/rest/issues/labels#delete-a-label](https://docs.github.com/rest/issues/labels#delete-a-label)
  pub fn delete_label(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    name: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let name = name.into();
    let url = format!("/repos/{owner}/{repo}/labels/{name}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List milestones**
  ///
  /// Lists milestones for a repository.
  ///
  /// *Documentation*: [https://docs.github.com/rest/issues/milestones#list-milestones](https://docs.github.com/rest/issues/milestones#list-milestones)
  pub fn list_milestones(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), IssuesListMilestonesQuery, MilestoneArray> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/milestones");

    Request::<(), IssuesListMilestonesQuery, MilestoneArray>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create a milestone**
  ///
  /// Creates a milestone.
  ///
  /// *Documentation*: [https://docs.github.com/rest/issues/milestones#create-a-milestone](https://docs.github.com/rest/issues/milestones#create-a-milestone)
  pub fn create_milestone(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<IssuesCreateMilestoneRequest, (), Milestone> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/milestones");

    Request::<IssuesCreateMilestoneRequest, (), Milestone>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Get a milestone**
  ///
  /// Gets a milestone using the given milestone number.
  ///
  /// *Documentation*: [https://docs.github.com/rest/issues/milestones#get-a-milestone](https://docs.github.com/rest/issues/milestones#get-a-milestone)
  pub fn get_milestone(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    milestone_number: impl Into<i64>,
  ) -> Request<(), (), Milestone> {
    let owner = owner.into();
    let repo = repo.into();
    let milestone_number = milestone_number.into();
    let url = format!("/repos/{owner}/{repo}/milestones/{milestone_number}");

    Request::<(), (), Milestone>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Update a milestone**
  ///
  ///
  /// *Documentation*: [https://docs.github.com/rest/issues/milestones#update-a-milestone](https://docs.github.com/rest/issues/milestones#update-a-milestone)
  pub fn update_milestone(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    milestone_number: impl Into<i64>,
  ) -> Request<IssuesUpdateMilestoneRequest, (), Milestone> {
    let owner = owner.into();
    let repo = repo.into();
    let milestone_number = milestone_number.into();
    let url = format!("/repos/{owner}/{repo}/milestones/{milestone_number}");

    Request::<IssuesUpdateMilestoneRequest, (), Milestone>::builder(&self.config)
      .patch(url)
      .build()
  }

  /// **Delete a milestone**
  ///
  /// Deletes a milestone using the given milestone number.
  ///
  /// *Documentation*: [https://docs.github.com/rest/issues/milestones#delete-a-milestone](https://docs.github.com/rest/issues/milestones#delete-a-milestone)
  pub fn delete_milestone(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    milestone_number: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let milestone_number = milestone_number.into();
    let url = format!("/repos/{owner}/{repo}/milestones/{milestone_number}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List labels for issues in a milestone**
  ///
  /// Lists labels for issues in a milestone.
  ///
  /// *Documentation*: [https://docs.github.com/rest/issues/labels#list-labels-for-issues-in-a-milestone](https://docs.github.com/rest/issues/labels#list-labels-for-issues-in-a-milestone)
  pub fn list_labels_for_milestone(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    milestone_number: impl Into<i64>,
  ) -> Request<(), IssuesListLabelsForMilestoneQuery, LabelArray> {
    let owner = owner.into();
    let repo = repo.into();
    let milestone_number = milestone_number.into();
    let url = format!("/repos/{owner}/{repo}/milestones/{milestone_number}/labels");

    Request::<(), IssuesListLabelsForMilestoneQuery, LabelArray>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List user account issues assigned to the authenticated user**
  ///
  /// List issues across owned and member repositories assigned to the authenticated user.
  ///
  /// **Note**: GitHub's REST API considers every pull request an issue, but not every issue is a pull request. For this
  /// reason, "Issues" endpoints may return both issues and pull requests in the response. You can identify pull requests by
  /// the `pull_request` key. Be aware that the `id` of a pull request returned from "Issues" endpoints will be an _issue id_. To find out the pull
  /// request id, use the "[List pull requests](https://docs.github.com/rest/pulls/pulls#list-pull-requests)" endpoint.
  ///
  /// This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
  ///
  /// - **`application/vnd.github.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
  /// - **`application/vnd.github.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
  /// - **`application/vnd.github.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
  /// - **`application/vnd.github.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
  ///
  /// *Documentation*: [https://docs.github.com/rest/issues/issues#list-user-account-issues-assigned-to-the-authenticated-user](https://docs.github.com/rest/issues/issues#list-user-account-issues-assigned-to-the-authenticated-user)
  pub fn list_for_authenticated_user(
    &self,
  ) -> Request<(), IssuesListForAuthenticatedUserQuery, IssueArray> {
    let url = format!("/user/issues");

    Request::<(), IssuesListForAuthenticatedUserQuery, IssueArray>::builder(&self.config)
      .get(url)
      .build()
  }
}
