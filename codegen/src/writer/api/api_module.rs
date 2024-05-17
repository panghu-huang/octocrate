use super::super::{File, References};
use crate::{
  common::{render_template, RenameRule},
  writer::format_code,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct Parameter {
  pub name: String,
  pub type_name: String,
  pub rename: Option<String>,
}

#[derive(Serialize)]
pub struct APIFunction {
  pub function_name: String,
  pub url: String,
  pub summary: String,
  pub description: String,
  pub document_url: String,
  pub method: String,
  pub parameters: Vec<Parameter>,
  pub body_type: String,
  pub query_type: String,
  pub response_type: Option<String>,
  pub stringify_params: Vec<String>,
  pub references: Option<References>,
}

#[derive(Serialize)]
pub struct APIModule {
  file_name: String,
  module_name: String,
  description: Option<String>,
  functions: Vec<APIFunction>,
}

impl APIModule {
  pub fn new(module_name: &str, description: Option<String>) -> APIModule {
    APIModule {
      module_name: RenameRule::VariantName.apply(module_name),
      file_name: RenameRule::FieldName.apply(module_name),
      description,
      functions: Vec::new(),
    }
  }

  pub fn add_function(&mut self, function: APIFunction) {
    self.functions.push(function);
  }
}

impl File for APIModule {
  fn file_name(&self) -> String {
    format!("{}.rs", self.file_name)
  }

  fn write(&self, path: &std::path::Path) {
    let template = include_str!("../../../templates/api/api.hbs");

    let rendered = format_code(render_template(template, self));

    let file_path = path.join(self.file_name());

    std::fs::write(file_path, rendered).expect("Unable to write file");
  }
}
