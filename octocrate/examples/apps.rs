use octocrate::{APIConfig, AppAuthorization, GitHubAPI};

#[tokio::test]
async fn test_app_installation() {
  dotenv::dotenv().ok();

  let app_id = std::env::var("GITHUB_APP_ID").expect("GITHUB_APP_ID is not set");
  let private_key_path =
    std::env::var("GITHUB_APP_PRIVATE_KEY_PATH").expect("GITHUB_APP_PRIVATE_KEY_PATH is not set");

  let private_key = std::fs::read_to_string(&private_key_path).unwrap();

  let app_authorization = AppAuthorization::new(app_id, private_key);

  let config = APIConfig::with_token(app_authorization).shared();

  let api = GitHubAPI::new(&config);

  let installation = api
    .apps
    .get_repo_installation("panghu-huang", "octocrate")
    .send()
    .await
    .unwrap();

  let installation_token = api
    .apps
    .create_installation_access_token(installation.id)
    .send()
    .await
    .unwrap();

  let config = APIConfig::with_token(installation_token).shared();

  let api = GitHubAPI::new(&config);

  let repository = api
    .repos
    .get("panghu-huang", "octocrate")
    .send()
    .await
    .unwrap();

  assert_eq!(repository.owner.login, "panghu-huang");
  assert_eq!(repository.name, "octocrate");
  assert_eq!(repository.full_name, "panghu-huang/octocrate");
}
