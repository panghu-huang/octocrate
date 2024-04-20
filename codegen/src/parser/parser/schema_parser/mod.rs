mod enums;
mod items;
mod one_of_like;
mod properties;
mod schema_types;

use super::super::{context::ParseContext, ParsedData};
use crate::{
  schemas::schema::{Schema, SchemaDefinition},
  structures::types::Type,
};
use schema_types::SchemaTypes;

pub struct SchemaParser {
  prefixs: Vec<String>,
}

impl SchemaParser {
  pub fn new() -> Self {
    Self { prefixs: vec![] }
  }

  pub fn parse(
    &mut self,
    ctx: &mut ParseContext,
    prefix_name: &String,
    schema: &SchemaDefinition,
  ) -> ParsedData {
    self.prefixs.push(prefix_name.to_string());

    let parsed = self.parse_schema_definition(ctx, schema);

    self.prefixs.pop();

    parsed
  }

  fn parse_schema_definition(
    &mut self,
    ctx: &mut ParseContext,
    schema: &SchemaDefinition,
  ) -> ParsedData {
    match schema {
      SchemaDefinition::Schema(schema) => self.parse_schema(ctx, schema),
      SchemaDefinition::Ref(reference) => {
        let reference_id = reference.get_reference_id();

        match ctx.reference_existing(&reference_id) {
          Some(parsed) => parsed.inner,
          None => {
            let schema = ctx.get_component(&reference_id).expect(&format!(
              "Failed to find reference schema with id: {}",
              reference_id
            ));

            let previous_prefixs = self.prefixs.clone();

            self.prefixs = vec![];

            let title = schema
              .title
              .clone()
              .and_then(|title| {
                if title.contains("_") {
                  None
                } else {
                  Some(title)
                }
              })
              .unwrap_or(reference_id.clone());

            let parsed = self.parse(ctx, &title, &SchemaDefinition::Schema(schema.clone()));

            self.prefixs = previous_prefixs;

            ctx.add_reference(&parsed.name(), parsed.clone());

            parsed
          }
        }
      }
    }
  }

  fn parse_schema(&mut self, ctx: &mut ParseContext, schema: &Schema) -> ParsedData {
    if schema.enum_.is_some() {
      return self.parse_enum(ctx, &schema);
    }

    if schema.one_of.is_some() || schema.all_of.is_some() || schema.any_of.is_some() {
      return self.parse_one_of_like(ctx, schema);
    }

    if schema.items.is_some() {
      return self.parse_items(ctx, &schema);
    }

    if schema.properties.is_some() {
      return self.parse_properties(ctx, &schema);
    }

    let schema_types = if let Some(types) = &schema.type_ {
      SchemaTypes::from(types)
    } else {
      SchemaTypes::default()
    };

    let type_name = schema_types.to_full_type();

    ParsedData::Type(Type::new(&type_name))
  }
}

#[cfg(test)]
mod schema_parser_tests {
  use super::*;

  #[test]
  fn test_schema_parser_with_string() {
    let json = r#"{
      "type": "string"
    }"#;

    let schema: Schema = serde_json::from_str(json).unwrap();

    let mut parser = SchemaParser::new();

    let mut ctx = ParseContext::default();

    let generated = parser.parse(
      &mut ctx,
      &"Response".to_string(),
      &SchemaDefinition::Schema(schema),
    );

    if let ParsedData::Type(type_) = generated {
      assert_eq!(type_.type_name, "String");
      assert!(type_.reference.is_none());
    } else {
      panic!("Expected struct");
    }
  }
}
