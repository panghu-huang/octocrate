use indexmap::IndexMap;

use crate::{
  common::RenameRule,
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
  parameters: IndexMap<String, ParsedData>,
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

    // Parse API paths
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

    // Parse webhooks
    self.parse_webhooks(&mut parse_context, &api_description);

    parse_context.finish_parsing();

    // Finished parsing
    ParsedAPIDescription {
      tags: parse_context.get_tags(),
      apis: parse_context.get_apis(),
      parameters: parse_context.get_parameters(),
      types: parse_context.get_references(),
      webhooks: parse_context.get_webhook_references(),
    }
  }

  pub fn write_apis(&self, parsed: ParsedAPIDescription, path: &Path) {
    let mut writer = Writer::new(path);

    // APIs
    let mut directory = Directory::new("apis");

    let mut api_entry_module = APIEntryModule::new();

    for (tag, api_functions) in parsed.apis {
      let description = parsed.tags.get(&tag).cloned();

      let mut api_module = APIModule::new(&tag, description);

      for api in api_functions {
        let api_name = api.name;

        let body_type = api
          .body
          // We don't need to return the real type name, we just need to return the Request under the API module
          .map(|_| format!("{}::Request", api_name))
          .unwrap_or("()".to_string());

        let mut parameters = vec![];
        let mut stringify_params = vec![];

        let mut url = api.path.clone();

        if let Some(p) = api.parameters {
          for field in p.fields {
            let type_name =
              if field.reference.is_some() && !field.type_name.starts_with("parameters::") {
                // If there is no reference, it means it is a basic type, such as i32, String
                // If it starts with parameters::, it means it references a type in `octocrate_core::parameters`
                format!("{}::{}", api_name, field.type_name)
              } else {
                field.type_name.clone()
              };

            let parameter = crate::writer::Parameter {
              name: field.name.clone(),
              type_name,
              rename: field.rename.clone(),
            };

            if let Some(rename) = &field.rename {
              // If there is a rename, it means this parameter has been renamed
              // e.g. /api/{ref} -> /api/{ref_}
              url = url.replace(&format!("{{{}}}", rename), &format!("{{{}}}", field.name));
            }

            if field.reference.is_some() {
              // If this field has a reference, it means it is not a basic type and needs to be serialized
              stringify_params.push(field.name.clone());
            }

            parameters.push(parameter);
          }
        }

        let references = api.references.map(|r| {
          let mut references = crate::writer::References::default();

          for (_, reference) in r {
            references.add_reference(&reference.inner);
          }

          references
        });

        let api_function = APIFunction {
          function_name: api_name.clone(),
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
            // We don't need to return the real type name, we just need to return the Query under the corresponding API name module
            .map(|_| format!("{}::Query", api_name))
            .unwrap_or("()".to_string()),
          // Same as query_type
          response_type: api.response.map(|_| format!("{}::Response", api_name)),
          references,
        };

        api_module.add_function(api_function);
      }

      api_entry_module.add_module(&tag);
      directory.add_file(api_module);
    }

    directory.add_file(api_entry_module);

    writer.add_file(directory);

    writer.write();
  }

  pub fn write_types(&self, parsed: ParsedAPIDescription, path: &Path) {
    let mut writer = Writer::new(path);

    let mut type_entry_module = TypeEntryModule::new("lib.rs");

    // Types
    let mut types_module = TypeModule::new("models");

    for (_, type_) in parsed.types.iter() {
      types_module.add_type(type_);
    }

    type_entry_module.add_module("models");

    writer.add_file(types_module);

    // Parameters
    let mut parameters_module = TypeModule::new("parameters");

    for (_, type_) in parsed.parameters.iter() {
      parameters_module.add_type(type_);
    }

    type_entry_module.add_module_with_exports("parameters", false);

    writer.add_file(parameters_module);

    // Webhooks
    let mut webhooks_module = TypeModule::new("webhooks");

    for (_, type_) in parsed.webhooks.iter() {
      webhooks_module.add_type(type_);
    }

    type_entry_module.add_module_with_exports("webhooks", false);

    writer.add_file(webhooks_module);

    // Add type entry module
    writer.add_file(type_entry_module);

    writer.write();
  }

  fn parse_webhooks(&self, parse_context: &mut ParseContext, api_description: &APIDescription) {
    let mut schema_parser = SchemaParser::new();

    let mut webhook_event = Enum::new("WebhookEvent");

    webhook_event.untagged();

    // {
    //   pull_request: Enum {
    //     Opened(PullRequestOpened),
    //     Closed(PullRequestClosed),
    //   }
    // }
    let mut webhook_categories: IndexMap<String, Enum> = IndexMap::new();

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

      let mut parts = webhook.operation_id.split('/');

      let category = parts.next().unwrap_or_else(|| {
        panic!("No category found in operationId for webhook {}", name);
      });

      parse_context.set_working_tag(&webhook_feature_name(category));

      if let Some(body_schema) = body_schema {
        let parsed = schema_parser.parse(parse_context, name, &body_schema);

        match &parsed {
          ParsedData::Struct(struct_) => {
            let mut field = EnumField::new(name);

            field.set_type_name(&struct_.name);
          }
          ParsedData::Enum(generated) => {
            let mut field = EnumField::new(name);

            field.set_type_name(&generated.name);
          }
          _ => panic!("Expected struct or enum for webhook {}", name),
        }

        if let Some(type_) = parts.next() {
          let category_webhook = webhook_categories
            .entry(category.to_string())
            .or_insert_with(|| {
              let mut enum_ = Enum::new(&format!("webhook_{}_event", category));

              enum_.untagged();

              let mut field = EnumField::new(category);

              field.set_type_name(&enum_.name);
              field.add_tag(&webhook_feature_name(category));

              webhook_event.add_field(field);

              enum_
            });

          let mut field = EnumField::new(type_);

          field.set_type_name(&parsed.name());

          category_webhook.add_field(field);
        } else {
          let mut field = EnumField::new(category);

          field.set_type_name(&parsed.name());
          field.add_tag(&webhook_feature_name(category));

          webhook_event.add_field(field);
        }

        parse_context.add_reference(&parsed.name(), parsed);
      } else {
        panic!("No body schema found for webhook {}", name);
      }
    }

    for enum_ in webhook_categories.values() {
      parse_context.add_reference(&enum_.name, ParsedData::Enum(enum_.clone()));
    }

    parse_context.set_working_tag("webhook_event");

    parse_context.add_reference("WebhookEvent", ParsedData::Enum(webhook_event));
  }
}

fn webhook_feature_name(name: &str) -> String {
  RenameRule::FieldName.apply(&format!("webhook_{}", name))
}
