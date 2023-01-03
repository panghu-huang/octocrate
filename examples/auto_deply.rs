use github_app::{events::GithubWebhookEvent, test_utils, GithubApp};

#[tokio::main]
async fn main() {
    let envs = test_utils::load_test_envs().unwrap();

    GithubApp::new(envs.github_app_id, envs.github_app_private_key)
        .on_webhook_event(|event, api| {
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
