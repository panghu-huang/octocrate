#[allow(unused_imports)]
use crate::types::*;
use octocrate_core::*;

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
  pub fn get(&self) -> Request<(), (), serde_json::Value> {
    let url = format!("/emojis");

    Request::<(), (), serde_json::Value>::builder(&self.config)
      .get(url)
      .build()
  }
}
