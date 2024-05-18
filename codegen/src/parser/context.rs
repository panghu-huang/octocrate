use crate::{
  parser::{api::API, ParsedData},
  schemas::{parameters::Parameter, schema::Schema, APIDescription},
};
use indexmap::IndexMap;
use indicatif::{ProgressBar, ProgressStyle};
use std::borrow::BorrowMut;

pub type ReferenceName = String;

pub type APITag = String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stage {
  ParsingAPI,
  ParsingWebhook,
}

#[derive(Debug, Clone)]
pub struct TypeReference {
  pub name: String,
  pub inner: ParsedData,
}

// When the type is globally referenced, there are two pieces of information, ID and Name, for the sake of uniformity, both are saved as Name
// The mapping between ID and Name is saved in id_name_map
pub type References = IndexMap<ReferenceName, TypeReference>;

struct ParseContextState {
  id_name_map: IndexMap<String, String>,
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
  state: ParseContextState,
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
      state: ParseContextState {
        id_name_map: IndexMap::new(),
        scoped_references: IndexMap::new(),
        references: IndexMap::new(),
        webhook_references: IndexMap::new(),
        apis: IndexMap::new(),
      },
    }
  }

  pub fn add_scoped_reference(&mut self, id: &str, inner: ParsedData) {
    let references = self.state.scoped_references.borrow_mut();

    references.insert(
      id.to_owned(),
      TypeReference {
        name: inner.name(),
        inner,
      },
    );
  }

  // Because the Name and ID of global types are not necessarily similar, the key in maps is either ID or name
  // For example, { title: "Name", properties: {}, type: ["object", "null"] } will generate the type Option<Name>
  // But when saving this type, it is based on the ID
  pub fn add_reference(&mut self, id_or_name: &str, inner: ParsedData) {
    let references = if self.stage == Stage::ParsingAPI {
      self.state.references.borrow_mut()
    } else {
      self.state.webhook_references.borrow_mut()
    };

    let inenr_name = inner.name();

    if id_or_name != inenr_name {
      self
        .state
        .id_name_map
        .insert(id_or_name.to_owned(), inenr_name.to_owned());
    }

    if references.contains_key(&inenr_name) {
      self.reference_existing(id_or_name);
    } else {
      let mut inner = inner;

      if let Some(tag) = &self.working_tag {
        inner.add_tag(tag);
      }

      references.insert(
        inenr_name.clone(),
        TypeReference {
          name: inenr_name,
          inner,
        },
      );
    }
  }

  pub fn get_scoped_references(&self) -> IndexMap<ReferenceName, TypeReference> {
    self.state.scoped_references.clone()
  }

  pub fn clear_scoped_references(&mut self) {
    self.state.scoped_references.clear();
  }

  pub fn reference_existing(&mut self, id_or_name: &str) -> Option<TypeReference> {
    let references = if self.stage == Stage::ParsingAPI {
      self.state.references.borrow_mut()
    } else {
      self.state.webhook_references.borrow_mut()
    };

    let name = self
      .state
      .id_name_map
      .get(id_or_name)
      .cloned()
      .unwrap_or(id_or_name.to_string());

    let reference = references.get_mut(&name);

    let Some(tag) = &self.working_tag else {
      return reference.cloned();
    };

    match reference {
      Some(reference) => {
        if reference.inner.has_tag(tag) {
          return Some(reference.clone());
        }

        reference.inner.add_tag(tag);

        let reference = reference.clone();

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

  pub fn add_api(&mut self, api: API) {
    let tag = self.working_tag.clone().expect("No working tag set");

    let tag = tag.clone();

    if let Some(apis) = self.state.apis.get_mut(&tag) {
      apis.push(api);
    } else {
      self.state.apis.insert(tag, vec![api]);
    }
  }

  pub fn set_working_tag(&mut self, tag: &str) {
    self.working_tag = Some(tag.replace('-', "_"));
  }

  pub fn add_tag_description(&mut self, tag: &str, description: &str) {
    self.tags.insert(tag.to_owned(), description.to_owned());
  }

  pub fn get_apis(&self) -> IndexMap<APITag, Vec<API>> {
    self.state.apis.clone()
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

  pub fn get_references(&self) -> IndexMap<ReferenceName, ParsedData> {
    self
      .state
      .references
      .iter()
      .map(|(name, reference)| (name.clone(), reference.inner.clone()))
      .collect()
  }

  pub fn get_webhook_references(&self) -> IndexMap<ReferenceName, ParsedData> {
    self
      .state
      .webhook_references
      .iter()
      .map(|(name, reference)| (name.clone(), reference.inner.clone()))
      .collect()
  }

  pub fn get_component(&self, id: &str) -> Option<Schema> {
    self.api_description.components.schemas.get(id).cloned()
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
      state: ParseContextState {
        id_name_map: IndexMap::new(),
        scoped_references: IndexMap::new(),
        references: IndexMap::new(),
        webhook_references: IndexMap::new(),
        apis: IndexMap::new(),
      },
    }
  }
}
