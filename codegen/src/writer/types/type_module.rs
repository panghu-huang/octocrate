use super::super::{File, References};
use crate::{
  common::{render_template, RenameRule},
  parser::ParsedData,
  writer::format_code,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct TypeModule {
  name: String,
  file_name: String,
  references: References,
}

impl TypeModule {
  pub fn new(name: impl Into<String>) -> TypeModule {
    let name = &name.into();
    TypeModule {
      name: RenameRule::VariantName.apply(name),
      file_name: RenameRule::FieldName.apply(name),
      references: References {
        render_features: true,
        ..References::default()
      },
    }
  }

  pub fn add_type(&mut self, parsed: &ParsedData) {
    self.references.add_reference(parsed);
  }
}

impl File for TypeModule {
  fn file_name(&self) -> String {
    format!("{}.rs", self.file_name)
  }

  fn write(&self, path: &std::path::Path) {
    let template = include_str!("../../../templates/types/type_module.hbs");

    let rendered = format_code(render_template(template, self));

    let file_path = path.join(self.file_name());

    std::fs::write(file_path, rendered).expect("Unable to write file");
  }
}
