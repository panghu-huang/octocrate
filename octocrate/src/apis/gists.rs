use octocrate_core::*;
#[allow(unused_imports)]
use octocrate_types::*;

/// View, modify your gists.
pub struct GitHubGistsAPI {
  config: SharedAPIConfig,
}

impl GitHubGistsAPI {
  pub fn new(config: &SharedAPIConfig) -> Self {
    Self {
      config: config.clone(),
    }
  }

  /// **Get a gist**
  ///
  /// Gets a specified gist.
  ///
  /// This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
  ///
  /// - **`application/vnd.github.raw+json`**: Returns the raw markdown. This is the default if you do not pass any specific media type.
  /// - **`application/vnd.github.base64+json`**: Returns the base64-encoded contents. This can be useful if your gist contains any invalid UTF-8 sequences.
  ///
  /// *Documentation*: [https://docs.github.com/rest/gists/gists#get-a-gist](https://docs.github.com/rest/gists/gists#get-a-gist)
  pub fn get(&self, gist_id: impl Into<String>) -> Request<(), (), GistSimple> {
    let gist_id = gist_id.into();
    let url = format!("/gists/{gist_id}");

    Request::<(), (), GistSimple>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Update a gist**
  ///
  /// Allows you to update a gist's description and to update, delete, or rename gist files. Files
  /// from the previous version of the gist that aren't explicitly changed during an edit
  /// are unchanged.
  ///
  /// At least one of `description` or `files` is required.
  ///
  /// This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
  ///
  /// - **`application/vnd.github.raw+json`**: Returns the raw markdown. This is the default if you do not pass any specific media type.
  /// - **`application/vnd.github.base64+json`**: Returns the base64-encoded contents. This can be useful if your gist contains any invalid UTF-8 sequences.
  ///
  /// *Documentation*: [https://docs.github.com/rest/gists/gists#update-a-gist](https://docs.github.com/rest/gists/gists#update-a-gist)
  pub fn update(&self, gist_id: impl Into<String>) -> Request<GistsUpdateRequest, (), GistSimple> {
    let gist_id = gist_id.into();
    let url = format!("/gists/{gist_id}");

    Request::<GistsUpdateRequest, (), GistSimple>::builder(&self.config)
      .patch(url)
      .build()
  }

  /// **Delete a gist**
  ///
  ///
  /// *Documentation*: [https://docs.github.com/rest/gists/gists#delete-a-gist](https://docs.github.com/rest/gists/gists#delete-a-gist)
  pub fn delete(&self, gist_id: impl Into<String>) -> NoContentRequest<(), ()> {
    let gist_id = gist_id.into();
    let url = format!("/gists/{gist_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List gist commits**
  ///
  ///
  /// *Documentation*: [https://docs.github.com/rest/gists/gists#list-gist-commits](https://docs.github.com/rest/gists/gists#list-gist-commits)
  pub fn list_commits(
    &self,
    gist_id: impl Into<String>,
  ) -> Request<(), GistsListCommitsQuery, GistCommitArray> {
    let gist_id = gist_id.into();
    let url = format!("/gists/{gist_id}/commits");

    Request::<(), GistsListCommitsQuery, GistCommitArray>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List gists for the authenticated user**
  ///
  /// Lists the authenticated user's gists or if called anonymously, this endpoint returns all public gists:
  ///
  /// *Documentation*: [https://docs.github.com/rest/gists/gists#list-gists-for-the-authenticated-user](https://docs.github.com/rest/gists/gists#list-gists-for-the-authenticated-user)
  pub fn list(&self) -> Request<(), GistsListQuery, BaseGistArray> {
    let url = format!("/gists");

    Request::<(), GistsListQuery, BaseGistArray>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create a gist**
  ///
  /// Allows you to add a new gist with one or more files.
  ///
  /// **Note:** Don't name your files "gistfile" with a numerical suffix. This is the format of the automatic naming scheme that Gist uses internally.
  ///
  /// *Documentation*: [https://docs.github.com/rest/gists/gists#create-a-gist](https://docs.github.com/rest/gists/gists#create-a-gist)
  pub fn create(&self) -> Request<GistsCreateRequest, (), GistSimple> {
    let url = format!("/gists");

    Request::<GistsCreateRequest, (), GistSimple>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Get a gist comment**
  ///
  /// Gets a comment on a gist.
  ///
  /// This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
  ///
  /// - **`application/vnd.github.raw+json`**: Returns the raw markdown. This is the default if you do not pass any specific media type.
  /// - **`application/vnd.github.base64+json`**: Returns the base64-encoded contents. This can be useful if your gist contains any invalid UTF-8 sequences.
  ///
  /// *Documentation*: [https://docs.github.com/rest/gists/comments#get-a-gist-comment](https://docs.github.com/rest/gists/comments#get-a-gist-comment)
  pub fn get_comment(
    &self,
    gist_id: impl Into<String>,
    comment_id: impl Into<i64>,
  ) -> Request<(), (), GistComment> {
    let gist_id = gist_id.into();
    let comment_id = comment_id.into();
    let url = format!("/gists/{gist_id}/comments/{comment_id}");

    Request::<(), (), GistComment>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Update a gist comment**
  ///
  /// Updates a comment on a gist.
  ///
  /// This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
  ///
  /// - **`application/vnd.github.raw+json`**: Returns the raw markdown. This is the default if you do not pass any specific media type.
  /// - **`application/vnd.github.base64+json`**: Returns the base64-encoded contents. This can be useful if your gist contains any invalid UTF-8 sequences.
  ///
  /// *Documentation*: [https://docs.github.com/rest/gists/comments#update-a-gist-comment](https://docs.github.com/rest/gists/comments#update-a-gist-comment)
  pub fn update_comment(
    &self,
    gist_id: impl Into<String>,
    comment_id: impl Into<i64>,
  ) -> Request<GistsUpdateCommentRequest, (), GistComment> {
    let gist_id = gist_id.into();
    let comment_id = comment_id.into();
    let url = format!("/gists/{gist_id}/comments/{comment_id}");

    Request::<GistsUpdateCommentRequest, (), GistComment>::builder(&self.config)
      .patch(url)
      .build()
  }

  /// **Delete a gist comment**
  ///
  ///
  /// *Documentation*: [https://docs.github.com/rest/gists/comments#delete-a-gist-comment](https://docs.github.com/rest/gists/comments#delete-a-gist-comment)
  pub fn delete_comment(
    &self,
    gist_id: impl Into<String>,
    comment_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let gist_id = gist_id.into();
    let comment_id = comment_id.into();
    let url = format!("/gists/{gist_id}/comments/{comment_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List gist comments**
  ///
  /// Lists the comments on a gist.
  ///
  /// This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
  ///
  /// - **`application/vnd.github.raw+json`**: Returns the raw markdown. This is the default if you do not pass any specific media type.
  /// - **`application/vnd.github.base64+json`**: Returns the base64-encoded contents. This can be useful if your gist contains any invalid UTF-8 sequences.
  ///
  /// *Documentation*: [https://docs.github.com/rest/gists/comments#list-gist-comments](https://docs.github.com/rest/gists/comments#list-gist-comments)
  pub fn list_comments(
    &self,
    gist_id: impl Into<String>,
  ) -> Request<(), GistsListCommentsQuery, GistCommentArray> {
    let gist_id = gist_id.into();
    let url = format!("/gists/{gist_id}/comments");

    Request::<(), GistsListCommentsQuery, GistCommentArray>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create a gist comment**
  ///
  /// Creates a comment on a gist.
  ///
  /// This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
  ///
  /// - **`application/vnd.github.raw+json`**: Returns the raw markdown. This is the default if you do not pass any specific media type.
  /// - **`application/vnd.github.base64+json`**: Returns the base64-encoded contents. This can be useful if your gist contains any invalid UTF-8 sequences.
  ///
  /// *Documentation*: [https://docs.github.com/rest/gists/comments#create-a-gist-comment](https://docs.github.com/rest/gists/comments#create-a-gist-comment)
  pub fn create_comment(
    &self,
    gist_id: impl Into<String>,
  ) -> Request<GistsCreateCommentRequest, (), GistComment> {
    let gist_id = gist_id.into();
    let url = format!("/gists/{gist_id}/comments");

    Request::<GistsCreateCommentRequest, (), GistComment>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **List gist forks**
  ///
  ///
  /// *Documentation*: [https://docs.github.com/rest/gists/gists#list-gist-forks](https://docs.github.com/rest/gists/gists#list-gist-forks)
  pub fn list_forks(
    &self,
    gist_id: impl Into<String>,
  ) -> Request<(), GistsListForksQuery, GistSimpleArray> {
    let gist_id = gist_id.into();
    let url = format!("/gists/{gist_id}/forks");

    Request::<(), GistsListForksQuery, GistSimpleArray>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Fork a gist**
  ///
  ///
  /// *Documentation*: [https://docs.github.com/rest/gists/gists#fork-a-gist](https://docs.github.com/rest/gists/gists#fork-a-gist)
  pub fn fork(&self, gist_id: impl Into<String>) -> Request<(), (), BaseGist> {
    let gist_id = gist_id.into();
    let url = format!("/gists/{gist_id}/forks");

    Request::<(), (), BaseGist>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **List public gists**
  ///
  /// List public gists sorted by most recently updated to least recently updated.
  ///
  /// Note: With [pagination](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api), you can fetch up to 3000 gists. For example, you can fetch 100 pages with 30 gists per page or 30 pages with 100 gists per page.
  ///
  /// *Documentation*: [https://docs.github.com/rest/gists/gists#list-public-gists](https://docs.github.com/rest/gists/gists#list-public-gists)
  pub fn list_public(&self) -> Request<(), GistsListPublicQuery, BaseGistArray> {
    let url = format!("/gists/public");

    Request::<(), GistsListPublicQuery, BaseGistArray>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Check if a gist is starred**
  ///
  ///
  /// *Documentation*: [https://docs.github.com/rest/gists/gists#check-if-a-gist-is-starred](https://docs.github.com/rest/gists/gists#check-if-a-gist-is-starred)
  pub fn check_is_starred(&self, gist_id: impl Into<String>) -> NoContentRequest<(), ()> {
    let gist_id = gist_id.into();
    let url = format!("/gists/{gist_id}/star");

    NoContentRequest::<(), ()>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Star a gist**
  ///
  /// Note that you'll need to set `Content-Length` to zero when calling out to this endpoint. For more information, see "[HTTP method](https://docs.github.com/rest/guides/getting-started-with-the-rest-api#http-method)."
  ///
  /// *Documentation*: [https://docs.github.com/rest/gists/gists#star-a-gist](https://docs.github.com/rest/gists/gists#star-a-gist)
  pub fn star(&self, gist_id: impl Into<String>) -> NoContentRequest<(), ()> {
    let gist_id = gist_id.into();
    let url = format!("/gists/{gist_id}/star");

    NoContentRequest::<(), ()>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **Unstar a gist**
  ///
  ///
  /// *Documentation*: [https://docs.github.com/rest/gists/gists#unstar-a-gist](https://docs.github.com/rest/gists/gists#unstar-a-gist)
  pub fn unstar(&self, gist_id: impl Into<String>) -> NoContentRequest<(), ()> {
    let gist_id = gist_id.into();
    let url = format!("/gists/{gist_id}/star");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List gists for a user**
  ///
  /// Lists public gists for the specified user:
  ///
  /// *Documentation*: [https://docs.github.com/rest/gists/gists#list-gists-for-a-user](https://docs.github.com/rest/gists/gists#list-gists-for-a-user)
  pub fn list_for_user(
    &self,
    username: impl Into<String>,
  ) -> Request<(), GistsListForUserQuery, BaseGistArray> {
    let username = username.into();
    let url = format!("/users/{username}/gists");

    Request::<(), GistsListForUserQuery, BaseGistArray>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get a gist revision**
  ///
  /// Gets a specified gist revision.
  ///
  /// This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
  ///
  /// - **`application/vnd.github.raw+json`**: Returns the raw markdown. This is the default if you do not pass any specific media type.
  /// - **`application/vnd.github.base64+json`**: Returns the base64-encoded contents. This can be useful if your gist contains any invalid UTF-8 sequences.
  ///
  /// *Documentation*: [https://docs.github.com/rest/gists/gists#get-a-gist-revision](https://docs.github.com/rest/gists/gists#get-a-gist-revision)
  pub fn get_revision(
    &self,
    gist_id: impl Into<String>,
    sha: impl Into<String>,
  ) -> Request<(), (), GistSimple> {
    let gist_id = gist_id.into();
    let sha = sha.into();
    let url = format!("/gists/{gist_id}/{sha}");

    Request::<(), (), GistSimple>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List starred gists**
  ///
  /// List the authenticated user's starred gists:
  ///
  /// *Documentation*: [https://docs.github.com/rest/gists/gists#list-starred-gists](https://docs.github.com/rest/gists/gists#list-starred-gists)
  pub fn list_starred(&self) -> Request<(), GistsListStarredQuery, BaseGistArray> {
    let url = format!("/gists/starred");

    Request::<(), GistsListStarredQuery, BaseGistArray>::builder(&self.config)
      .get(url)
      .build()
  }
}
