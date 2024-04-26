use indexmap::IndexMap;

use crate::{
  parser::{api::API, context::ParseContext, schema_parser::SchemaParser, ParsedData},
  schemas::{schema::SchemaDefinition, APIDescription},
  structures::enums::{Enum, EnumField},
  writer::{
    APIEntryModule, APIFunction, APIModule, Directory, TypeEntryModule, TypeModule, Writer,
  },
};
use std::path::Path;

#[derive(Clone)]
pub struct ParsedAPIDescription {
  tags: IndexMap<String, String>,
  types: IndexMap<String, ParsedData>,
  webhooks: IndexMap<String, ParsedData>,
  apis: IndexMap<String, Vec<API>>,
}

pub struct Codegen;

impl Codegen {
  pub fn new() -> Self {
    Codegen
  }

  pub fn parse(&self) -> ParsedAPIDescription {
    let api_description = APIDescription::try_load().unwrap();

    let mut parse_context = ParseContext::new(api_description.clone());

    // Add tag descriptions
    for tag in &api_description.tags {
      parse_context.add_tag_description(&tag.name, &tag.description);
    }

    // Parse paths
    let paths = &api_description.paths;

    for (path, path_item) in paths {
      for (method, api) in path_item.clone().into_iter() {
        if api.tags.len() != 1 {
          panic!("Expected exactly one tag");
        }

        parse_context.start_parsing_api(&format!("{} {}", method.to_uppercase(), path));

        parse_context.set_working_tag(&api.tags[0]);

        let parsed_api = API::parse(&mut parse_context, &method, path, &api);

        parse_context.add_api(parsed_api);
      }
    }

    let mut schema_parser = SchemaParser::new();

    let mut enum_ = Enum::new("WebhookEvent");

    for (name, webhook) in &api_description.webhooks {
      parse_context.start_parsing_webhook(name);

      let webhook = webhook
        .post
        .clone()
        .unwrap_or_else(|| panic!("No post for webhook {}", name));

      let request_body = webhook
        .request_body
        .clone()
        .unwrap_or_else(|| panic!("No request body for webhook {}", name));

      let body_schema: Option<SchemaDefinition> = request_body.into();

      if let Some(body_schema) = body_schema {
        let parsed = schema_parser.parse(&mut parse_context, name, &body_schema);

        match &parsed {
          ParsedData::Struct(struct_) => {
            let mut field = EnumField::new(name);

            field.set_type_name(&struct_.name);

            enum_.add_field(field);
          }
          ParsedData::Enum(generated) => {
            let mut field = EnumField::new(name);

            field.set_type_name(&generated.name);

            enum_.add_field(field);
          }
          _ => panic!("Expected struct or enum for webhook {}", name),
        }

        parse_context.add_reference(&parsed.name(), parsed);
      } else {
        panic!("No body schema found for webhook {}", name);
      }
    }

    enum_.untagged();

    parse_context.add_reference(&"WebhookEvent".to_string(), ParsedData::Enum(enum_));

    parse_context.finish_parsing();

    // Finished parsing
    ParsedAPIDescription {
      tags: parse_context.get_tags(),
      apis: parse_context.get_apis(),
      types: parse_context.get_references(),
      webhooks: parse_context.get_webhooks(),
    }
  }

  pub fn write_apis(&self, parsed: ParsedAPIDescription, path: &Path) {
    let mut writer = Writer::new(path);

    // APIs
    println!("Writing apis to file system");

    let mut directory = Directory::new("apis");

    let mut api_entry_module = APIEntryModule::new();

    for (tag, api_functions) in parsed.apis {
      let description = parsed.tags.get(&tag).cloned();

      let mut api_module = APIModule::new(&tag, description);

      for api in api_functions {
        let body_type = api
          .body
          .map(|b| match b {
            ParsedData::Type(t) => {
              if t.type_name.starts_with("Option<") {
                // Extract the inner type
                t.reference
                  .unwrap_or(t.type_name[7..t.type_name.len() - 1].to_string())
              } else {
                t.alias.unwrap_or(t.type_name)
              }
            }
            _ => b.name(),
          })
          .unwrap_or("()".to_string());

        let mut parameters = vec![];
        let mut stringify_params = vec![];

        let mut url = api.path.clone();

        if let Some(p) = api.parameters {
          for field in p.fields {
            let parameter = crate::writer::Parameter {
              name: field.name.clone(),
              type_name: field.type_name.clone(),
              rename: field.rename.clone(),
            };

            if let Some(rename) = &field.rename {
              url = url.replace(&format!("{{{}}}", rename), &format!("{{{}}}", field.name));
            }

            if field.reference.is_some() {
              stringify_params.push(field.name.clone());
            }

            parameters.push(parameter);
          }
        }

        let api_function = APIFunction {
          function_name: api.name.clone(),
          method: api.method.to_string(),
          url,
          parameters,
          stringify_params,
          document_url: api.document_url.clone(),
          description: api.description.clone(),
          summary: api.summary.clone(),
          body_type,
          query_type: api
            .query
            .map(|q| q.name.clone())
            .unwrap_or("()".to_string()),
          response_type: api.response.map(|r| r.name()),
        };

        api_module.add_function(api_function);
      }

      api_entry_module.add_module(&tag);
      directory.add_file(api_module);
    }

    directory.add_file(api_entry_module);

    writer.add_file(directory);

    writer.write();

    println!("Finished writing");
  }

  pub fn write_types(&self, parsed: ParsedAPIDescription, path: &Path) {
    let mut writer = Writer::new(path);

    println!("Writing to file system");

    let mut type_entry_module = TypeEntryModule::new("lib.rs");

    // Types
    let mut types_module = TypeModule::new("models");

    for (_, type_) in parsed.types.iter() {
      types_module.add_type(type_);
    }

    type_entry_module.add_module("models");

    writer.add_file(types_module);

    // Webhooks
    let mut webhooks_module = TypeModule::new("webhooks");

    for (_, type_) in parsed.webhooks.iter() {
      webhooks_module.add_type(type_);
    }

    type_entry_module.add_module("webhooks");

    writer.add_file(webhooks_module);

    // Add type entry module
    writer.add_file(type_entry_module);

    writer.write();

    println!("Finished writing");
  }
}
