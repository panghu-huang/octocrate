use super::super::File;
use crate::{common::render_template, writer::format_code};
use serde::Serialize;

#[derive(Serialize)]
pub struct Module {
  name: String,
  exports_all: bool,
}

#[derive(Serialize)]
pub struct TypeEntryModule {
  name: String,
  modules: Vec<Module>,
}

impl TypeEntryModule {
  pub fn new(name: impl Into<String>) -> TypeEntryModule {
    TypeEntryModule {
      modules: Vec::new(),
      name: name.into(),
    }
  }

  pub fn add_module(&mut self, module_name: impl Into<String>) {
    self.modules.push(Module {
      name: module_name.into(),
      exports_all: true,
    });
  }

  pub fn add_module_with_exports(&mut self, module_name: impl Into<String>, exports_all: bool) {
    self.modules.push(Module {
      name: module_name.into(),
      exports_all,
    });
  }
}

impl File for TypeEntryModule {
  fn file_name(&self) -> String {
    self.name.clone()
  }

  fn write(&self, path: &std::path::Path) {
    let template = include_str!("../../../templates/types/entry_module.hbs");

    let rendered = format_code(render_template(template, self));

    let file_path = path.join(self.file_name());

    std::fs::write(file_path, rendered).expect("Unable to write file");
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_render() {
    let template = r#"
  {{#each modules}}
  {{#if exports_all}}
  mod {{ name }};

  pub use {{ name }}::*;
  {{else}}
  pub mod {{ name }};
  {{/if}}
  {{/each}}
    "#;

    let mut module = TypeEntryModule::new("mod.rs");

    module.add_module("test_module");
    module.add_module_with_exports("test_module_2", false);

    let rendered = render_template(template, &module);

    let expected = r#"
  mod test_module;

  pub use test_module::*;
  pub mod test_module_2;
    "#;
    assert_eq!(rendered, expected);
  }
}
