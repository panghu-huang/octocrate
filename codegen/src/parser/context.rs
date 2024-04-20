use crate::{
  parser::{api::API, ParsedData},
  schemas::{parameters::Parameter, schema::Schema, APIDescription},
};
use indicatif::{ProgressBar, ProgressStyle};
use std::{collections::HashMap, sync::Mutex};

pub type IdentifierOrName = String;

pub type APITag = String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stage {
  ParsingAPI,
  ParsingWebhook,
}

#[derive(Debug, Clone)]
pub struct TypeReference {
  pub name: String,
  pub stage: Stage,
  pub inner: ParsedData,
}

struct ParseContextInner {
  references: HashMap<IdentifierOrName, TypeReference>,
  apis: HashMap<APITag, Vec<API>>,
}

pub struct ParseContext {
  stage: Stage,
  working_tag: Option<String>,
  tags: HashMap<String, String>,
  api_description: APIDescription,
  inner: Mutex<ParseContextInner>,
  progress_bar: ProgressBar,
}

impl ParseContext {
  pub fn new(api_description: APIDescription) -> Self {
    let mut total = api_description.webhooks.len() as u64;
    for path in api_description.paths.values() {
      for _ in path {
        total += 1;
      }
    }

    let spinner_style =
      ProgressStyle::with_template("[{elapsed_precise}] {spinner:.green} {pos}/{len} {msg:.cyan}")
        .unwrap();

    let progress_bar = ProgressBar::new(total).with_style(spinner_style);

    Self {
      stage: Stage::ParsingAPI,
      working_tag: None,
      tags: HashMap::new(),
      api_description,
      progress_bar,
      inner: Mutex::new(ParseContextInner {
        references: HashMap::new(),
        apis: HashMap::new(),
      }),
    }
  }

  #[allow(dead_code)]
  pub fn default() -> Self {
    Self {
      stage: Stage::ParsingAPI,
      working_tag: None,
      progress_bar: ProgressBar::new_spinner(),
      tags: HashMap::new(),
      api_description: APIDescription::default(),
      inner: Mutex::new(ParseContextInner {
        references: HashMap::new(),
        apis: HashMap::new(),
      }),
    }
  }

  pub fn add_reference(&self, id: &IdentifierOrName, inner: ParsedData) {
    let mut guard = self.inner.lock().expect("Failed to lock the inner context");

    if guard.references.get(id).is_some() {
      drop(guard);

      self.reference_existing(id);
    } else {
      let mut inner = inner.clone();

      if let Some(tag) = &self.working_tag {
        inner.add_tag(tag);
      }

      guard.references.insert(
        id.clone(),
        TypeReference {
          name: id.clone(),
          stage: self.stage.clone(),
          inner,
        },
      );
    }
  }

  pub fn reference_existing(&self, name: &String) -> Option<TypeReference> {
    let mut guard = self.inner.lock().expect("Failed to lock the inner context");
    let reference = guard.references.get_mut(name);

    if self.working_tag.is_none() {
      return reference.cloned();
    }

    match reference {
      Some(reference) => {
        reference.inner.add_tag(self.working_tag.as_ref().unwrap());

        let reference = reference.clone();

        drop(guard);

        match &reference.inner {
          ParsedData::Enum(enum_) => {
            for field in &enum_.fields {
              if let Some(field_reference) = &field.reference {
                self.reference_existing(&field_reference);
              }
            }
          }
          ParsedData::Type(type_) => {
            if let Some(field_reference) = &type_.reference {
              self.reference_existing(&field_reference);
            }
          }
          ParsedData::Struct(struct_) => {
            for field in &struct_.fields {
              if let Some(field_reference) = &field.reference {
                self.reference_existing(&field_reference);
              }
            }
          }
        }

        Some(reference)
      }
      None => None,
    }
  }

  pub fn add_api(&self, api: API) {
    let tag = self.working_tag.clone().expect("No working tag set");

    let mut guard = self.inner.lock().expect("Failed to lock the inner context");
    let tag = tag.clone();

    if let Some(apis) = guard.apis.get_mut(&tag) {
      apis.push(api);
    } else {
      guard.apis.insert(tag, vec![api]);
    }
  }

  pub fn set_working_tag(&mut self, tag: &String) {
    self.working_tag = Some(tag.replace("-", "_"));
  }

  pub fn add_tag_description(&mut self, tag: &String, description: &String) {
    self.tags.insert(tag.clone(), description.clone());
  }

  pub fn get_apis(&self) -> HashMap<APITag, Vec<API>> {
    self
      .inner
      .lock()
      .expect("Failed to lock the inner context")
      .apis
      .clone()
  }

  pub fn get_tags(&self) -> HashMap<String, String> {
    self.tags.clone()
  }

  pub fn start_parsing_webhook(&mut self, webhook: &String) {
    if self.stage == Stage::ParsingAPI {
      self.stage = Stage::ParsingWebhook;
      self.working_tag = Some("webhook".to_string());
    }

    self
      .progress_bar
      .set_message(format!("Parsing webhook {} ...", webhook));

    self.progress_bar.inc(1);
  }

  pub fn start_parsing_api(&mut self, api: &String) {
    if self.stage == Stage::ParsingWebhook {
      panic!("Cannot start parsing API while parsing webhook");
    }

    self
      .progress_bar
      .set_message(format!("Parsing API {} ...", api));

    self.progress_bar.inc(1);
  }

  pub fn finish_parsing(&mut self) {
    self.progress_bar.finish_with_message("✅ Finished parsing");
  }

  pub fn get_references(&self) -> HashMap<IdentifierOrName, ParsedData> {
    self
      .inner
      .lock()
      .expect("Failed to lock the inner context")
      .references
      .iter()
      .filter(|(_, reference)| reference.stage == Stage::ParsingAPI)
      .map(|(name, reference)| (name.clone(), reference.inner.clone()))
      .collect()
  }

  pub fn get_webhooks(&self) -> HashMap<IdentifierOrName, ParsedData> {
    self
      .inner
      .lock()
      .expect("Failed to lock the inner context")
      .references
      .iter()
      .filter(|(_, reference)| reference.stage == Stage::ParsingWebhook)
      .map(|(name, reference)| (name.clone(), reference.inner.clone()))
      .collect()
  }

  pub fn get_component(&self, name: &String) -> Option<Schema> {
    self
      .api_description
      .components
      .schemas
      .get(name)
      .map(|component| component.clone())
  }

  pub fn get_parameter(&self, name: &String) -> Option<Parameter> {
    self
      .api_description
      .components
      .parameters
      .get(name)
      .map(|parameter| parameter.clone())
  }
}
