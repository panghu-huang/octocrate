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
  pub fn new_with_reference(type_name: &str, reference: &str) -> Self {
    Type {
      type_name: type_name.to_owned(),
      alias: None,
      reference: Some(reference.to_owned()),
      description: None,
      tags: vec!["full".to_string()],
    }
  }

  pub fn new(type_name: &str) -> Self {
    Type {
      type_name: type_name.to_owned(),
      alias: None,
      reference: None,
      description: None,
      tags: vec!["full".to_string()],
    }
  }

  pub fn set_reference(&mut self, reference: &str) {
    self.reference = Some(reference.to_owned());
  }

  pub fn set_description(&mut self, description: &str) {
    self.description = Some(description.to_owned());
  }

  pub fn set_alias(&mut self, alias: &str) {
    self.alias = Some(RenameRule::VariantName.apply(alias));
  }

  pub fn add_tag(&mut self, tag: &String) {
    if !self.has_tag(tag) {
      self.tags.push(tag.clone());
    }
  }

  pub fn has_tag(&self, tag: &String) -> bool {
    self.tags.contains(tag)
  }
}
