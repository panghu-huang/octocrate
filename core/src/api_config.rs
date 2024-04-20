use crate::expirable_token::ExpirableToken;
use std::sync::Arc;

pub struct APIConfig {
  pub base_url: String,
  pub token: Option<Arc<dyn ExpirableToken>>,
}

pub type SharedAPIConfig = Arc<APIConfig>;

impl APIConfig {
  pub fn default() -> Self {
    APIConfig {
      base_url: "https://api.github.com".to_string(),
      token: None,
    }
  }

  pub fn with_token<T: ExpirableToken + 'static>(token: T) -> Self {
    APIConfig {
      base_url: "https://api.github.com".to_string(),
      token: Some(Arc::new(token)),
    }
  }

  pub fn with_base_url(base_url: &str) -> Self {
    APIConfig {
      base_url: base_url.to_string(),
      token: None,
    }
  }

  pub fn new<T: ExpirableToken + 'static>(base_url: &str, token: T) -> SharedAPIConfig {
    Arc::new(APIConfig {
      base_url: base_url.to_string(),
      token: Some(Arc::new(token)),
    })
  }

  pub fn shared(self) -> SharedAPIConfig {
    Arc::new(self)
  }
}
