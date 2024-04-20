use super::{
  super::{context::ParseContext, ParsedData},
  parameter_parser::ParameterParser,
};
use crate::{
  schemas::parameters::Parameter,
  structures::structs::{Struct, StructField},
};

#[derive(Debug, Clone)]
pub struct ParametersParser {
  pub parameters: Vec<Parameter>,
}

impl ParametersParser {
  pub fn new(parameters: Vec<Parameter>) -> Self {
    Self { parameters }
  }

  pub fn parse(
    &self,
    ctx: &mut ParseContext,
    struct_name: &String,
    description: String,
  ) -> Option<Struct> {
    if self.parameters.is_empty() {
      return None;
    }

    let mut new_struct = Struct::new_with_description(struct_name, &description);

    for parameter in &self.parameters {
      let parameter_parser = ParameterParser::new(parameter);

      let generated_struct =
        parameter_parser.parse(ctx, &format!("{}_{}", struct_name, &parameter.name));

      let field = self.create_field_from_parsed_data(
        &parameter.name,
        &generated_struct,
        &parameter.description,
      );

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

        field.reference = type_.reference.clone();

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
      ParsedData::Struct(struct_) => {
        let mut field = StructField::new(parameter_name, &struct_.name);

        let description = description.as_ref().or(struct_.description.as_ref());
        if let Some(description) = description {
          field.set_description(description);
        }

        field.reference(&struct_.name);

        field
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::schemas::parameters::Parameter;

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

    let parameter: Parameter = serde_json::from_str(json).unwrap();

    let mut ctx = ParseContext::default();

    ctx.set_working_tag(&"testing".to_string());

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

    // ctx.add_reference_with_parameter_name(
    //   &"pagination-before".to_string(),
    //   &"before".to_string(),
    //   parsed_data.clone(),
    // );

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

    let parameters: Vec<Parameter> = serde_json::from_str(json).unwrap();

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
