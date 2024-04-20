pub trait ExpirableToken: Send + Sync {
  // fn is_expired(&self) -> bool;
  fn get_token(&self) -> Option<String>;
}

#[cfg(any(feature = "full", feature = "apps"))]
impl ExpirableToken for octocrate_types::InstallationToken {
  fn get_token(&self) -> Option<String> {
    let is_expired = self.expires_at < chrono::Utc::now().to_rfc3339();

    if is_expired {
      None
    } else {
      Some(self.token.clone())
    }
  }
}
