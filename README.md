# Octocrate (Octocat + Crate)

A Github API library based on Rust

![octocrate](https://img.shields.io/crates/v/octocrate.svg)
![MIT](https://img.shields.io/badge/license-MIT-blue.svg)

## Install

Install `Octocrate` with `cargo`:

```bash
cargo add octocrate
```

Or modify your `Cargo.toml` to include `octocrate` as a dependency:

```toml
[dependencies]
octocrate = "0.1.2"
```

## Usage

### Github API

```rust
use octocrate::{GithubPersonalAccessToken, GithubAPI};

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

### Github App
  
```rust
use octocrate::{GithubApp};

#[tokio::main]
async fn main() {
  let app = GithubApp::builder()
    .app_id("GITHUB_APP_ID")
    .private_key("GITHUB_APP_PRIVATE_KEY")
    .build()
    .unwrap();

  let installation = app
    .get_repository_installation("panghu-huang", "octocrate")
    .await
    .unwrap();

  let api = app.get_api(installation.id).await.unwrap();

  let repository = api
    .repositories
    .get_repository("panghu-huang", "octocrate")
    .send()
    .await
    .unwrap();

  println!("Repository: {:?}", repository);
}
```

### Github Webhook Server

Here's a simple example showing how to create a Github webhook server and handle the issue comment event. **The `webhook-server` feature needs to be enabled**

```rust
use octocrate::{GithubApp, GithubWebhookEvent, WebhookServer};

#[tokio::main]
async fn main() {
  let mut server = WebhookServer::builder()
    .app_id("GITHUB_APP_ID")
    .private_key("GITHUB_APP_PRIVATE_KEY")
    .build()
    .unwrap();

  server
    .on_webhook_event(|event, api| {
      match event {
        GithubWebhookEvent::IssueComment(evt) => {
          // ... handle issue comment event
        }
        _ => {}
      };

      Ok(())
    })
    .start()
    .await
    .unwrap();
}
```

The `github_app.start()` method opens a web server to listen for webhook requests from Github, the default path is `/github/webhook`

### Examples

For more examples, please refer to: [examples](./crates/octocrate/examples)
