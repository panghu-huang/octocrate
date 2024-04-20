use octocrate::{APIConfig, GitHubAPI, PersonalAccessToken, SearchIssuesAndPullRequestsQuery};

#[tokio::test]
async fn test_search_pull_requests() {
  dotenv::dotenv().ok();

  let personal_access_token =
    std::env::var("GITHUB_PERSONAL_ACCESS_TOKEN").expect("GITHUB_PERSONAL_ACCESS_TOKEN is not set");

  let personal_access_token = PersonalAccessToken::new(personal_access_token);

  // Use the personal access token to create issue comments
  let config = APIConfig::with_token(personal_access_token).shared();

  let api = GitHubAPI::new(&config);

  let prs = api
    .search
    .issues_and_pull_requests()
    .query(
      &SearchIssuesAndPullRequestsQuery::builder()
        .q("repo:facebook/react is:pr is:open")
        .build()
        .unwrap(),
    )
    .send()
    .await
    .unwrap();

  assert!(prs.total_count > 0);
}
