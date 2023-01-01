use dotenv::dotenv;
use github_app::GithubApp;
use std::env;
use std::fs;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let github_app_id = env::var("TEST_GITHUB_APP_ID").unwrap();
    let github_app_private_key_path = env::var("TEST_GITHUB_APP_PRIVATE_KEY_PATH").unwrap();
    let private_key = fs::read_to_string(github_app_private_key_path).unwrap();

    GithubApp::new(github_app_id, private_key)
        .on_webhook_event(|event| {
            println!("event {:#?}", event);
        })
        .connect()
        .await
        .unwrap();
}
