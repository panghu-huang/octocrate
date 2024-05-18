use super::{schema_types::SchemaTypes, SchemaParser};
use crate::{
  parser::{context::ParseContext, ParsedData},
  schemas::schema::{Schema, SchemaDefinition},
  structures::{
    structs::{Struct, StructField},
    types::Type,
  },
};

impl SchemaParser {
  pub(super) fn parse_properties(&mut self, ctx: &mut ParseContext, schema: &Schema) -> ParsedData {
    let struct_name;
    let previous_prefixs = self.prefixs.clone();

    if schema
      .title
      .as_ref()
      .is_some_and(|title| !title.contains('_'))
    {
      let title = schema.title.as_ref().unwrap();
      struct_name = title.to_string();
      self.prefixs = vec![struct_name.clone()];
    } else {
      struct_name = self.prefixs.join("_");
    }

    let struct_ = 'g: {
      if let Some(parsed) = ctx.reference_existing(&struct_name) {
        if let ParsedData::Struct(struct_) = parsed.inner {
          break 'g struct_;
        }
      }

      let mut struct_ = Struct::new(struct_name);

      let properties = schema.properties.as_ref().unwrap();

      for (name, schema) in properties {
        self.prefixs.push(name.clone());

        let parsed = self.parse_schema_definition(ctx, schema);

        self.prefixs.pop();

        let description = if let SchemaDefinition::Schema(schema) = &schema {
          schema.description.clone()
        } else {
          None
        };

        match parsed {
          ParsedData::Struct(generated) => {
            let mut struct_field = StructField::new(name, &generated.name);

            if let Some(description) = &description {
              struct_field.set_description(description);
            }

            struct_field.reference(&generated.name);

            struct_.add_field(struct_field);
          }
          ParsedData::Enum(generated) => {
            let mut struct_field = StructField::new(name, &generated.name);

            if let Some(description) = &description {
              struct_field.set_description(description);
            }

            struct_field.reference(&generated.name);

            struct_.add_field(struct_field);
          }
          ParsedData::Type(type_) => {
            let mut struct_field = StructField::new(name, &type_.type_name);

            if let Some(description) = &description {
              struct_field.set_description(description);
            }

            struct_field.reference = type_.reference;

            struct_.add_field(struct_field);
          }
        }
      }

      if let Some(description) = &schema.description {
        struct_.set_description(description);
      }

      let required = schema.required.clone().unwrap_or_default();

      for field in struct_.fields.iter_mut() {
        let field_name = field
          // If the field has a rename, use that instead of the field name
          .rename
          .clone()
          .unwrap_or_else(|| field.name.clone());

        if !required.contains(&field_name) && !field.type_name.starts_with("Option<") {
          field.type_name = format!("Option<{}>", field.type_name);
        }
      }

      self.add_reference(ctx, &struct_.name, ParsedData::Struct(struct_.clone()));

      struct_
    };

    self.prefixs = previous_prefixs;

    if let Some(type_) = &schema.type_ {
      let schema_types = SchemaTypes::from(type_);

      let type_name = schema_types.to_full_type_with_object(&struct_.name.to_string());

      if type_name != struct_.name {
        return ParsedData::Type(Type::new_with_reference(&type_name, &struct_.name));
      }
    }

    ParsedData::Struct(struct_)
  }
}

#[cfg(test)]
mod schema_parser_properties_tests {
  use super::*;
  use crate::schemas::APIDescription;

  #[test]
  fn test_schema_parser_with_properties() {
    let api_description = APIDescription::try_load().unwrap();

    let mut parse_context = ParseContext::new(api_description);

    parse_context.set_working_tag("testing");

    let json = r#"{
      "title": "User Search Result Item",
      "description": "User Search Result Item",
      "type": "object",
      "properties": {
        "text_matches": {
          "$ref": "/components/schemas/search-result-text-matches"
        },
        "blog": {
          "type": [
            "string",
            "null"
          ]
        }
      },
      "required": []
    }"#;

    let schema: Schema = serde_json::from_str(json).unwrap();

    let mut parser = SchemaParser::new();

    let mut generated = parser.parse(
      &mut parse_context,
      &"Response".to_string(),
      &SchemaDefinition::Schema(schema.clone()),
    );

    for _ in 0..5 {
      generated = parser.parse(
        &mut parse_context,
        &"Response".to_string(),
        &SchemaDefinition::Schema(schema.clone()),
      );
    }

    if let ParsedData::Struct(struct_) = generated {
      assert_eq!(struct_.name.to_string(), "UserSearchResultItem");
      assert_eq!(struct_.fields.len(), 2);

      for field in struct_.fields.iter() {
        match field.name.to_string().as_str() {
          "text_matches" => {
            assert_eq!(field.type_name, "Option<Vec<SearchResultTextMatchesItem>>");
            assert_eq!(
              field.reference.as_ref().unwrap(),
              "SearchResultTextMatchesItem"
            );
          }
          "blog" => {
            assert_eq!(field.type_name, "Option<String>");
            assert!(field.reference.is_none());
          }
          _ => panic!("Unexpected field"),
        }
      }
    } else {
      panic!("Expected struct");
    }
  }
}
