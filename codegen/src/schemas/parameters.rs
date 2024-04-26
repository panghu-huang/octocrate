use super::{refs::Reference, schema::Schema};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Position {
  Query,
  Path,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Parameter {
  pub name: String,
  pub description: Option<String>,
  #[serde(rename = "in")]
  pub position: Position,
  pub required: Option<bool>,
  pub schema: Schema,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum ParameterDefinition {
  Ref(Reference),
  Parameter(Parameter),
}

impl Position {
  pub fn is_query(&self) -> bool {
    matches!(self, Position::Query)
  }
}
