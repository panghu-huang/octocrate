mod enums;
mod items;
mod one_of_like;
mod properties;
mod schema_types;

use super::{context::ParseContext, ParsedData};
use crate::{
  common::RenameRule,
  schemas::schema::{Schema, SchemaDefinition},
  structures::types::Type,
};
use schema_types::SchemaTypes;

pub struct SchemaParser {
  // Use prefixs to record the path of the current schema, used to generate the name of the schema without a title
  // ```rust
  // struct A {
  //   // The name here is generated through prefixs, a_b -> A_B -> AB
  //   b: AB,
  // }
  //
  // struct AB {}
  // ```
  prefixs: Vec<String>,
  // Use is_scoped to mark whether the current schema is a type unique to an API (non-global)
  is_scoped: bool,
  // Use is_global_parameter to mark whether the current schema is a global parameter
  is_global_parameter: bool,
}

impl SchemaParser {
  pub fn new() -> Self {
    Self {
      prefixs: vec![],
      // Default is_scoped to true
      is_scoped: true,
      is_global_parameter: false,
    }
  }

  pub fn parse(
    &mut self,
    ctx: &mut ParseContext,
    prefix_name: &str,
    schema: &SchemaDefinition,
  ) -> ParsedData {
    self.prefixs.push(prefix_name.to_owned());

    let parsed = self.parse_schema_definition(ctx, schema);

    self.prefixs.pop();

    parsed
  }

  // Global parameter
  pub fn parse_global_parameter(
    &mut self,
    ctx: &mut ParseContext,
    prefix_name: &str,
    schema: &SchemaDefinition,
  ) -> ParsedData {
    self.is_global_parameter = true;

    let parsed = self.parse(ctx, prefix_name, schema);

    self.is_global_parameter = false;

    parsed
  }

  fn parse_schema_definition(
    &mut self,
    ctx: &mut ParseContext,
    schema: &SchemaDefinition,
  ) -> ParsedData {
    let last_is_scoped = self.is_scoped;

    match schema {
      SchemaDefinition::Schema(schema) => self.parse_schema(ctx, schema),
      SchemaDefinition::Ref(reference) => {
        // If the reference is a global reference, set is_scoped to false
        self.is_scoped = false;
        let reference_id = reference.get_reference_id();

        let parsed = match ctx.reference_existing(&reference_id) {
          Some(parsed) => parsed.inner,
          None => {
            let mut schema = ctx.get_schema_component(&reference_id).unwrap_or_else(|| {
              panic!("Failed to find reference schema with id: {}", reference_id)
            });

            let previous_prefixs = self.prefixs.clone();

            self.prefixs = vec![];

            if schema.title.is_some() {
              // Use the reference id as generated type name to avoid conflict
              // In the webhooks type, there are two data structures, RepositoryWebhooks and Repository, with the title being Repository
              schema.title = Some(RenameRule::VariantName.apply(&reference_id));
            }

            let parsed = self.parse(
              ctx,
              // Use the reference id as generated type name
              &reference_id,
              &SchemaDefinition::Schema(schema.clone()),
            );

            self.prefixs = previous_prefixs;

            self.add_reference(ctx, &reference_id, parsed.clone());

            parsed
          }
        };

        self.is_scoped = last_is_scoped;

        parsed
      }
    }
  }

  fn parse_schema(&mut self, ctx: &mut ParseContext, schema: &Schema) -> ParsedData {
    if schema.enum_.is_some() {
      return self.parse_enum(ctx, schema);
    }

    if schema.one_of.is_some() || schema.all_of.is_some() || schema.any_of.is_some() {
      return self.parse_one_of_like(ctx, schema);
    }

    if schema.items.is_some() {
      return self.parse_items(ctx, schema);
    }

    if schema.properties.is_some() {
      return self.parse_properties(ctx, schema);
    }

    let schema_types = if let Some(types) = &schema.type_ {
      SchemaTypes::from(types)
    } else {
      SchemaTypes::default()
    };

    let type_name = schema_types.to_full_type();

    ParsedData::Type(Type::new(&type_name))
  }

  fn add_reference(&self, ctx: &mut ParseContext, id_or_name: &str, reference: ParsedData) {
    if self.is_global_parameter {
      ctx.add_parameter(id_or_name, reference);
    } else if self.is_scoped {
      ctx.add_scoped_reference(id_or_name, reference);
    } else {
      ctx.add_reference(id_or_name, reference);
    }
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
