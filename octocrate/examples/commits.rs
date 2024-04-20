use octocrate::{APIConfig, GitHubAPI, PersonalAccessToken};

#[tokio::test]
async fn get_commit() {
  dotenv::dotenv().ok();

  let personal_access_token =
    std::env::var("GITHUB_PERSONAL_ACCESS_TOKEN").expect("GITHUB_PERSONAL_ACCESS_TOKEN is not set");

  let personal_access_token = PersonalAccessToken::new(personal_access_token);

  // Use the personal access token to create issue comments
  let config = APIConfig::with_token(personal_access_token).shared();

  let api = GitHubAPI::new(&config);

  let commit = api
    .repos
    // https://github.com/panghu-huang/astro-run/commit/18ff8ed1a3437649e7d87bec9a7d4fe5562f6ad3
    .get_commit(
      "panghu-huang",
      "astro-run",
      "18ff8ed1a3437649e7d87bec9a7d4fe5562f6ad3",
    )
    .send()
    .await
    .unwrap();

  assert_eq!(commit.sha, "18ff8ed1a3437649e7d87bec9a7d4fe5562f6ad3");
}
