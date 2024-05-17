use crate::structures::{enums::Enum, structs::Struct, types::Type};
use serde::Serialize;

#[derive(Serialize, Default)]
pub struct References {
  pub render_features: bool,
  pub enums: Vec<Enum>,
  pub structs: Vec<Struct>,
  pub types: Vec<Type>,
}

impl References {
  pub fn add_reference(&mut self, parsed: &crate::parser::ParsedData) {
    match parsed {
      crate::parser::ParsedData::Enum(e) => self.enums.push(e.clone()),
      crate::parser::ParsedData::Struct(s) => self.structs.push(s.clone()),
      crate::parser::ParsedData::Type(t) => {
        // Only add types that have an alias
        // having an alias --> pub type Alias = Type;
        if t.alias.is_some() {
          self.types.push(t.clone());
        }
      }
    }
  }
}
