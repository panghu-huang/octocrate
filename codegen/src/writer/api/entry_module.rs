use super::super::File;
use crate::{
  common::{render_template, RenameRule},
  writer::format_code,
};
use serde::Serialize;

#[derive(Serialize)]
struct APIModule {
  module_name: String,
  file_name: String,
}

#[derive(Serialize)]
pub struct APIEntryModule {
  modules: Vec<APIModule>,
}

impl APIEntryModule {
  pub fn new() -> APIEntryModule {
    APIEntryModule {
      modules: Vec::new(),
    }
  }

  pub fn add_module(&mut self, module_name: &String) {
    self.modules.push(APIModule {
      module_name: RenameRule::VariantName.apply(module_name),
      file_name: RenameRule::FieldName.apply(module_name),
    });
  }
}

impl File for APIEntryModule {
  fn file_name(&self) -> String {
    "mod.rs".to_string()
  }

  fn write(&self, path: &std::path::PathBuf) {
    let template = include_str!("../../../templates/api/entry_module.hbs");

    let rendered = format_code(render_template(template, self));

    let file_path = path.join(self.file_name());

    std::fs::write(file_path, rendered).expect("Unable to write file");
  }
}
