use octocrate_core::*;
#[allow(unused_imports)]
use octocrate_types::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use typed_builder::TypedBuilder;

pub mod code {
  #[allow(unused_imports)]
  use super::*;

  #[allow(clippy::large_enum_variant)]
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QuerySort {
    #[serde(rename = "indexed")]
    Indexed,
  }

  impl std::fmt::Display for QuerySort {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match self {
        QuerySort::Indexed => write!(f, "indexed"),
      }
    }
  }

  #[allow(clippy::large_enum_variant)]
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QueryOrder {
    #[serde(rename = "desc")]
    Desc,
    #[serde(rename = "asc")]
    Asc,
  }

  impl std::fmt::Display for QueryOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match self {
        QueryOrder::Desc => write!(f, "desc"),
        QueryOrder::Asc => write!(f, "asc"),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The query contains one or more search keywords and qualifiers. Qualifiers allow you to limit your search to specific areas of GitHub. The REST API supports the same qualifiers as the web interface for GitHub. To learn more about the format of the query, see [Constructing a search query](https://docs.github.com/rest/search/search#constructing-a-search-query). See "[Searching code](https://docs.github.com/search-github/searching-on-github/searching-code)" for a detailed list of qualifiers.
    pub q: String,
    /// **This field is deprecated.** Sorts the results of your query. Can only be `indexed`, which indicates how recently a file has been indexed by the GitHub search infrastructure. Default: [best match](https://docs.github.com/rest/search/search#ranking-search-results)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub sort: Option<QuerySort>,
    /// **This field is deprecated.** Determines whether the first search result returned is the highest number of matches (`desc`) or lowest number of matches (`asc`). This parameter is ignored unless you provide `sort`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub order: Option<QueryOrder>,
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
    pub incomplete_results: bool,
    pub items: Vec<CodeSearchResultItem>,
    pub total_count: i64,
  }
}

pub mod commits {
  #[allow(unused_imports)]
  use super::*;

  #[allow(clippy::large_enum_variant)]
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QuerySort {
    #[serde(rename = "author-date")]
    AuthorDate,
    #[serde(rename = "committer-date")]
    CommitterDate,
  }

  impl std::fmt::Display for QuerySort {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match self {
        QuerySort::AuthorDate => write!(f, "author-date"),
        QuerySort::CommitterDate => write!(f, "committer-date"),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The query contains one or more search keywords and qualifiers. Qualifiers allow you to limit your search to specific areas of GitHub. The REST API supports the same qualifiers as the web interface for GitHub. To learn more about the format of the query, see [Constructing a search query](https://docs.github.com/rest/search/search#constructing-a-search-query). See "[Searching commits](https://docs.github.com/search-github/searching-on-github/searching-commits)" for a detailed list of qualifiers.
    pub q: String,
    /// Sorts the results of your query by `author-date` or `committer-date`. Default: [best match](https://docs.github.com/rest/search/search#ranking-search-results)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub sort: Option<QuerySort>,
    /// Determines whether the first search result returned is the highest number of matches (`desc`) or lowest number of matches (`asc`). This parameter is ignored unless you provide `sort`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub order: Option<parameters::Order>,
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
    pub incomplete_results: bool,
    pub items: Vec<CommitSearchResultItem>,
    pub total_count: i64,
  }
}

pub mod issues_and_pull_requests {
  #[allow(unused_imports)]
  use super::*;

  #[allow(clippy::large_enum_variant)]
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QuerySort {
    #[serde(rename = "comments")]
    Comments,
    #[serde(rename = "reactions")]
    Reactions,
    #[serde(rename = "reactions-+1")]
    ReactionsMinusPlusOne,
    #[serde(rename = "reactions--1")]
    ReactionsMinusMinusOne,
    #[serde(rename = "reactions-smile")]
    ReactionsSmile,
    #[serde(rename = "reactions-thinking_face")]
    ReactionsThinkingFace,
    #[serde(rename = "reactions-heart")]
    ReactionsHeart,
    #[serde(rename = "reactions-tada")]
    ReactionsTada,
    #[serde(rename = "interactions")]
    Interactions,
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "updated")]
    Updated,
  }

  impl std::fmt::Display for QuerySort {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match self {
        QuerySort::Comments => write!(f, "comments"),
        QuerySort::Reactions => write!(f, "reactions"),
        QuerySort::ReactionsMinusPlusOne => write!(f, "reactions-+1"),
        QuerySort::ReactionsMinusMinusOne => write!(f, "reactions--1"),
        QuerySort::ReactionsSmile => write!(f, "reactions-smile"),
        QuerySort::ReactionsThinkingFace => write!(f, "reactions-thinking_face"),
        QuerySort::ReactionsHeart => write!(f, "reactions-heart"),
        QuerySort::ReactionsTada => write!(f, "reactions-tada"),
        QuerySort::Interactions => write!(f, "interactions"),
        QuerySort::Created => write!(f, "created"),
        QuerySort::Updated => write!(f, "updated"),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The query contains one or more search keywords and qualifiers. Qualifiers allow you to limit your search to specific areas of GitHub. The REST API supports the same qualifiers as the web interface for GitHub. To learn more about the format of the query, see [Constructing a search query](https://docs.github.com/rest/search/search#constructing-a-search-query). See "[Searching issues and pull requests](https://docs.github.com/search-github/searching-on-github/searching-issues-and-pull-requests)" for a detailed list of qualifiers.
    pub q: String,
    /// Sorts the results of your query by the number of `comments`, `reactions`, `reactions-+1`, `reactions--1`, `reactions-smile`, `reactions-thinking_face`, `reactions-heart`, `reactions-tada`, or `interactions`. You can also sort results by how recently the items were `created` or `updated`, Default: [best match](https://docs.github.com/rest/search/search#ranking-search-results)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub sort: Option<QuerySort>,
    /// Determines whether the first search result returned is the highest number of matches (`desc`) or lowest number of matches (`asc`). This parameter is ignored unless you provide `sort`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub order: Option<parameters::Order>,
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
    pub incomplete_results: bool,
    pub items: Vec<IssueSearchResultItem>,
    pub total_count: i64,
  }
}

pub mod labels {
  #[allow(unused_imports)]
  use super::*;

  #[allow(clippy::large_enum_variant)]
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QuerySort {
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "updated")]
    Updated,
  }

  impl std::fmt::Display for QuerySort {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match self {
        QuerySort::Created => write!(f, "created"),
        QuerySort::Updated => write!(f, "updated"),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The id of the repository.
    pub repository_id: i64,
    /// The search keywords. This endpoint does not accept qualifiers in the query. To learn more about the format of the query, see [Constructing a search query](https://docs.github.com/rest/search/search#constructing-a-search-query).
    pub q: String,
    /// Sorts the results of your query by when the label was `created` or `updated`. Default: [best match](https://docs.github.com/rest/search/search#ranking-search-results)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub sort: Option<QuerySort>,
    /// Determines whether the first search result returned is the highest number of matches (`desc`) or lowest number of matches (`asc`). This parameter is ignored unless you provide `sort`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub order: Option<parameters::Order>,
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
    pub incomplete_results: bool,
    pub items: Vec<LabelSearchResultItem>,
    pub total_count: i64,
  }
}

pub mod repos {
  #[allow(unused_imports)]
  use super::*;

  #[allow(clippy::large_enum_variant)]
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QuerySort {
    #[serde(rename = "stars")]
    Stars,
    #[serde(rename = "forks")]
    Forks,
    #[serde(rename = "help-wanted-issues")]
    HelpWantedIssues,
    #[serde(rename = "updated")]
    Updated,
  }

  impl std::fmt::Display for QuerySort {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match self {
        QuerySort::Stars => write!(f, "stars"),
        QuerySort::Forks => write!(f, "forks"),
        QuerySort::HelpWantedIssues => write!(f, "help-wanted-issues"),
        QuerySort::Updated => write!(f, "updated"),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The query contains one or more search keywords and qualifiers. Qualifiers allow you to limit your search to specific areas of GitHub. The REST API supports the same qualifiers as the web interface for GitHub. To learn more about the format of the query, see [Constructing a search query](https://docs.github.com/rest/search/search#constructing-a-search-query). See "[Searching for repositories](https://docs.github.com/articles/searching-for-repositories/)" for a detailed list of qualifiers.
    pub q: String,
    /// Sorts the results of your query by number of `stars`, `forks`, or `help-wanted-issues` or how recently the items were `updated`. Default: [best match](https://docs.github.com/rest/search/search#ranking-search-results)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub sort: Option<QuerySort>,
    /// Determines whether the first search result returned is the highest number of matches (`desc`) or lowest number of matches (`asc`). This parameter is ignored unless you provide `sort`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub order: Option<parameters::Order>,
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
    pub incomplete_results: bool,
    pub items: Vec<RepoSearchResultItem>,
    pub total_count: i64,
  }
}

pub mod topics {
  #[allow(unused_imports)]
  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The query contains one or more search keywords and qualifiers. Qualifiers allow you to limit your search to specific areas of GitHub. The REST API supports the same qualifiers as the web interface for GitHub. To learn more about the format of the query, see [Constructing a search query](https://docs.github.com/rest/search/search#constructing-a-search-query).
    pub q: String,
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
    pub incomplete_results: bool,
    pub items: Vec<TopicSearchResultItem>,
    pub total_count: i64,
  }
}

pub mod users {
  #[allow(unused_imports)]
  use super::*;

  #[allow(clippy::large_enum_variant)]
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QuerySort {
    #[serde(rename = "followers")]
    Followers,
    #[serde(rename = "repositories")]
    Repositories,
    #[serde(rename = "joined")]
    Joined,
  }

  impl std::fmt::Display for QuerySort {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match self {
        QuerySort::Followers => write!(f, "followers"),
        QuerySort::Repositories => write!(f, "repositories"),
        QuerySort::Joined => write!(f, "joined"),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The query contains one or more search keywords and qualifiers. Qualifiers allow you to limit your search to specific areas of GitHub. The REST API supports the same qualifiers as the web interface for GitHub. To learn more about the format of the query, see [Constructing a search query](https://docs.github.com/rest/search/search#constructing-a-search-query). See "[Searching users](https://docs.github.com/search-github/searching-on-github/searching-users)" for a detailed list of qualifiers.
    pub q: String,
    /// Sorts the results of your query by number of `followers` or `repositories`, or when the person `joined` GitHub. Default: [best match](https://docs.github.com/rest/search/search#ranking-search-results)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub sort: Option<QuerySort>,
    /// Determines whether the first search result returned is the highest number of matches (`desc`) or lowest number of matches (`asc`). This parameter is ignored unless you provide `sort`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub order: Option<parameters::Order>,
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
    pub incomplete_results: bool,
    pub items: Vec<UserSearchResultItem>,
    pub total_count: i64,
  }
}

/// Look for stuff on GitHub.
pub struct GitHubSearchAPI {
  config: SharedAPIConfig,
}

impl GitHubSearchAPI {
  pub fn new(config: &SharedAPIConfig) -> Self {
    Self {
      config: config.clone(),
    }
  }

  /// **Search code**
  ///
  /// Searches for query terms inside of a file. This method returns up to 100 results [per page](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api).
  ///
  /// When searching for code, you can get text match metadata for the file **content** and file **path** fields when you pass the `text-match` media type. For more details about how to receive highlighted search results, see [Text match metadata](https://docs.github.com/rest/search/search#text-match-metadata).
  ///
  /// For example, if you want to find the definition of the `addClass` function inside [jQuery](https://github.com/jquery/jquery) repository, your query would look something like this:
  ///
  /// `q=addClass+in:file+language:js+repo:jquery/jquery`
  ///
  /// This query searches for the keyword `addClass` within a file's contents. The query limits the search to files where the language is JavaScript in the `jquery/jquery` repository.
  ///
  /// Considerations for code search:
  ///
  /// Due to the complexity of searching code, there are a few restrictions on how searches are performed:
  ///
  /// *   Only the _default branch_ is considered. In most cases, this will be the `master` branch.
  /// *   Only files smaller than 384 KB are searchable.
  /// *   You must always include at least one search term when searching source code. For example, searching for [`language:go`](https://github.com/search?utf8=%E2%9C%93&q=language%3Ago&type=Code) is not valid, while [`amazing
  /// language:go`](https://github.com/search?utf8=%E2%9C%93&q=amazing+language%3Ago&type=Code) is.
  ///
  /// This endpoint requires you to authenticate and limits you to 10 requests per minute.
  ///
  /// *Documentation*: [https://docs.github.com/rest/search/search#search-code](https://docs.github.com/rest/search/search#search-code)
  pub fn code(&self) -> Request<(), code::Query, code::Response> {
    let url = format!("/search/code");

    Request::<(), code::Query, code::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Search commits**
  ///
  /// Find commits via various criteria on the default branch (usually `main`). This method returns up to 100 results [per page](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api).
  ///
  /// When searching for commits, you can get text match metadata for the **message** field when you provide the `text-match` media type. For more details about how to receive highlighted search results, see [Text match
  /// metadata](https://docs.github.com/rest/search/search#text-match-metadata).
  ///
  /// For example, if you want to find commits related to CSS in the [octocat/Spoon-Knife](https://github.com/octocat/Spoon-Knife) repository. Your query would look something like this:
  ///
  /// `q=repo:octocat/Spoon-Knife+css`
  ///
  /// *Documentation*: [https://docs.github.com/rest/search/search#search-commits](https://docs.github.com/rest/search/search#search-commits)
  pub fn commits(&self) -> Request<(), commits::Query, commits::Response> {
    let url = format!("/search/commits");

    Request::<(), commits::Query, commits::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Search issues and pull requests**
  ///
  /// Find issues by state and keyword. This method returns up to 100 results [per page](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api).
  ///
  /// When searching for issues, you can get text match metadata for the issue **title**, issue **body**, and issue **comment body** fields when you pass the `text-match` media type. For more details about how to receive highlighted
  /// search results, see [Text match metadata](https://docs.github.com/rest/search/search#text-match-metadata).
  ///
  /// For example, if you want to find the oldest unresolved Python bugs on Windows. Your query might look something like this.
  ///
  /// `q=windows+label:bug+language:python+state:open&sort=created&order=asc`
  ///
  /// This query searches for the keyword `windows`, within any open issue that is labeled as `bug`. The search runs across repositories whose primary language is Python. The results are sorted by creation date in ascending order, which means the oldest issues appear first in the search results.
  ///
  /// **Note:** For requests made by GitHub Apps with a user access token, you can't retrieve a combination of issues and pull requests in a single query. Requests that don't include the `is:issue` or `is:pull-request` qualifier will receive an HTTP `422 Unprocessable Entity` response. To get results for both issues and pull requests, you must send separate queries for issues and pull requests. For more information about the `is` qualifier, see "[Searching only issues or pull requests](https://docs.github.com/github/searching-for-information-on-github/searching-issues-and-pull-requests#search-only-issues-or-pull-requests)."
  ///
  /// *Documentation*: [https://docs.github.com/rest/search/search#search-issues-and-pull-requests](https://docs.github.com/rest/search/search#search-issues-and-pull-requests)
  pub fn issues_and_pull_requests(
    &self,
  ) -> Request<(), issues_and_pull_requests::Query, issues_and_pull_requests::Response> {
    let url = format!("/search/issues");

    Request::<(), issues_and_pull_requests::Query, issues_and_pull_requests::Response>::builder(
      &self.config,
    )
    .get(url)
    .build()
  }

  /// **Search labels**
  ///
  /// Find labels in a repository with names or descriptions that match search keywords. Returns up to 100 results [per page](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api).
  ///
  /// When searching for labels, you can get text match metadata for the label **name** and **description** fields when you pass the `text-match` media type. For more details about how to receive highlighted search results, see [Text match metadata](https://docs.github.com/rest/search/search#text-match-metadata).
  ///
  /// For example, if you want to find labels in the `linguist` repository that match `bug`, `defect`, or `enhancement`. Your query might look like this:
  ///
  /// `q=bug+defect+enhancement&repository_id=64778136`
  ///
  /// The labels that best match the query appear first in the search results.
  ///
  /// *Documentation*: [https://docs.github.com/rest/search/search#search-labels](https://docs.github.com/rest/search/search#search-labels)
  pub fn labels(&self) -> Request<(), labels::Query, labels::Response> {
    let url = format!("/search/labels");

    Request::<(), labels::Query, labels::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Search repositories**
  ///
  /// Find repositories via various criteria. This method returns up to 100 results [per page](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api).
  ///
  /// When searching for repositories, you can get text match metadata for the **name** and **description** fields when you pass the `text-match` media type. For more details about how to receive highlighted search results, see [Text match metadata](https://docs.github.com/rest/search/search#text-match-metadata).
  ///
  /// For example, if you want to search for popular Tetris repositories written in assembly code, your query might look like this:
  ///
  /// `q=tetris+language:assembly&sort=stars&order=desc`
  ///
  /// This query searches for repositories with the word `tetris` in the name, the description, or the README. The results are limited to repositories where the primary language is assembly. The results are sorted by stars in descending order, so that the most popular repositories appear first in the search results.
  ///
  /// *Documentation*: [https://docs.github.com/rest/search/search#search-repositories](https://docs.github.com/rest/search/search#search-repositories)
  pub fn repos(&self) -> Request<(), repos::Query, repos::Response> {
    let url = format!("/search/repositories");

    Request::<(), repos::Query, repos::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Search topics**
  ///
  /// Find topics via various criteria. Results are sorted by best match. This method returns up to 100 results [per page](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api). See "[Searching topics](https://docs.github.com/articles/searching-topics/)" for a detailed list of qualifiers.
  ///
  /// When searching for topics, you can get text match metadata for the topic's **short\_description**, **description**, **name**, or **display\_name** field when you pass the `text-match` media type. For more details about how to receive highlighted search results, see [Text match metadata](https://docs.github.com/rest/search/search#text-match-metadata).
  ///
  /// For example, if you want to search for topics related to Ruby that are featured on https://github.com/topics. Your query might look like this:
  ///
  /// `q=ruby+is:featured`
  ///
  /// This query searches for topics with the keyword `ruby` and limits the results to find only topics that are featured. The topics that are the best match for the query appear first in the search results.
  ///
  /// *Documentation*: [https://docs.github.com/rest/search/search#search-topics](https://docs.github.com/rest/search/search#search-topics)
  pub fn topics(&self) -> Request<(), topics::Query, topics::Response> {
    let url = format!("/search/topics");

    Request::<(), topics::Query, topics::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Search users**
  ///
  /// Find users via various criteria. This method returns up to 100 results [per page](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api).
  ///
  /// When searching for users, you can get text match metadata for the issue **login**, public **email**, and **name** fields when you pass the `text-match` media type. For more details about highlighting search results, see [Text match metadata](https://docs.github.com/rest/search/search#text-match-metadata). For more details about how to receive highlighted search results, see [Text match metadata](https://docs.github.com/rest/search/search#text-match-metadata).
  ///
  /// For example, if you're looking for a list of popular users, you might try this query:
  ///
  /// `q=tom+repos:%3E42+followers:%3E1000`
  ///
  /// This query searches for users with the name `tom`. The results are restricted to users with more than 42 repositories and over 1,000 followers.
  ///
  /// This endpoint does not accept authentication and will only include publicly visible users. As an alternative, you can use the GraphQL API. The GraphQL API requires authentication and will return private users, including Enterprise Managed Users (EMUs), that you are authorized to view. For more information, see "[GraphQL Queries](https://docs.github.com/graphql/reference/queries#search)."
  ///
  /// *Documentation*: [https://docs.github.com/rest/search/search#search-users](https://docs.github.com/rest/search/search#search-users)
  pub fn users(&self) -> Request<(), users::Query, users::Response> {
    let url = format!("/search/users");

    Request::<(), users::Query, users::Response>::builder(&self.config)
      .get(url)
      .build()
  }
}
