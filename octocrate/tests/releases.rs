use octocrate::{repos, APIConfig, GitHubAPI, PersonalAccessToken};
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
    .tag_name("v2.0.1")
    .draft(true)
    .prerelease(false)
    .build();

  let release = api
    .repos
    .create_release("panghu-huang", "octocrate")
    .body(&request)
    .send()
    .await
    .unwrap();

  assert_eq!(release.tag_name, "v2.0.1");

  // https://docs.github.com/en/rest/releases/assets?apiVersion=2022-11-28#upload-a-release-asset
  let config_for_upload = APIConfig::new("https://uploads.github.com", personal_access_token);
  let api_for_upload = GitHubAPI::new(&config_for_upload);

  let file_path = std::path::Path::new("test.txt");
  tokio::fs::write(file_path, "Hello, world!").await.unwrap();

  let file = File::open(file_path).await.unwrap();
  let content_length = file.metadata().await.unwrap().len();

  let query = repos::upload_release_asset::Query::builder()
    .name("test.txt")
    .build();
  let release_asset = api_for_upload
    .repos
    .upload_release_asset("panghu-huang", "octocrate", release.id)
    .query(&query)
    // Set the content type of the file
    .header("Content-Type", "text/plain")
    // Set the content length of the file
    .header("Content-Length", content_length.to_string())
    // Put the file into the body of the request
    .file(file)
    .send()
    .await
    .unwrap();

  assert_eq!(release_asset.name, "test.txt");

  tokio::fs::remove_file(file_path).await.unwrap();

  // Delete the release after the test
  api
    .repos
    .delete_release("panghu-huang", "octocrate", release.id)
    .send()
    .await
    .unwrap();
}
