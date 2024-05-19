mod parameter_parser;

use super::{context::ParseContext, ParsedData};
use crate::{
  schemas::parameters::ParameterDefinition,
  structures::structs::{Struct, StructField},
};
use parameter_parser::ParameterParser;

#[derive(Debug, Clone)]
pub struct ParametersParser {
  pub parameters: Vec<ParameterDefinition>,
}

impl ParametersParser {
  pub fn new(parameters: Vec<ParameterDefinition>) -> Self {
    Self { parameters }
  }

  pub fn parse(
    &self,
    ctx: &mut ParseContext,
    struct_name: &str,
    description: String,
  ) -> Option<Struct> {
    if self.parameters.is_empty() {
      return None;
    }

    let mut new_struct = Struct::new_with_description(struct_name, &description);

    for parameter in &self.parameters {
      let (name, description) = match parameter {
        ParameterDefinition::Parameter(parameter) => {
          (parameter.name.clone(), parameter.description.clone())
        }
        ParameterDefinition::Ref(reference) => {
          let reference_id = reference.get_reference_id();
          ctx
            .get_parameter_component(&reference_id)
            .map(|parameter| (parameter.name.clone(), parameter.description.clone()))
            .unwrap_or_else(|| panic!("Reference to parameter {} not found", reference_id))
        }
      };

      let parameter_parser = ParameterParser::new(parameter);

      let parsed = parameter_parser.parse(ctx, &format!("{}_{}", struct_name, &name));

      let field = self.create_field_from_parsed_data(&name, &parsed, &description);

      new_struct.add_field(field);
    }

    Some(new_struct)
  }

  fn create_field_from_parsed_data(
    &self,
    parameter_name: &String,
    parsed: &ParsedData,
    description: &Option<String>,
  ) -> StructField {
    match &parsed {
      ParsedData::Type(type_) => {
        let mut field = StructField::new(
          parameter_name,
          type_.alias.as_ref().unwrap_or(&type_.type_name),
        );

        let description = description.as_ref().or(type_.description.as_ref());
        if let Some(description) = description {
          field.set_description(description);
        }

        field.reference.clone_from(&type_.reference);

        field
      }
      ParsedData::Enum(enum_) => {
        let mut field = StructField::new(parameter_name, &enum_.name);

        let description = description.as_ref().or(enum_.description.as_ref());
        if let Some(description) = description {
          field.set_description(description);
        }

        field.reference(&enum_.name);

        field
      }
      _ => unreachable!("ParameterParser should not return a Struct"),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parameters_parser() {
    let json = r#"{
      "name": "before",
      "description": "Fetch results before the cursor",
      "in": "query",
      "required": false,
      "schema": {
        "type": "string"
      }
    }"#;

    let parameter: ParameterDefinition = serde_json::from_str(json).unwrap();

    let mut ctx = ParseContext::default();

    ctx.set_working_tag("testing");

    let parser = ParameterParser::new(&parameter);

    let mut parsed_data = parser.parse(&mut ctx, &"pagination-before".to_string());

    match &mut parsed_data {
      ParsedData::Type(type_) => {
        assert_eq!(type_.type_name, "Option<String>");
        assert_eq!(
          type_.description,
          Some("Fetch results before the cursor".to_string())
        );
      }
      _ => panic!("Expected ParsedData::Type"),
    };

    let json = r#"[
      {
        "name": "before",
        "description": "Fetch results before the cursor",
        "in": "query",
        "required": false,
        "schema": {
          "type": "string"
        }
      },
      {
        "name": "id",
        "in": "query",
        "description": "ID of the object to fetch",
        "required": true,
        "schema": {
          "type": "string"
        }
      }
      ]"#;

    let parameters: Vec<ParameterDefinition> = serde_json::from_str(json).unwrap();

    let parameters_parser = ParametersParser::new(parameters);

    let struct_ = parameters_parser
      .parse(&mut ctx, &"Query".to_string(), "Description".to_string())
      .unwrap();

    assert_eq!(struct_.name, "Query");
    assert_eq!(struct_.description.unwrap(), "Description");

    for (index, field) in struct_.fields.iter().enumerate() {
      match index {
        0 => {
          assert_eq!(field.name, "before");
          assert_eq!(field.type_name, "Option<String>");
          assert_eq!(
            field.description,
            Some("Fetch results before the cursor".to_string())
          );
        }
        1 => {
          assert_eq!(field.name, "id");
          assert_eq!(field.type_name, "String");
          assert_eq!(
            field.description,
            Some("ID of the object to fetch".to_string())
          );
        }
        _ => panic!("Unexpected field"),
      }
    }
  }
}
