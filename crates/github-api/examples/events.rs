use github_api::{events::GithubWebhookEvent, test_utils, GithubApp};

#[tokio::main]
async fn main() {
    let envs = test_utils::load_test_envs().unwrap();

    let mut app = GithubApp::builder()
        .app_id(envs.github_app_id)
        .private_key(envs.github_app_private_key)
        .build()
        .unwrap();
    app.on_webhook_event(|event, _api| {
        match event {
            GithubWebhookEvent::PullRequest(evt) => {
                println!("Pull request {:#?}", evt);
            }
            GithubWebhookEvent::Push(evt) => {
                println!("Push {:#?}", evt);
            }
            GithubWebhookEvent::IssueComment(evt) => {
                println!("Issue comment {:#?}", evt);
            }
            GithubWebhookEvent::Unsupported {
                installation: _,
                payload,
            } => {
                println!("Unsupported {:#?}", payload);
            }
        };

        Ok(())
    })
    .serve()
    .await
    .unwrap();
}
