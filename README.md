# Github API

## Install

Install `github-api` by adding the following to your `Cargo.toml` file:

```toml
[dependencies]
github-api = { git = "https://github.com/panghu-huang/github-api" }
```

## Usage

### Github App

Here's a simple example showing how to create a Github App and handle the issue comment event.

```rust
use github_api::{events::GithubWebhookEvent, GithubApp};

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
                    // handle issue comment event
                    // ...
                }
                _ => {
                    // handle other events
                    // ...
                }
            };

            Ok(())
        })
        .serve()
        .await
        .unwrap();
}
```

The `github_app.serve()` method opens a web server to listen for webhook requests from Github, the default path is `/github/webhook`

If you want to customize the server, use `start()` instead of `serve()`. Example: [examples/manual-trigger.rs](./crates/github-api/examples/manual-trigger.rs)

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