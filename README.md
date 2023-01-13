# Github API

## Usage

### Github App
```rust
use github_api::{events::GithubWebhookEvent, GithubApp};
use serde_json::json;

#[tokio::main]
async fn main() {
    GithubApp::new("GITHUB_APP_ID", "GITHUB_APP_PRIVATE_KEY")
        .on_webhook_event(|event, api| {
            match event {
                GithubWebhookEvent::IssueComment(evt) => {
                    if evt.comment.body.starts_with("Hello") {
                        tokio::spawn(async move {
                            let owner = evt.repository.owner.login;
                            let repo = evt.repository.name;
                            let issue_number = evt.issue.number;

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
```

### Github API
```rust
use github_api::{personal_access_token::GithubPersonalAccessToken, GithubAPI};

#[tokio::main]
async fn main() {
    let token = GithubPersonalAccessToken::new("YOUR_PERSONAL_ACCESS_TOKEN");

    let api = GithubAPI::new(token);

    let repositories = api
        .repositories
        .list_user_repositories("panghu-huang")
        .send()
        .await
        .unwrap();

    println!("Repositories: {:#?}", repositories);
}
```