use crate::structures::{enums::Enum, structs::Struct, types::Type};

pub mod api;
pub mod context;
pub mod parameter_parser;
pub mod parameters_parser;
pub mod schema_parser;

#[derive(Clone, Debug)]
pub enum ParsedData {
  Struct(Struct),
  Enum(Enum),
  Type(Type),
}

impl ParsedData {
  pub fn name(&self) -> String {
    match self {
      ParsedData::Struct(s) => s.name.clone(),
      ParsedData::Enum(e) => e.name.clone(),
      ParsedData::Type(t) => t.alias.clone().unwrap_or(t.type_name.clone()),
    }
  }

  pub fn add_tag(&mut self, tag: &String) {
    match self {
      ParsedData::Struct(s) => s.add_tag(tag),
      ParsedData::Enum(e) => e.add_tag(tag),
      ParsedData::Type(t) => t.add_tag(tag),
    }
  }
}
