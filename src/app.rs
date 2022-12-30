use crate::domains::{
    account::GithubAccount, installation::GithubInstallation, permissions::GithubPermissions,
    repository::GithubRepository,
};
use crate::expirable_token::ExpirableToken;
use crate::infrastructure::error::GithubError;
use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, EncodingKey, Header, Algorithm};

pub type GithubEventListener = Box<dyn Fn(GithubEvent) + Send + Sync>;

pub struct GithubApp {
    id: String,
    private_key: String,
    token: ExpirableToken,
    listeners: Vec<GithubEventListener>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GithubTokenPayload {
    iat: String,
    exp: String,
    iss: String,
}

pub struct GithubEvent {
    action: String,
    installation: GithubInstallation,
    repository: GithubRepository,
    sender: GithubAccount,
}

impl GithubApp {
    pub fn new(id: String, private_key: String) -> Self {
        Self {
            id,
            private_key,
            token: ExpirableToken::default(),
            listeners: Vec::new(),
        }
    }

    pub fn get_installations(&self) -> String {
        format!("https://api.github.com/app/installations")
    }

    pub fn get_installation(&self, installation_id: String) -> String {
        format!(
            "https://api.github.com/app/installations/{}",
            installation_id
        )
    }

    pub fn get_installation_access_token(&self, installation_id: String) -> String {
        format!(
            "https://api.github.com/app/installations/{}/access_tokens",
            installation_id
        )
    }

    fn get_access_token(&self) -> Result<String, GithubError> {
        let now = chrono::Utc::now().timestamp();
        let payload = GithubTokenPayload {
            iat: now.to_string(),
            exp: (now + 60).to_string(),
            iss: self.id.clone(),
        };
        let token = encode(
            &Header::new(Algorithm::RS256),
            &payload,
            &EncodingKey::from_rsa_pem(self.private_key.as_bytes()).unwrap(),
        )?;
        println!("token: {}", token);

        Ok(token)
    }

    pub fn on_event<F>(&mut self, listener: F)
    where
        F: Fn(GithubEvent) + Send + Sync + 'static,
    {
        self.listeners.push(Box::new(listener));
    }
}
