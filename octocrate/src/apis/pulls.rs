use octocrate_core::*;
#[allow(unused_imports)]
use crate::types::*;

/// Interact with GitHub Pull Requests.
pub struct GitHubPullsAPI {
  config: SharedAPIConfig,
}

impl GitHubPullsAPI {
  pub fn new(config: &SharedAPIConfig) -> Self {
    Self {
      config: config.clone(),
    }
  }

  /// **List comments for a pull request review**
  ///
  /// Lists comments for a specific pull request review.
  /// 
  /// This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
  /// 
  /// - **`application/vnd.github-commitcomment.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
  /// - **`application/vnd.github-commitcomment.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
  /// - **`application/vnd.github-commitcomment.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
  /// - **`application/vnd.github-commitcomment.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
  ///
  /// *Documentation*: [https://docs.github.com/rest/pulls/reviews#list-comments-for-a-pull-request-review](https://docs.github.com/rest/pulls/reviews#list-comments-for-a-pull-request-review)
  pub fn list_comments_for_review(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    pull_number: impl Into<i64>,
    review_id: impl Into<i64>,
  ) -> Request<(), PullsListCommentsForReviewQuery, LegacyReviewCommentArray> {
    let owner = owner.into();
    let repo = repo.into();
    let pull_number = pull_number.into();
    let review_id = review_id.into();
    let url = format!("/repos/{owner}/{repo}/pulls/{pull_number}/reviews/{review_id}/comments");

    Request::<(), PullsListCommentsForReviewQuery, LegacyReviewCommentArray>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List pull requests files**
  ///
  /// Lists the files in a specified pull request.
  /// 
  /// **Note:** Responses include a maximum of 3000 files. The paginated response
  /// returns 30 files per page by default.
  /// 
  /// This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
  /// 
  /// - **`application/vnd.github.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
  /// - **`application/vnd.github.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
  /// - **`application/vnd.github.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
  /// - **`application/vnd.github.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
  /// - **`application/vnd.github.diff`**: For more information, see "[git-diff](https://git-scm.com/docs/git-diff)" in the Git documentation. If a diff is corrupt, contact us through the [GitHub Support portal](https://support.github.com/). Include the repository name and pull request ID in your message.
  ///
  /// *Documentation*: [https://docs.github.com/rest/pulls/pulls#list-pull-requests-files](https://docs.github.com/rest/pulls/pulls#list-pull-requests-files)
  pub fn list_files(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    pull_number: impl Into<i64>,
  ) -> Request<(), PullsListFilesQuery, DiffEntryArray> {
    let owner = owner.into();
    let repo = repo.into();
    let pull_number = pull_number.into();
    let url = format!("/repos/{owner}/{repo}/pulls/{pull_number}/files");

    Request::<(), PullsListFilesQuery, DiffEntryArray>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get a review comment for a pull request**
  ///
  /// Provides details for a specified review comment.
  /// 
  /// This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
  /// 
  /// - **`application/vnd.github-commitcomment.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
  /// - **`application/vnd.github-commitcomment.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
  /// - **`application/vnd.github-commitcomment.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
  /// - **`application/vnd.github-commitcomment.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
  ///
  /// *Documentation*: [https://docs.github.com/rest/pulls/comments#get-a-review-comment-for-a-pull-request](https://docs.github.com/rest/pulls/comments#get-a-review-comment-for-a-pull-request)
  pub fn get_review_comment(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    comment_id: impl Into<i64>,
  ) -> Request<(), (), PullRequestReviewComment> {
    let owner = owner.into();
    let repo = repo.into();
    let comment_id = comment_id.into();
    let url = format!("/repos/{owner}/{repo}/pulls/comments/{comment_id}");

    Request::<(), (), PullRequestReviewComment>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Update a review comment for a pull request**
  ///
  /// Edits the content of a specified review comment.
  /// 
  /// This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
  /// 
  /// - **`application/vnd.github-commitcomment.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
  /// - **`application/vnd.github-commitcomment.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
  /// - **`application/vnd.github-commitcomment.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
  /// - **`application/vnd.github-commitcomment.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
  ///
  /// *Documentation*: [https://docs.github.com/rest/pulls/comments#update-a-review-comment-for-a-pull-request](https://docs.github.com/rest/pulls/comments#update-a-review-comment-for-a-pull-request)
  pub fn update_review_comment(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    comment_id: impl Into<i64>,
  ) -> Request<PullsUpdateReviewCommentRequest, (), PullRequestReviewComment> {
    let owner = owner.into();
    let repo = repo.into();
    let comment_id = comment_id.into();
    let url = format!("/repos/{owner}/{repo}/pulls/comments/{comment_id}");

    Request::<PullsUpdateReviewCommentRequest, (), PullRequestReviewComment>::builder(&self.config)
      .patch(url)
      .build()
  }

  /// **Delete a review comment for a pull request**
  ///
  /// Deletes a review comment.
  ///
  /// *Documentation*: [https://docs.github.com/rest/pulls/comments#delete-a-review-comment-for-a-pull-request](https://docs.github.com/rest/pulls/comments#delete-a-review-comment-for-a-pull-request)
  pub fn delete_review_comment(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    comment_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let comment_id = comment_id.into();
    let url = format!("/repos/{owner}/{repo}/pulls/comments/{comment_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Get a review for a pull request**
  ///
  /// Retrieves a pull request review by its ID.
  /// 
  /// This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
  /// 
  /// - **`application/vnd.github-commitcomment.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
  /// - **`application/vnd.github-commitcomment.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
  /// - **`application/vnd.github-commitcomment.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
  /// - **`application/vnd.github-commitcomment.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
  ///
  /// *Documentation*: [https://docs.github.com/rest/pulls/reviews#get-a-review-for-a-pull-request](https://docs.github.com/rest/pulls/reviews#get-a-review-for-a-pull-request)
  pub fn get_review(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    pull_number: impl Into<i64>,
    review_id: impl Into<i64>,
  ) -> Request<(), (), PullRequestReview> {
    let owner = owner.into();
    let repo = repo.into();
    let pull_number = pull_number.into();
    let review_id = review_id.into();
    let url = format!("/repos/{owner}/{repo}/pulls/{pull_number}/reviews/{review_id}");

    Request::<(), (), PullRequestReview>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Update a review for a pull request**
  ///
  /// Updates the contents of a specified review summary comment.
  /// 
  /// This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
  /// 
  /// - **`application/vnd.github-commitcomment.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
  /// - **`application/vnd.github-commitcomment.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
  /// - **`application/vnd.github-commitcomment.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
  /// - **`application/vnd.github-commitcomment.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
  ///
  /// *Documentation*: [https://docs.github.com/rest/pulls/reviews#update-a-review-for-a-pull-request](https://docs.github.com/rest/pulls/reviews#update-a-review-for-a-pull-request)
  pub fn update_review(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    pull_number: impl Into<i64>,
    review_id: impl Into<i64>,
  ) -> Request<PullsUpdateReviewRequest, (), PullRequestReview> {
    let owner = owner.into();
    let repo = repo.into();
    let pull_number = pull_number.into();
    let review_id = review_id.into();
    let url = format!("/repos/{owner}/{repo}/pulls/{pull_number}/reviews/{review_id}");

    Request::<PullsUpdateReviewRequest, (), PullRequestReview>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **Delete a pending review for a pull request**
  ///
  /// Deletes a pull request review that has not been submitted. Submitted reviews cannot be deleted.
  /// 
  /// This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
  /// 
  /// - **`application/vnd.github-commitcomment.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
  /// - **`application/vnd.github-commitcomment.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
  /// - **`application/vnd.github-commitcomment.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
  /// - **`application/vnd.github-commitcomment.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
  ///
  /// *Documentation*: [https://docs.github.com/rest/pulls/reviews#delete-a-pending-review-for-a-pull-request](https://docs.github.com/rest/pulls/reviews#delete-a-pending-review-for-a-pull-request)
  pub fn delete_pending_review(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    pull_number: impl Into<i64>,
    review_id: impl Into<i64>,
  ) -> Request<(), (), PullRequestReview> {
    let owner = owner.into();
    let repo = repo.into();
    let pull_number = pull_number.into();
    let review_id = review_id.into();
    let url = format!("/repos/{owner}/{repo}/pulls/{pull_number}/reviews/{review_id}");

    Request::<(), (), PullRequestReview>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List review comments on a pull request**
  ///
  /// Lists all review comments for a specified pull request. By default, review comments
  /// are in ascending order by ID.
  /// 
  /// This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
  /// 
  /// - **`application/vnd.github-commitcomment.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
  /// - **`application/vnd.github-commitcomment.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
  /// - **`application/vnd.github-commitcomment.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
  /// - **`application/vnd.github-commitcomment.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
  ///
  /// *Documentation*: [https://docs.github.com/rest/pulls/comments#list-review-comments-on-a-pull-request](https://docs.github.com/rest/pulls/comments#list-review-comments-on-a-pull-request)
  pub fn list_review_comments(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    pull_number: impl Into<i64>,
  ) -> Request<(), PullsListReviewCommentsQuery, PullRequestReviewCommentArray> {
    let owner = owner.into();
    let repo = repo.into();
    let pull_number = pull_number.into();
    let url = format!("/repos/{owner}/{repo}/pulls/{pull_number}/comments");

    Request::<(), PullsListReviewCommentsQuery, PullRequestReviewCommentArray>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create a review comment for a pull request**
  ///
  /// Creates a review comment on the diff of a specified pull request. To add a regular comment to a pull request timeline, see "[Create an issue comment](https://docs.github.com/rest/issues/comments#create-an-issue-comment)."
  /// 
  /// If your comment applies to more than one line in the pull request diff, you should use the parameters `line`, `side`, and optionally `start_line` and `start_side` in your request.
  /// 
  /// The `position` parameter is deprecated. If you use `position`, the `line`, `side`, `start_line`, and `start_side` parameters are not required.
  /// 
  /// This endpoint triggers [notifications](https://docs.github.com/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. For more information, see "[Rate limits for the API](https://docs.github.com/rest/overview/rate-limits-for-the-rest-api#about-secondary-rate-limits)"
  /// and "[Best practices for using the REST API](https://docs.github.com/rest/guides/best-practices-for-using-the-rest-api)."
  /// 
  /// This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
  /// 
  /// - **`application/vnd.github-commitcomment.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
  /// - **`application/vnd.github-commitcomment.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
  /// - **`application/vnd.github-commitcomment.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
  /// - **`application/vnd.github-commitcomment.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
  ///
  /// *Documentation*: [https://docs.github.com/rest/pulls/comments#create-a-review-comment-for-a-pull-request](https://docs.github.com/rest/pulls/comments#create-a-review-comment-for-a-pull-request)
  pub fn create_review_comment(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    pull_number: impl Into<i64>,
  ) -> Request<PullsCreateReviewCommentRequest, (), PullRequestReviewComment> {
    let owner = owner.into();
    let repo = repo.into();
    let pull_number = pull_number.into();
    let url = format!("/repos/{owner}/{repo}/pulls/{pull_number}/comments");

    Request::<PullsCreateReviewCommentRequest, (), PullRequestReviewComment>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Check if a pull request has been merged**
  ///
  /// Checks if a pull request has been merged into the base branch. The HTTP status of the response indicates whether or not the pull request has been merged; the response body is empty.
  ///
  /// *Documentation*: [https://docs.github.com/rest/pulls/pulls#check-if-a-pull-request-has-been-merged](https://docs.github.com/rest/pulls/pulls#check-if-a-pull-request-has-been-merged)
  pub fn check_if_merged(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    pull_number: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let pull_number = pull_number.into();
    let url = format!("/repos/{owner}/{repo}/pulls/{pull_number}/merge");

    NoContentRequest::<(), ()>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Merge a pull request**
  ///
  /// Merges a pull request into the base branch.
  /// This endpoint triggers [notifications](https://docs.github.com/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. For more information, see "[Rate limits for the API](https://docs.github.com/rest/overview/rate-limits-for-the-rest-api#about-secondary-rate-limits)" and "[Best practices for using the REST API](https://docs.github.com/rest/guides/best-practices-for-using-the-rest-api)."
  ///
  /// *Documentation*: [https://docs.github.com/rest/pulls/pulls#merge-a-pull-request](https://docs.github.com/rest/pulls/pulls#merge-a-pull-request)
  pub fn merge(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    pull_number: impl Into<i64>,
  ) -> Request<PullsMergeRequest, (), PullRequestMergeResult> {
    let owner = owner.into();
    let repo = repo.into();
    let pull_number = pull_number.into();
    let url = format!("/repos/{owner}/{repo}/pulls/{pull_number}/merge");

    Request::<PullsMergeRequest, (), PullRequestMergeResult>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **List review comments in a repository**
  ///
  /// Lists review comments for all pull requests in a repository. By default,
  /// review comments are in ascending order by ID.
  /// 
  /// This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
  /// 
  /// - **`application/vnd.github-commitcomment.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
  /// - **`application/vnd.github-commitcomment.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
  /// - **`application/vnd.github-commitcomment.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
  /// - **`application/vnd.github-commitcomment.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
  ///
  /// *Documentation*: [https://docs.github.com/rest/pulls/comments#list-review-comments-in-a-repository](https://docs.github.com/rest/pulls/comments#list-review-comments-in-a-repository)
  pub fn list_review_comments_for_repo(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), PullsListReviewCommentsForRepoQuery, PullRequestReviewCommentArray> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/pulls/comments");

    Request::<(), PullsListReviewCommentsForRepoQuery, PullRequestReviewCommentArray>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Submit a review for a pull request**
  ///
  /// Submits a pending review for a pull request. For more information about creating a pending review for a pull request, see "[Create a review for a pull request](https://docs.github.com/rest/pulls/reviews#create-a-review-for-a-pull-request)."
  /// 
  /// This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
  /// 
  /// - **`application/vnd.github-commitcomment.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
  /// - **`application/vnd.github-commitcomment.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
  /// - **`application/vnd.github-commitcomment.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
  /// - **`application/vnd.github-commitcomment.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
  ///
  /// *Documentation*: [https://docs.github.com/rest/pulls/reviews#submit-a-review-for-a-pull-request](https://docs.github.com/rest/pulls/reviews#submit-a-review-for-a-pull-request)
  pub fn submit_review(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    pull_number: impl Into<i64>,
    review_id: impl Into<i64>,
  ) -> Request<PullsSubmitReviewRequest, (), PullRequestReview> {
    let owner = owner.into();
    let repo = repo.into();
    let pull_number = pull_number.into();
    let review_id = review_id.into();
    let url = format!("/repos/{owner}/{repo}/pulls/{pull_number}/reviews/{review_id}/events");

    Request::<PullsSubmitReviewRequest, (), PullRequestReview>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **List pull requests**
  ///
  /// Lists pull requests in a specified repository.
  /// 
  /// Draft pull requests are available in public repositories with GitHub
  /// Free and GitHub Free for organizations, GitHub Pro, and legacy per-repository billing
  /// plans, and in public and private repositories with GitHub Team and GitHub Enterprise
  /// Cloud. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products)
  /// in the GitHub Help documentation.
  /// 
  /// This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
  /// 
  /// - **`application/vnd.github.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
  /// - **`application/vnd.github.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
  /// - **`application/vnd.github.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
  /// - **`application/vnd.github.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
  /// - **`application/vnd.github.diff`**: For more information, see "[git-diff](https://git-scm.com/docs/git-diff)" in the Git documentation. If a diff is corrupt, contact us through the [GitHub Support portal](https://support.github.com/). Include the repository name and pull request ID in your message.
  /// - **`application/vnd.github.patch`**: For more information, see "[git-format-patch](https://git-scm.com/docs/git-format-patch)" in the Git documentation.
  ///
  /// *Documentation*: [https://docs.github.com/rest/pulls/pulls#list-pull-requests](https://docs.github.com/rest/pulls/pulls#list-pull-requests)
  pub fn list(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), PullsListQuery, PullRequestSimpleArray> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/pulls");

    Request::<(), PullsListQuery, PullRequestSimpleArray>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create a pull request**
  ///
  /// Draft pull requests are available in public repositories with GitHub Free and GitHub Free for organizations, GitHub Pro, and legacy per-repository billing plans, and in public and private repositories with GitHub Team and GitHub Enterprise Cloud. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
  /// 
  /// To open or update a pull request in a public repository, you must have write access to the head or the source branch. For organization-owned repositories, you must be a member of the organization that owns the repository to open or update a pull request.
  /// 
  /// This endpoint triggers [notifications](https://docs.github.com/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. For more information, see "[Rate limits for the API](https://docs.github.com/rest/overview/rate-limits-for-the-rest-api#about-secondary-rate-limits)" and "[Best practices for using the REST API](https://docs.github.com/rest/guides/best-practices-for-using-the-rest-api)."
  /// 
  /// This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
  /// 
  /// - **`application/vnd.github.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
  /// - **`application/vnd.github.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
  /// - **`application/vnd.github.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
  /// - **`application/vnd.github.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
  /// - **`application/vnd.github.diff`**: For more information, see "[git-diff](https://git-scm.com/docs/git-diff)" in the Git documentation. If a diff is corrupt, contact us through the [GitHub Support portal](https://support.github.com/). Include the repository name and pull request ID in your message.
  ///
  /// *Documentation*: [https://docs.github.com/rest/pulls/pulls#create-a-pull-request](https://docs.github.com/rest/pulls/pulls#create-a-pull-request)
  pub fn create(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<PullsCreateRequest, (), PullRequest> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/pulls");

    Request::<PullsCreateRequest, (), PullRequest>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Create a reply for a review comment**
  ///
  /// Creates a reply to a review comment for a pull request. For the `comment_id`, provide the ID of the review comment you are replying to. This must be the ID of a _top-level review comment_, not a reply to that comment. Replies to replies are not supported.
  /// 
  /// This endpoint triggers [notifications](https://docs.github.com/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. For more information, see "[Rate limits for the API](https://docs.github.com/rest/overview/rate-limits-for-the-rest-api#about-secondary-rate-limits)"
  /// and "[Best practices for using the REST API](https://docs.github.com/rest/guides/best-practices-for-using-the-rest-api)."
  /// 
  /// This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
  /// 
  /// - **`application/vnd.github-commitcomment.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
  /// - **`application/vnd.github-commitcomment.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
  /// - **`application/vnd.github-commitcomment.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
  /// - **`application/vnd.github-commitcomment.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
  ///
  /// *Documentation*: [https://docs.github.com/rest/pulls/comments#create-a-reply-for-a-review-comment](https://docs.github.com/rest/pulls/comments#create-a-reply-for-a-review-comment)
  pub fn create_reply_for_review_comment(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    pull_number: impl Into<i64>,
    comment_id: impl Into<i64>,
  ) -> Request<PullsCreateReplyForReviewCommentRequest, (), PullRequestReviewComment> {
    let owner = owner.into();
    let repo = repo.into();
    let pull_number = pull_number.into();
    let comment_id = comment_id.into();
    let url = format!("/repos/{owner}/{repo}/pulls/{pull_number}/comments/{comment_id}/replies");

    Request::<PullsCreateReplyForReviewCommentRequest, (), PullRequestReviewComment>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Get all requested reviewers for a pull request**
  ///
  /// Gets the users or teams whose review is requested for a pull request. Once a requested reviewer submits a review, they are no longer considered a requested reviewer. Their review will instead be returned by the [List reviews for a pull request](https://docs.github.com/rest/pulls/reviews#list-reviews-for-a-pull-request) operation.
  ///
  /// *Documentation*: [https://docs.github.com/rest/pulls/review-requests#get-all-requested-reviewers-for-a-pull-request](https://docs.github.com/rest/pulls/review-requests#get-all-requested-reviewers-for-a-pull-request)
  pub fn list_requested_reviewers(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    pull_number: impl Into<i64>,
  ) -> Request<(), (), PullRequestReviewRequest> {
    let owner = owner.into();
    let repo = repo.into();
    let pull_number = pull_number.into();
    let url = format!("/repos/{owner}/{repo}/pulls/{pull_number}/requested_reviewers");

    Request::<(), (), PullRequestReviewRequest>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Request reviewers for a pull request**
  ///
  /// Requests reviews for a pull request from a given set of users and/or teams.
  /// This endpoint triggers [notifications](https://docs.github.com/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. For more information, see "[Rate limits for the API](https://docs.github.com/rest/overview/rate-limits-for-the-rest-api#about-secondary-rate-limits)" and "[Best practices for using the REST API](https://docs.github.com/rest/guides/best-practices-for-using-the-rest-api)."
  ///
  /// *Documentation*: [https://docs.github.com/rest/pulls/review-requests#request-reviewers-for-a-pull-request](https://docs.github.com/rest/pulls/review-requests#request-reviewers-for-a-pull-request)
  pub fn request_reviewers(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    pull_number: impl Into<i64>,
  ) -> Request<serde_json::Value, (), PullRequestSimple> {
    let owner = owner.into();
    let repo = repo.into();
    let pull_number = pull_number.into();
    let url = format!("/repos/{owner}/{repo}/pulls/{pull_number}/requested_reviewers");

    Request::<serde_json::Value, (), PullRequestSimple>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Remove requested reviewers from a pull request**
  ///
  /// Removes review requests from a pull request for a given set of users and/or teams.
  ///
  /// *Documentation*: [https://docs.github.com/rest/pulls/review-requests#remove-requested-reviewers-from-a-pull-request](https://docs.github.com/rest/pulls/review-requests#remove-requested-reviewers-from-a-pull-request)
  pub fn remove_requested_reviewers(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    pull_number: impl Into<i64>,
  ) -> Request<PullsRemoveRequestedReviewersRequest, (), PullRequestSimple> {
    let owner = owner.into();
    let repo = repo.into();
    let pull_number = pull_number.into();
    let url = format!("/repos/{owner}/{repo}/pulls/{pull_number}/requested_reviewers");

    Request::<PullsRemoveRequestedReviewersRequest, (), PullRequestSimple>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Update a pull request branch**
  ///
  /// Updates the pull request branch with the latest upstream changes by merging HEAD from the base branch into the pull request branch.
  ///
  /// *Documentation*: [https://docs.github.com/rest/pulls/pulls#update-a-pull-request-branch](https://docs.github.com/rest/pulls/pulls#update-a-pull-request-branch)
  pub fn update_branch(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    pull_number: impl Into<i64>,
  ) -> Request<PullsUpdateBranchRequest, (), PullsUpdateBranchResponse> {
    let owner = owner.into();
    let repo = repo.into();
    let pull_number = pull_number.into();
    let url = format!("/repos/{owner}/{repo}/pulls/{pull_number}/update-branch");

    Request::<PullsUpdateBranchRequest, (), PullsUpdateBranchResponse>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **List commits on a pull request**
  ///
  /// Lists a maximum of 250 commits for a pull request. To receive a complete
  /// commit list for pull requests with more than 250 commits, use the [List commits](https://docs.github.com/rest/commits/commits#list-commits)
  /// endpoint.
  /// 
  /// This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
  /// 
  /// - **`application/vnd.github.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
  /// - **`application/vnd.github.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
  /// - **`application/vnd.github.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
  /// - **`application/vnd.github.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
  /// - **`application/vnd.github.diff`**: For more information, see "[git-diff](https://git-scm.com/docs/git-diff)" in the Git documentation. If a diff is corrupt, contact us through the [GitHub Support portal](https://support.github.com/). Include the repository name and pull request ID in your message.
  ///
  /// *Documentation*: [https://docs.github.com/rest/pulls/pulls#list-commits-on-a-pull-request](https://docs.github.com/rest/pulls/pulls#list-commits-on-a-pull-request)
  pub fn list_commits(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    pull_number: impl Into<i64>,
  ) -> Request<(), PullsListCommitsQuery, CommitArray> {
    let owner = owner.into();
    let repo = repo.into();
    let pull_number = pull_number.into();
    let url = format!("/repos/{owner}/{repo}/pulls/{pull_number}/commits");

    Request::<(), PullsListCommitsQuery, CommitArray>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List reviews for a pull request**
  ///
  /// Lists all reviews for a specified pull request. The list of reviews returns in chronological order.
  /// 
  /// This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
  /// 
  /// - **`application/vnd.github-commitcomment.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
  /// - **`application/vnd.github-commitcomment.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
  /// - **`application/vnd.github-commitcomment.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
  /// - **`application/vnd.github-commitcomment.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
  ///
  /// *Documentation*: [https://docs.github.com/rest/pulls/reviews#list-reviews-for-a-pull-request](https://docs.github.com/rest/pulls/reviews#list-reviews-for-a-pull-request)
  pub fn list_reviews(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    pull_number: impl Into<i64>,
  ) -> Request<(), PullsListReviewsQuery, PullRequestReviewArray> {
    let owner = owner.into();
    let repo = repo.into();
    let pull_number = pull_number.into();
    let url = format!("/repos/{owner}/{repo}/pulls/{pull_number}/reviews");

    Request::<(), PullsListReviewsQuery, PullRequestReviewArray>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create a review for a pull request**
  ///
  /// Creates a review on a specified pull request.
  /// 
  /// This endpoint triggers [notifications](https://docs.github.com/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. For more information, see "[Rate limits for the API](https://docs.github.com/rest/overview/rate-limits-for-the-rest-api#about-secondary-rate-limits)" and "[Best practices for using the REST API](https://docs.github.com/rest/guides/best-practices-for-using-the-rest-api)."
  /// 
  /// Pull request reviews created in the `PENDING` state are not submitted and therefore do not include the `submitted_at` property in the response. To create a pending review for a pull request, leave the `event` parameter blank. For more information about submitting a `PENDING` review, see "[Submit a review for a pull request](https://docs.github.com/rest/pulls/reviews#submit-a-review-for-a-pull-request)."
  /// 
  /// **Note:** To comment on a specific line in a file, you need to first determine the position of that line in the diff. To see a pull request diff, add the `application/vnd.github.v3.diff` media type to the `Accept` header of a call to the [Get a pull request](https://docs.github.com/rest/pulls/pulls#get-a-pull-request) endpoint.
  /// 
  /// The `position` value equals the number of lines down from the first "@@" hunk header in the file you want to add a comment. The line just below the "@@" line is position 1, the next line is position 2, and so on. The position in the diff continues to increase through lines of whitespace and additional hunks until the beginning of a new file.
  /// 
  /// This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
  /// 
  /// - **`application/vnd.github-commitcomment.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
  /// - **`application/vnd.github-commitcomment.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
  /// - **`application/vnd.github-commitcomment.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
  /// - **`application/vnd.github-commitcomment.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
  ///
  /// *Documentation*: [https://docs.github.com/rest/pulls/reviews#create-a-review-for-a-pull-request](https://docs.github.com/rest/pulls/reviews#create-a-review-for-a-pull-request)
  pub fn create_review(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    pull_number: impl Into<i64>,
  ) -> Request<PullsCreateReviewRequest, (), PullRequestReview> {
    let owner = owner.into();
    let repo = repo.into();
    let pull_number = pull_number.into();
    let url = format!("/repos/{owner}/{repo}/pulls/{pull_number}/reviews");

    Request::<PullsCreateReviewRequest, (), PullRequestReview>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Get a pull request**
  ///
  /// Draft pull requests are available in public repositories with GitHub Free and GitHub Free for organizations, GitHub Pro, and legacy per-repository billing plans, and in public and private repositories with GitHub Team and GitHub Enterprise Cloud. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
  /// 
  /// Lists details of a pull request by providing its number.
  /// 
  /// When you get, [create](https://docs.github.com/rest/pulls/pulls/#create-a-pull-request), or [edit](https://docs.github.com/rest/pulls/pulls#update-a-pull-request) a pull request, GitHub creates a merge commit to test whether the pull request can be automatically merged into the base branch. This test commit is not added to the base branch or the head branch. You can review the status of the test commit using the `mergeable` key. For more information, see "[Checking mergeability of pull requests](https://docs.github.com/rest/guides/getting-started-with-the-git-database-api#checking-mergeability-of-pull-requests)".
  /// 
  /// The value of the `mergeable` attribute can be `true`, `false`, or `null`. If the value is `null`, then GitHub has started a background job to compute the mergeability. After giving the job time to complete, resubmit the request. When the job finishes, you will see a non-`null` value for the `mergeable` attribute in the response. If `mergeable` is `true`, then `merge_commit_sha` will be the SHA of the _test_ merge commit.
  /// 
  /// The value of the `merge_commit_sha` attribute changes depending on the state of the pull request. Before merging a pull request, the `merge_commit_sha` attribute holds the SHA of the _test_ merge commit. After merging a pull request, the `merge_commit_sha` attribute changes depending on how you merged the pull request:
  /// 
  /// *   If merged as a [merge commit](https://docs.github.com/articles/about-merge-methods-on-github/), `merge_commit_sha` represents the SHA of the merge commit.
  /// *   If merged via a [squash](https://docs.github.com/articles/about-merge-methods-on-github/#squashing-your-merge-commits), `merge_commit_sha` represents the SHA of the squashed commit on the base branch.
  /// *   If [rebased](https://docs.github.com/articles/about-merge-methods-on-github/#rebasing-and-merging-your-commits), `merge_commit_sha` represents the commit that the base branch was updated to.
  /// 
  /// Pass the appropriate [media type](https://docs.github.com/rest/overview/media-types/#commits-commit-comparison-and-pull-requests) to fetch diff and patch formats.
  /// 
  /// This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
  /// 
  /// - **`application/vnd.github.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
  /// - **`application/vnd.github.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
  /// - **`application/vnd.github.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
  /// - **`application/vnd.github.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
  /// - **`application/vnd.github.diff`**: For more information, see "[git-diff](https://git-scm.com/docs/git-diff)" in the Git documentation. If a diff is corrupt, contact us through the [GitHub Support portal](https://support.github.com/). Include the repository name and pull request ID in your message.
  ///
  /// *Documentation*: [https://docs.github.com/rest/pulls/pulls#get-a-pull-request](https://docs.github.com/rest/pulls/pulls#get-a-pull-request)
  pub fn get(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    pull_number: impl Into<i64>,
  ) -> Request<(), (), PullRequest> {
    let owner = owner.into();
    let repo = repo.into();
    let pull_number = pull_number.into();
    let url = format!("/repos/{owner}/{repo}/pulls/{pull_number}");

    Request::<(), (), PullRequest>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Update a pull request**
  ///
  /// Draft pull requests are available in public repositories with GitHub Free and GitHub Free for organizations, GitHub Pro, and legacy per-repository billing plans, and in public and private repositories with GitHub Team and GitHub Enterprise Cloud. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
  /// 
  /// To open or update a pull request in a public repository, you must have write access to the head or the source branch. For organization-owned repositories, you must be a member of the organization that owns the repository to open or update a pull request.
  /// 
  /// This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
  /// 
  /// - **`application/vnd.github.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
  /// - **`application/vnd.github.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
  /// - **`application/vnd.github.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
  /// - **`application/vnd.github.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
  /// - **`application/vnd.github.diff`**: For more information, see "[git-diff](https://git-scm.com/docs/git-diff)" in the Git documentation. If a diff is corrupt, contact us through the [GitHub Support portal](https://support.github.com/). Include the repository name and pull request ID in your message.
  ///
  /// *Documentation*: [https://docs.github.com/rest/pulls/pulls#update-a-pull-request](https://docs.github.com/rest/pulls/pulls#update-a-pull-request)
  pub fn update(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    pull_number: impl Into<i64>,
  ) -> Request<PullsUpdateRequest, (), PullRequest> {
    let owner = owner.into();
    let repo = repo.into();
    let pull_number = pull_number.into();
    let url = format!("/repos/{owner}/{repo}/pulls/{pull_number}");

    Request::<PullsUpdateRequest, (), PullRequest>::builder(&self.config)
      .patch(url)
      .build()
  }

  /// **Dismiss a review for a pull request**
  ///
  /// Dismisses a specified review on a pull request.
  /// 
  /// **Note:** To dismiss a pull request review on a [protected branch](https://docs.github.com/rest/branches/branch-protection),
  /// you must be a repository administrator or be included in the list of people or teams
  /// who can dismiss pull request reviews.
  /// 
  /// This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
  /// 
  /// - **`application/vnd.github-commitcomment.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
  /// - **`application/vnd.github-commitcomment.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
  /// - **`application/vnd.github-commitcomment.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
  /// - **`application/vnd.github-commitcomment.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
  ///
  /// *Documentation*: [https://docs.github.com/rest/pulls/reviews#dismiss-a-review-for-a-pull-request](https://docs.github.com/rest/pulls/reviews#dismiss-a-review-for-a-pull-request)
  pub fn dismiss_review(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    pull_number: impl Into<i64>,
    review_id: impl Into<i64>,
  ) -> Request<PullsDismissReviewRequest, (), PullRequestReview> {
    let owner = owner.into();
    let repo = repo.into();
    let pull_number = pull_number.into();
    let review_id = review_id.into();
    let url = format!("/repos/{owner}/{repo}/pulls/{pull_number}/reviews/{review_id}/dismissals");

    Request::<PullsDismissReviewRequest, (), PullRequestReview>::builder(&self.config)
      .put(url)
      .build()
  }


}
