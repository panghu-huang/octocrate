use octocrate::{repos, APIConfig, GitHubAPI, PersonalAccessToken};

#[tokio::test]
async fn test_list_user_repositories() {
  dotenv::dotenv().ok();

  let personal_access_token =
    std::env::var("GITHUB_PERSONAL_ACCESS_TOKEN").expect("GITHUB_PERSONAL_ACCESS_TOKEN is not set");

  let personal_access_token = PersonalAccessToken::new(personal_access_token);

  let config = APIConfig::with_token(personal_access_token).shared();

  let api = GitHubAPI::new(&config);

  let query = repos::list_for_user::Query::builder()
    .page(1)
    .per_page(10)
    .build();

  let repositories = api
    .repos
    .list_for_user("panghu-huang")
    .query(&query)
    .send()
    .await
    .unwrap();

  let repository = repositories.first().unwrap();

  assert_eq!(repository.owner.login, "panghu-huang");

  let query = repos::list_for_user::Query::builder()
    .page(2)
    .per_page(10)
    .build();

  let repositories = api
    .repos
    .list_for_user("panghu-huang")
    .query(&query)
    .send()
    .await
    .unwrap();

  assert!(repositories.get(0).unwrap().full_name != repository.full_name);
}

#[tokio::test]
async fn test_get_repository() {
  dotenv::dotenv().ok();

  let personal_access_token =
    std::env::var("GITHUB_PERSONAL_ACCESS_TOKEN").expect("GITHUB_PERSONAL_ACCESS_TOKEN is not set");

  let personal_access_token = PersonalAccessToken::new(personal_access_token);

  let config = APIConfig::with_token(personal_access_token).shared();

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

#[tokio::test]
async fn test_get_repository_file_content() {
  dotenv::dotenv().ok();

  let personal_access_token =
    std::env::var("GITHUB_PERSONAL_ACCESS_TOKEN").expect("GITHUB_PERSONAL_ACCESS_TOKEN is not set");

  let personal_access_token = PersonalAccessToken::new(personal_access_token);

  let config = APIConfig::with_token(personal_access_token).shared();

  let api = GitHubAPI::new(&config);

  let content = api
    .repos
    .get_content("panghu-huang", "astro-run", "README.md")
    .send()
    .await
    .unwrap();

  match content {
    repos::get_content::Response::ContentFile(file) => {
      assert_eq!(file.name, "README.md");
    }
    _ => {
      println!("{:#?}", content);
      panic!("Expected file content");
    }
  }
}

#[tokio::test]
async fn test_get_repository_dir_content() {
  dotenv::dotenv().ok();

  let personal_access_token =
    std::env::var("GITHUB_PERSONAL_ACCESS_TOKEN").expect("GITHUB_PERSONAL_ACCESS_TOKEN is not set");

  let personal_access_token = PersonalAccessToken::new(personal_access_token);

  let config = APIConfig::with_token(personal_access_token).shared();

  let api = GitHubAPI::new(&config);

  let content = api
    .repos
    .get_content("panghu-huang", "astro-run", "crates")
    .send()
    .await
    .unwrap();

  match content {
    repos::get_content::Response::ContentDirectory(dir) => {
      assert!(dir.len() > 1);
    }
    _ => panic!("Expected directory content"),
  }
}
