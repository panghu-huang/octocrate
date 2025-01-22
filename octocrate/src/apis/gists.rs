use octocrate_core::*;
#[allow(unused_imports)]
use octocrate_types::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use typed_builder::TypedBuilder;

pub mod list {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<BaseGist>;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// Only show results that were last updated after the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub since: Option<String>,
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

pub mod create {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = GistSimple;

  #[allow(clippy::large_enum_variant)]
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum RequestPublicItem2 {
    #[serde(rename = "true")]
    True,
    #[serde(rename = "false")]
    False,
  }

  impl std::fmt::Display for RequestPublicItem2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match self {
        RequestPublicItem2::True => write!(f, "true"),
        RequestPublicItem2::False => write!(f, "false"),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// Description of the gist
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub description: Option<String>,
    /// Names and content for the files that make up the gist
    pub files: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub public: Option<serde_json::Value>,
  }
}

pub mod list_public {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<BaseGist>;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// Only show results that were last updated after the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub since: Option<String>,
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

pub mod list_starred {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<BaseGist>;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// Only show results that were last updated after the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub since: Option<String>,
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

  pub type Response = GistSimple;
}

pub mod update {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = GistSimple;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The description of the gist.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub description: Option<String>,
    /// The gist files to be updated, renamed, or deleted. Each `key` must match the current filename
    /// (including extension) of the targeted gist file. For example: `hello.py`.
    ///
    /// To delete a file, set the whole file to null. For example: `hello.py : null`. The file will also be
    /// deleted if the specified object does not contain at least one of `content` or `filename`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub files: Option<serde_json::Value>,
  }
}

pub mod list_comments {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<GistComment>;

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

pub mod create_comment {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = GistComment;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The comment text.
    pub body: String,
  }
}

pub mod get_comment {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = GistComment;
}

pub mod update_comment {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = GistComment;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The comment text.
    pub body: String,
  }
}

pub mod list_commits {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<GistCommit>;

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

pub mod list_forks {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<GistSimple>;

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

pub mod fork {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = BaseGist;
}

pub mod get_revision {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = GistSimple;
}

pub mod list_for_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<BaseGist>;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// Only show results that were last updated after the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub since: Option<String>,
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

  /// **List gists for the authenticated user**
  ///
  /// Lists the authenticated user's gists or if called anonymously, this endpoint returns all public gists:
  ///
  /// *Documentation*: [https://docs.github.com/rest/gists/gists#list-gists-for-the-authenticated-user](https://docs.github.com/rest/gists/gists#list-gists-for-the-authenticated-user)
  pub fn list(&self) -> Request<(), list::Query, list::Response> {
    let url = format!("/gists");

    Request::<(), list::Query, list::Response>::builder(&self.config)
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
  pub fn create(&self) -> Request<create::Request, (), create::Response> {
    let url = format!("/gists");

    Request::<create::Request, (), create::Response>::builder(&self.config)
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
  pub fn list_public(&self) -> Request<(), list_public::Query, list_public::Response> {
    let url = format!("/gists/public");

    Request::<(), list_public::Query, list_public::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List starred gists**
  ///
  /// List the authenticated user's starred gists:
  ///
  /// *Documentation*: [https://docs.github.com/rest/gists/gists#list-starred-gists](https://docs.github.com/rest/gists/gists#list-starred-gists)
  pub fn list_starred(&self) -> Request<(), list_starred::Query, list_starred::Response> {
    let url = format!("/gists/starred");

    Request::<(), list_starred::Query, list_starred::Response>::builder(&self.config)
      .get(url)
      .build()
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
  pub fn get(&self, gist_id: impl Into<String>) -> Request<(), (), get::Response> {
    let gist_id = gist_id.into();
    let url = format!("/gists/{gist_id}");

    Request::<(), (), get::Response>::builder(&self.config)
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
  pub fn update(
    &self,
    gist_id: impl Into<String>,
  ) -> Request<update::Request, (), update::Response> {
    let gist_id = gist_id.into();
    let url = format!("/gists/{gist_id}");

    Request::<update::Request, (), update::Response>::builder(&self.config)
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
  ) -> Request<(), list_comments::Query, list_comments::Response> {
    let gist_id = gist_id.into();
    let url = format!("/gists/{gist_id}/comments");

    Request::<(), list_comments::Query, list_comments::Response>::builder(&self.config)
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
  ) -> Request<create_comment::Request, (), create_comment::Response> {
    let gist_id = gist_id.into();
    let url = format!("/gists/{gist_id}/comments");

    Request::<create_comment::Request, (), create_comment::Response>::builder(&self.config)
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
  ) -> Request<(), (), get_comment::Response> {
    let gist_id = gist_id.into();
    let comment_id = comment_id.into();
    let url = format!("/gists/{gist_id}/comments/{comment_id}");

    Request::<(), (), get_comment::Response>::builder(&self.config)
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
  ) -> Request<update_comment::Request, (), update_comment::Response> {
    let gist_id = gist_id.into();
    let comment_id = comment_id.into();
    let url = format!("/gists/{gist_id}/comments/{comment_id}");

    Request::<update_comment::Request, (), update_comment::Response>::builder(&self.config)
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

  /// **List gist commits**
  ///
  ///
  /// *Documentation*: [https://docs.github.com/rest/gists/gists#list-gist-commits](https://docs.github.com/rest/gists/gists#list-gist-commits)
  pub fn list_commits(
    &self,
    gist_id: impl Into<String>,
  ) -> Request<(), list_commits::Query, list_commits::Response> {
    let gist_id = gist_id.into();
    let url = format!("/gists/{gist_id}/commits");

    Request::<(), list_commits::Query, list_commits::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List gist forks**
  ///
  ///
  /// *Documentation*: [https://docs.github.com/rest/gists/gists#list-gist-forks](https://docs.github.com/rest/gists/gists#list-gist-forks)
  pub fn list_forks(
    &self,
    gist_id: impl Into<String>,
  ) -> Request<(), list_forks::Query, list_forks::Response> {
    let gist_id = gist_id.into();
    let url = format!("/gists/{gist_id}/forks");

    Request::<(), list_forks::Query, list_forks::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Fork a gist**
  ///
  ///
  /// *Documentation*: [https://docs.github.com/rest/gists/gists#fork-a-gist](https://docs.github.com/rest/gists/gists#fork-a-gist)
  pub fn fork(&self, gist_id: impl Into<String>) -> Request<(), (), fork::Response> {
    let gist_id = gist_id.into();
    let url = format!("/gists/{gist_id}/forks");

    Request::<(), (), fork::Response>::builder(&self.config)
      .post(url)
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
  ) -> Request<(), (), get_revision::Response> {
    let gist_id = gist_id.into();
    let sha = sha.into();
    let url = format!("/gists/{gist_id}/{sha}");

    Request::<(), (), get_revision::Response>::builder(&self.config)
      .get(url)
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
  ) -> Request<(), list_for_user::Query, list_for_user::Response> {
    let username = username.into();
    let url = format!("/users/{username}/gists");

    Request::<(), list_for_user::Query, list_for_user::Response>::builder(&self.config)
      .get(url)
      .build()
  }
}
