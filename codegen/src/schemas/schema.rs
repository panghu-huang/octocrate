use super::refs::Reference;
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub enum SchemaType {
  Null,
  Integer,
  Number,
  Boolean,
  String,
  Object,
  Array,
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum SchemaTypeDefinition {
  String(String),
  Array(Vec<String>),
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum StringOrBool {
  String(String),
  Bool(bool),
}

#[derive(Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum SchemaDefinition {
  Ref(Reference),
  Schema(Schema),
}

#[derive(Deserialize, Clone, Debug)]
pub struct Schema {
  pub title: Option<String>,
  #[serde(rename = "type")]
  pub type_: Option<SchemaTypeDefinition>,
  // pub format: Option<String>,
  pub items: Option<Box<SchemaDefinition>>,
  pub properties: Option<BTreeMap<String, SchemaDefinition>>,
  pub required: Option<Vec<String>>,
  pub description: Option<String>,
  #[serde(rename = "enum")]
  pub enum_: Option<Vec<Option<StringOrBool>>>,
  /// enum no title
  #[serde(rename = "allOf")]
  pub all_of: Option<Vec<SchemaDefinition>>,
  /// struct with title or empty object
  #[serde(rename = "anyOf")]
  pub any_of: Option<Vec<SchemaDefinition>>,
  #[serde(rename = "oneOf")]
  pub one_of: Option<Vec<SchemaDefinition>>,
}

impl TryFrom<&String> for SchemaType {
  type Error = String;

  fn try_from(value: &String) -> Result<Self, Self::Error> {
    match value.as_str() {
      "null" => Ok(SchemaType::Null),
      "integer" => Ok(SchemaType::Integer),
      "number" => Ok(SchemaType::Number),
      "string" => Ok(SchemaType::String),
      "object" => Ok(SchemaType::Object),
      "array" => Ok(SchemaType::Array),
      "boolean" => Ok(SchemaType::Boolean),
      _ => Err(format!("Unknown schema type: {}", value)),
    }
  }
}

impl ToString for SchemaType {
  fn to_string(&self) -> String {
    match self {
      SchemaType::Null => "null".to_string(),
      SchemaType::Integer => "integer".to_string(),
      SchemaType::String => "string".to_string(),
      SchemaType::Object => "object".to_string(),
      SchemaType::Array => "array".to_string(),
      SchemaType::Boolean => "boolean".to_string(),
      SchemaType::Number => "number".to_string(),
    }
  }
}
