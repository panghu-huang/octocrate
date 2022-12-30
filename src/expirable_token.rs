use chrono::{DateTime, Utc};

pub struct ExpirableToken {
    pub token: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
}

impl ExpirableToken {
    pub fn default() -> Self {
        Self { token: None, expires_at: None }
    }

    pub fn new(token: String, expires_at: DateTime<Utc>) -> Self {
        Self { token: Some(token), expires_at: Some(expires_at) }
    }

    pub fn is_expired(&self) -> bool {
        self.expires_at.is_none() || self.expires_at.unwrap() < Utc::now()
    }

    pub fn is_valid(&self) -> bool {
        !self.is_expired()
    }

    pub fn get_token(&self) -> Option<String> {
        if self.is_valid() {
            self.token.clone()
        } else {
            None
        }
    }

    pub fn set_token(&mut self, token: String, expires_at: DateTime<Utc>) {
        self.token = Some(token);
        self.expires_at = Some(expires_at);
    }
}