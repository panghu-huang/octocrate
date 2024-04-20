use octocrate::{test_utils, GithubWebhookEvent, ServerHandle, WebhookServer};
use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
  let (tx, rx) = oneshot::channel::<ServerHandle>();
  tokio::spawn(async move {
    let envs = test_utils::load_test_envs().unwrap();

    let mut server = WebhookServer::builder()
      .app_id(envs.github_app_id)
      .private_key(envs.github_app_private_key)
      .build()
      .unwrap();

    let server_handle = server.server_handle();

    tx.send(server_handle).unwrap();

    server
      .on_webhook_event(|event, _api| {
        if let GithubWebhookEvent::Push(evt) = event {
          println!("Push {:#?}", evt);
        }
        Ok(())
      })
      .start()
      .await
      .unwrap();
  });

  let server_handle = rx.await.unwrap();
  let json = include_str!("./push.json");
  let event: GithubWebhookEvent = serde_json::from_str(json).unwrap();

  server_handle.trigger_webhook_event(event).await.unwrap();
  tokio::time::sleep(std::time::Duration::from_secs(3)).await;
}
