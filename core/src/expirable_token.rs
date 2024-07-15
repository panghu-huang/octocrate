pub trait ExpirableToken: Send + Sync {
  fn get_token(&self) -> Option<String>;
}
