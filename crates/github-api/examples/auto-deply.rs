use github_api::{events::GithubWebhookEvent, test_utils, GithubApp};
use serde_json::json;

#[tokio::main]
async fn main() {
    let envs = test_utils::load_test_envs().unwrap();

    let mut app = GithubApp::builder()
        .app_id(envs.github_app_id)
        .private_key(envs.github_app_private_key)
        .build()
        .unwrap();

    app.on_webhook_event(|event, api| {
        match event {
            GithubWebhookEvent::IssueComment(evt) => {
                if evt.comment.body.starts_with("Hello") {
                    tokio::spawn(async move {
                        let owner = evt.repository.owner.login;
                        let repo = evt.repository.name;
                        let issue_number = evt.issue.number;

                        println!("Reply issue comment {}/{}#{}", owner, repo, issue_number);
                        let _issue_comment = api
                            .issues
                            .create_issue_comment(owner, repo, issue_number)
                            .body(&json!({
                                "body": "Reply from Coodev CI"
                            }))
                            .send()
                            .await;
                    });
                }
            }
            _ => {}
        };

        Ok(())
    })
    .serve()
    .await
    .unwrap();
}
