use octocrate::{
  APIConfig, Error, GitHubAPI, PersonalAccessToken, PullsListQuery, PullsListQueryState,
};

#[tokio::test]
async fn test_list_pull_request() {
  dotenv::dotenv().ok();

  let personal_access_token =
    std::env::var("GITHUB_PERSONAL_ACCESS_TOKEN").expect("GITHUB_PERSONAL_ACCESS_TOKEN is not set");

  let personal_access_token = PersonalAccessToken::new(personal_access_token);

  let config = APIConfig::with_token(personal_access_token).shared();

  let api = GitHubAPI::new(&config);

  let pull_requests = api
    .pulls
    .list("facebook", "react")
    .query(
      &PullsListQuery::builder()
        .state(PullsListQueryState::Open)
        .per_page(10)
        .build(),
    )
    .send()
    .await
    .unwrap();

  assert_eq!(pull_requests.len(), 10);
}

#[tokio::test]
async fn test_get_pull_request() {
  dotenv::dotenv().ok();

  let personal_access_token =
    std::env::var("GITHUB_PERSONAL_ACCESS_TOKEN").expect("GITHUB_PERSONAL_ACCESS_TOKEN is not set");

  let personal_access_token = PersonalAccessToken::new(personal_access_token);

  let config = APIConfig::with_token(personal_access_token).shared();

  let api = GitHubAPI::new(&config);

  let pull_request = api
    .pulls
    .get("panghu-huang", "octocrate", 1)
    .send()
    .await
    .unwrap();

  assert_eq!(pull_request.number, 1);
}

#[tokio::test]
async fn test_list_pull_request_changed_files() {
  dotenv::dotenv().ok();

  let personal_access_token =
    std::env::var("GITHUB_PERSONAL_ACCESS_TOKEN").expect("GITHUB_PERSONAL_ACCESS_TOKEN is not set");

  let personal_access_token = PersonalAccessToken::new(personal_access_token);

  let config = APIConfig::with_token(personal_access_token).shared();

  let api = GitHubAPI::new(&config);

  let files = api
    .pulls
    .list_files("panghu-huang", "octocrate", 1)
    .send()
    .await
    .unwrap();

  // https://github.com/panghu-huang/octocrate/pull/1/files
  assert_eq!(files.len(), 1);
}

#[tokio::test]
async fn test_check_if_pull_request_has_been_merged() {
  dotenv::dotenv().ok();

  let personal_access_token =
    std::env::var("GITHUB_PERSONAL_ACCESS_TOKEN").expect("GITHUB_PERSONAL_ACCESS_TOKEN is not set");

  let personal_access_token = PersonalAccessToken::new(personal_access_token);

  let config = APIConfig::with_token(personal_access_token).shared();

  let api = GitHubAPI::new(&config);

  // 404 not merged
  let res = api
    .pulls
    // https://github.com/panghu-huang/octocrate/pull/1
    .check_if_merged("panghu-huang", "octocrate", 1)
    .send()
    .await;

  let err = res.unwrap_err();

  if let Error::RequestFailed(error) = err {
    assert_eq!(error.message, "Not Found");
  } else {
    panic!("Expected RequestFailed");
  }

  // 204 merged
  let res = api
    .pulls
    // https://github.com/panghu-huang/octocrate/pull/2
    .check_if_merged("panghu-huang", "octocrate", 2)
    .send()
    .await;

  assert!(res.is_ok());
}
