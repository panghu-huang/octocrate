mod api_client;
mod error;

use std::fmt::Debug;

pub use api_client::*;
pub use error::*;

pub trait ExpirableToken: Debug {
    fn is_expired(&self) -> bool;
    fn get_token(&self) -> Option<String>;
}

pub type GithubResult<T> = std::result::Result<T, GithubError>;
