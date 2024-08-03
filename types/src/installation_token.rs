use super::expirable_token::ExpirableToken;
use super::models::InstallationToken;

impl ExpirableToken for InstallationToken {
  fn get_token(&self) -> Option<String> {
    let is_expired = self.expires_at < chrono::Utc::now().to_rfc3339();

    if is_expired {
      None
    } else {
      Some(self.token.clone())
    }
  }
}
