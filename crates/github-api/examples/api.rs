use github_api::{personal_access_token::GithubPersonalAccessToken, test_utils, GithubAPI};

#[tokio::main]
async fn main() {
    let envs = test_utils::load_test_envs().unwrap();

    let token = GithubPersonalAccessToken::new(envs.personal_access_token);

    let api = GithubAPI::new(token);

    let repositories = api
        .repositories
        .list_user_repositories("panghu-huang")
        .send()
        .await
        .unwrap();

    println!("Repositories: {:#?}", repositories);
}
