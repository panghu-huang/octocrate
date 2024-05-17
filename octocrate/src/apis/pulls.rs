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

  pub type Response = Vec<PullRequestSimple>;

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
    #[serde(rename = "popularity")]
    Popularity,
    #[serde(rename = "long-running")]
    LongRunning,
  }

  impl ToString for QuerySort {
    fn to_string(&self) -> String {
      match self {
        QuerySort::Created => "created".to_string(),
        QuerySort::Updated => "updated".to_string(),
        QuerySort::Popularity => "popularity".to_string(),
        QuerySort::LongRunning => "long-running".to_string(),
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

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// Either `open`, `closed`, or `all` to filter by state.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub state: Option<QueryState>,
    /// Filter pulls by head user or head organization and branch name in the format of `user:ref-name` or `organization:ref-name`. For example: `github:new-script-format` or `octocat:test-branch`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub head: Option<String>,
    /// Filter pulls by base branch name. Example: `gh-pages`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub base: Option<String>,
    /// What to sort results by. `popularity` will sort by the number of comments. `long-running` will sort by date created and will limit the results to pull requests that have been open for more than a month and have had activity within the past month.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub sort: Option<QuerySort>,
    /// The direction of the sort. Default: `desc` when sort is `created` or sort is not specified, otherwise `asc`.
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

pub mod create {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = PullRequest;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The name of the branch you want the changes pulled into. This should be an existing branch on the current repository. You cannot submit a pull request to one repository that requests a merge to a base of another repository.
    pub base: String,
    /// The contents of the pull request.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub body: Option<String>,
    /// Indicates whether the pull request is a draft. See "[Draft Pull Requests](https://docs.github.com/articles/about-pull-requests#draft-pull-requests)" in the GitHub Help documentation to learn more.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub draft: Option<bool>,
    /// The name of the branch where your changes are implemented. For cross-repository pull requests in the same network, namespace `head` with a user like this: `username:branch`.
    pub head: String,
    /// The name of the repository where the changes in the pull request were made. This field is required for cross-repository pull requests if both repositories are owned by the same organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub head_repo: Option<String>,
    /// An issue in the repository to convert to a pull request. The issue title, body, and comments will become the title, body, and comments on the new pull request. Required unless `title` is specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub issue: Option<i64>,
    /// Indicates whether [maintainers can modify](https://docs.github.com/articles/allowing-changes-to-a-pull-request-branch-created-from-a-fork/) the pull request.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub maintainer_can_modify: Option<bool>,
    /// The title of the new pull request. Required unless `issue` is specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub title: Option<String>,
  }
}

pub mod list_review_comments_for_repo {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<PullRequestReviewComment>;

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QuerySort {
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "updated")]
    Updated,
    #[serde(rename = "created_at")]
    CreatedAt,
  }

  impl ToString for QuerySort {
    fn to_string(&self) -> String {
      match self {
        QuerySort::Created => "created".to_string(),
        QuerySort::Updated => "updated".to_string(),
        QuerySort::CreatedAt => "created_at".to_string(),
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

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub sort: Option<QuerySort>,
    /// The direction to sort results. Ignored without `sort` parameter.
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

pub mod get_review_comment {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = PullRequestReviewComment;
}

pub mod update_review_comment {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = PullRequestReviewComment;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The text of the reply to the review comment.
    pub body: String,
  }
}

pub mod delete_review_comment {
  #[allow(unused_imports)]
  use super::*;
}

pub mod get {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = PullRequest;
}

pub mod update {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = PullRequest;

  /// State of this Pull Request. Either `open` or `closed`.
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
    /// The name of the branch you want your changes pulled into. This should be an existing branch on the current repository. You cannot update the base branch on a pull request to point to another repository.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub base: Option<String>,
    /// The contents of the pull request.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub body: Option<String>,
    /// Indicates whether [maintainers can modify](https://docs.github.com/articles/allowing-changes-to-a-pull-request-branch-created-from-a-fork/) the pull request.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub maintainer_can_modify: Option<bool>,
    /// State of this Pull Request. Either `open` or `closed`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub state: Option<RequestState>,
    /// The title of the pull request.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub title: Option<String>,
  }
}

pub mod list_review_comments {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<PullRequestReviewComment>;

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

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The property to sort the results by.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub sort: Option<QuerySort>,
    /// The direction to sort results. Ignored without `sort` parameter.
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

pub mod create_review_comment {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = PullRequestReviewComment;

  /// In a split diff view, the side of the diff that the pull request's changes appear on. Can be `LEFT` or `RIGHT`. Use `LEFT` for deletions that appear in red. Use `RIGHT` for additions that appear in green or unchanged lines that appear in white and are shown for context. For a multi-line comment, side represents whether the last line of the comment range is a deletion or addition. For more information, see "[Diff view options](https://docs.github.com/articles/about-comparing-branches-in-pull-requests#diff-view-options)" in the GitHub Help documentation.
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum RequestSide {
    #[serde(rename = "LEFT")]
    Left,
    #[serde(rename = "RIGHT")]
    Right,
  }

  impl ToString for RequestSide {
    fn to_string(&self) -> String {
      match self {
        RequestSide::Left => "LEFT".to_string(),
        RequestSide::Right => "RIGHT".to_string(),
      }
    }
  }

  /// **Required when using multi-line comments unless using `in_reply_to`**. The `start_side` is the starting side of the diff that the comment applies to. Can be `LEFT` or `RIGHT`. To learn more about multi-line comments, see "[Commenting on a pull request](https://docs.github.com/articles/commenting-on-a-pull-request#adding-line-comments-to-a-pull-request)" in the GitHub Help documentation. See `side` in this table for additional context.
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum RequestStartSide {
    #[serde(rename = "LEFT")]
    Left,
    #[serde(rename = "RIGHT")]
    Right,
    #[serde(rename = "side")]
    Side,
  }

  impl ToString for RequestStartSide {
    fn to_string(&self) -> String {
      match self {
        RequestStartSide::Left => "LEFT".to_string(),
        RequestStartSide::Right => "RIGHT".to_string(),
        RequestStartSide::Side => "side".to_string(),
      }
    }
  }

  /// The level at which the comment is targeted.
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum RequestSubjectType {
    #[serde(rename = "line")]
    Line,
    #[serde(rename = "file")]
    File,
  }

  impl ToString for RequestSubjectType {
    fn to_string(&self) -> String {
      match self {
        RequestSubjectType::Line => "line".to_string(),
        RequestSubjectType::File => "file".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The text of the review comment.
    pub body: String,
    /// The SHA of the commit needing a comment. Not using the latest commit SHA may render your comment outdated if a subsequent commit modifies the line you specify as the `position`.
    pub commit_id: String,
    /// The ID of the review comment to reply to. To find the ID of a review comment with ["List review comments on a pull request"](#list-review-comments-on-a-pull-request). When specified, all parameters other than `body` in the request body are ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub in_reply_to: Option<i64>,
    /// **Required unless using `subject_type:file`**. The line of the blob in the pull request diff that the comment applies to. For a multi-line comment, the last line of the range that your comment applies to.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub line: Option<i64>,
    /// The relative path to the file that necessitates a comment.
    pub path: String,
    /// **This parameter is deprecated. Use `line` instead**. The position in the diff where you want to add a review comment. Note this value is not the same as the line number in the file. The position value equals the number of lines down from the first "@@" hunk header in the file you want to add a comment. The line just below the "@@" line is position 1, the next line is position 2, and so on. The position in the diff continues to increase through lines of whitespace and additional hunks until the beginning of a new file.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub position: Option<i64>,
    /// In a split diff view, the side of the diff that the pull request's changes appear on. Can be `LEFT` or `RIGHT`. Use `LEFT` for deletions that appear in red. Use `RIGHT` for additions that appear in green or unchanged lines that appear in white and are shown for context. For a multi-line comment, side represents whether the last line of the comment range is a deletion or addition. For more information, see "[Diff view options](https://docs.github.com/articles/about-comparing-branches-in-pull-requests#diff-view-options)" in the GitHub Help documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub side: Option<RequestSide>,
    /// **Required when using multi-line comments unless using `in_reply_to`**. The `start_line` is the first line in the pull request diff that your multi-line comment applies to. To learn more about multi-line comments, see "[Commenting on a pull request](https://docs.github.com/articles/commenting-on-a-pull-request#adding-line-comments-to-a-pull-request)" in the GitHub Help documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub start_line: Option<i64>,
    /// **Required when using multi-line comments unless using `in_reply_to`**. The `start_side` is the starting side of the diff that the comment applies to. Can be `LEFT` or `RIGHT`. To learn more about multi-line comments, see "[Commenting on a pull request](https://docs.github.com/articles/commenting-on-a-pull-request#adding-line-comments-to-a-pull-request)" in the GitHub Help documentation. See `side` in this table for additional context.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub start_side: Option<RequestStartSide>,
    /// The level at which the comment is targeted.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub subject_type: Option<RequestSubjectType>,
  }
}

pub mod create_reply_for_review_comment {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = PullRequestReviewComment;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The text of the review comment.
    pub body: String,
  }
}

pub mod list_commits {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Commit>;

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

pub mod list_files {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<DiffEntry>;

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

pub mod check_if_merged {
  #[allow(unused_imports)]
  use super::*;
}

pub mod merge {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = PullRequestMergeResult;

  /// The merge method to use.
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum RequestMergeMethod {
    #[serde(rename = "merge")]
    Merge,
    #[serde(rename = "squash")]
    Squash,
    #[serde(rename = "rebase")]
    Rebase,
  }

  impl ToString for RequestMergeMethod {
    fn to_string(&self) -> String {
      match self {
        RequestMergeMethod::Merge => "merge".to_string(),
        RequestMergeMethod::Squash => "squash".to_string(),
        RequestMergeMethod::Rebase => "rebase".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// Extra detail to append to automatic commit message.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub commit_message: Option<String>,
    /// Title for the automatic commit message.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub commit_title: Option<String>,
    /// The merge method to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub merge_method: Option<RequestMergeMethod>,
    /// SHA that pull request head must match to allow merge.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub sha: Option<String>,
  }
}

pub mod list_requested_reviewers {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = PullRequestReviewRequest;
}

pub mod request_reviewers {
  #[allow(unused_imports)]
  use super::*;

  pub type Request = serde_json::Value;
  pub type Response = PullRequestSimple;
}

pub mod remove_requested_reviewers {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = PullRequestSimple;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// An array of user `login`s that will be removed.
    pub reviewers: Vec<String>,
    /// An array of team `slug`s that will be removed.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub team_reviewers: Option<Vec<String>>,
  }
}

pub mod list_reviews {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<PullRequestReview>;

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

pub mod create_review {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = PullRequestReview;

  /// The review action you want to perform. The review actions include: `APPROVE`, `REQUEST_CHANGES`, or `COMMENT`. By leaving this blank, you set the review action state to `PENDING`, which means you will need to [submit the pull request review](https://docs.github.com/rest/pulls/reviews#submit-a-review-for-a-pull-request) when you are ready.
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum RequestEvent {
    #[serde(rename = "APPROVE")]
    Approve,
    #[serde(rename = "REQUEST_CHANGES")]
    RequestChanges,
    #[serde(rename = "COMMENT")]
    Comment,
  }

  impl ToString for RequestEvent {
    fn to_string(&self) -> String {
      match self {
        RequestEvent::Approve => "APPROVE".to_string(),
        RequestEvent::RequestChanges => "REQUEST_CHANGES".to_string(),
        RequestEvent::Comment => "COMMENT".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct RequestComments {
    /// Text of the review comment.
    pub body: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub line: Option<i64>,
    /// The relative path to the file that necessitates a review comment.
    pub path: String,
    /// The position in the diff where you want to add a review comment. Note this value is not the same as the line number in the file. The `position` value equals the number of lines down from the first "@@" hunk header in the file you want to add a comment. The line just below the "@@" line is position 1, the next line is position 2, and so on. The position in the diff continues to increase through lines of whitespace and additional hunks until the beginning of a new file.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub position: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub side: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub start_line: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub start_side: Option<String>,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// **Required** when using `REQUEST_CHANGES` or `COMMENT` for the `event` parameter. The body text of the pull request review.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub body: Option<String>,
    /// Use the following table to specify the location, destination, and contents of the draft review comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub comments: Option<Vec<RequestComments>>,
    /// The SHA of the commit that needs a review. Not using the latest commit SHA may render your review comment outdated if a subsequent commit modifies the line you specify as the `position`. Defaults to the most recent commit in the pull request when you do not specify a value.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub commit_id: Option<String>,
    /// The review action you want to perform. The review actions include: `APPROVE`, `REQUEST_CHANGES`, or `COMMENT`. By leaving this blank, you set the review action state to `PENDING`, which means you will need to [submit the pull request review](https://docs.github.com/rest/pulls/reviews#submit-a-review-for-a-pull-request) when you are ready.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub event: Option<RequestEvent>,
  }
}

pub mod get_review {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = PullRequestReview;
}

pub mod update_review {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = PullRequestReview;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The body text of the pull request review.
    pub body: String,
  }
}

pub mod delete_pending_review {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = PullRequestReview;
}

pub mod list_comments_for_review {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<ReviewComment>;

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

pub mod dismiss_review {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = PullRequestReview;

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum RequestEvent {
    #[serde(rename = "DISMISS")]
    Dismiss,
  }

  impl ToString for RequestEvent {
    fn to_string(&self) -> String {
      match self {
        RequestEvent::Dismiss => "DISMISS".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub event: Option<RequestEvent>,
    /// The message for the pull request review dismissal
    pub message: String,
  }
}

pub mod submit_review {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = PullRequestReview;

  /// The review action you want to perform. The review actions include: `APPROVE`, `REQUEST_CHANGES`, or `COMMENT`. When you leave this blank, the API returns _HTTP 422 (Unrecognizable entity)_ and sets the review action state to `PENDING`, which means you will need to re-submit the pull request review using a review action.
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum RequestEvent {
    #[serde(rename = "APPROVE")]
    Approve,
    #[serde(rename = "REQUEST_CHANGES")]
    RequestChanges,
    #[serde(rename = "COMMENT")]
    Comment,
  }

  impl ToString for RequestEvent {
    fn to_string(&self) -> String {
      match self {
        RequestEvent::Approve => "APPROVE".to_string(),
        RequestEvent::RequestChanges => "REQUEST_CHANGES".to_string(),
        RequestEvent::Comment => "COMMENT".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The body text of the pull request review
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub body: Option<String>,
    /// The review action you want to perform. The review actions include: `APPROVE`, `REQUEST_CHANGES`, or `COMMENT`. When you leave this blank, the API returns _HTTP 422 (Unrecognizable entity)_ and sets the review action state to `PENDING`, which means you will need to re-submit the pull request review using a review action.
    pub event: RequestEvent,
  }
}

pub mod update_branch {
  #[allow(unused_imports)]
  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The expected SHA of the pull request's HEAD ref. This is the most recent commit on the pull request's branch. If the expected SHA does not match the pull request's HEAD, you will receive a `422 Unprocessable Entity` status. You can use the "[List commits](https://docs.github.com/rest/commits/commits#list-commits)" endpoint to find the most recent commit SHA. Default: SHA of the pull request's current HEAD ref.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub expected_head_sha: Option<String>,
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
  ) -> Request<(), list::Query, list::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/pulls");

    Request::<(), list::Query, list::Response>::builder(&self.config)
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
  ) -> Request<create::Request, (), create::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/pulls");

    Request::<create::Request, (), create::Response>::builder(&self.config)
      .post(url)
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
  ) -> Request<(), list_review_comments_for_repo::Query, list_review_comments_for_repo::Response>
  {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/pulls/comments");

    Request::<(), list_review_comments_for_repo::Query, list_review_comments_for_repo::Response>::builder(&self.config)
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
  ) -> Request<(), (), get_review_comment::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let comment_id = comment_id.into();
    let url = format!("/repos/{owner}/{repo}/pulls/comments/{comment_id}");

    Request::<(), (), get_review_comment::Response>::builder(&self.config)
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
  ) -> Request<update_review_comment::Request, (), update_review_comment::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let comment_id = comment_id.into();
    let url = format!("/repos/{owner}/{repo}/pulls/comments/{comment_id}");

    Request::<update_review_comment::Request, (), update_review_comment::Response>::builder(
      &self.config,
    )
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
  ) -> Request<(), (), get::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let pull_number = pull_number.into();
    let url = format!("/repos/{owner}/{repo}/pulls/{pull_number}");

    Request::<(), (), get::Response>::builder(&self.config)
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
  ) -> Request<update::Request, (), update::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let pull_number = pull_number.into();
    let url = format!("/repos/{owner}/{repo}/pulls/{pull_number}");

    Request::<update::Request, (), update::Response>::builder(&self.config)
      .patch(url)
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
  ) -> Request<(), list_review_comments::Query, list_review_comments::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let pull_number = pull_number.into();
    let url = format!("/repos/{owner}/{repo}/pulls/{pull_number}/comments");

    Request::<(), list_review_comments::Query, list_review_comments::Response>::builder(
      &self.config,
    )
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
  ) -> Request<create_review_comment::Request, (), create_review_comment::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let pull_number = pull_number.into();
    let url = format!("/repos/{owner}/{repo}/pulls/{pull_number}/comments");

    Request::<create_review_comment::Request, (), create_review_comment::Response>::builder(
      &self.config,
    )
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
  ) -> Request<
    create_reply_for_review_comment::Request,
    (),
    create_reply_for_review_comment::Response,
  > {
    let owner = owner.into();
    let repo = repo.into();
    let pull_number = pull_number.into();
    let comment_id = comment_id.into();
    let url = format!("/repos/{owner}/{repo}/pulls/{pull_number}/comments/{comment_id}/replies");

    Request::<create_reply_for_review_comment::Request, (), create_reply_for_review_comment::Response>::builder(&self.config)
      .post(url)
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
  ) -> Request<(), list_commits::Query, list_commits::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let pull_number = pull_number.into();
    let url = format!("/repos/{owner}/{repo}/pulls/{pull_number}/commits");

    Request::<(), list_commits::Query, list_commits::Response>::builder(&self.config)
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
  ) -> Request<(), list_files::Query, list_files::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let pull_number = pull_number.into();
    let url = format!("/repos/{owner}/{repo}/pulls/{pull_number}/files");

    Request::<(), list_files::Query, list_files::Response>::builder(&self.config)
      .get(url)
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
  ) -> Request<merge::Request, (), merge::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let pull_number = pull_number.into();
    let url = format!("/repos/{owner}/{repo}/pulls/{pull_number}/merge");

    Request::<merge::Request, (), merge::Response>::builder(&self.config)
      .put(url)
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
  ) -> Request<(), (), list_requested_reviewers::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let pull_number = pull_number.into();
    let url = format!("/repos/{owner}/{repo}/pulls/{pull_number}/requested_reviewers");

    Request::<(), (), list_requested_reviewers::Response>::builder(&self.config)
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
  ) -> Request<request_reviewers::Request, (), request_reviewers::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let pull_number = pull_number.into();
    let url = format!("/repos/{owner}/{repo}/pulls/{pull_number}/requested_reviewers");

    Request::<request_reviewers::Request, (), request_reviewers::Response>::builder(&self.config)
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
  ) -> Request<remove_requested_reviewers::Request, (), remove_requested_reviewers::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let pull_number = pull_number.into();
    let url = format!("/repos/{owner}/{repo}/pulls/{pull_number}/requested_reviewers");

    Request::<remove_requested_reviewers::Request, (), remove_requested_reviewers::Response>::builder(&self.config)
      .delete(url)
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
  ) -> Request<(), list_reviews::Query, list_reviews::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let pull_number = pull_number.into();
    let url = format!("/repos/{owner}/{repo}/pulls/{pull_number}/reviews");

    Request::<(), list_reviews::Query, list_reviews::Response>::builder(&self.config)
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
  ) -> Request<create_review::Request, (), create_review::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let pull_number = pull_number.into();
    let url = format!("/repos/{owner}/{repo}/pulls/{pull_number}/reviews");

    Request::<create_review::Request, (), create_review::Response>::builder(&self.config)
      .post(url)
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
  ) -> Request<(), (), get_review::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let pull_number = pull_number.into();
    let review_id = review_id.into();
    let url = format!("/repos/{owner}/{repo}/pulls/{pull_number}/reviews/{review_id}");

    Request::<(), (), get_review::Response>::builder(&self.config)
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
  ) -> Request<update_review::Request, (), update_review::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let pull_number = pull_number.into();
    let review_id = review_id.into();
    let url = format!("/repos/{owner}/{repo}/pulls/{pull_number}/reviews/{review_id}");

    Request::<update_review::Request, (), update_review::Response>::builder(&self.config)
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
  ) -> Request<(), (), delete_pending_review::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let pull_number = pull_number.into();
    let review_id = review_id.into();
    let url = format!("/repos/{owner}/{repo}/pulls/{pull_number}/reviews/{review_id}");

    Request::<(), (), delete_pending_review::Response>::builder(&self.config)
      .delete(url)
      .build()
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
  ) -> Request<(), list_comments_for_review::Query, list_comments_for_review::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let pull_number = pull_number.into();
    let review_id = review_id.into();
    let url = format!("/repos/{owner}/{repo}/pulls/{pull_number}/reviews/{review_id}/comments");

    Request::<(), list_comments_for_review::Query, list_comments_for_review::Response>::builder(
      &self.config,
    )
    .get(url)
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
  ) -> Request<dismiss_review::Request, (), dismiss_review::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let pull_number = pull_number.into();
    let review_id = review_id.into();
    let url = format!("/repos/{owner}/{repo}/pulls/{pull_number}/reviews/{review_id}/dismissals");

    Request::<dismiss_review::Request, (), dismiss_review::Response>::builder(&self.config)
      .put(url)
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
  ) -> Request<submit_review::Request, (), submit_review::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let pull_number = pull_number.into();
    let review_id = review_id.into();
    let url = format!("/repos/{owner}/{repo}/pulls/{pull_number}/reviews/{review_id}/events");

    Request::<submit_review::Request, (), submit_review::Response>::builder(&self.config)
      .post(url)
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
  ) -> Request<update_branch::Request, (), update_branch::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let pull_number = pull_number.into();
    let url = format!("/repos/{owner}/{repo}/pulls/{pull_number}/update-branch");

    Request::<update_branch::Request, (), update_branch::Response>::builder(&self.config)
      .put(url)
      .build()
  }
}
