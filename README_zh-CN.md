# Octocrate

octocrate 是一个基于 Rust 的完整的 GitHub REST API 库。

![octocrate](https://img.shields.io/crates/v/octocrate.svg)
![MIT](https://img.shields.io/badge/license-MIT-blue.svg)

[English](./README.md) | [简体中文](./README_zh-CN.md)

## 特性

- 完全对标官方文档 [GitHub REST API Documentation](https://docs.github.com/en/rest?apiVersion=2022-11-28)
- 对 Body / Query 参数拥有完整的类型限制
- 支持利用 feature 单独引用某个 API 依赖
- 支持 GitHub app 请求 installation API
- 支持 installation access token 和 personal access token
- 定义了所有的 Webhooks 事件类型

## 依赖

```toml
[dependencies]
octocrate = "*"
```

通过 features 按需引入所需要的 API：

```toml
octocrate = { version = "*", features = ["repos", "git", "pulls", "issues", "users", "search"] }
```

或者通过 `full` 特性引入所有的 API 和 Webhooks（注意这会增加编译时间）：

```toml
octocrate = { version = "*", features = ["full"] }
```

### 类型依赖

octocrate 同样支持只引入类型而不使用对应的 API：

```toml
octocrate-types = "*"
```

通过 features 按需引入所需要的类型：

```toml
octocrate-types = { version = "*", features = ["repos", "git", "pulls", "issues", "users", "search"] }
```

引入 Webhooks 类型：

```toml
octocrate-webhooks = { version = "*", features = ["pull_request", "push"] }
```

你可以查看 [octocrate-types 文档](https://docs.rs/crate/octocrate-types/latest/features) 和 [octocrate-webhooks 文档](https://docs.rs/crate/octocrate-webhooks/latest/features) 了解所有支持的 features 和类型。

## 示例

创建一个默认的 GitHub API 配置并使用它来获取一个仓库的 Pull Request：

```rust
use octocrate::{APIConfig, Error, GitHubAPI};

#[tokio::main]
async fn main() {
  // 创建一个默认的 GitHub API 配置
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

#### 直接引入对应的 API

```rust
// 引入 GitHubReposAPI
use octocrate::{repos::GitHubReposAPI, APIConfig, GitHubAPI, PersonalAccessToken};

#[tokio::main]
async fn main() {
  // 创建一个 personal access token
  let personal_access_token = PersonalAccessToken::new("YOUR_PERSONAL_ACCESS_TOKEN");

  // 使用 personal access token 创建一个 API 配置
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

#### 使用 GitHub App

```rust
use octocrate::{APIConfig, AppAuthorization, GitHubAPI};

#[tokio::main]
async fn main() {
  let app_id = "YOUR_APP_ID";
  let private_key = "YOUR_PRIVATE_KEY";

  // 创建一个 GitHub App 授权
  let app_authorization = AppAuthorization::new(app_id, private_key);

  // 使用 GitHub App 授权创建一个 API 配置
  let config = APIConfig::with_token(app_authorization).shared();

  let api = GitHubAPI::new(&config);

  // 获取仓库仓库的 Installation
  let installation = api
    .apps
    .get_repo_installation("panghu-huang", "octocrate")
    .send()
    .await
    .unwrap();

  // 获取这个 Installation 的 Installation Access Token
  let installation_token = api
    .apps
    .create_installation_access_token(installation.id)
    .send()
    .await
    .unwrap();

  // 使用 Installation Access Token 创建一个新的 API 配置
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

#### 请求 Body 参数

```rust
use octocrate::{
  issues, APIConfig, AuthorAssociation, GitHubAPI, PersonalAccessToken,
};

#[tokio::main]
async fn main() {
  let personal_access_token = PersonalAccessToken::new("YOUR_PERSONAL_ACCESS_TOKEN");

  let config = APIConfig::with_token(personal_access_token).shared();

  let api = GitHubAPI::new(&config);

  // 创建一个 Issue Comment 请求
  // https://github.com/panghu-huang/octocrate/pull/1#issuecomment-2041280635
  let comment = issues::create_comment::Request {
    body: "Hello, world! (Created by octocrate)".to_string(),
  };

  let issue_comment = api
    .issues
    .create_comment("panghu-huang", "octocrate", 1)
    .body(&comment)
    .send()
    .await
    .unwrap();
}
```

#### 请求 Query 参数

```rust
use octocrate::{
  pulls, APIConfig, Error, GitHubAPI, PersonalAccessToken,
};

#[tokio::main]
async fn main() {
  let personal_access_token = PersonalAccessToken::new("YOUR_PERSONAL_ACCESS_TOKEN");

  let config = APIConfig::with_token(personal_access_token).shared();

  let api = GitHubAPI::new(&config);

  // 使用构建器创建一个 Pull Request 列表查询
  let query = pulls::list::Query::builder()
    .state(pulls::list::QueryState::Open)
    .per_page(10)
    .build()

  let pull_requests = api
    .pulls
    .list("facebook", "react")
    .query(&query)
    .send()
    .await
    .unwrap();

  // ..
}
```

你可以在 [octocrate/tests](./octocrate/tests) 目录下找到更多的示例。

## 贡献

欢迎随时提交 issue 或者 pull request 来改进项目。

## 许可证

这个项目是根据 MIT 许可证发布的。
