use crate::common::RenameRule;
use serde::Serialize;
use std::fmt::Debug;

#[derive(Clone, Debug, Serialize)]
pub struct EnumField {
  pub name: String,
  pub rename: Option<String>,
  /// type name
  /// String(String) / Data(Data)
  pub type_name: Option<String>,
  pub description: Option<String>,
  pub reference: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct Enum {
  pub name: String,
  pub description: Option<String>,
  pub fields: Vec<EnumField>,
  pub untagged: bool,
  tags: Vec<String>,
  impl_to_string: bool,
}

impl Enum {
  pub fn new(name: &str) -> Self {
    Self {
      name: RenameRule::VariantName.apply(name),
      fields: vec![],
      description: None,
      untagged: false,
      impl_to_string: false,
      tags: vec!["full".to_string()],
    }
  }

  pub fn add_field(&mut self, field: EnumField) {
    self.fields.push(field);

    self.impl_to_string =
      !self.fields.is_empty() && self.fields.iter().all(|f| f.type_name.is_none());
  }

  pub fn set_description(&mut self, description: &str) {
    self.description = Some(description.to_owned());
  }

  pub fn set_name(&mut self, name: &str) {
    self.name = RenameRule::VariantName.apply(name);
  }

  pub fn add_tag(&mut self, tag: &String) {
    if !self.tags.contains(tag) {
      self.tags.push(tag.clone());
    }
  }

  pub fn untagged(&mut self) {
    self.untagged = true;
  }
}

impl EnumField {
  pub fn new(name: &String) -> Self {
    let normalized_name = RenameRule::VariantName.apply(name);

    if normalized_name != *name {
      Self {
        name: normalized_name,
        rename: Some(name.clone()),
        type_name: None,
        description: None,
        reference: None,
      }
    } else {
      Self {
        name: normalized_name,
        rename: None,
        type_name: None,
        description: None,
        reference: None,
      }
    }
  }

  pub fn reference(&mut self, reference: &str) {
    self.reference = Some(reference.to_owned());
  }

  pub fn set_description(&mut self, description: &str) {
    self.description = Some(description.to_owned());
  }

  pub fn set_type_name(&mut self, type_name: &str) {
    self.type_name = Some(type_name.to_owned());
  }
}
