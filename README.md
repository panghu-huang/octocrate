# Octocrate

octocrate is a comprehensive GitHub REST API library based on Rust.

![octocrate](https://img.shields.io/crates/v/octocrate.svg)
![MIT](https://img.shields.io/badge/license-MIT-blue.svg)

## Features

- Fully compliant with the official documentation at [GitHub REST API Documentation](https://docs.github.com/en/rest?apiVersion=2022-11-28)
- Complete type restrictions for Body / Query parameters
- Utilizes feature flags for individual API dependencies
- Supports GitHub app requests for installation API
- Supports installation access tokens and personal access tokens
- Defines all Webhooks event types

## Dependencies

```toml
[dependencies]
octocrate = "1.0"
```

Use features to selectively import the required APIs:

```toml
octocrate = { version = "1.0", features = ["repos", "git", "pulls", "issues", "users", "search"] }
```

Or use the `full` feature to import all APIs and Webhooks (note: this will increase compilation time):

```toml
octocrate = { version = "1.0", features = ["full"] }
```

### Type Dependencies

You can also import only types without using the corresponding APIs:

```toml
octocrate-types = "1.0"
```

Use features to selectively import the required types:

```toml
octocrate-types = { version = "1.0", features = ["repos", "git", "pulls", "issues", "users", "search"] }
```

Import Webhooks types:

```toml
octocrate-types = { version = "1.0", features = ["webhook_pull_request", "webhook_push"] }
```

You can check the [octocrate-types documentation](https://docs.rs/crate/octocrate-types/latest/features) for all supported features and types.

## Example

Create a default GitHub API configuration and use it to get a repository's Pull Request:

```rust
use octocrate::{APIConfig, Error, GitHubAPI};

#[tokio::main]
async fn main() {
  // Create a default GitHub API configuration
  let config = APIConfig::default().shared();

  let api = GitHubAPI::new(&config);

  let pull_request = api
    .pulls
    .get("panghu-huang", "octocrate", 1)
    .send()
    .await
    .unwrap();

  // ..
}
```

#### Directly import the corresponding API

```rust
// Import the repos API instead of GitHubAPI
use octocrate::{repos::GitHubReposAPI, APIConfig, GitHubAPI, PersonalAccessToken};

#[tokio::main]
async fn main() {
  // Create a personal access token
  let personal_access_token = PersonalAccessToken::new(personal_access_token);

  // Use the personal access token to create a API configuration
  let config = APIConfig::with_token(personal_access_token).shared();

  let repos_api = GitHubReposAPI::new(&config);

  let commit = repos_api
    .get_commit(
      "panghu-huang",
      "octocrate",
      "18ff8ed1a3437649e7d87bec9a7d4fe5562f6ad3",
    )
    .send()
    .await
    .unwrap();
}
```

#### Using GitHub App

```rust
use octocrate::{APIConfig, AppAuthorization, GitHubAPI};

#[tokio::main]
async fn main() {
  let app_id = 12345;
  let private_key = "YOUR_PRIVATE_KEY";

  // Create a GitHub App authorization
  let app_authorization = AppAuthorization::new(app_id, private_key);

  // Use the GitHub App authorization to create an API configuration
  let config = APIConfig::with_token(app_authorization).shared();

  let api = GitHubAPI::new(&config);

  // Get the Installation for a repository
  let installation = api
    .apps
    .get_repo_installation("panghu-huang", "octocrate")
    .send()
    .await
    .unwrap();

  // Get the Installation Access Token for this Installation
  let installation_token = api
    .apps
    .create_installation_access_token(installation.id)
    .send()
    .await
    .unwrap();

  // Use the Installation Access Token to create a new API configuration
  let config = APIConfig::with_token(installation_token).shared();

  let api = GitHubAPI::new(&config);

  let repository = api
    .repos
    .get("panghu-huang", "octocrate")
    .send()
    .await
    .unwrap();

  // ..
}
```

#### Request Body Parameters

```rust
use octocrate::{
  APIConfig, AuthorAssociation, GitHubAPI, IssuesCreateCommentRequest, PersonalAccessToken,
};

#[tokio::main]
async fn main() {
  let personal_access_token = PersonalAccessToken::new("YOUR_PERSONAL_ACCESS_TOKEN");

  let config = APIConfig::with_token(personal_access_token).shared();

  let api = GitHubAPI::new(&config);

  // Create a comment request
  // https://github.com/panghu-huang/octocrate/pull/1#issuecomment-2041280635
  let comment = IssuesCreateCommentRequest {
    body: "Hello, world! (Created by octocrate)".to_string(),
  };

  // Create a comment on the issue
  let issue_comment = api
    .issues
    .create_comment("panghu-huang", "octocrate", 1)
    .body(&comment)
    .send()
    .await
    .unwrap();
}
```

#### Request Query Parameters

```rust
use octocrate::{
  APIConfig, Error, GitHubAPI, PersonalAccessToken, PullsListQuery, PullsListQueryState,
};

#[tokio::main]
async fn main() {
  let personal_access_token = PersonalAccessToken::new("YOUR_PERSONAL_ACCESS_TOKEN");

  let config = APIConfig::with_token(personal_access_token).shared();

  let api = GitHubAPI::new(&config);

  let pull_requests = api
    .pulls
    .list("facebook", "react")
    .query(
      // Use builder pattern to construct query parameters
      &PullsListQuery::builder()
        .state(PullsListQueryState::Open)
        .per_page(10)
        .build(),
    )
    .send()
    .await
    .unwrap();

  // ..
}
```

You can find more example code in the [octocrate/examples](./octocrate/examples) directory.

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests to improve the project.

## License

This project is licensed under the MIT License.
