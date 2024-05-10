use super::{schema_types::SchemaTypes, SchemaParser};
use crate::{
  common::RenameRule,
  parser::{context::ParseContext, ParsedData},
  schemas::schema::Schema,
  structures::types::Type,
};

impl SchemaParser {
  pub(super) fn parse_items(&mut self, ctx: &mut ParseContext, schema: &Schema) -> ParsedData {
    let title = schema
      .title
      .clone()
      .and_then(|t| if t.contains('_') { None } else { Some(t) });

    let previous_prefix = self.prefixs.clone();

    if let Some(title) = &title {
      self.prefixs = vec![title.clone(), "item".to_string()];
    }

    let items = schema.items.as_ref().unwrap();

    let schema_types = if let Some(types) = &schema.type_ {
      SchemaTypes::from(types)
    } else {
      SchemaTypes::default()
    };

    // let generated = self.parse_schema(ctx, items);

    let generated = {
      if let Some(title) = &title {
        let name = RenameRule::VariantName.apply(title);
        if let Some(existing) = ctx.reference_existing(&name) {
          return existing.inner;
        }
      }

      self.parse_schema_definition(ctx, items)
    };

    let (type_name, reference) = match generated {
      ParsedData::Struct(struct_) => {
        let struct_ = struct_.clone();

        let type_name = struct_.name.to_string();

        let reference = Some(struct_.name.clone());

        (type_name, reference)
      }
      ParsedData::Enum(enum_) => {
        let enum_ = enum_.clone();

        let type_name = enum_.name.to_string();
        let reference = Some(enum_.name.clone());

        (type_name, reference)
      }
      ParsedData::Type(type_) => (type_.type_name, type_.reference),
    };

    let full_type = schema_types.to_full_type_with_object(&type_name);

    let alias = title
      .as_ref()
      .map(|title| RenameRule::VariantName.apply(title).to_string());

    let mut type_ = Type::new(&full_type);

    if let Some(reference) = &reference {
      type_.set_reference(reference);
    }

    if let Some(alias) = &alias {
      type_.set_alias(alias);
    }

    let parsed = ParsedData::Type(type_);

    if let Some(alias) = &alias {
      ctx.add_reference(alias, parsed.clone());
    }

    self.prefixs = previous_prefix;

    parsed
  }
}

#[cfg(test)]
mod schema_parser_items_tests {
  use super::*;
  use crate::schemas::{schema::SchemaDefinition, APIDescription};

  #[test]
  fn test_schema_parser_parse_items() {
    let json = r#"{
      "type": "array",
      "items": {
        "title": "Tag",
        "description": "Tag",
        "type": "object",
        "properties": {
          "name": {
            "type": "string",
            "examples": [
              "v0.1"
            ]
          },
          "commit": {
            "type": "object",
            "properties": {
              "sha": {
                "type": "string"
              },
              "url": {
                "type": "string",
                "format": "uri"
              }
            },
            "required": [
              "sha",
              "url"
            ]
          },
          "zipball_url": {
            "type": "string",
            "format": "uri",
            "examples": [
              "https://github.com/octocat/Hello-World/zipball/v0.1"
            ]
          },
          "tarball_url": {
            "type": "string",
            "format": "uri",
            "examples": [
              "https://github.com/octocat/Hello-World/tarball/v0.1"
            ]
          },
          "node_id": {
            "type": "string"
          }
        },
        "required": [
          "name",
          "node_id",
          "commit",
          "zipball_url",
          "tarball_url"
        ]
      }
    }"#;

    let schema: Schema = serde_json::from_str(json).unwrap();

    let mut parser = SchemaParser::new();

    let mut ctx = ParseContext::default();

    let generated = parser.parse(
      &mut ctx,
      &"Tag".to_string(),
      &SchemaDefinition::Schema(schema),
    );

    match generated {
      ParsedData::Type(type_) => {
        assert_eq!(type_.type_name, "Vec<Tag>");
        assert!(type_.reference.is_some());
      }
      _ => panic!("Expected struct"),
    }
  }

  #[test]
  fn test_schema_parser_array_item_ref() {
    let api_description = APIDescription::try_load().unwrap();

    let mut parse_context = ParseContext::new(api_description.clone());

    let json = r#"{
      "type": "array",
      "items": {
        "$ref": "/components/schemas/team-simple"
      }
    }"#;

    let schema: Schema = serde_json::from_str(json).unwrap();

    let mut schema_parser = SchemaParser::new();

    let generated = schema_parser.parse(
      &mut parse_context,
      &"Response".to_string(),
      &SchemaDefinition::Schema(schema),
    );

    if let ParsedData::Type(type_) = generated {
      // assert_eq!(type_.alias.as_ref().unwrap(), "TeamSimpleArray");
      assert_eq!(type_.type_name, "Vec<TeamSimple>");
      assert_eq!(type_.reference.as_ref().unwrap(), "TeamSimple");
    } else {
      println!("{:#?}", generated);
      panic!("Expected enum");
    }

    let json = r#"{
      "type": [
        "array",
        "null"
      ],
      "items": {
        "$ref": "/components/schemas/team-simple"
      }
    }"#;

    let schema: Schema = serde_json::from_str(json).unwrap();

    let generated = schema_parser.parse(
      &mut parse_context,
      &"NullableResponse".to_string(),
      &SchemaDefinition::Schema(schema),
    );

    if let ParsedData::Type(type_) = generated {
      assert!(type_.alias.is_none());
      assert_eq!(type_.type_name, "Option<Vec<TeamSimple>>");
      assert_eq!(type_.reference.as_ref().unwrap(), "TeamSimple");
    } else {
      println!("{:#?}", generated);
      panic!("Expected enum");
    }
  }
}
