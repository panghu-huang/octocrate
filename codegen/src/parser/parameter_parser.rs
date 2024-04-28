use super::{context::ParseContext, schema_parser::SchemaParser, ParsedData};
use crate::{
  schemas::{parameters::Parameter, schema::SchemaDefinition},
  structures::types::Type,
};

#[derive(Debug, Clone)]
pub struct ParameterParser<'a> {
  pub parameter: &'a Parameter,
}

impl<'a> ParameterParser<'a> {
  pub fn new(parameter: &'a Parameter) -> Self {
    Self { parameter }
  }

  pub fn parse(&self, ctx: &mut ParseContext, name: &String) -> ParsedData {
    let mut schema_parser = SchemaParser::new();
    let parameter = &self.parameter;
    let is_required = parameter.required.unwrap_or(false);

    let generated_struct = schema_parser.parse(
      ctx,
      name,
      &SchemaDefinition::Schema(parameter.schema.clone()),
    );

    match &generated_struct {
      ParsedData::Type(type_) => {
        let mut type_ = type_.clone();

        if let Some(description) = &parameter.description {
          type_.set_description(description);
        }

        if !is_required && !type_.type_name.starts_with("Option<") {
          type_.type_name = format!("Option<{}>", type_.type_name);
        };

        ParsedData::Type(type_)
      }
      ParsedData::Enum(enum_) => {
        let mut enum_ = enum_.clone();

        if let Some(description) = &parameter.description {
          enum_.set_description(description);
        }

        if !is_required {
          return ParsedData::Type(Type::new_with_reference(
            &format!("Option<{}>", enum_.name),
            &enum_.name,
          ));
        };

        ParsedData::Enum(enum_)
      }
      ParsedData::Struct(struct_) => {
        let mut struct_ = struct_.clone();

        if let Some(description) = &parameter.description {
          struct_.set_description(description);
        }

        if !is_required {
          return ParsedData::Type(Type::new_with_reference(
            &format!("Option<{}>", struct_.name),
            &struct_.name,
          ));
        };

        ParsedData::Struct(struct_)
      }
    }
  }
}
