use super::SchemaParser;
use crate::{
  parser::{context::ParseContext, ParsedData},
  schemas::schema::{Schema, StringOrBool},
  structures::{
    enums::{Enum, EnumField},
    types::Type,
  },
};

impl SchemaParser {
  pub(super) fn parse_enum(&mut self, ctx: &mut ParseContext, schema: &Schema) -> ParsedData {
    let enum_name = self.prefixs.join("_");
    let mut enum_ = Enum::new(&enum_name);
    let mut is_optional = false;

    if let Some(description) = &schema.description {
      enum_.set_description(description);
    }

    if let Some(enum_values) = &schema.enum_ {
      if let Some(enum_) = self.check_if_preset_enum(enum_values) {
        return enum_;
      }

      for enum_value in enum_values {
        if let Some(enum_value) = &enum_value {
          match enum_value {
            StringOrBool::String(value) => {
              let enum_field = EnumField::new(value);

              enum_.add_field(enum_field);
            }
            StringOrBool::Bool(_) => {
              let mut enum_field = EnumField::new("Boolean");

              enum_field.set_type_name("bool");

              enum_.add_field(enum_field);
            }
          }
        } else {
          is_optional = true;
        }
      }
    }

    self.add_reference(ctx, &enum_.name, ParsedData::Enum(enum_.clone()));

    if is_optional {
      return ParsedData::Type(Type::new_with_reference(
        &format!("Option<{}>", enum_.name),
        &enum_.name,
      ));
    }

    ParsedData::Enum(enum_)
  }

  fn check_if_preset_enum(&self, enums: &Vec<Option<StringOrBool>>) -> Option<ParsedData> {
    // if read write -> ReadWritePermission
    // if read -> ReadPermission
    // if write -> WritePermission
    let len = enums.len();
    if len <= 2 {
      let mut is_read = false;
      let mut is_write = false;

      for enum_value in enums {
        if let Some(StringOrBool::String(value)) = &enum_value {
          if value == "read" {
            is_read = true;
          } else if value == "write" {
            is_write = true;
          }
        }
      }

      match (len, is_read, is_write) {
        (1, true, false) => {
          return Some(ParsedData::Type(Type::new("ReadPermission")));
        }
        (1, false, true) => {
          return Some(ParsedData::Type(Type::new("WritePermission")));
        }
        (2, true, true) => {
          return Some(ParsedData::Type(Type::new("ReadWritePermission")));
        }
        _ => {}
      }
    }

    None
  }
}

#[cfg(test)]
mod schema_parser_enums_tests {
  use super::*;
  use crate::schemas::schema::SchemaDefinition;

  #[test]
  fn test_schema_parser_with_enum() {
    let json = r#"{
      "type": "object",
      "properties": {
        "state": {
          "type": "string",
          "enum": [
            "open",
            "closed",
            null
          ]
        }
      }
    }"#;

    let schema: Schema = serde_json::from_str(json).unwrap();

    let mut parser = SchemaParser::new();

    let mut ctx = ParseContext::default();

    let generated = parser.parse(
      &mut ctx,
      &"Response".to_string(),
      &SchemaDefinition::Schema(schema),
    );

    if let ParsedData::Struct(struct_) = generated {
      assert_eq!(struct_.name.to_string(), "Response");
      assert_eq!(struct_.fields.len(), 1);

      let field = &struct_.fields[0];
      assert_eq!(field.name, "state");
      assert_eq!(field.type_name, "Option<ResponseState>");

      let reference = &field.reference.clone().unwrap();

      assert_eq!(reference, "ResponseState");
    } else {
      panic!("Expected struct");
    }
  }
}
