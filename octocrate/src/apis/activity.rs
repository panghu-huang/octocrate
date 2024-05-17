use octocrate_core::*;
#[allow(unused_imports)]
use octocrate_types::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use typed_builder::TypedBuilder;

pub mod list_public_events {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Event>;

  /// Query for `List public events`
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

pub mod get_feeds {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Feed;
}

pub mod list_public_events_for_repo_network {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Event>;

  /// Query for `List public events for a network of repositories`
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

pub mod list_notifications_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Thread>;

  /// Query for `List notifications for the authenticated user`
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// If `true`, show notifications marked as read.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub all: Option<bool>,
    /// If `true`, only shows notifications in which the user is directly participating or mentioned.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub participating: Option<bool>,
    /// Only show results that were last updated after the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub since: Option<String>,
    /// Only show notifications updated before the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub before: Option<String>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
    /// The number of results per page (max 50). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
  }
}

pub mod mark_notifications_as_read {
  #[allow(unused_imports)]
  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// Describes the last point that notifications were checked. Anything updated since this time will not be marked as read. If you omit this parameter, all notifications are marked as read. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`. Default: The current timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub last_read_at: Option<String>,
    /// Whether the notification has been read.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub read: Option<bool>,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub message: Option<String>,
  }
}

pub mod get_thread {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Thread;
}

pub mod mark_thread_as_read {
  #[allow(unused_imports)]
  use super::*;
}

pub mod mark_thread_as_done {
  #[allow(unused_imports)]
  use super::*;
}

pub mod get_thread_subscription_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = ThreadSubscription;
}

pub mod set_thread_subscription {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = ThreadSubscription;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// Whether to block all notifications from a thread.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub ignored: Option<bool>,
  }
}

pub mod delete_thread_subscription {
  #[allow(unused_imports)]
  use super::*;
}

pub mod list_public_org_events {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Event>;

  /// Query for `List public organization events`
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

pub mod list_repo_events {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Event>;

  /// Query for `List repository events`
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

pub mod list_repo_notifications_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Thread>;

  /// Query for `List repository notifications for the authenticated user`
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// If `true`, show notifications marked as read.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub all: Option<bool>,
    /// If `true`, only shows notifications in which the user is directly participating or mentioned.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub participating: Option<bool>,
    /// Only show results that were last updated after the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub since: Option<String>,
    /// Only show notifications updated before the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub before: Option<String>,
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

pub mod mark_repo_notifications_as_read {
  #[allow(unused_imports)]
  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// Describes the last point that notifications were checked. Anything updated since this time will not be marked as read. If you omit this parameter, all notifications are marked as read. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`. Default: The current timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub last_read_at: Option<String>,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub url: Option<String>,
  }
}

pub mod list_stargazers_for_repo {
  #[allow(unused_imports)]
  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize)]
  #[serde(untagged)]
  pub enum Response {
    SimpleUserArray(Vec<SimpleUser>),
    StargazerArray(Vec<Stargazer>),
  }

  /// Query for `List stargazers`
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

pub mod list_watchers_for_repo {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<SimpleUser>;

  /// Query for `List watchers`
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

pub mod get_repo_subscription {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = RepositorySubscription;
}

pub mod set_repo_subscription {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = RepositorySubscription;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// Determines if all notifications should be blocked from this repository.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub ignored: Option<bool>,
    /// Determines if notifications should be received from this repository.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub subscribed: Option<bool>,
  }
}

pub mod delete_repo_subscription {
  #[allow(unused_imports)]
  use super::*;
}

pub mod list_repos_starred_by_authenticated_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Repository>;

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QuerySort {
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "updated")]
    Updated,
  }

  impl ToString for QuerySort {
    fn to_string(&self) -> String {
      match self {
        QuerySort::Created => "created".to_string(),
        QuerySort::Updated => "updated".to_string(),
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

  /// Query for `List repositories starred by the authenticated user`
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The property to sort the results by. `created` means when the repository was starred. `updated` means when the repository was last pushed to.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub sort: Option<QuerySort>,
    /// The direction to sort the results by.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub direction: Option<QueryDirection>,
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

pub mod check_repo_is_starred_by_authenticated_user {
  #[allow(unused_imports)]
  use super::*;
}

pub mod star_repo_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;
}

pub mod unstar_repo_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;
}

pub mod list_watched_repos_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<MinimalRepository>;

  /// Query for `List repositories watched by the authenticated user`
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

pub mod list_events_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Event>;

  /// Query for `List events for the authenticated user`
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

pub mod list_org_events_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Event>;

  /// Query for `List organization events for the authenticated user`
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

pub mod list_public_events_for_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Event>;

  /// Query for `List public events for a user`
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

pub mod list_received_events_for_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Event>;

  /// Query for `List events received by the authenticated user`
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

pub mod list_received_public_events_for_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Event>;

  /// Query for `List public events received by a user`
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

pub mod list_repos_starred_by_user {
  #[allow(unused_imports)]
  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QuerySort {
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "updated")]
    Updated,
  }

  impl ToString for QuerySort {
    fn to_string(&self) -> String {
      match self {
        QuerySort::Created => "created".to_string(),
        QuerySort::Updated => "updated".to_string(),
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

  #[derive(Debug, Clone, Serialize, Deserialize)]
  #[serde(untagged)]
  pub enum Response {
    StarredRepositoryArray(Vec<StarredRepository>),
    RepositoryArray(Vec<Repository>),
  }

  /// Query for `List repositories starred by a user`
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The property to sort the results by. `created` means when the repository was starred. `updated` means when the repository was last pushed to.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub sort: Option<QuerySort>,
    /// The direction to sort the results by.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub direction: Option<QueryDirection>,
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

pub mod list_repos_watched_by_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<MinimalRepository>;

  /// Query for `List repositories watched by a user`
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

/// Activity APIs provide access to notifications, subscriptions, and timelines.
pub struct GitHubActivityAPI {
  config: SharedAPIConfig,
}

impl GitHubActivityAPI {
  pub fn new(config: &SharedAPIConfig) -> Self {
    Self {
      config: config.clone(),
    }
  }

  /// **List public events**
  ///
  /// We delay the public events feed by five minutes, which means the most recent event returned by the public events API actually occurred at least five minutes ago.
  ///
  /// *Documentation*: [https://docs.github.com/rest/activity/events#list-public-events](https://docs.github.com/rest/activity/events#list-public-events)
  pub fn list_public_events(
    &self,
  ) -> Request<(), list_public_events::Query, list_public_events::Response> {
    let url = format!("/events");

    Request::<(), list_public_events::Query, list_public_events::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get feeds**
  ///
  /// Lists the feeds available to the authenticated user. The response provides a URL for each feed. You can then get a specific feed by sending a request to one of the feed URLs.
  ///
  /// *   **Timeline**: The GitHub global public timeline
  /// *   **User**: The public timeline for any user, using `uri_template`. For more information, see "[Hypermedia](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#hypermedia)."
  /// *   **Current user public**: The public timeline for the authenticated user
  /// *   **Current user**: The private timeline for the authenticated user
  /// *   **Current user actor**: The private timeline for activity created by the authenticated user
  /// *   **Current user organizations**: The private timeline for the organizations the authenticated user is a member of.
  /// *   **Security advisories**: A collection of public announcements that provide information about security-related vulnerabilities in software on GitHub.
  ///
  /// By default, timeline resources are returned in JSON. You can specify the `application/atom+xml` type in the `Accept` header to return timeline resources in Atom format. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
  ///
  /// **Note**: Private feeds are only returned when [authenticating via Basic Auth](https://docs.github.com/rest/overview/other-authentication-methods#basic-authentication) since current feed URIs use the older, non revocable auth tokens.
  ///
  /// *Documentation*: [https://docs.github.com/rest/activity/feeds#get-feeds](https://docs.github.com/rest/activity/feeds#get-feeds)
  pub fn get_feeds(&self) -> Request<(), (), get_feeds::Response> {
    let url = format!("/feeds");

    Request::<(), (), get_feeds::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List public events for a network of repositories**
  ///
  ///
  /// *Documentation*: [https://docs.github.com/rest/activity/events#list-public-events-for-a-network-of-repositories](https://docs.github.com/rest/activity/events#list-public-events-for-a-network-of-repositories)
  pub fn list_public_events_for_repo_network(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<
    (),
    list_public_events_for_repo_network::Query,
    list_public_events_for_repo_network::Response,
  > {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/networks/{owner}/{repo}/events");

    Request::<
      (),
      list_public_events_for_repo_network::Query,
      list_public_events_for_repo_network::Response,
    >::builder(&self.config)
    .get(url)
    .build()
  }

  /// **List notifications for the authenticated user**
  ///
  /// List all notifications for the current user, sorted by most recently updated.
  ///
  /// *Documentation*: [https://docs.github.com/rest/activity/notifications#list-notifications-for-the-authenticated-user](https://docs.github.com/rest/activity/notifications#list-notifications-for-the-authenticated-user)
  pub fn list_notifications_for_authenticated_user(
    &self,
  ) -> Request<
    (),
    list_notifications_for_authenticated_user::Query,
    list_notifications_for_authenticated_user::Response,
  > {
    let url = format!("/notifications");

    Request::<
      (),
      list_notifications_for_authenticated_user::Query,
      list_notifications_for_authenticated_user::Response,
    >::builder(&self.config)
    .get(url)
    .build()
  }

  /// **Mark notifications as read**
  ///
  /// Marks all notifications as "read" for the current user. If the number of notifications is too large to complete in one request, you will receive a `202 Accepted` status and GitHub will run an asynchronous process to mark notifications as "read." To check whether any "unread" notifications remain, you can use the [List notifications for the authenticated user](https://docs.github.com/rest/activity/notifications#list-notifications-for-the-authenticated-user) endpoint and pass the query parameter `all=false`.
  ///
  /// *Documentation*: [https://docs.github.com/rest/activity/notifications#mark-notifications-as-read](https://docs.github.com/rest/activity/notifications#mark-notifications-as-read)
  pub fn mark_notifications_as_read(
    &self,
  ) -> Request<mark_notifications_as_read::Request, (), mark_notifications_as_read::Response> {
    let url = format!("/notifications");

    Request::<mark_notifications_as_read::Request, (), mark_notifications_as_read::Response>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **Get a thread**
  ///
  /// Gets information about a notification thread.
  ///
  /// *Documentation*: [https://docs.github.com/rest/activity/notifications#get-a-thread](https://docs.github.com/rest/activity/notifications#get-a-thread)
  pub fn get_thread(&self, thread_id: impl Into<i64>) -> Request<(), (), get_thread::Response> {
    let thread_id = thread_id.into();
    let url = format!("/notifications/threads/{thread_id}");

    Request::<(), (), get_thread::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Mark a thread as read**
  ///
  /// Marks a thread as "read." Marking a thread as "read" is equivalent to clicking a notification in your notification inbox on GitHub: https://github.com/notifications.
  ///
  /// *Documentation*: [https://docs.github.com/rest/activity/notifications#mark-a-thread-as-read](https://docs.github.com/rest/activity/notifications#mark-a-thread-as-read)
  pub fn mark_thread_as_read(&self, thread_id: impl Into<i64>) -> NoContentRequest<(), ()> {
    let thread_id = thread_id.into();
    let url = format!("/notifications/threads/{thread_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .patch(url)
      .build()
  }

  /// **Mark a thread as done**
  ///
  /// Marks a thread as "done." Marking a thread as "done" is equivalent to marking a notification in your notification inbox on GitHub as done: https://github.com/notifications.
  ///
  /// *Documentation*: [https://docs.github.com/rest/activity/notifications#mark-a-thread-as-done](https://docs.github.com/rest/activity/notifications#mark-a-thread-as-done)
  pub fn mark_thread_as_done(&self, thread_id: impl Into<i64>) -> NoContentRequest<(), ()> {
    let thread_id = thread_id.into();
    let url = format!("/notifications/threads/{thread_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Get a thread subscription for the authenticated user**
  ///
  /// This checks to see if the current user is subscribed to a thread. You can also [get a repository subscription](https://docs.github.com/rest/activity/watching#get-a-repository-subscription).
  ///
  /// Note that subscriptions are only generated if a user is participating in a conversation--for example, they've replied to the thread, were **@mentioned**, or manually subscribe to a thread.
  ///
  /// *Documentation*: [https://docs.github.com/rest/activity/notifications#get-a-thread-subscription-for-the-authenticated-user](https://docs.github.com/rest/activity/notifications#get-a-thread-subscription-for-the-authenticated-user)
  pub fn get_thread_subscription_for_authenticated_user(
    &self,
    thread_id: impl Into<i64>,
  ) -> Request<(), (), get_thread_subscription_for_authenticated_user::Response> {
    let thread_id = thread_id.into();
    let url = format!("/notifications/threads/{thread_id}/subscription");

    Request::<(), (), get_thread_subscription_for_authenticated_user::Response>::builder(
      &self.config,
    )
    .get(url)
    .build()
  }

  /// **Set a thread subscription**
  ///
  /// If you are watching a repository, you receive notifications for all threads by default. Use this endpoint to ignore future notifications for threads until you comment on the thread or get an **@mention**.
  ///
  /// You can also use this endpoint to subscribe to threads that you are currently not receiving notifications for or to subscribed to threads that you have previously ignored.
  ///
  /// Unsubscribing from a conversation in a repository that you are not watching is functionally equivalent to the [Delete a thread subscription](https://docs.github.com/rest/activity/notifications#delete-a-thread-subscription) endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/activity/notifications#set-a-thread-subscription](https://docs.github.com/rest/activity/notifications#set-a-thread-subscription)
  pub fn set_thread_subscription(
    &self,
    thread_id: impl Into<i64>,
  ) -> Request<set_thread_subscription::Request, (), set_thread_subscription::Response> {
    let thread_id = thread_id.into();
    let url = format!("/notifications/threads/{thread_id}/subscription");

    Request::<set_thread_subscription::Request, (), set_thread_subscription::Response>::builder(
      &self.config,
    )
    .put(url)
    .build()
  }

  /// **Delete a thread subscription**
  ///
  /// Mutes all future notifications for a conversation until you comment on the thread or get an **@mention**. If you are watching the repository of the thread, you will still receive notifications. To ignore future notifications for a repository you are watching, use the [Set a thread subscription](https://docs.github.com/rest/activity/notifications#set-a-thread-subscription) endpoint and set `ignore` to `true`.
  ///
  /// *Documentation*: [https://docs.github.com/rest/activity/notifications#delete-a-thread-subscription](https://docs.github.com/rest/activity/notifications#delete-a-thread-subscription)
  pub fn delete_thread_subscription(&self, thread_id: impl Into<i64>) -> NoContentRequest<(), ()> {
    let thread_id = thread_id.into();
    let url = format!("/notifications/threads/{thread_id}/subscription");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List public organization events**
  ///
  ///
  /// *Documentation*: [https://docs.github.com/rest/activity/events#list-public-organization-events](https://docs.github.com/rest/activity/events#list-public-organization-events)
  pub fn list_public_org_events(
    &self,
    org: impl Into<String>,
  ) -> Request<(), list_public_org_events::Query, list_public_org_events::Response> {
    let org = org.into();
    let url = format!("/orgs/{org}/events");

    Request::<(), list_public_org_events::Query, list_public_org_events::Response>::builder(
      &self.config,
    )
    .get(url)
    .build()
  }

  /// **List repository events**
  ///
  /// **Note**: This API is not built to serve real-time use cases. Depending on the time of day, event latency can be anywhere from 30s to 6h.
  ///
  ///
  /// *Documentation*: [https://docs.github.com/rest/activity/events#list-repository-events](https://docs.github.com/rest/activity/events#list-repository-events)
  pub fn list_repo_events(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), list_repo_events::Query, list_repo_events::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/events");

    Request::<(), list_repo_events::Query, list_repo_events::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List repository notifications for the authenticated user**
  ///
  /// Lists all notifications for the current user in the specified repository.
  ///
  /// *Documentation*: [https://docs.github.com/rest/activity/notifications#list-repository-notifications-for-the-authenticated-user](https://docs.github.com/rest/activity/notifications#list-repository-notifications-for-the-authenticated-user)
  pub fn list_repo_notifications_for_authenticated_user(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<
    (),
    list_repo_notifications_for_authenticated_user::Query,
    list_repo_notifications_for_authenticated_user::Response,
  > {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/notifications");

    Request::<
      (),
      list_repo_notifications_for_authenticated_user::Query,
      list_repo_notifications_for_authenticated_user::Response,
    >::builder(&self.config)
    .get(url)
    .build()
  }

  /// **Mark repository notifications as read**
  ///
  /// Marks all notifications in a repository as "read" for the current user. If the number of notifications is too large to complete in one request, you will receive a `202 Accepted` status and GitHub will run an asynchronous process to mark notifications as "read." To check whether any "unread" notifications remain, you can use the [List repository notifications for the authenticated user](https://docs.github.com/rest/activity/notifications#list-repository-notifications-for-the-authenticated-user) endpoint and pass the query parameter `all=false`.
  ///
  /// *Documentation*: [https://docs.github.com/rest/activity/notifications#mark-repository-notifications-as-read](https://docs.github.com/rest/activity/notifications#mark-repository-notifications-as-read)
  pub fn mark_repo_notifications_as_read(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<
    mark_repo_notifications_as_read::Request,
    (),
    mark_repo_notifications_as_read::Response,
  > {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/notifications");

    Request::<mark_repo_notifications_as_read::Request, (), mark_repo_notifications_as_read::Response>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **List stargazers**
  ///
  /// Lists the people that have starred the repository.
  ///
  /// This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
  ///
  /// - **`application/vnd.github.star+json`**: Includes a timestamp of when the star was created.
  ///
  /// *Documentation*: [https://docs.github.com/rest/activity/starring#list-stargazers](https://docs.github.com/rest/activity/starring#list-stargazers)
  pub fn list_stargazers_for_repo(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), list_stargazers_for_repo::Query, list_stargazers_for_repo::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/stargazers");

    Request::<(), list_stargazers_for_repo::Query, list_stargazers_for_repo::Response>::builder(
      &self.config,
    )
    .get(url)
    .build()
  }

  /// **List watchers**
  ///
  /// Lists the people watching the specified repository.
  ///
  /// *Documentation*: [https://docs.github.com/rest/activity/watching#list-watchers](https://docs.github.com/rest/activity/watching#list-watchers)
  pub fn list_watchers_for_repo(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), list_watchers_for_repo::Query, list_watchers_for_repo::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/subscribers");

    Request::<(), list_watchers_for_repo::Query, list_watchers_for_repo::Response>::builder(
      &self.config,
    )
    .get(url)
    .build()
  }

  /// **Get a repository subscription**
  ///
  /// Gets information about whether the authenticated user is subscribed to the repository.
  ///
  /// *Documentation*: [https://docs.github.com/rest/activity/watching#get-a-repository-subscription](https://docs.github.com/rest/activity/watching#get-a-repository-subscription)
  pub fn get_repo_subscription(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), (), get_repo_subscription::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/subscription");

    Request::<(), (), get_repo_subscription::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Set a repository subscription**
  ///
  /// If you would like to watch a repository, set `subscribed` to `true`. If you would like to ignore notifications made within a repository, set `ignored` to `true`. If you would like to stop watching a repository, [delete the repository's subscription](https://docs.github.com/rest/activity/watching#delete-a-repository-subscription) completely.
  ///
  /// *Documentation*: [https://docs.github.com/rest/activity/watching#set-a-repository-subscription](https://docs.github.com/rest/activity/watching#set-a-repository-subscription)
  pub fn set_repo_subscription(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<set_repo_subscription::Request, (), set_repo_subscription::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/subscription");

    Request::<set_repo_subscription::Request, (), set_repo_subscription::Response>::builder(
      &self.config,
    )
    .put(url)
    .build()
  }

  /// **Delete a repository subscription**
  ///
  /// This endpoint should only be used to stop watching a repository. To control whether or not you wish to receive notifications from a repository, [set the repository's subscription manually](https://docs.github.com/rest/activity/watching#set-a-repository-subscription).
  ///
  /// *Documentation*: [https://docs.github.com/rest/activity/watching#delete-a-repository-subscription](https://docs.github.com/rest/activity/watching#delete-a-repository-subscription)
  pub fn delete_repo_subscription(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/subscription");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List repositories starred by the authenticated user**
  ///
  /// Lists repositories the authenticated user has starred.
  ///
  /// This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
  ///
  /// - **`application/vnd.github.star+json`**: Includes a timestamp of when the star was created.
  ///
  /// *Documentation*: [https://docs.github.com/rest/activity/starring#list-repositories-starred-by-the-authenticated-user](https://docs.github.com/rest/activity/starring#list-repositories-starred-by-the-authenticated-user)
  pub fn list_repos_starred_by_authenticated_user(
    &self,
  ) -> Request<
    (),
    list_repos_starred_by_authenticated_user::Query,
    list_repos_starred_by_authenticated_user::Response,
  > {
    let url = format!("/user/starred");

    Request::<
      (),
      list_repos_starred_by_authenticated_user::Query,
      list_repos_starred_by_authenticated_user::Response,
    >::builder(&self.config)
    .get(url)
    .build()
  }

  /// **Check if a repository is starred by the authenticated user**
  ///
  /// Whether the authenticated user has starred the repository.
  ///
  /// *Documentation*: [https://docs.github.com/rest/activity/starring#check-if-a-repository-is-starred-by-the-authenticated-user](https://docs.github.com/rest/activity/starring#check-if-a-repository-is-starred-by-the-authenticated-user)
  pub fn check_repo_is_starred_by_authenticated_user(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/user/starred/{owner}/{repo}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Star a repository for the authenticated user**
  ///
  /// Note that you'll need to set `Content-Length` to zero when calling out to this endpoint. For more information, see "[HTTP method](https://docs.github.com/rest/guides/getting-started-with-the-rest-api#http-method)."
  ///
  /// *Documentation*: [https://docs.github.com/rest/activity/starring#star-a-repository-for-the-authenticated-user](https://docs.github.com/rest/activity/starring#star-a-repository-for-the-authenticated-user)
  pub fn star_repo_for_authenticated_user(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/user/starred/{owner}/{repo}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **Unstar a repository for the authenticated user**
  ///
  /// Unstar a repository that the authenticated user has previously starred.
  ///
  /// *Documentation*: [https://docs.github.com/rest/activity/starring#unstar-a-repository-for-the-authenticated-user](https://docs.github.com/rest/activity/starring#unstar-a-repository-for-the-authenticated-user)
  pub fn unstar_repo_for_authenticated_user(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/user/starred/{owner}/{repo}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List repositories watched by the authenticated user**
  ///
  /// Lists repositories the authenticated user is watching.
  ///
  /// *Documentation*: [https://docs.github.com/rest/activity/watching#list-repositories-watched-by-the-authenticated-user](https://docs.github.com/rest/activity/watching#list-repositories-watched-by-the-authenticated-user)
  pub fn list_watched_repos_for_authenticated_user(
    &self,
  ) -> Request<
    (),
    list_watched_repos_for_authenticated_user::Query,
    list_watched_repos_for_authenticated_user::Response,
  > {
    let url = format!("/user/subscriptions");

    Request::<
      (),
      list_watched_repos_for_authenticated_user::Query,
      list_watched_repos_for_authenticated_user::Response,
    >::builder(&self.config)
    .get(url)
    .build()
  }

  /// **List events for the authenticated user**
  ///
  /// If you are authenticated as the given user, you will see your private events. Otherwise, you'll only see public events.
  ///
  /// *Documentation*: [https://docs.github.com/rest/activity/events#list-events-for-the-authenticated-user](https://docs.github.com/rest/activity/events#list-events-for-the-authenticated-user)
  pub fn list_events_for_authenticated_user(
    &self,
    username: impl Into<String>,
  ) -> Request<
    (),
    list_events_for_authenticated_user::Query,
    list_events_for_authenticated_user::Response,
  > {
    let username = username.into();
    let url = format!("/users/{username}/events");

    Request::<
      (),
      list_events_for_authenticated_user::Query,
      list_events_for_authenticated_user::Response,
    >::builder(&self.config)
    .get(url)
    .build()
  }

  /// **List organization events for the authenticated user**
  ///
  /// This is the user's organization dashboard. You must be authenticated as the user to view this.
  ///
  /// *Documentation*: [https://docs.github.com/rest/activity/events#list-organization-events-for-the-authenticated-user](https://docs.github.com/rest/activity/events#list-organization-events-for-the-authenticated-user)
  pub fn list_org_events_for_authenticated_user(
    &self,
    username: impl Into<String>,
    org: impl Into<String>,
  ) -> Request<
    (),
    list_org_events_for_authenticated_user::Query,
    list_org_events_for_authenticated_user::Response,
  > {
    let username = username.into();
    let org = org.into();
    let url = format!("/users/{username}/events/orgs/{org}");

    Request::<
      (),
      list_org_events_for_authenticated_user::Query,
      list_org_events_for_authenticated_user::Response,
    >::builder(&self.config)
    .get(url)
    .build()
  }

  /// **List public events for a user**
  ///
  ///
  /// *Documentation*: [https://docs.github.com/rest/activity/events#list-public-events-for-a-user](https://docs.github.com/rest/activity/events#list-public-events-for-a-user)
  pub fn list_public_events_for_user(
    &self,
    username: impl Into<String>,
  ) -> Request<(), list_public_events_for_user::Query, list_public_events_for_user::Response> {
    let username = username.into();
    let url = format!("/users/{username}/events/public");

    Request::<(), list_public_events_for_user::Query, list_public_events_for_user::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List events received by the authenticated user**
  ///
  /// These are events that you've received by watching repositories and following users. If you are authenticated as the given user, you will see private events. Otherwise, you'll only see public events.
  ///
  /// *Documentation*: [https://docs.github.com/rest/activity/events#list-events-received-by-the-authenticated-user](https://docs.github.com/rest/activity/events#list-events-received-by-the-authenticated-user)
  pub fn list_received_events_for_user(
    &self,
    username: impl Into<String>,
  ) -> Request<(), list_received_events_for_user::Query, list_received_events_for_user::Response>
  {
    let username = username.into();
    let url = format!("/users/{username}/received_events");

    Request::<(), list_received_events_for_user::Query, list_received_events_for_user::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List public events received by a user**
  ///
  ///
  /// *Documentation*: [https://docs.github.com/rest/activity/events#list-public-events-received-by-a-user](https://docs.github.com/rest/activity/events#list-public-events-received-by-a-user)
  pub fn list_received_public_events_for_user(
    &self,
    username: impl Into<String>,
  ) -> Request<
    (),
    list_received_public_events_for_user::Query,
    list_received_public_events_for_user::Response,
  > {
    let username = username.into();
    let url = format!("/users/{username}/received_events/public");

    Request::<
      (),
      list_received_public_events_for_user::Query,
      list_received_public_events_for_user::Response,
    >::builder(&self.config)
    .get(url)
    .build()
  }

  /// **List repositories starred by a user**
  ///
  /// Lists repositories a user has starred.
  ///
  /// This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
  ///
  /// - **`application/vnd.github.star+json`**: Includes a timestamp of when the star was created.
  ///
  /// *Documentation*: [https://docs.github.com/rest/activity/starring#list-repositories-starred-by-a-user](https://docs.github.com/rest/activity/starring#list-repositories-starred-by-a-user)
  pub fn list_repos_starred_by_user(
    &self,
    username: impl Into<String>,
  ) -> Request<(), list_repos_starred_by_user::Query, list_repos_starred_by_user::Response> {
    let username = username.into();
    let url = format!("/users/{username}/starred");

    Request::<(), list_repos_starred_by_user::Query, list_repos_starred_by_user::Response>::builder(
      &self.config,
    )
    .get(url)
    .build()
  }

  /// **List repositories watched by a user**
  ///
  /// Lists repositories a user is watching.
  ///
  /// *Documentation*: [https://docs.github.com/rest/activity/watching#list-repositories-watched-by-a-user](https://docs.github.com/rest/activity/watching#list-repositories-watched-by-a-user)
  pub fn list_repos_watched_by_user(
    &self,
    username: impl Into<String>,
  ) -> Request<(), list_repos_watched_by_user::Query, list_repos_watched_by_user::Response> {
    let username = username.into();
    let url = format!("/users/{username}/subscriptions");

    Request::<(), list_repos_watched_by_user::Query, list_repos_watched_by_user::Response>::builder(
      &self.config,
    )
    .get(url)
    .build()
  }
}
