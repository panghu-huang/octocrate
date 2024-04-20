use crate::expirable_token::ExpirableToken;

pub struct PersonalAccessToken {
  token: String,
}

impl PersonalAccessToken {
  pub fn new(token: String) -> Self {
    Self { token }
  }
}

impl ExpirableToken for PersonalAccessToken {
  fn get_token(&self) -> Option<String> {
    Some(self.token.clone())
  }
}
