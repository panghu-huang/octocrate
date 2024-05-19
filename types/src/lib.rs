mod models;

#[allow(unused_imports)]
pub use models::*;

pub mod parameters;

pub mod webhooks;

#[cfg(feature = "pagination")]
mod pagination;

#[cfg(feature = "pagination")]
pub use pagination::*;
use serde::{Deserialize, Serialize};

/// Predefined types
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum StringOrInteger {
  String(String),
  Number(i64),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum StringOrNumber {
  String(String),
  Number(f64),
}

/// Predefined types
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ObjectOrString<T> {
  String(String),
  Object(T),
}

/// Predefined types
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum StringOrBool {
  String(String),
  Bool(bool),
}

/// Predefined types
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ReadWritePermission {
  #[serde(rename = "read")]
  Read,
  #[serde(rename = "write")]
  Write,
}

/// Predefined types
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ReadPermission {
  #[serde(rename = "read")]
  Read,
}

/// Predefined types
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum WritePermission {
  #[serde(rename = "write")]
  Write,
}

impl std::fmt::Display for StringOrNumber {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      StringOrNumber::String(x) => write!(f, "{}", x),
      StringOrNumber::Number(x) => write!(f, "{}", x),
    }
  }
}

impl std::fmt::Display for StringOrInteger {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      StringOrInteger::String(x) => write!(f, "{}", x),
      StringOrInteger::Number(x) => write!(f, "{}", x),
    }
  }
}

impl std::fmt::Display for StringOrBool {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      StringOrBool::String(x) => write!(f, "{}", x),
      StringOrBool::Bool(x) => write!(f, "{}", x),
    }
  }
}

impl std::fmt::Display for ReadWritePermission {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      ReadWritePermission::Read => write!(f, "read"),
      ReadWritePermission::Write => write!(f, "write"),
    }
  }
}

impl std::fmt::Display for ReadPermission {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      ReadPermission::Read => write!(f, "read"),
    }
  }
}

impl std::fmt::Display for WritePermission {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      WritePermission::Write => write!(f, "write"),
    }
  }
}
