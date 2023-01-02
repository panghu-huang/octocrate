use dotenv::dotenv;
use github_app::{events::GithubWebhookEvent, GithubApp};
use std::env;
use std::fs;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let github_app_id = env::var("TEST_GITHUB_APP_ID").unwrap();
    let github_app_private_key_path = env::var("TEST_GITHUB_APP_PRIVATE_KEY_PATH").unwrap();
    let private_key = fs::read_to_string(github_app_private_key_path).unwrap();

    GithubApp::new(github_app_id, private_key)
        .on_webhook_event(|event, _installation, api| {
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
                                .create_issue_comment(
                                    owner,
                                    repo,
                                    issue_number,
                                    "Reply from Coodev CI",
                                )
                                .await;
                        });
                    }
                }
                _ => {}
            };

            Ok(())
        })
        .start()
        .await
        .unwrap();
}
