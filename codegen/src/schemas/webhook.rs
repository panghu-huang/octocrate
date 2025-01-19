use super::{external_docs::ExternalDocs, ContentOrRef};
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize, Clone, Debug)]
pub struct Webhook {
  pub summary: String,
  pub description: Option<String>,
  // pub tags: Vec<String>,
  #[serde(rename = "operationId")]
  pub operation_id: String,
  #[serde(rename = "requestBody")]
  pub request_body: Option<ContentOrRef>,
  #[serde(rename = "externalDocs")]
  pub external_docs: ExternalDocs,
}
