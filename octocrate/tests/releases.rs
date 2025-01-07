use octocrate::{multipart, repos, APIConfig, GitHubAPI, PersonalAccessToken};
use tokio::fs::File;

#[tokio::test]
async fn test_create_release_with_assets() {
  dotenv::dotenv().ok();

  let personal_access_token =
    std::env::var("GITHUB_PERSONAL_ACCESS_TOKEN").expect("GITHUB_PERSONAL_ACCESS_TOKEN is not set");

  let personal_access_token = PersonalAccessToken::new(personal_access_token);

  let config = APIConfig::with_token(personal_access_token.clone()).shared();

  let api = GitHubAPI::new(&config);

  let request = repos::create_release::Request::builder()
    .tag_name("ui/bootstrap")
    .draft(true)
    .prerelease(false)
    .build();

  let release = api
    .repos
    .create_release("panghu-huang", "react-github")
    .body(&request)
    .send()
    .await
    .unwrap();

  assert_eq!(release.tag_name, "ui/bootstrap");

  // let part = multipart::Part::file_name(, filename)
  // let form = multipart::Form::new()
  //   .file("test.txt", "/Users/panghu/Desktop/test.txt")
  //   .await
  //   .unwrap();

  let config = APIConfig::new("https://uploads.github.com", personal_access_token);
  let api = GitHubAPI::new(&config);

  let file = File::open("/Users/panghu/Desktop/test.txt").await.unwrap();

  let query = repos::upload_release_asset::Query::builder()
    .name("test.txt")
    .build();
  let release_asset = api
    .repos
    .upload_release_asset("panghu-huang", "react-github", release.id)
    .query(&query)
    .file(file)
    .await
    .send()
    .await
    .map_err(|e| {
      eprintln!("{:#?}", e);
      e
    })
    .unwrap();

  assert_eq!(release_asset.name, "test.txt");
}
