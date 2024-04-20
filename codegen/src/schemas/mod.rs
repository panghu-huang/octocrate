pub mod api;
pub mod body_parameters;
pub mod external_docs;
pub mod parameters;
pub mod refs;
pub mod schema;
pub mod tag;
pub mod webhook;

use refs::Reference;
use schema::SchemaDefinition;
use serde::Deserialize;
use std::collections::HashMap;

pub type Path = String;

#[derive(Deserialize, Clone, Debug)]
pub struct ApplicationJSONContent {
  pub schema: SchemaDefinition,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Content {
  #[serde(rename = "application/json")]
  pub application_json: Option<ApplicationJSONContent>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct RequestBody {
  pub required: bool,
  pub content: Content,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum ContentOrRef {
  Content { content: Option<Content> },
  Ref(Reference),
}

#[derive(Deserialize, Clone, Debug)]
pub struct Paths {
  pub get: Option<api::API>,
  pub post: Option<api::API>,
  pub put: Option<api::API>,
  pub patch: Option<api::API>,
  pub delete: Option<api::API>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Webhooks {
  pub post: Option<webhook::Webhook>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Components {
  pub parameters: HashMap<String, parameters::Parameter>,
  pub schemas: HashMap<String, schema::Schema>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct APIDescription {
  pub paths: HashMap<Path, Paths>,
  pub webhooks: HashMap<Path, Webhooks>,
  pub components: Components,
  pub tags: Vec<tag::Tag>,
  #[serde(rename = "externalDocs")]
  pub external_docs: external_docs::ExternalDocs,
}

impl APIDescription {
  pub fn try_load() -> Result<Self, String> {
    let json = include_str!("../../resources/api.json");

    let api_description = APIDescription::try_from(&json.to_string())?;

    Ok(api_description)
  }
}

impl TryFrom<&String> for APIDescription {
  type Error = String;

  fn try_from(value: &String) -> Result<Self, Self::Error> {
    serde_json::from_str(value).map_err(|e| e.to_string())
  }
}

impl Default for APIDescription {
  fn default() -> Self {
    Self {
      paths: HashMap::new(),
      webhooks: HashMap::new(),
      components: Components {
        parameters: HashMap::new(),
        schemas: HashMap::new(),
      },
      tags: Vec::new(),
      external_docs: external_docs::ExternalDocs {
        description: None,
        url: "".to_string(),
      },
    }
  }
}

impl IntoIterator for &Paths {
  type Item = (String, api::API);
  type IntoIter = std::vec::IntoIter<Self::Item>;

  fn into_iter(self) -> Self::IntoIter {
    let mut vec = Vec::new();

    if let Some(api) = &self.get {
      vec.push(("get".to_string(), api.clone()));
    }

    if let Some(api) = &self.post {
      vec.push(("post".to_string(), api.clone()));
    }

    if let Some(api) = &self.put {
      vec.push(("put".to_string(), api.clone()));
    }

    if let Some(api) = &self.patch {
      vec.push(("patch".to_string(), api.clone()));
    }

    if let Some(api) = &self.delete {
      vec.push(("delete".to_string(), api.clone()));
    }

    vec.into_iter()
  }
}

impl From<ContentOrRef> for Option<SchemaDefinition> {
  fn from(val: ContentOrRef) -> Self {
    match val {
      ContentOrRef::Content { content } => match content {
        Some(content) => content.application_json.map(|json| json.schema),
        None => None,
      },
      ContentOrRef::Ref(reference) => Some(SchemaDefinition::Ref(reference)),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_api_description() {
    let json = include_str!("../../resources/api.json");

    let api_description = APIDescription::try_from(&json.to_string()).unwrap();

    // println!("{:?}", api_description.external_docs);

    let _api = api_description
      .paths
      .get("/repos/{owner}/{repo}/contents/{path}")
      .unwrap()
      .put
      .clone()
      .unwrap();

    // println!("{:#?}", api);
  }
}
