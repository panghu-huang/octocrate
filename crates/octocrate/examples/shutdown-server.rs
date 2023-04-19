use octocrate::{test_utils, WebhookServer};

#[tokio::main]
async fn main() {
  let envs = test_utils::load_test_envs().unwrap();

  let mut server = WebhookServer::builder()
    .app_id(envs.github_app_id)
    .private_key(envs.github_app_private_key)
    .build()
    .unwrap();

  let server_handle = server.server_handle();

  tokio::spawn(async move {
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    server_handle.stop().await.unwrap();
  });

  server.start().await.unwrap();
}
