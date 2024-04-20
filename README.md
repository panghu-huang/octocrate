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

## Dependencies

```toml
[dependencies]
octocrate = "0.3"
```

## Example

```rust
use octocrate::{APIConfig, Error, GitHubAPI};

#[tokio::main]
async fn main() {
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

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests to improve the project.

## License

This project is licensed under the MIT License.
