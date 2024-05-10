use super::{schema_types::SchemaTypes, SchemaParser};
use crate::{
  parser::{context::ParseContext, ParsedData},
  schemas::schema::{Schema, SchemaDefinition},
  structures::{
    enums::{Enum, EnumField},
    types::Type,
  },
};

impl SchemaParser {
  pub(super) fn parse_one_of_like(
    &mut self,
    ctx: &mut ParseContext,
    schema: &Schema,
  ) -> ParsedData {
    let schemas = if let Some(one_of) = &schema.one_of {
      one_of
    } else if let Some(all_of) = &schema.all_of {
      all_of
    } else if let Some(any_of) = &schema.any_of {
      any_of
    } else {
      unreachable!()
    };

    let mut parsed = vec![];

    let mut schema_types = SchemaTypes::default();

    if let Some(type_) = &schema.type_ {
      schema_types.merge(&SchemaTypes::from(type_));
    }

    let mut index = 0;
    for schema in schemas {
      if let SchemaDefinition::Schema(schema) = schema {
        if let Some(type_) = &schema.type_ {
          schema_types.merge(&SchemaTypes::from(type_));
        }
      }

      index += 1;

      // For each item, we will generate a new type name
      self.prefixs.push(format!("item{}", index));

      let generated = self.parse_schema_definition(ctx, schema);

      parsed.push(generated);

      self.prefixs.pop();
    }

    let struct_or_enum_count = parsed
      .iter()
      .filter(|parsed| is_data_structure(parsed))
      .count();

    match struct_or_enum_count {
      0 => {
        let type_name = schema_types.to_full_type();

        ParsedData::Type(Type::new(&type_name))
      }
      1 => {
        let struct_or_enum_index = parsed.iter().position(is_data_structure).unwrap();

        let struct_or_enum = parsed.get(struct_or_enum_index).unwrap();
        let schema = &schemas.get(struct_or_enum_index).unwrap();

        let title = match schema {
          SchemaDefinition::Ref(_) => None,
          SchemaDefinition::Schema(schema) => schema.title.as_ref().and_then(|title| {
            if !title.contains('_') {
              Some(title.clone())
            } else {
              None
            }
          }),
        };

        match struct_or_enum {
          ParsedData::Struct(struct_) => {
            let mut struct_ = struct_.clone();
            if let Some(title) = &title {
              struct_.set_name(title);
            }

            let struct_name = struct_.name.to_string();
            let full_type = schema_types.to_full_type_with_object(&struct_name);

            if struct_name == full_type {
              return ParsedData::Struct(struct_.clone());
            }

            ParsedData::Type(Type::new_with_reference(&full_type, &struct_.name))
          }
          ParsedData::Enum(enum_) => {
            let mut enum_ = enum_.clone();
            if let Some(title) = &title {
              enum_.set_name(title);
            }

            enum_.untagged();

            let enum_name = enum_.name.to_string();
            let full_type = schema_types.to_full_type_with_object(&enum_name);

            if enum_name == full_type {
              return ParsedData::Enum(enum_.clone());
            }

            ParsedData::Type(Type::new_with_reference(&full_type, &enum_.name))
          }
          ParsedData::Type(type_) => {
            if type_.alias.is_none() {
              return ParsedData::Type(type_.clone());
            }

            let type_name = type_.alias.clone().unwrap();
            let full_type = schema_types.to_full_type_with_object(&type_name);

            if type_name == full_type {
              return ParsedData::Type(type_.clone());
            }

            ParsedData::Type(Type::new_with_reference(&full_type, &type_name))
          }
        }
      }
      _ => {
        let enum_name = self.prefixs.join("_");
        let mut enum_ = Enum::new(&enum_name);
        let mut is_optional = false;

        enum_.untagged();

        // TODO: Give a better name to the enum field referencing the struct
        for (index, p) in parsed.iter().enumerate() {
          let schema = &schemas.get(index).unwrap();

          let title = match schema {
            SchemaDefinition::Ref(_) => None,
            SchemaDefinition::Schema(schema) => schema.title.as_ref().and_then(|title| {
              if !title.contains('_') {
                Some(title.clone())
              } else {
                None
              }
            }),
          };

          match p {
            ParsedData::Struct(struct_) => {
              let mut struct_ = struct_.clone();
              if let Some(title) = &title {
                struct_.set_name(title);
              }

              let struct_name = &struct_.name;

              if check_if_same_name(struct_name, &enum_) {
                continue;
              }

              let mut enum_field = EnumField::new(struct_name);

              if let Some(description) = &struct_.description {
                enum_field.set_description(description);
              }

              enum_field.reference(&struct_.name);

              enum_field.set_type_name(struct_name);

              enum_.add_field(enum_field);
            }
            ParsedData::Enum(generated) => {
              let mut generated = generated.clone();
              if let Some(title) = &title {
                generated.set_name(title);
              }

              let enum_name = &generated.name;
              if check_if_same_name(enum_name, &enum_) {
                continue;
              }

              let mut enum_field = EnumField::new(enum_name);

              if let Some(description) = &generated.description {
                enum_field.set_description(description);
              }

              enum_field.set_type_name(enum_name);

              enum_field.reference(&generated.name);

              enum_.add_field(enum_field);
            }
            ParsedData::Type(type_) => {
              if check_if_same_name(&type_.type_name, &enum_) {
                continue;
              }

              if type_.type_name == "Null" {
                is_optional = true;
              } else {
                let alias = match &type_.alias {
                  Some(alias) => alias.clone(),
                  None => {
                    if type_.type_name.starts_with("Vec<") {
                      let inner = type_.type_name.replace("Vec<", "").replace('>', "");

                      format!("{}Array", inner)
                    } else {
                      type_.type_name.clone()
                    }
                  }
                };

                let mut enum_field = EnumField::new(&alias);

                if let Some(reference) = &type_.reference {
                  enum_field.reference(reference);
                }

                enum_field.set_type_name(&type_.type_name);

                enum_.add_field(enum_field);
              }
            }
          }
        }

        ctx.add_reference(&enum_.name, ParsedData::Enum(enum_.clone()));

        if is_optional {
          let type_name = schema_types.to_full_type_with_object(&enum_.name);

          return ParsedData::Type(Type::new_with_reference(&type_name, &enum_.name));
        }

        ParsedData::Enum(enum_)
      }
    }
  }
}

fn is_data_structure(parsed: &ParsedData) -> bool {
  match parsed {
    ParsedData::Struct(_) | ParsedData::Enum(_) => true,
    ParsedData::Type(type_) => type_.type_name.starts_with("Vec<"),
  }
}

fn check_if_same_name(name: &String, enum_: &Enum) -> bool {
  if name == &enum_.name.to_string() {
    return true;
  }

  for field in enum_.fields.iter() {
    if name == &field.name.to_string() {
      return true;
    }
  }

  false
}

#[cfg(test)]
mod schema_parser_one_of_like_tests {
  use super::*;
  use crate::schemas::APIDescription;

  #[test]
  fn test_schema_parser_one_of_and_ref() {
    let api_description = APIDescription::try_load().unwrap();

    let mut parse_context = ParseContext::default();
    let mut schema_parser = SchemaParser::new();

    parse_context.set_working_tag("testing");

    let content_directory = api_description
      .components
      .schemas
      .get("content-directory")
      .unwrap()
      .clone();

    let _content_directory = schema_parser.parse(
      &mut ParseContext::default(),
      &"Content Directory".to_string(),
      &SchemaDefinition::Schema(content_directory.clone()),
    );

    // Parse twice to test if the reference is working
    let content_directory = schema_parser.parse(
      &mut ParseContext::default(),
      &"Content Directory".to_string(),
      &SchemaDefinition::Schema(content_directory),
    );

    if let ParsedData::Type(type_) = &content_directory {
      assert_eq!(type_.alias.as_ref().unwrap(), "ContentDirectory");
      assert_eq!(type_.reference.as_ref().unwrap(), "ContentDirectoryItem");
      assert_eq!(type_.type_name, "Vec<ContentDirectoryItem>");
    } else {
      println!("{:#?}", content_directory);
      panic!("Expected type");
    }

    parse_context.add_reference(&"content-directory".to_string(), content_directory);

    let content_file = api_description
      .components
      .schemas
      .get("content-file")
      .unwrap()
      .clone();

    let _generated = schema_parser.parse(
      &mut ParseContext::default(),
      &"Content File".to_string(),
      &SchemaDefinition::Schema(content_file.clone()),
    );

    // Parse twice to test if the reference is working
    let content_file = schema_parser.parse(
      &mut ParseContext::default(),
      &"Content File".to_string(),
      &SchemaDefinition::Schema(content_file),
    );

    if let ParsedData::Struct(struct_) = &content_file {
      assert_eq!(struct_.name.to_string(), "ContentFile");
    } else {
      println!("{:#?}", content_file);
      panic!("Expected struct");
    }

    parse_context.add_reference(&"content-file".to_string(), content_file);

    let json = r#"{
      "oneOf": [
        {
          "$ref": "\\#/components/schemas/content-directory"
        },
        {
          "$ref": "\\#/components/schemas/content-file"
        }
      ]
    }"#;

    let schema: Schema = serde_json::from_str(json).unwrap();

    let generated = schema_parser.parse(
      &mut parse_context,
      &"Response".to_string(),
      &SchemaDefinition::Schema(schema),
    );

    if let ParsedData::Enum(enum_) = generated {
      assert_eq!(enum_.name.to_string(), "Response");
      assert_eq!(enum_.fields.len(), 2);

      for (index, field) in enum_.fields.iter().enumerate() {
        if index == 0 {
          assert_eq!(field.name, "ContentDirectory");
          assert_eq!(
            field.type_name.as_ref().unwrap(),
            "Vec<ContentDirectoryItem>"
          );
        } else {
          assert_eq!(field.name, "ContentFile");
          assert_eq!(field.type_name.as_ref().unwrap(), "ContentFile");
        }
      }
    } else {
      println!("{:#?}", generated);
      panic!("Expected enum");
    }
  }

  #[test]
  fn test_schema_parser_nullable_all_of() {
    let api_description = APIDescription::try_load().unwrap();

    let mut parse_context = ParseContext::new(api_description.clone());

    let issue_event = api_description
      .components
      .schemas
      .get("issue-event")
      .unwrap()
      .clone();

    let mut schema_parser = SchemaParser::new();

    let generated = schema_parser.parse(
      &mut parse_context,
      &"IssueEvent".to_string(),
      &SchemaDefinition::Schema(issue_event),
    );

    assert!(parse_context.get_references().get("SimpleUser").is_some());

    if let ParsedData::Struct(struct_) = generated {
      let field = struct_
        .fields
        .iter()
        .find(|field| field.name == "assignee")
        .unwrap();

      assert_eq!(field.type_name, "Option<SimpleUser>");
    } else {
      panic!("Expected struct");
    }

    let json = r#"{
      "readOnly": true,
      "description": "The author of the advisory.",
      "allOf": [
        {
          "$ref": "/components/schemas/simple-user"
        }
      ],
      "type": [
        "null"
      ]
    }"#;

    let author_schema: Schema = serde_json::from_str(json).unwrap();

    let generated = schema_parser.parse(
      &mut parse_context,
      &"Author".to_string(),
      &SchemaDefinition::Schema(author_schema),
    );

    if let ParsedData::Type(type_) = generated {
      assert_eq!(type_.reference.as_ref().unwrap(), "SimpleUser");
      assert_eq!(type_.type_name, "Option<SimpleUser>");
    } else {
      panic!("Expected type Option<SimpleUser>");
    }
  }

  #[test]
  fn test_schema_parser_one_of_response() {
    let api_description = APIDescription::try_load().unwrap();

    let mut parse_context = ParseContext::new(api_description.clone());

    let json = r#"{
      "anyOf": [
        {
          "type": "array",
          "items": {
            "$ref": "/components/schemas/starred-repository"
          }
        },
        {
          "type": "array",
          "items": {
            "$ref": "/components/schemas/repository"
          }
        }
      ]
    }"#;

    let schema: Schema = serde_json::from_str(json).unwrap();

    let mut schema_parser = SchemaParser::new();

    let generated = schema_parser.parse(
      &mut parse_context,
      &"Response".to_string(),
      &SchemaDefinition::Schema(schema),
    );

    if let ParsedData::Enum(enum_) = generated {
      assert_eq!(enum_.name.to_string(), "Response");
      assert_eq!(enum_.fields.len(), 2);

      for (index, field) in enum_.fields.iter().enumerate() {
        if index == 0 {
          assert_eq!(field.name, "StarredRepositoryArray");
          assert_eq!(field.type_name.as_ref().unwrap(), "Vec<StarredRepository>");
        } else {
          assert_eq!(field.name, "RepositoryArray");
          assert_eq!(field.type_name.as_ref().unwrap(), "Vec<Repository>");
        }
      }
    } else {
      println!("{:#?}", generated);
      panic!("Expected enum");
    }
  }

  #[test]
  fn test_schema_parser_one_of_difference_type() {
    // Request body of `Add team access restrictions`
    let json = r#"{
      "oneOf": [
        {
          "type": "object",
          "properties": {
            "teams": {
              "type": "array",
              "description": "The slug values for teams",
              "items": {
                "type": "string"
              }
            }
          },
          "required": [
            "teams"
          ],
          "example": {
            "teams": [
              "my-team"
            ]
          }
        },
        {
          "type": "array",
          "description": "The slug values for teams",
          "items": {
            "type": "string"
          }
        }
      ]
    }"#;

    let mut parse_context = ParseContext::default();

    let schema: Schema = serde_json::from_str(json).unwrap();

    let mut schema_parser = SchemaParser::new();

    let generated = schema_parser.parse(
      &mut parse_context,
      &"RequestBody".to_string(),
      &SchemaDefinition::Schema(schema),
    );

    if let ParsedData::Enum(enum_) = generated {
      assert_eq!(enum_.name.to_string(), "RequestBody");
      assert_eq!(enum_.fields.len(), 2);

      for (index, field) in enum_.fields.iter().enumerate() {
        if index == 0 {
          assert_eq!(field.name, "RequestBodyItem1");
          assert_eq!(field.type_name.as_ref().unwrap(), "RequestBodyItem1");
        } else {
          assert_eq!(field.name, "StringArray");
          assert_eq!(field.type_name.as_ref().unwrap(), "Vec<String>");
        }
      }
    } else {
      println!("{:#?}", generated);
      println!("{:#?}", parse_context.get_references());
      panic!("Expected enum");
    }
  }
}
