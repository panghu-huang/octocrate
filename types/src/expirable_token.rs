pub trait ExpirableToken: Send + Sync {
  // fn is_expired(&self) -> bool;
  fn get_token(&self) -> Option<String>;
}
