use super::super::File;
use crate::common::render_template;
use serde::Serialize;

#[derive(Serialize)]
pub struct TypeEntryModule {
  modules: Vec<String>,
}

impl TypeEntryModule {
  pub fn new() -> TypeEntryModule {
    TypeEntryModule {
      modules: Vec::new(),
    }
  }

  pub fn add_module(&mut self, module_name: impl Into<String>) {
    self.modules.push(module_name.into());
  }
}

impl File for TypeEntryModule {
  fn file_name(&self) -> String {
    "mod.rs".to_string()
  }

  fn write(&self, path: &std::path::PathBuf) {
    let template = include_str!("../../../templates/types/entry_module.hbs");

    let rendered = render_template(template, self);

    let file_path = path.join(self.file_name());

    std::fs::write(file_path, rendered).expect("Unable to write file");
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_render() {
    let template = include_str!("../../../templates/types/entry_module.hbs");

    let mut module = TypeEntryModule::new();

    module.add_module("test_module");

    let _rendered = render_template(template, &module);

    // println!("{}", rendered);
  }
}
