use crate::infrastructure::error::GithubError;

pub type GithubResult<T> = std::result::Result<T, GithubError>;