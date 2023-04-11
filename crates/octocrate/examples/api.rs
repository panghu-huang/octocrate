use octocrate::{test_utils, GithubAPI, GithubPersonalAccessToken};

#[tokio::main]
async fn main() {
    let envs = test_utils::load_test_envs().unwrap();

    let token = GithubPersonalAccessToken::new(envs.personal_access_token);

    let api = GithubAPI::with_token(token);

    let repositories = api
        .repositories
        .list_user_repositories("panghu-huang")
        .send()
        .await
        .unwrap();

    println!("Repositories: {:#?}", repositories);
}
