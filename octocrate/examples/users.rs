use octocrate::{APIConfig, GitHubAPI, PersonalAccessToken, UsersGetByUsernameResponse};

#[tokio::test]
async fn test_get_user() {
  dotenv::dotenv().ok();

  let personal_access_token =
    std::env::var("GITHUB_PERSONAL_ACCESS_TOKEN").expect("GITHUB_PERSONAL_ACCESS_TOKEN is not set");

  let personal_access_token = PersonalAccessToken::new(personal_access_token);

  let config = APIConfig::with_token(personal_access_token).shared();

  let api = GitHubAPI::new(&config);

  let user = api
    .users
    .get_by_username("panghu-huang")
    .send()
    .await
    .unwrap();

  let user_name = match user {
    UsersGetByUsernameResponse::PublicUser(user) => user.login,
    UsersGetByUsernameResponse::PrivateUser(user) => user.login,
  };

  assert_eq!(user_name, "panghu-huang");
}
