use super::{
  external_docs::ExternalDocs, parameters::ParameterDefinition, schema::SchemaDefinition,
  ContentOrRef,
};
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct Responses {
  #[serde(rename = "200")]
  pub success: Option<ContentOrRef>,
  #[serde(rename = "201")]
  pub created: Option<ContentOrRef>,
  #[serde(rename = "202")]
  pub accepted: Option<ContentOrRef>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct API {
  pub summary: String,
  pub description: String,
  pub tags: Vec<String>,
  #[serde(rename = "operationId")]
  pub operation_id: String,
  pub responses: Responses,
  pub parameters: Option<Vec<ParameterDefinition>>,
  #[serde(rename = "requestBody")]
  pub request_body: Option<ContentOrRef>,
  #[serde(rename = "externalDocs")]
  pub external_docs: ExternalDocs,
}

impl IntoIterator for Responses {
  type Item = (String, Option<SchemaDefinition>);
  type IntoIter = std::vec::IntoIter<Self::Item>;

  fn into_iter(self) -> Self::IntoIter {
    vec![
      (
        "success".to_string(),
        self.success.and_then(|content| content.into()),
      ),
      (
        "created".to_string(),
        self.created.and_then(|content| content.into()),
      ),
      (
        "accepted".to_string(),
        self.accepted.and_then(|content| content.into()),
      ),
    ]
    .into_iter()
  }
}
