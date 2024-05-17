use crate::{
  parser::{api::API, ParsedData},
  schemas::{parameters::Parameter, schema::Schema, APIDescription},
};
use indexmap::IndexMap;
use indicatif::{ProgressBar, ProgressStyle};
use std::{borrow::BorrowMut, sync::Mutex};

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
  // pub stage: Stage,
  pub inner: ParsedData,
}

pub type References = IndexMap<IdentifierOrName, TypeReference>;

struct ParseContextInner {
  references: References,
  scoped_references: References,
  // webhook types and api types can't share certain types, such as Repository / Committer and so on.
  webhook_references: References,
  apis: IndexMap<APITag, Vec<API>>,
}

pub struct ParseContext {
  stage: Stage,
  working_tag: Option<String>,
  tags: IndexMap<String, String>,
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
      tags: IndexMap::new(),
      api_description,
      progress_bar,
      inner: Mutex::new(ParseContextInner {
        scoped_references: IndexMap::new(),
        references: IndexMap::new(),
        webhook_references: IndexMap::new(),
        apis: IndexMap::new(),
      }),
    }
  }

  pub fn add_scoped_reference(&self, id: &str, inner: ParsedData) {
    let mut guard = self.inner.lock().expect("Failed to lock the inner context");

    let references = guard.scoped_references.borrow_mut();

    references.insert(
      id.to_owned(),
      TypeReference {
        name: id.to_owned(),
        inner,
      },
    );
  }

  pub fn add_reference(&self, id: &str, inner: ParsedData) {
    let mut guard = self.inner.lock().expect("Failed to lock the inner context");

    let references = if self.stage == Stage::ParsingAPI {
      guard.references.borrow_mut()
    } else {
      guard.webhook_references.borrow_mut()
    };

    if references.get(id).is_some() {
      drop(guard);

      self.reference_existing(id);
    } else {
      let mut inner = inner.clone();

      if let Some(tag) = &self.working_tag {
        inner.add_tag(tag);
      }

      references.insert(
        id.to_owned(),
        TypeReference {
          name: id.to_owned(),
          // stage: self.stage.clone(),
          inner,
        },
      );
    }
  }

  pub fn get_scoped_references(&self) -> IndexMap<IdentifierOrName, TypeReference> {
    self
      .inner
      .lock()
      .expect("Failed to lock the inner context")
      .scoped_references
      .clone()
  }

  pub fn clear_scoped_references(&self) {
    let mut guard = self.inner.lock().expect("Failed to lock the inner context");

    guard.scoped_references.clear();
  }

  pub fn reference_existing(&self, name: &str) -> Option<TypeReference> {
    let mut guard = self.inner.lock().expect("Failed to lock the inner context");
    let reference = if self.stage == Stage::ParsingAPI {
      guard.references.get_mut(name)
    } else {
      guard.webhook_references.get_mut(name)
    };

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
                self.reference_existing(field_reference);
              }
            }
          }
          ParsedData::Type(type_) => {
            if let Some(field_reference) = &type_.reference {
              self.reference_existing(field_reference);
            }
          }
          ParsedData::Struct(struct_) => {
            for field in &struct_.fields {
              if let Some(field_reference) = &field.reference {
                self.reference_existing(field_reference);
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

  pub fn set_working_tag(&mut self, tag: &str) {
    self.working_tag = Some(tag.replace('-', "_"));
  }

  pub fn add_tag_description(&mut self, tag: &str, description: &str) {
    self.tags.insert(tag.to_owned(), description.to_owned());
  }

  pub fn get_apis(&self) -> IndexMap<APITag, Vec<API>> {
    self
      .inner
      .lock()
      .expect("Failed to lock the inner context")
      .apis
      .clone()
  }

  pub fn get_tags(&self) -> IndexMap<String, String> {
    self.tags.clone()
  }

  pub fn start_parsing_webhook(&mut self, webhook: &str) {
    if self.stage == Stage::ParsingAPI {
      self.stage = Stage::ParsingWebhook;
      self.working_tag = Some("webhooks".to_string());
    }

    self
      .progress_bar
      .set_message(format!("Parsing webhook {} ...", webhook));

    self.progress_bar.inc(1);
  }

  pub fn start_parsing_api(&mut self, api: &str) {
    if self.stage == Stage::ParsingWebhook {
      panic!("Cannot start parsing API while parsing webhook");
    }

    self
      .progress_bar
      .set_message(format!("Parsing API {} ...", api));

    self.progress_bar.inc(1);

    // Clear the scoped references when starting to parse a new API
    self.clear_scoped_references();
  }

  pub fn finish_parsing(&mut self) {
    self.progress_bar.finish_with_message("✅ Finished parsing");
  }

  pub fn get_references(&self) -> IndexMap<IdentifierOrName, ParsedData> {
    let guard = self.inner.lock().expect("Failed to lock the inner context");

    guard
      .references
      .iter()
      .map(|(name, reference)| (name.clone(), reference.inner.clone()))
      .collect()
  }

  pub fn get_webhook_references(&self) -> IndexMap<IdentifierOrName, ParsedData> {
    let guard = self.inner.lock().expect("Failed to lock the inner context");

    let webhooks: IndexMap<IdentifierOrName, ParsedData> = guard
      .webhook_references
      .iter()
      .map(|(name, reference)| (name.clone(), reference.inner.clone()))
      .collect();

    webhooks
  }

  pub fn get_component(&self, name: &str) -> Option<Schema> {
    self.api_description.components.schemas.get(name).cloned()
  }

  pub fn get_parameter(&self, name: &str) -> Option<Parameter> {
    self
      .api_description
      .components
      .parameters
      .get(name)
      .cloned()
  }
}

impl Default for ParseContext {
  fn default() -> Self {
    Self {
      stage: Stage::ParsingAPI,
      working_tag: None,
      progress_bar: ProgressBar::new_spinner(),
      tags: IndexMap::new(),
      api_description: APIDescription::default(),
      inner: Mutex::new(ParseContextInner {
        scoped_references: IndexMap::new(),
        references: IndexMap::new(),
        webhook_references: IndexMap::new(),
        apis: IndexMap::new(),
      }),
    }
  }
}
