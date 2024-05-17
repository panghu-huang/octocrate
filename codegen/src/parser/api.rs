use crate::{
  common::RenameRule,
  parser::{
    context::{ParseContext, References},
    parameters_parser::ParametersParser,
    schema_parser::SchemaParser,
    ParsedData,
  },
  schemas::{
    api::{Responses, API as APISchema},
    parameters::ParameterDefinition,
    schema::SchemaDefinition,
  },
  structures::{
    enums::{Enum, EnumField},
    structs::Struct,
    types::Type,
  },
};

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Debug)]
pub enum Method {
  GET,
  POST,
  PUT,
  PATCH,
  DELETE,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Debug)]
pub struct API {
  pub name: String,
  pub summary: String,
  pub document_url: String,
  pub description: String,
  pub path: String,
  pub method: Method,
  pub parameters: Option<Struct>,
  pub body: Option<ParsedData>,
  pub query: Option<Struct>,
  pub response: Option<ParsedData>,
  pub references: Option<References>,
}

impl API {
  pub fn parse(ctx: &mut ParseContext, method: &String, path: &str, schema: &APISchema) -> Self {
    let api_name = schema.operation_id.split('/').last().unwrap().to_string();
    let api_name = RenameRule::FieldName.apply(&api_name);

    let method = Method::try_from(method).unwrap();

    let mut path_parameters = vec![];
    let mut query = vec![];

    if let Some(parameters) = &schema.parameters {
      for parameter in parameters {
        let parameter = match parameter {
          ParameterDefinition::Parameter(parameter) => parameter.clone(),
          ParameterDefinition::Ref(reference) => {
            let reference_id = reference.get_reference_id();
            ctx
              .get_parameter(&reference_id)
              .expect("Reference not found")
          }
        };

        if parameter.position.is_query() {
          query.push(parameter);
        } else {
          path_parameters.push(parameter);
        }
      }
    }

    // Dot add reference for path parameters
    let parameters =
      ParametersParser::new(path_parameters).parse(ctx, "Parameters", "".to_string());

    let query = ParametersParser::new(query).parse(ctx, "Query", "".to_string());

    if let Some(query) = &query {
      // Query can only be a type unique to the API, so there is no need to check if it is a global type here
      ctx.add_scoped_reference(&query.name, ParsedData::Struct(query.clone()));
    }

    let mut schema_parser = SchemaParser::new();

    let body_schema: Option<SchemaDefinition> =
      schema.request_body.clone().and_then(|body| body.into());

    let body = body_schema.map(|schema| {
      let body = schema_parser.parse(ctx, "Request", &schema);

      if schema.is_ref() {
        ctx.add_reference(&body.name(), body.clone());
      } else {
        ctx.add_scoped_reference(&body.name(), body.clone());
      }

      body
    });

    if let Some(body) = &body {
      // If body is not Request or Option<Request>
      // it means that this Request is a global type, and an alias needs to be created for this global type
      if body.name() != "Request" && body.name() != "Option<Request>" {
        let mut type_ = Type::new(&body.name());
        type_.set_alias("Request");
        ctx.add_scoped_reference("Request", ParsedData::Type(type_));
      }
    }

    // parse response
    let response = Self::parse_responses(ctx, &mut schema_parser, schema.responses.clone());

    if let Some(response) = &response {
      // If response is not Response
      // it means that this Response is a global type, and an alias needs to be created for this global type
      if response.name() != "Response" {
        let mut type_ = Type::new(&response.name());
        type_.set_alias("Response");
        ctx.add_scoped_reference("Response", ParsedData::Type(type_));
      }
    }

    let references = ctx.get_and_clear_scoped_references();

    Self {
      name: api_name,
      summary: schema.summary.clone(),
      document_url: schema.external_docs.url.clone(),
      description: schema.description.clone(),
      path: path.to_owned(),
      method,
      parameters,
      body,
      query,
      response,
      references: if references.is_empty() {
        None
      } else {
        Some(references)
      },
    }
  }

  fn parse_responses(
    ctx: &mut ParseContext,
    schema_parser: &mut SchemaParser,
    responses: Responses,
  ) -> Option<ParsedData> {
    let available_responses = responses
      .into_iter()
      .filter(|(_, res)| res.is_some())
      .collect::<Vec<_>>();

    if available_responses.is_empty() {
      return None;
    }

    if available_responses.len() == 1 {
      let response = available_responses[0].1.as_ref().unwrap();
      let parsed = schema_parser.parse(ctx, "Response", response);

      if response.is_ref() {
        ctx.add_reference(&parsed.name(), parsed.clone());
      } else {
        ctx.add_scoped_reference(&parsed.name(), parsed.clone());
      }

      return Some(parsed);
    }

    let mut enum_ = Enum::new("Response");

    for (status_name, response) in available_responses {
      let response = response.unwrap();

      let parsed = schema_parser.parse(ctx, &format!("{}_response", status_name), &response);

      let is_scoped = !response.is_ref();

      let mut field = EnumField::new(&status_name);

      match &parsed {
        ParsedData::Struct(struct_) => {
          if is_same_type(&struct_.name, &enum_) {
            continue;
          }
          field.set_type_name(&struct_.name);
          field.reference(&struct_.name);

          if is_scoped {
            ctx.add_scoped_reference(&struct_.name, parsed.clone());
          } else {
            ctx.add_reference(&struct_.name, parsed.clone());
          }
        }
        ParsedData::Enum(generated) => {
          if is_same_type(&generated.name, &enum_) {
            continue;
          }

          field.set_type_name(&generated.name);
          field.reference(&generated.name);

          if is_scoped {
            ctx.add_scoped_reference(&generated.name, parsed.clone());
          } else {
            ctx.add_reference(&generated.name, parsed.clone());
          }
        }
        ParsedData::Type(type_) => {
          let type_name = if let Some(alias) = &type_.alias {
            alias.clone()
          } else {
            type_.type_name.clone()
          };

          if is_same_type(&type_name, &enum_) {
            continue;
          }

          field.set_type_name(&type_name);

          if let Some(reference) = &type_.reference {
            field.reference(reference);
          }
        }
      }

      enum_.add_field(field);
    }

    if enum_.fields.len() == 1 {
      let field = enum_.fields.pop().unwrap();
      return Some(ParsedData::Type(Type::new(&field.type_name.unwrap())));
    }

    let enum_name = enum_.name.clone();

    let parsed = ParsedData::Enum(enum_);

    ctx.add_scoped_reference(&enum_name, parsed.clone());

    Some(parsed)
  }
}

fn is_same_type(target_type_name: &String, enum_: &Enum) -> bool {
  for field in enum_.fields.iter() {
    if &field.name == target_type_name {
      return true;
    }

    if let Some(type_name) = &field.type_name {
      if target_type_name == type_name {
        return true;
      }
    }
  }

  false
}

impl TryFrom<&String> for Method {
  type Error = String;

  fn try_from(value: &String) -> Result<Self, Self::Error> {
    match value.as_str() {
      "get" => Ok(Method::GET),
      "post" => Ok(Method::POST),
      "put" => Ok(Method::PUT),
      "patch" => Ok(Method::PATCH),
      "delete" => Ok(Method::DELETE),
      _ => Err(format!("Invalid method: {}", value)),
    }
  }
}

impl std::fmt::Display for Method {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match &self {
      Method::GET => write!(f, "get"),
      Method::POST => write!(f, "post"),
      Method::PUT => write!(f, "put"),
      Method::PATCH => write!(f, "patch"),
      Method::DELETE => write!(f, "delete"),
    }
  }
}
