use crate::{app::GithubApp, GithubPersonalAccessToken};
use dotenv::dotenv;
use octocrate_infra::{GithubAPIClient, GithubError, GithubResult};
use serde::{Deserialize, Serialize};
use std::{env, fs};

#[derive(Debug, Serialize, Deserialize)]
pub struct TestEnvs {
  pub github_app_id: String,
  pub github_app_private_key: String,
  pub installation_id: u64,
  pub repo_owner: String,
  pub repo_name: String,
  pub issue_number: u64,
  pub personal_access_token: String,
  pub branch_name: String,
  pub commit_sha: String,
  pub deployment_id: u64,
}

fn read_env(key: &str) -> GithubResult<String> {
  env::var(key).map_err(|_| GithubError::new(format!("{} is not set", key)))
}

fn resolve_dotenv() {
  if let Err(err) = dotenv() {
    eprintln!("Failed to load .env file: {}", err);
  }
}

pub fn load_test_envs() -> GithubResult<TestEnvs> {
  resolve_dotenv();

  let github_app_id = read_env("TEST_GITHUB_APP_ID")?;
  let github_app_private_key_path = read_env("TEST_GITHUB_APP_PRIVATE_KEY_PATH")?;
  let installation_id = read_env("TEST_GITHUB_INSTALLATION_ID")?;
  let repo_owner = read_env("TEST_GITHUB_REPO_OWNER")?;
  let repo_name = read_env("TEST_GITHUB_REPO_NAME")?;
  let issue_number = read_env("TEST_GITHUB_ISSUE_NUMBER")?;
  let personal_access_token = read_env("TEST_GITHUB_PERSONAL_ACCESS_TOKEN")?;
  let branch_name = read_env("TEST_GITHUB_BRANCH_NAME")?;
  let commit_sha = read_env("TEST_GITHUB_COMMIT_SHA")?;
  let deployment_id = read_env("TEST_GITHUB_DEPLOYMENT_ID")?;

  let github_app_private_key = fs::read_to_string(github_app_private_key_path)
    .map_err(|err| GithubError::new(format!("Failed to read private key: {:?}", err)))?;

  Ok(TestEnvs {
    github_app_id,
    github_app_private_key,
    installation_id: installation_id.parse::<u64>().map_err(|_| {
      GithubError::new(format!(
        "Failed to parse installation id: {}",
        installation_id
      ))
    })?,
    repo_owner,
    repo_name,
    issue_number: issue_number
      .parse::<u64>()
      .map_err(|_| GithubError::new(format!("Failed to parse issue number: {}", issue_number)))?,
    personal_access_token,
    branch_name,
    commit_sha,
    deployment_id: deployment_id
      .parse::<u64>()
      .map_err(|_| GithubError::new(format!("Failed to parse deployment id: {}", deployment_id)))?,
  })
}

pub fn create_github_app() -> GithubResult<GithubApp> {
  resolve_dotenv();

  let envs = load_test_envs()?;
  let app = GithubApp::builder()
    .app_id(envs.github_app_id)
    .private_key(envs.github_app_private_key)
    .build()?;

  Ok(app)
}

pub fn create_api_client() -> GithubResult<GithubAPIClient> {
  resolve_dotenv();

  let envs = load_test_envs()?;
  let token = GithubPersonalAccessToken::new(envs.personal_access_token);

  Ok(GithubAPIClient::with_token(token))
}
