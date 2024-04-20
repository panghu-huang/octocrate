use crate::common::RenameRule;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct Type {
  pub type_name: String,
  pub alias: Option<String>,
  pub reference: Option<String>,
  pub description: Option<String>,
  tags: Vec<String>,
}

impl Type {
  pub fn new_with_reference(type_name: &String, reference: &String) -> Self {
    Type {
      type_name: type_name.clone(),
      alias: None,
      reference: Some(reference.clone()),
      description: None,
      tags: vec!["full".to_string()],
    }
  }

  pub fn new(type_name: &String) -> Self {
    Type {
      type_name: type_name.clone(),
      alias: None,
      reference: None,
      description: None,
      tags: vec!["full".to_string()],
    }
  }

  pub fn set_reference(&mut self, reference: &String) {
    self.reference = Some(reference.clone());
  }

  pub fn set_description(&mut self, description: &String) {
    self.description = Some(description.clone());
  }

  pub fn set_alias(&mut self, alias: &String) {
    self.alias = Some(RenameRule::VariantName.apply(alias));
  }

  pub fn add_tag(&mut self, tag: &String) {
    if !self.tags.contains(tag) {
      self.tags.push(tag.clone());
    }
  }
}
