use octocrate_core::*;
#[allow(unused_imports)]
use octocrate_types::*;

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
  pub fn repos(&self) -> Request<(), SearchReposQuery, SearchReposResponse> {
    let url = format!("/search/repositories");

    Request::<(), SearchReposQuery, SearchReposResponse>::builder(&self.config)
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
  pub fn topics(&self) -> Request<(), SearchTopicsQuery, SearchTopicsResponse> {
    let url = format!("/search/topics");

    Request::<(), SearchTopicsQuery, SearchTopicsResponse>::builder(&self.config)
      .get(url)
      .build()
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
  pub fn code(&self) -> Request<(), SearchCodeQuery, SearchCodeResponse> {
    let url = format!("/search/code");

    Request::<(), SearchCodeQuery, SearchCodeResponse>::builder(&self.config)
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
  pub fn labels(&self) -> Request<(), SearchLabelsQuery, SearchLabelsResponse> {
    let url = format!("/search/labels");

    Request::<(), SearchLabelsQuery, SearchLabelsResponse>::builder(&self.config)
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
  ) -> Request<(), SearchIssuesAndPullRequestsQuery, SearchIssuesAndPullRequestsResponse> {
    let url = format!("/search/issues");

    Request::<(), SearchIssuesAndPullRequestsQuery, SearchIssuesAndPullRequestsResponse>::builder(
      &self.config,
    )
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
  pub fn commits(&self) -> Request<(), SearchCommitsQuery, SearchCommitsResponse> {
    let url = format!("/search/commits");

    Request::<(), SearchCommitsQuery, SearchCommitsResponse>::builder(&self.config)
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
  pub fn users(&self) -> Request<(), SearchUsersQuery, SearchUsersResponse> {
    let url = format!("/search/users");

    Request::<(), SearchUsersQuery, SearchUsersResponse>::builder(&self.config)
      .get(url)
      .build()
  }
}
