use std::sync::Mutex;

use crate::{error::Error, expirable_token::ExpirableToken};
use chrono::{DateTime, Utc};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthorizationPayload {
  iat: i64,
  exp: i64,
  iss: String,
}

#[derive(Debug, Clone)]
pub struct GeneratedToken {
  token: String,
  expires_at: DateTime<Utc>,
}

pub struct AppAuthorizationInner {
  token: Mutex<Option<GeneratedToken>>,
}

pub struct AppAuthorization {
  app_id: String,
  private_key: String,
  inner: AppAuthorizationInner,
}

impl AppAuthorization {
  pub fn new(app_id: String, private_key: String) -> Self {
    Self {
      app_id,
      private_key,
      inner: AppAuthorizationInner {
        token: Mutex::new(None),
      },
    }
  }

  fn generate_token(&self) -> Result<GeneratedToken, Error> {
    let now = chrono::Utc::now().timestamp();
    let expires_at = chrono::Utc::now() + chrono::Duration::seconds(60);
    let payload = AuthorizationPayload {
      iat: now,
      exp: expires_at.timestamp(),
      iss: self.app_id.clone(),
    };

    let token = encode(
      &Header::new(Algorithm::RS256),
      &payload,
      &EncodingKey::from_rsa_pem(self.private_key.as_bytes()).unwrap(),
    )
    .map_err(|_| Error::Error("Failed to generate token".to_string()))?;

    Ok(GeneratedToken { token, expires_at })
  }
}

impl ExpirableToken for AppAuthorization {
  fn get_token(&self) -> Option<String> {
    let mut token = self.inner.token.lock().unwrap();

    if let Some(token) = &*token {
      if token.expires_at > chrono::Utc::now() {
        return Some(token.token.clone());
      }
    }

    match self.generate_token() {
      Ok(new_token) => {
        *token = Some(new_token.clone());
        Some(new_token.token)
      }
      Err(_) => None,
    }
  }
}
