use octocrate_types::ExpirableToken;

pub struct PersonalAccessToken {
  token: String,
}

impl PersonalAccessToken {
  pub fn new(token: impl Into<String>) -> Self {
    Self {
      token: token.into(),
    }
  }
}

impl ExpirableToken for PersonalAccessToken {
  fn get_token(&self) -> Option<String> {
    Some(self.token.clone())
  }
}
