use octocrate::{test_utils, GithubAPI, GithubAPIConfig, GithubPersonalAccessToken};

#[tokio::main]
async fn main() {
    let envs = test_utils::load_test_envs().unwrap();

    let token = GithubPersonalAccessToken::new(envs.personal_access_token);

    let config = GithubAPIConfig::new("https://api.github.com", token);
    let api = GithubAPI::new(config);

    let repositories = api
        .repositories
        .list_user_repositories("panghu-huang")
        .send()
        .await
        .unwrap();

    println!("Repositories: {:#?}", repositories);
}
