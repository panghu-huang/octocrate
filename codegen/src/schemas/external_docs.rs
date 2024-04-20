use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct ExternalDocs {
  pub url: String,
  pub description: Option<String>,
}
