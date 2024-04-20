use super::super::File;
use crate::{
  common::{render_template, RenameRule},
  parser::ParsedData,
  structures::{enums::Enum, structs::Struct, types::Type},
};
use serde::Serialize;

#[derive(Serialize)]
pub struct TypeModule {
  name: String,
  file_name: String,
  enums: Vec<Enum>,
  structs: Vec<Struct>,
  types: Vec<Type>,
}

impl TypeModule {
  pub fn new(name: impl Into<String>) -> TypeModule {
    let name = &name.into();
    TypeModule {
      name: RenameRule::VariantName.apply(name),
      file_name: RenameRule::FieldName.apply(name),
      enums: Vec::new(),
      structs: Vec::new(),
      types: Vec::new(),
    }
  }

  pub fn add_type(&mut self, parsed: &ParsedData) {
    match parsed {
      ParsedData::Enum(e) => self.enums.push(e.clone()),
      ParsedData::Struct(s) => self.structs.push(s.clone()),
      ParsedData::Type(t) => self.types.push(t.clone()),
    }
  }
}

impl File for TypeModule {
  fn file_name(&self) -> String {
    format!("{}.rs", self.file_name)
  }

  fn write(&self, path: &std::path::PathBuf) {
    let template = include_str!("../../../templates/types/type_module.hbs");

    let rendered = render_template(template, self);

    let file_path = path.join(self.file_name());

    std::fs::write(file_path, rendered).expect("Unable to write file");
  }
}
