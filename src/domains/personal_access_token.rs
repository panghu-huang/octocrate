use crate::infrastructure::ExpirableToken;

#[derive(Clone, Debug)]
pub struct GithubPersonalAccessToken {
    pub token: String,
}

impl GithubPersonalAccessToken {
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            token: token.into(),
        }
    }
}

impl ExpirableToken for GithubPersonalAccessToken {
    fn is_expired(&self) -> bool {
        false
    }

    fn get_token(&self) -> Option<String> {
        Some(self.token.clone())
    }
}
