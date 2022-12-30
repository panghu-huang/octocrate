use serde::{Deserialize, Serialize};
use std::error::Error;
use jsonwebtoken::errors::Error as JwtError;
use std::fmt;

#[derive(Debug, Deserialize, Serialize)]
pub struct GithubError {
    pub message: String,
}

impl fmt::Display for GithubError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for GithubError {}

impl GithubError {
    pub fn new<StringLike>(err_msg: StringLike) -> Self
    where
        StringLike: Into<String>,
    {
        GithubError {
            message: err_msg.into(),
        }
    }
}

impl From<JwtError> for GithubError {
    fn from(err: JwtError) -> Self {
        GithubError {
            message: err.to_string(),
        }
    }
}