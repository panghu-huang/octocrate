use octocrate_core::*;
#[allow(unused_imports)]
use octocrate_types::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use typed_builder::TypedBuilder;

pub mod get {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = serde_json::Value;
}

/// List emojis available to use on GitHub.
pub struct GitHubEmojisAPI {
  config: SharedAPIConfig,
}

impl GitHubEmojisAPI {
  pub fn new(config: &SharedAPIConfig) -> Self {
    Self {
      config: config.clone(),
    }
  }

  /// **Get emojis**
  ///
  /// Lists all the emojis available to use on GitHub.
  ///
  /// *Documentation*: [https://docs.github.com/rest/emojis/emojis#get-emojis](https://docs.github.com/rest/emojis/emojis#get-emojis)
  pub fn get(&self) -> Request<(), (), get::Response> {
    let url = format!("/emojis");

    Request::<(), (), get::Response>::builder(&self.config)
      .get(url)
      .build()
  }
}
