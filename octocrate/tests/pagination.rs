use octocrate::{repos, APIConfig, GitHubAPI, PersonalAccessToken};

#[tokio::test]
async fn test_pagination() {
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
    .list_for_user("ifiokjr")
    .query(&query)
    .paginated_send()
    .await
    .unwrap();

  let repository = repositories.data.first().unwrap();

  assert_eq!(repository.owner.login, "ifiokjr");

  let query = repos::list_for_user::Query::builder()
    .page(repositories.pages.last.unwrap())
    .per_page(10)
    .build();

  let repositories = api
    .repos
    .list_for_user("ifiokjr")
    .query(&query)
    .paginated_send()
    .await
    .unwrap();

  assert!(repositories.data.get(0).unwrap().full_name != repository.full_name);
}

fn main() {}
