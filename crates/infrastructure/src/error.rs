use dotenv::Error as DotEnvError;
use jsonwebtoken::errors::Error as JwtError;
use serde::{Deserialize, Serialize};
use serde_json::Error as SerdeJsonError;
use std::error::Error;
use std::fmt;
use std::io::Error as IoError;
use tokio::sync::{mpsc, oneshot};

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

impl From<SerdeJsonError> for GithubError {
    fn from(err: SerdeJsonError) -> Self {
        GithubError {
            message: err.to_string(),
        }
    }
}

impl From<DotEnvError> for GithubError {
    fn from(err: DotEnvError) -> Self {
        GithubError {
            message: err.to_string(),
        }
    }
}

impl From<IoError> for GithubError {
    fn from(err: IoError) -> Self {
        GithubError {
            message: err.to_string(),
        }
    }
}

impl<T> From<mpsc::error::SendError<T>> for GithubError {
    fn from(err: mpsc::error::SendError<T>) -> Self {
        GithubError {
            message: err.to_string(),
        }
    }
}

#[allow(deprecated)]
impl From<mpsc::error::RecvError> for GithubError {
    fn from(err: mpsc::error::RecvError) -> Self {
        GithubError {
            message: err.to_string(),
        }
    }
}

impl From<oneshot::error::RecvError> for GithubError {
    fn from(err: oneshot::error::RecvError) -> Self {
        GithubError {
            message: err.to_string(),
        }
    }
}
