use crate::{
  common::RenameRule,
  parser::{
    context::ParseContext, parameters_parser::ParametersParser, schema_parser::SchemaParser,
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
}

impl API {
  pub fn parse(ctx: &mut ParseContext, method: &String, path: &str, schema: &APISchema) -> Self {
    let api_name = schema.operation_id.split('/').last().unwrap().to_string();
    let api_name = RenameRule::FieldName.apply(&api_name);

    let scope_name = RenameRule::FieldName.apply(&schema.operation_id.replace('/', "-"));

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
    let parameters = ParametersParser::new(path_parameters).parse(
      ctx,
      &format!("{}_parameters", scope_name),
      "".to_string(),
    );

    let query = ParametersParser::new(query).parse(
      ctx,
      &format!("{}_query", scope_name),
      format!("Query for `{}`", schema.summary),
    );

    if let Some(query) = &query {
      ctx.add_reference(&query.name, ParsedData::Struct(query.clone()));
    }

    let mut schema_parser = SchemaParser::new();

    let body_schema: Option<SchemaDefinition> =
      schema.request_body.clone().and_then(|body| body.into());

    let body = body_schema.map(|schema| {
      let body = schema_parser.parse(ctx, &format!("{}_request", scope_name), &schema);

      ctx.add_reference(&body.name(), body.clone());

      body
    });

    // let body = match body {
    //   Some(ParsedData::Struct(struct_)) => Some(struct_),
    //   Some(ParsedData::Type(type_)) => {
    //     if type_.type_name.starts_with("Option<") {
    //       let type_name = type_.type_name.replace("Option<", "").replace(">", "");
    //       let mut struct_ = Struct::new(&format!("{}_request", api_name));
    //       struct_.add_field(&type_name, "value");
    //       Some(struct_)
    //     } else {
    //       Some(type_.)
    //     }
    //   }
    //   None => None,
    //   _ => {
    //     println!("{:#?}", body);
    //     unreachable!("Body must be a struct");
    //   }
    // };

    // parse response
    let response = Self::parse_responses(
      ctx,
      &scope_name,
      &mut schema_parser,
      schema.responses.clone(),
    );

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
    }
  }

  fn parse_responses(
    ctx: &mut ParseContext,
    scope_name: &String,
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
      let parsed = schema_parser.parse(
        ctx,
        &format!("{}_response", scope_name),
        available_responses[0].1.as_ref().unwrap(),
      );

      ctx.add_reference(&parsed.name(), parsed.clone());

      return Some(parsed);
    }

    let mut enum_ = Enum::new(&format!("{}_response", scope_name));

    for (status_name, response) in available_responses {
      let response = response.unwrap();

      let parsed = schema_parser.parse(
        ctx,
        &format!("{}_response_{}", scope_name, status_name),
        &response,
      );

      let mut field = EnumField::new(&status_name);

      match &parsed {
        ParsedData::Struct(struct_) => {
          if is_same_type(&struct_.name, &enum_) {
            continue;
          }
          field.set_type_name(&struct_.name);
          field.reference(&struct_.name);
          ctx.add_reference(&struct_.name, parsed.clone());
        }
        ParsedData::Enum(generated) => {
          if is_same_type(&generated.name, &enum_) {
            continue;
          }

          field.set_type_name(&generated.name);
          field.reference(&generated.name);
          ctx.add_reference(&generated.name, parsed.clone());
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

    ctx.add_reference(&enum_name, parsed.clone());

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
