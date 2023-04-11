use octocrate::{events::GithubWebhookEvent, test_utils, AppHandle, GithubApp};
use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
    let (tx, rx) = oneshot::channel::<AppHandle>();
    tokio::spawn(async move {
        let envs = test_utils::load_test_envs().unwrap();

        let mut app = GithubApp::builder()
            .app_id(envs.github_app_id)
            .private_key(envs.github_app_private_key)
            .build()
            .unwrap();

        let app_handle = app.app_handle();

        tx.send(app_handle).unwrap();

        app.on_webhook_event(|event, _api| {
            if let GithubWebhookEvent::Push(evt) = event {
                println!("Push {:#?}", evt);
            }
            Ok(())
        })
        .start()
        .await
        .unwrap();
    });

    let app_handle = rx.await.unwrap();
    let json = include_str!("./push.json");
    let event = GithubWebhookEvent::try_parse("push".to_string(), json.to_string()).unwrap();

    app_handle.trigger_webhook_event(event).await.unwrap();
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
}
