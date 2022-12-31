use crate::infrastructure::{
  error::GithubError,
  expirable_token::{ExpirableToken},
};
use chrono::{DateTime, Utc};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GithubTokenPayload {
    iat: i64,
    exp: i64,
    iss: String,
}

#[derive(Debug, Clone)]
pub struct GithubInstallationExpirableToken {
    github_app_id: String,
    github_app_private_key: String,
    token: Option<String>,
    expires_at: Option<DateTime<Utc>>,
}

impl GithubInstallationExpirableToken {
    pub fn new(github_app_id: String, github_app_private_key: String) -> Self {
        Self {
            token: None,
            expires_at: None,
            github_app_id,
            github_app_private_key,
        }
    }

    
    fn generate_token(&mut self) -> Result<String, GithubError> {
      let now = chrono::Utc::now().timestamp();
      let expires_at = chrono::Utc::now() + chrono::Duration::seconds(60);
      let payload = GithubTokenPayload {
          iat: now,
          exp: expires_at.timestamp(),
          iss: self.github_app_id.clone(),
      };
      let token = encode(
          &Header::new(Algorithm::RS256),
          &payload,
          &EncodingKey::from_rsa_pem(
            self.github_app_private_key
              .as_bytes()
          ).unwrap(),
      )?;
      println!("token: {}", token);
      self.token = Some(token.clone());
      self.expires_at = Some(expires_at);

      Ok(token)
  }

  pub fn generate_token_if_expired(&mut self) -> Result<String, GithubError> {
      if self.is_expired() {
          self.generate_token()
      } else {
          Ok(self.token.clone().unwrap())
      }
  }
}

impl ExpirableToken for GithubInstallationExpirableToken {
    fn is_expired(&self) -> bool {
        self.expires_at.is_none() || self.expires_at.unwrap() < Utc::now()
    }

    fn get_token(&self) -> Option<String> {
        if self.is_expired() {
            None
        } else {
            self.token.clone()
        }
    }
}
