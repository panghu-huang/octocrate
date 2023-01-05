use crate::app::GithubApp;
use crate::domains::personal_access_token::GithubPersonalAccessToken;
use infrastructure::{GithubAPIClient, GithubError, GithubResult};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct TestEnvs {
    pub github_app_id: String,
    pub github_app_private_key: String,
    pub installation_id: u64,
    pub repo_owner: String,
    pub repo_name: String,
    pub issue_number: u64,
    pub personal_access_token: String,
}

fn read_env(key: &str) -> GithubResult<String> {
    env::var(key).map_err(|_| GithubError::new(format!("{} is not set", key)))
}

pub fn load_test_envs() -> GithubResult<TestEnvs> {
    dotenv()?;
    let github_app_id = read_env("TEST_GITHUB_APP_ID")?;
    let github_app_private_key_path = read_env("TEST_GITHUB_APP_PRIVATE_KEY_PATH")?;
    let installation_id = read_env("TEST_GITHUB_INSTALLATION_ID")?;
    let repo_owner = read_env("TEST_GITHUB_REPO_OWNER")?;
    let repo_name = read_env("TEST_GITHUB_REPO_NAME")?;
    let issue_number = read_env("TEST_GITHUB_ISSUE_NUMBER")?;
    let personal_access_token = read_env("TEST_GITHUB_PERSONAL_ACCESS_TOKEN")?;

    let github_app_private_key = fs::read_to_string(github_app_private_key_path)?;

    Ok(TestEnvs {
        github_app_id,
        github_app_private_key,
        installation_id: installation_id.parse::<u64>().unwrap(),
        repo_owner,
        repo_name,
        issue_number: issue_number.parse::<u64>().unwrap(),
        personal_access_token,
    })
}

pub fn create_github_app() -> GithubResult<GithubApp> {
    dotenv()?;

    let envs = load_test_envs()?;

    Ok(GithubApp::new(
        envs.github_app_id,
        envs.github_app_private_key,
    ))
}

pub fn create_api_client() -> GithubResult<GithubAPIClient<GithubPersonalAccessToken>> {
    dotenv()?;

    let envs = load_test_envs()?;

    let token = GithubPersonalAccessToken::new(envs.personal_access_token);

    Ok(GithubAPIClient::new(token))
}
