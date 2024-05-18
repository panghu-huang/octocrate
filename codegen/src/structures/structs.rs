use crate::common::RenameRule;
use serde::Serialize;
use std::fmt::Debug;

#[derive(Clone, Debug, Serialize)]
pub struct StructField {
  pub name: String,
  pub rename: Option<String>,
  pub description: Option<String>,
  pub type_name: String,
  pub reference: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct Struct {
  pub name: String,
  pub description: Option<String>,
  pub fields: Vec<StructField>,
  tags: Vec<String>,
}

impl Struct {
  pub fn new_with_description(name: &str, description: &str) -> Self {
    Self {
      name: RenameRule::VariantName.apply(name),
      description: Some(description.to_owned()),
      fields: vec![],
      tags: vec!["full".to_string()],
    }
  }

  pub fn new(name: String) -> Self {
    Self {
      name: RenameRule::VariantName.apply(&name),
      description: None,
      fields: vec![],
      tags: vec!["full".to_string()],
    }
  }

  pub fn add_tag(&mut self, tag: &String) {
    if !self.has_tag(tag) {
      self.tags.push(tag.clone());
    }
  }

  pub fn has_tag(&self, tag: &String) -> bool {
    self.tags.contains(tag)
  }

  pub fn add_field(&mut self, field: StructField) {
    self.fields.push(field);
  }

  pub fn set_description(&mut self, description: &str) {
    self.description = Some(description.to_owned());
  }

  pub fn set_name(&mut self, name: &str) {
    self.name = RenameRule::VariantName.apply(name);
  }
}

impl StructField {
  // pub fn new_with_description(name: &String, type_name: &String, description: &String) -> Self {
  //   let normalized_name = RenameRule::FieldName.apply(&name);

  //   let rename = if &normalized_name != name {
  //     Some(name.clone())
  //   } else {
  //     None
  //   };

  //   Self {
  //     name: normalized_name,
  //     rename,
  //     description: Some(description.clone()),
  //     type_name: type_name.clone(),
  //     reference: None,
  //   }
  // }

  pub fn new(name: &String, type_name: &str) -> Self {
    let normalized_name = RenameRule::FieldName.apply(name);

    let rename = if &normalized_name != name {
      Some(name.clone())
    } else {
      None
    };

    Self {
      name: normalized_name,
      rename,
      description: None,
      type_name: type_name.to_owned(),
      reference: None,
    }
  }

  pub fn set_description(&mut self, description: &str) {
    self.description = Some(description.to_owned());
  }

  pub fn reference(&mut self, reference: &str) {
    self.reference = Some(reference.to_owned());
  }
}
