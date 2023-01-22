# Github API

## Install

Install `github-api` by adding the following to your `Cargo.toml` file:

```toml
[dependencies]
github-api = { git = "https://github.com/panghu-huang/github-api" }
```

## Usage

### Github App
```rust
use github_api::{events::GithubWebhookEvent, GithubApp};
use serde_json::json;

#[tokio::main]
async fn main() {
    let mut app = GithubApp::builder()
        .app_id("GITHUB_APP_ID")
        .private_key("GITHUB_APP_PRIVATE_KEY")
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
use github_api::{GithubPersonalAccessToken, GithubAPI};

#[tokio::main]
async fn main() {
    let token = GithubPersonalAccessToken::new("YOUR_PERSONAL_ACCESS_TOKEN");

    let api = GithubAPI::with_token(token);

    let repositories = api
        .repositories
        .list_user_repositories("panghu-huang")
        .send()
        .await
        .unwrap();

    println!("Repositories: {:#?}", repositories);
}
```