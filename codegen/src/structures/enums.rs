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
  pub tags: Vec<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct Enum {
  pub name: String,
  pub description: Option<String>,
  pub fields: Vec<EnumField>,
  pub untagged: bool,
  tags: Vec<String>,
  copiable: bool,
}

impl Enum {
  pub fn new(name: &str) -> Self {
    Self {
      name: RenameRule::VariantName.apply(name),
      fields: vec![],
      description: None,
      untagged: false,
      copiable: true,
      tags: vec!["full".to_string()],
    }
  }

  pub fn add_field(&mut self, field: EnumField) {
    if self.copiable && field.type_name.is_some() {
      self.copiable = false;
    }

    self.fields.push(field);
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
  pub fn new(name: &str) -> Self {
    let normalized_name = RenameRule::VariantName.apply(name);

    if normalized_name != *name {
      Self {
        name: normalized_name,
        rename: Some(name.to_owned()),
        type_name: None,
        description: None,
        reference: None,
        tags: vec![],
      }
    } else {
      Self {
        name: normalized_name,
        rename: None,
        type_name: None,
        description: None,
        reference: None,
        tags: vec![],
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

  pub fn add_tag(&mut self, tag: &String) {
    if !self.tags.contains(tag) {
      self.tags.push(tag.clone());
    }
  }
}
