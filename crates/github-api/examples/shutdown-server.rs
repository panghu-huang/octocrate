use github_api::{test_utils, GithubApp};

#[tokio::main]
async fn main() {
    let envs = test_utils::load_test_envs().unwrap();

    let mut app = GithubApp::builder()
        .app_id(envs.github_app_id)
        .private_key(envs.github_app_private_key)
        .build()
        .unwrap();

    let app_handle = app.app_handle();

    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        app_handle.stop().await.unwrap();
    });

    app.serve().await.unwrap();
}
