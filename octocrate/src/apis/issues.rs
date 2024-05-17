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

  pub type Response = Vec<Issue>;

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QueryFilter {
    #[serde(rename = "assigned")]
    Assigned,
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "mentioned")]
    Mentioned,
    #[serde(rename = "subscribed")]
    Subscribed,
    #[serde(rename = "repos")]
    Repos,
    #[serde(rename = "all")]
    All,
  }

  impl ToString for QueryFilter {
    fn to_string(&self) -> String {
      match self {
        QueryFilter::Assigned => "assigned".to_string(),
        QueryFilter::Created => "created".to_string(),
        QueryFilter::Mentioned => "mentioned".to_string(),
        QueryFilter::Subscribed => "subscribed".to_string(),
        QueryFilter::Repos => "repos".to_string(),
        QueryFilter::All => "all".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QueryState {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "closed")]
    Closed,
    #[serde(rename = "all")]
    All,
  }

  impl ToString for QueryState {
    fn to_string(&self) -> String {
      match self {
        QueryState::Open => "open".to_string(),
        QueryState::Closed => "closed".to_string(),
        QueryState::All => "all".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QuerySort {
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "updated")]
    Updated,
    #[serde(rename = "comments")]
    Comments,
  }

  impl ToString for QuerySort {
    fn to_string(&self) -> String {
      match self {
        QuerySort::Created => "created".to_string(),
        QuerySort::Updated => "updated".to_string(),
        QuerySort::Comments => "comments".to_string(),
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

  /// Query for `List issues assigned to the authenticated user`
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// Indicates which sorts of issues to return. `assigned` means issues assigned to you. `created` means issues created by you. `mentioned` means issues mentioning you. `subscribed` means issues you're subscribed to updates for. `all` or `repos` means all issues you can see, regardless of participation or creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub filter: Option<QueryFilter>,
    /// Indicates the state of the issues to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub state: Option<QueryState>,
    /// A list of comma separated label names. Example: `bug,ui,@high`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub labels: Option<String>,
    /// What to sort results by.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub sort: Option<QuerySort>,
    /// The direction to sort the results by.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub direction: Option<QueryDirection>,
    /// Only show results that were last updated after the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub since: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub collab: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub orgs: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub owned: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub pulls: Option<bool>,
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

pub mod list_for_org {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Issue>;

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QueryFilter {
    #[serde(rename = "assigned")]
    Assigned,
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "mentioned")]
    Mentioned,
    #[serde(rename = "subscribed")]
    Subscribed,
    #[serde(rename = "repos")]
    Repos,
    #[serde(rename = "all")]
    All,
  }

  impl ToString for QueryFilter {
    fn to_string(&self) -> String {
      match self {
        QueryFilter::Assigned => "assigned".to_string(),
        QueryFilter::Created => "created".to_string(),
        QueryFilter::Mentioned => "mentioned".to_string(),
        QueryFilter::Subscribed => "subscribed".to_string(),
        QueryFilter::Repos => "repos".to_string(),
        QueryFilter::All => "all".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QueryState {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "closed")]
    Closed,
    #[serde(rename = "all")]
    All,
  }

  impl ToString for QueryState {
    fn to_string(&self) -> String {
      match self {
        QueryState::Open => "open".to_string(),
        QueryState::Closed => "closed".to_string(),
        QueryState::All => "all".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QuerySort {
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "updated")]
    Updated,
    #[serde(rename = "comments")]
    Comments,
  }

  impl ToString for QuerySort {
    fn to_string(&self) -> String {
      match self {
        QuerySort::Created => "created".to_string(),
        QuerySort::Updated => "updated".to_string(),
        QuerySort::Comments => "comments".to_string(),
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

  /// Query for `List organization issues assigned to the authenticated user`
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// Indicates which sorts of issues to return. `assigned` means issues assigned to you. `created` means issues created by you. `mentioned` means issues mentioning you. `subscribed` means issues you're subscribed to updates for. `all` or `repos` means all issues you can see, regardless of participation or creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub filter: Option<QueryFilter>,
    /// Indicates the state of the issues to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub state: Option<QueryState>,
    /// A list of comma separated label names. Example: `bug,ui,@high`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub labels: Option<String>,
    /// What to sort results by.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub sort: Option<QuerySort>,
    /// The direction to sort the results by.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub direction: Option<QueryDirection>,
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

pub mod list_assignees {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<SimpleUser>;

  /// Query for `List assignees`
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

pub mod check_user_can_be_assigned {
  #[allow(unused_imports)]
  use super::*;
}

pub mod list_for_repo {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Issue>;

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QueryState {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "closed")]
    Closed,
    #[serde(rename = "all")]
    All,
  }

  impl ToString for QueryState {
    fn to_string(&self) -> String {
      match self {
        QueryState::Open => "open".to_string(),
        QueryState::Closed => "closed".to_string(),
        QueryState::All => "all".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QuerySort {
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "updated")]
    Updated,
    #[serde(rename = "comments")]
    Comments,
  }

  impl ToString for QuerySort {
    fn to_string(&self) -> String {
      match self {
        QuerySort::Created => "created".to_string(),
        QuerySort::Updated => "updated".to_string(),
        QuerySort::Comments => "comments".to_string(),
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

  /// Query for `List repository issues`
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// If an `integer` is passed, it should refer to a milestone by its `number` field. If the string `*` is passed, issues with any milestone are accepted. If the string `none` is passed, issues without milestones are returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub milestone: Option<String>,
    /// Indicates the state of the issues to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub state: Option<QueryState>,
    /// Can be the name of a user. Pass in `none` for issues with no assigned user, and `*` for issues assigned to any user.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub assignee: Option<String>,
    /// The user that created the issue.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub creator: Option<String>,
    /// A user that's mentioned in the issue.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub mentioned: Option<String>,
    /// A list of comma separated label names. Example: `bug,ui,@high`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub labels: Option<String>,
    /// What to sort results by.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub sort: Option<QuerySort>,
    /// The direction to sort the results by.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub direction: Option<QueryDirection>,
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

  pub type Response = Issue;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct RequestLabelsItem2 {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub name: Option<String>,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// Login for the user that this issue should be assigned to. _NOTE: Only users with push access can set the assignee for new issues. The assignee is silently dropped otherwise. **This field is deprecated.**_
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub assignee: Option<String>,
    /// Logins for Users to assign to this issue. _NOTE: Only users with push access can set assignees for new issues. Assignees are silently dropped otherwise._
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub assignees: Option<Vec<String>>,
    /// The contents of the issue.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub body: Option<String>,
    /// Labels to associate with this issue. _NOTE: Only users with push access can set labels for new issues. Labels are silently dropped otherwise._
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub labels: Option<Vec<ObjectOrString<RequestLabelsItem2>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub milestone: Option<StringOrInteger>,
    /// The title of the issue.
    pub title: StringOrInteger,
  }
}

pub mod list_comments_for_repo {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<IssueComment>;

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

  /// Query for `List issue comments for a repository`
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The property to sort the results by.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub sort: Option<QuerySort>,
    /// Either `asc` or `desc`. Ignored without the `sort` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub direction: Option<QueryDirection>,
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

pub mod get_comment {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = IssueComment;
}

pub mod update_comment {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = IssueComment;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The contents of the comment.
    pub body: String,
  }
}

pub mod delete_comment {
  #[allow(unused_imports)]
  use super::*;
}

pub mod list_events_for_repo {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<IssueEvent>;

  /// Query for `List issue events for a repository`
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

pub mod get_event {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = IssueEvent;
}

pub mod get {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Issue;
}

pub mod update {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Issue;

  /// The open or closed state of the issue.
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum RequestState {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "closed")]
    Closed,
  }

  impl ToString for RequestState {
    fn to_string(&self) -> String {
      match self {
        RequestState::Open => "open".to_string(),
        RequestState::Closed => "closed".to_string(),
      }
    }
  }

  /// The reason for the state change. Ignored unless `state` is changed.
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum RequestStateReason {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "not_planned")]
    NotPlanned,
    #[serde(rename = "reopened")]
    Reopened,
  }

  impl ToString for RequestStateReason {
    fn to_string(&self) -> String {
      match self {
        RequestStateReason::Completed => "completed".to_string(),
        RequestStateReason::NotPlanned => "not_planned".to_string(),
        RequestStateReason::Reopened => "reopened".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct RequestLabelsItem2 {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub name: Option<String>,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// Username to assign to this issue. **This field is deprecated.**
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub assignee: Option<String>,
    /// Usernames to assign to this issue. Pass one or more user logins to _replace_ the set of assignees on this issue. Send an empty array (`[]`) to clear all assignees from the issue. Only users with push access can set assignees for new issues. Without push access to the repository, assignee changes are silently dropped.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub assignees: Option<Vec<String>>,
    /// The contents of the issue.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub body: Option<String>,
    /// Labels to associate with this issue. Pass one or more labels to _replace_ the set of labels on this issue. Send an empty array (`[]`) to clear all labels from the issue. Only users with push access can set labels for issues. Without push access to the repository, label changes are silently dropped.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub labels: Option<Vec<ObjectOrString<RequestLabelsItem2>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub milestone: Option<StringOrInteger>,
    /// The open or closed state of the issue.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub state: Option<RequestState>,
    /// The reason for the state change. Ignored unless `state` is changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub state_reason: Option<RequestStateReason>,
    /// The title of the issue.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub title: Option<StringOrInteger>,
  }
}

pub mod add_assignees {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Issue;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// Usernames of people to assign this issue to. _NOTE: Only users with push access can add assignees to an issue. Assignees are silently ignored otherwise._
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub assignees: Option<Vec<String>>,
  }
}

pub mod remove_assignees {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Issue;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// Usernames of assignees to remove from an issue. _NOTE: Only users with push access can remove assignees from an issue. Assignees are silently ignored otherwise._
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub assignees: Option<Vec<String>>,
  }
}

pub mod check_user_can_be_assigned_to_issue {
  #[allow(unused_imports)]
  use super::*;
}

pub mod list_comments {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<IssueComment>;

  /// Query for `List issue comments`
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

pub mod create_comment {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = IssueComment;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The contents of the comment.
    pub body: String,
  }
}

pub mod list_events {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<IssueEventForIssue>;

  /// Query for `List issue events`
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

pub mod list_labels_on_issue {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Label>;

  /// Query for `List labels for an issue`
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

pub mod add_labels {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Label>;

  #[derive(Debug, Clone, Serialize, Deserialize)]
  #[serde(untagged)]
  pub enum Request {
    RequestItem1(RequestItem1),
    StringArray(Vec<String>),
    RequestItem3(RequestItem3),
    RequestItem4Array(Vec<RequestItem4>),
    String(String),
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct RequestItem1 {
    /// The names of the labels to add to the issue's existing labels. You can pass an empty array to remove all labels. Alternatively, you can pass a single label as a `string` or an `array` of labels directly, but GitHub recommends passing an object with the `labels` key. You can also replace all of the labels for an issue. For more information, see "[Set labels for an issue](https://docs.github.com/rest/issues/labels#set-labels-for-an-issue)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub labels: Option<Vec<String>>,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct RequestItem3Labels {
    pub name: String,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct RequestItem3 {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub labels: Option<Vec<RequestItem3Labels>>,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct RequestItem4 {
    pub name: String,
  }
}

pub mod set_labels {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Label>;

  #[derive(Debug, Clone, Serialize, Deserialize)]
  #[serde(untagged)]
  pub enum Request {
    RequestItem1(RequestItem1),
    StringArray(Vec<String>),
    RequestItem3(RequestItem3),
    RequestItem4Array(Vec<RequestItem4>),
    String(String),
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct RequestItem1 {
    /// The names of the labels to set for the issue. The labels you set replace any existing labels. You can pass an empty array to remove all labels. Alternatively, you can pass a single label as a `string` or an `array` of labels directly, but GitHub recommends passing an object with the `labels` key. You can also add labels to the existing labels for an issue. For more information, see "[Add labels to an issue](https://docs.github.com/rest/issues/labels#add-labels-to-an-issue)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub labels: Option<Vec<String>>,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct RequestItem3Labels {
    pub name: String,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct RequestItem3 {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub labels: Option<Vec<RequestItem3Labels>>,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct RequestItem4 {
    pub name: String,
  }
}

pub mod remove_all_labels {
  #[allow(unused_imports)]
  use super::*;
}

pub mod remove_label {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Label>;
}

pub mod lock {
  #[allow(unused_imports)]
  use super::*;

  /// The reason for locking the issue or pull request conversation. Lock will fail if you don't use one of these reasons:  
  ///  * `off-topic`  
  ///  * `too heated`  
  ///  * `resolved`  
  ///  * `spam`
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum RequestLockReason {
    #[serde(rename = "off-topic")]
    OffTopic,
    #[serde(rename = "too heated")]
    TooHeated,
    #[serde(rename = "resolved")]
    Resolved,
    #[serde(rename = "spam")]
    Spam,
  }

  impl ToString for RequestLockReason {
    fn to_string(&self) -> String {
      match self {
        RequestLockReason::OffTopic => "off-topic".to_string(),
        RequestLockReason::TooHeated => "too heated".to_string(),
        RequestLockReason::Resolved => "resolved".to_string(),
        RequestLockReason::Spam => "spam".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The reason for locking the issue or pull request conversation. Lock will fail if you don't use one of these reasons:  
    ///  * `off-topic`  
    ///  * `too heated`  
    ///  * `resolved`  
    ///  * `spam`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub lock_reason: Option<RequestLockReason>,
  }
}

pub mod unlock {
  #[allow(unused_imports)]
  use super::*;
}

pub mod list_events_for_timeline {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<TimelineIssueEvents>;

  /// Query for `List timeline events for an issue`
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

pub mod list_labels_for_repo {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Label>;

  /// Query for `List labels for a repository`
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

pub mod create_label {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Label;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The [hexadecimal color code](http://www.color-hex.com/) for the label, without the leading `#`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub color: Option<String>,
    /// A short description of the label. Must be 100 characters or fewer.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub description: Option<String>,
    /// The name of the label. Emoji can be added to label names, using either native emoji or colon-style markup. For example, typing `:strawberry:` will render the emoji ![:strawberry:](https://github.githubassets.com/images/icons/emoji/unicode/1f353.png ":strawberry:"). For a full list of available emoji and codes, see "[Emoji cheat sheet](https://github.com/ikatyang/emoji-cheat-sheet)."
    pub name: String,
  }
}

pub mod get_label {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Label;
}

pub mod update_label {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Label;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The [hexadecimal color code](http://www.color-hex.com/) for the label, without the leading `#`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub color: Option<String>,
    /// A short description of the label. Must be 100 characters or fewer.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub description: Option<String>,
    /// The new name of the label. Emoji can be added to label names, using either native emoji or colon-style markup. For example, typing `:strawberry:` will render the emoji ![:strawberry:](https://github.githubassets.com/images/icons/emoji/unicode/1f353.png ":strawberry:"). For a full list of available emoji and codes, see "[Emoji cheat sheet](https://github.com/ikatyang/emoji-cheat-sheet)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub new_name: Option<String>,
  }
}

pub mod delete_label {
  #[allow(unused_imports)]
  use super::*;
}

pub mod list_milestones {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Milestone>;

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QueryState {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "closed")]
    Closed,
    #[serde(rename = "all")]
    All,
  }

  impl ToString for QueryState {
    fn to_string(&self) -> String {
      match self {
        QueryState::Open => "open".to_string(),
        QueryState::Closed => "closed".to_string(),
        QueryState::All => "all".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QuerySort {
    #[serde(rename = "due_on")]
    DueOn,
    #[serde(rename = "completeness")]
    Completeness,
  }

  impl ToString for QuerySort {
    fn to_string(&self) -> String {
      match self {
        QuerySort::DueOn => "due_on".to_string(),
        QuerySort::Completeness => "completeness".to_string(),
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

  /// Query for `List milestones`
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The state of the milestone. Either `open`, `closed`, or `all`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub state: Option<QueryState>,
    /// What to sort results by. Either `due_on` or `completeness`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub sort: Option<QuerySort>,
    /// The direction of the sort. Either `asc` or `desc`.
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

pub mod create_milestone {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Milestone;

  /// The state of the milestone. Either `open` or `closed`.
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum RequestState {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "closed")]
    Closed,
  }

  impl ToString for RequestState {
    fn to_string(&self) -> String {
      match self {
        RequestState::Open => "open".to_string(),
        RequestState::Closed => "closed".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// A description of the milestone.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub description: Option<String>,
    /// The milestone due date. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub due_on: Option<String>,
    /// The state of the milestone. Either `open` or `closed`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub state: Option<RequestState>,
    /// The title of the milestone.
    pub title: String,
  }
}

pub mod get_milestone {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Milestone;
}

pub mod update_milestone {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Milestone;

  /// The state of the milestone. Either `open` or `closed`.
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum RequestState {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "closed")]
    Closed,
  }

  impl ToString for RequestState {
    fn to_string(&self) -> String {
      match self {
        RequestState::Open => "open".to_string(),
        RequestState::Closed => "closed".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// A description of the milestone.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub description: Option<String>,
    /// The milestone due date. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub due_on: Option<String>,
    /// The state of the milestone. Either `open` or `closed`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub state: Option<RequestState>,
    /// The title of the milestone.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub title: Option<String>,
  }
}

pub mod delete_milestone {
  #[allow(unused_imports)]
  use super::*;
}

pub mod list_labels_for_milestone {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Label>;

  /// Query for `List labels for issues in a milestone`
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

pub mod list_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Issue>;

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QueryFilter {
    #[serde(rename = "assigned")]
    Assigned,
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "mentioned")]
    Mentioned,
    #[serde(rename = "subscribed")]
    Subscribed,
    #[serde(rename = "repos")]
    Repos,
    #[serde(rename = "all")]
    All,
  }

  impl ToString for QueryFilter {
    fn to_string(&self) -> String {
      match self {
        QueryFilter::Assigned => "assigned".to_string(),
        QueryFilter::Created => "created".to_string(),
        QueryFilter::Mentioned => "mentioned".to_string(),
        QueryFilter::Subscribed => "subscribed".to_string(),
        QueryFilter::Repos => "repos".to_string(),
        QueryFilter::All => "all".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QueryState {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "closed")]
    Closed,
    #[serde(rename = "all")]
    All,
  }

  impl ToString for QueryState {
    fn to_string(&self) -> String {
      match self {
        QueryState::Open => "open".to_string(),
        QueryState::Closed => "closed".to_string(),
        QueryState::All => "all".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QuerySort {
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "updated")]
    Updated,
    #[serde(rename = "comments")]
    Comments,
  }

  impl ToString for QuerySort {
    fn to_string(&self) -> String {
      match self {
        QuerySort::Created => "created".to_string(),
        QuerySort::Updated => "updated".to_string(),
        QuerySort::Comments => "comments".to_string(),
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

  /// Query for `List user account issues assigned to the authenticated user`
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// Indicates which sorts of issues to return. `assigned` means issues assigned to you. `created` means issues created by you. `mentioned` means issues mentioning you. `subscribed` means issues you're subscribed to updates for. `all` or `repos` means all issues you can see, regardless of participation or creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub filter: Option<QueryFilter>,
    /// Indicates the state of the issues to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub state: Option<QueryState>,
    /// A list of comma separated label names. Example: `bug,ui,@high`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub labels: Option<String>,
    /// What to sort results by.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub sort: Option<QuerySort>,
    /// The direction to sort the results by.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub direction: Option<QueryDirection>,
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
  pub fn list(&self) -> Request<(), list::Query, list::Response> {
    let url = format!("/issues");

    Request::<(), list::Query, list::Response>::builder(&self.config)
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
  ) -> Request<(), list_for_org::Query, list_for_org::Response> {
    let org = org.into();
    let url = format!("/orgs/{org}/issues");

    Request::<(), list_for_org::Query, list_for_org::Response>::builder(&self.config)
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
  ) -> Request<(), list_assignees::Query, list_assignees::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/assignees");

    Request::<(), list_assignees::Query, list_assignees::Response>::builder(&self.config)
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
  ) -> Request<(), list_for_repo::Query, list_for_repo::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/issues");

    Request::<(), list_for_repo::Query, list_for_repo::Response>::builder(&self.config)
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
  ) -> Request<create::Request, (), create::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/issues");

    Request::<create::Request, (), create::Response>::builder(&self.config)
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
  ) -> Request<(), list_comments_for_repo::Query, list_comments_for_repo::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/issues/comments");

    Request::<(), list_comments_for_repo::Query, list_comments_for_repo::Response>::builder(
      &self.config,
    )
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
  ) -> Request<(), (), get_comment::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let comment_id = comment_id.into();
    let url = format!("/repos/{owner}/{repo}/issues/comments/{comment_id}");

    Request::<(), (), get_comment::Response>::builder(&self.config)
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
  ) -> Request<update_comment::Request, (), update_comment::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let comment_id = comment_id.into();
    let url = format!("/repos/{owner}/{repo}/issues/comments/{comment_id}");

    Request::<update_comment::Request, (), update_comment::Response>::builder(&self.config)
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
  ) -> Request<(), list_events_for_repo::Query, list_events_for_repo::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/issues/events");

    Request::<(), list_events_for_repo::Query, list_events_for_repo::Response>::builder(
      &self.config,
    )
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
  ) -> Request<(), (), get_event::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let event_id = event_id.into();
    let url = format!("/repos/{owner}/{repo}/issues/events/{event_id}");

    Request::<(), (), get_event::Response>::builder(&self.config)
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
  ) -> Request<(), (), get::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let issue_number = issue_number.into();
    let url = format!("/repos/{owner}/{repo}/issues/{issue_number}");

    Request::<(), (), get::Response>::builder(&self.config)
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
  ) -> Request<update::Request, (), update::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let issue_number = issue_number.into();
    let url = format!("/repos/{owner}/{repo}/issues/{issue_number}");

    Request::<update::Request, (), update::Response>::builder(&self.config)
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
  ) -> Request<add_assignees::Request, (), add_assignees::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let issue_number = issue_number.into();
    let url = format!("/repos/{owner}/{repo}/issues/{issue_number}/assignees");

    Request::<add_assignees::Request, (), add_assignees::Response>::builder(&self.config)
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
  ) -> Request<remove_assignees::Request, (), remove_assignees::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let issue_number = issue_number.into();
    let url = format!("/repos/{owner}/{repo}/issues/{issue_number}/assignees");

    Request::<remove_assignees::Request, (), remove_assignees::Response>::builder(&self.config)
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
  ) -> Request<(), list_comments::Query, list_comments::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let issue_number = issue_number.into();
    let url = format!("/repos/{owner}/{repo}/issues/{issue_number}/comments");

    Request::<(), list_comments::Query, list_comments::Response>::builder(&self.config)
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
  ) -> Request<create_comment::Request, (), create_comment::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let issue_number = issue_number.into();
    let url = format!("/repos/{owner}/{repo}/issues/{issue_number}/comments");

    Request::<create_comment::Request, (), create_comment::Response>::builder(&self.config)
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
  ) -> Request<(), list_events::Query, list_events::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let issue_number = issue_number.into();
    let url = format!("/repos/{owner}/{repo}/issues/{issue_number}/events");

    Request::<(), list_events::Query, list_events::Response>::builder(&self.config)
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
  ) -> Request<(), list_labels_on_issue::Query, list_labels_on_issue::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let issue_number = issue_number.into();
    let url = format!("/repos/{owner}/{repo}/issues/{issue_number}/labels");

    Request::<(), list_labels_on_issue::Query, list_labels_on_issue::Response>::builder(
      &self.config,
    )
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
  ) -> Request<add_labels::Request, (), add_labels::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let issue_number = issue_number.into();
    let url = format!("/repos/{owner}/{repo}/issues/{issue_number}/labels");

    Request::<add_labels::Request, (), add_labels::Response>::builder(&self.config)
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
  ) -> Request<set_labels::Request, (), set_labels::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let issue_number = issue_number.into();
    let url = format!("/repos/{owner}/{repo}/issues/{issue_number}/labels");

    Request::<set_labels::Request, (), set_labels::Response>::builder(&self.config)
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
  ) -> Request<(), (), remove_label::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let issue_number = issue_number.into();
    let name = name.into();
    let url = format!("/repos/{owner}/{repo}/issues/{issue_number}/labels/{name}");

    Request::<(), (), remove_label::Response>::builder(&self.config)
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
  ) -> NoContentRequest<lock::Request, ()> {
    let owner = owner.into();
    let repo = repo.into();
    let issue_number = issue_number.into();
    let url = format!("/repos/{owner}/{repo}/issues/{issue_number}/lock");

    NoContentRequest::<lock::Request, ()>::builder(&self.config)
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
  ) -> Request<(), list_events_for_timeline::Query, list_events_for_timeline::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let issue_number = issue_number.into();
    let url = format!("/repos/{owner}/{repo}/issues/{issue_number}/timeline");

    Request::<(), list_events_for_timeline::Query, list_events_for_timeline::Response>::builder(
      &self.config,
    )
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
  ) -> Request<(), list_labels_for_repo::Query, list_labels_for_repo::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/labels");

    Request::<(), list_labels_for_repo::Query, list_labels_for_repo::Response>::builder(
      &self.config,
    )
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
  ) -> Request<create_label::Request, (), create_label::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/labels");

    Request::<create_label::Request, (), create_label::Response>::builder(&self.config)
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
  ) -> Request<(), (), get_label::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let name = name.into();
    let url = format!("/repos/{owner}/{repo}/labels/{name}");

    Request::<(), (), get_label::Response>::builder(&self.config)
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
  ) -> Request<update_label::Request, (), update_label::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let name = name.into();
    let url = format!("/repos/{owner}/{repo}/labels/{name}");

    Request::<update_label::Request, (), update_label::Response>::builder(&self.config)
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
  ) -> Request<(), list_milestones::Query, list_milestones::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/milestones");

    Request::<(), list_milestones::Query, list_milestones::Response>::builder(&self.config)
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
  ) -> Request<create_milestone::Request, (), create_milestone::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/milestones");

    Request::<create_milestone::Request, (), create_milestone::Response>::builder(&self.config)
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
  ) -> Request<(), (), get_milestone::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let milestone_number = milestone_number.into();
    let url = format!("/repos/{owner}/{repo}/milestones/{milestone_number}");

    Request::<(), (), get_milestone::Response>::builder(&self.config)
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
  ) -> Request<update_milestone::Request, (), update_milestone::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let milestone_number = milestone_number.into();
    let url = format!("/repos/{owner}/{repo}/milestones/{milestone_number}");

    Request::<update_milestone::Request, (), update_milestone::Response>::builder(&self.config)
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
  ) -> Request<(), list_labels_for_milestone::Query, list_labels_for_milestone::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let milestone_number = milestone_number.into();
    let url = format!("/repos/{owner}/{repo}/milestones/{milestone_number}/labels");

    Request::<(), list_labels_for_milestone::Query, list_labels_for_milestone::Response>::builder(
      &self.config,
    )
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
  ) -> Request<(), list_for_authenticated_user::Query, list_for_authenticated_user::Response> {
    let url = format!("/user/issues");

    Request::<(), list_for_authenticated_user::Query, list_for_authenticated_user::Response>::builder(&self.config)
      .get(url)
      .build()
  }
}
