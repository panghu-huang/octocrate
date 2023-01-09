use github_api::{test_utils, GithubApp};

#[tokio::main]
async fn main() {
    let envs = test_utils::load_test_envs().unwrap();

    let mut app = GithubApp::new(envs.github_app_id, envs.github_app_private_key);

    let handle = app.handle();

    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        handle.stop().await.unwrap();
    });

    app.start().await.unwrap();
}
