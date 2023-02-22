use chrono::{DateTime, Utc};
use infrastructure::{ExpirableToken, GithubError};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GithubTokenPayload {
    iat: i64,
    exp: i64,
    iss: String,
}

#[derive(Debug, Clone)]
pub struct GithubInstallationToken {
    token: String,
    #[allow(dead_code)]
    expires_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct GithubInstallationExpirableToken {
    github_app_id: String,
    github_app_private_key: String,
}

impl GithubInstallationExpirableToken {
    pub fn new(github_app_id: String, github_app_private_key: String) -> Self {
        Self {
            github_app_id,
            github_app_private_key,
        }
    }

    fn generate_token(&self) -> Result<GithubInstallationToken, GithubError> {
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
            &EncodingKey::from_rsa_pem(self.github_app_private_key.as_bytes()).unwrap(),
        )?;

        Ok(GithubInstallationToken { token, expires_at })
    }
}

impl ExpirableToken for GithubInstallationExpirableToken {
    fn is_expired(&self) -> bool {
        true
    }

    fn get_token(&self) -> Option<String> {
        match self.generate_token() {
            Ok(token) => Some(token.token),
            Err(_) => None,
        }
    }
}
