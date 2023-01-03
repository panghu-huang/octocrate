use github_app::{events::GithubWebhookEvent, test_utils, GithubApp};

#[tokio::main]
async fn main() {
    let envs = test_utils::load_test_envs().unwrap();

    GithubApp::new(envs.github_app_id, envs.github_app_private_key)
        .on_webhook_event(|event, _api| {
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
                GithubWebhookEvent::Unsupported(evt) => {
                    println!("Unsupported {:#?}", evt);
                }
            };

            Ok(())
        })
        .start()
        .await
        .unwrap();
}
