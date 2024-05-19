use super::super::{context::ParseContext, schema_parser::SchemaParser, ParsedData};
use crate::{
  schemas::parameters::{Parameter, ParameterDefinition},
  structures::types::Type,
};

#[derive(Debug, Clone)]
pub struct ParameterParser<'a> {
  pub parameter: &'a ParameterDefinition,
}

impl<'a> ParameterParser<'a> {
  pub fn new(parameter: &'a ParameterDefinition) -> Self {
    Self { parameter }
  }

  pub fn parse(&self, ctx: &mut ParseContext, name: &str) -> ParsedData {
    let mut schema_parser = SchemaParser::new();

    match &self.parameter {
      ParameterDefinition::Parameter(parameter) => {
        let parsed = schema_parser.parse(ctx, name, &parameter.schema);

        self.normalize(&parsed, parameter)
      }
      ParameterDefinition::Ref(reference) => {
        let reference_id = reference.get_reference_id();

        let parsed = match ctx.reference_parameter(&reference_id) {
          Some(parameter) => parameter.inner,
          None => {
            let parameter = ctx.get_parameter_component(&reference_id).expect(&format!(
              "Reference to parameter {} not found",
              reference_id
            ));

            let parsed =
              schema_parser.parse_global_parameter(ctx, &reference_id, &parameter.schema);

            let mut normalized = self.normalize(&parsed, &parameter);

            if let ParsedData::Type(type_) = &mut normalized {
              if type_.type_name.contains("Option<")
                && type_.alias.is_none()
                && type_.reference.is_some()
              {
                //When the generation is an optional Enum, need to replace Option<Enum> with Option<parameters::Enum>.
                type_.type_name = type_.type_name.replace("Option<", "Option<parameters::");
              }

              // If generating a Type, don't need to add it to the global parameter to avoid
              //
              // ```rust
              // pub type PerPage = Option<i64>
              // Query {
              //   per: PerPage
              // }
              // ```
              // This will make the typed builder think that per is required
              // and it creates redundant alias
              return normalized;
            }

            ctx.add_parameter(&reference_id, normalized.clone());

            normalized
          }
        };

        if let ParsedData::Enum(enum_) = parsed {
          let type_ = Type::new_with_reference(&format!("parameters::{}", enum_.name), &enum_.name);

          ParsedData::Type(type_)
        } else {
          unreachable!("ParsedData::Struct is not expected")
        }
      }
    }
  }

  fn normalize(&self, parsed: &ParsedData, parameter: &Parameter) -> ParsedData {
    let is_required = parameter.required.unwrap_or(false);

    match parsed {
      ParsedData::Type(type_) => {
        let mut type_ = type_.clone();

        if let Some(description) = &parameter.description {
          type_.set_description(description);
        }

        if !is_required && !type_.type_name.starts_with("Option<") {
          type_.type_name = format!("Option<{}>", type_.type_name);
        };

        ParsedData::Type(type_)
      }
      ParsedData::Enum(enum_) => {
        let mut enum_ = enum_.clone();

        if let Some(description) = &parameter.description {
          enum_.set_description(description);
        }

        if !is_required {
          return ParsedData::Type(Type::new_with_reference(
            &format!("Option<{}>", enum_.name),
            &enum_.name,
          ));
        };

        ParsedData::Enum(enum_)
      }
      _ => unreachable!("ParsedData::Struct is not expected"),
    }
  }
}
