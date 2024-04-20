use serde::Deserialize;
use thiserror::Error as ThisError;

#[derive(Deserialize, Debug, Clone)]
pub struct APIErrorResponse {
  pub message: String,
  pub documentation_url: String,
}

#[derive(ThisError, Debug, Clone)]
pub enum Error {
  #[error("Error: {0}")]
  Error(String),
  #[error("Request failed with {0}")]
  RequestFailed(APIErrorResponse),
}

impl std::fmt::Display for APIErrorResponse {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "Error: {}", self.message)
  }
}
