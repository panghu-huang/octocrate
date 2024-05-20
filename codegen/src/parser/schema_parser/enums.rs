use super::SchemaParser;
use crate::{
  parser::{context::ParseContext, ParsedData},
  schemas::schema::{Schema, StringOrBool},
  structures::{
    enums::{Enum, EnumField},
    types::Type,
  },
};

// Hardcoded enums for reducing the number of generated enums
// We don't directly modify the api.json because the https://github.com/github/rest-api-description repository is always being updated
// To avoid conflicts, we made some modifications here
// Please note that these enum values are generated based on the current state of the GitHub API, and if the GitHub API changes, these enum values may become outdated
const PRESET_ENUMS: [(&str, &[&str]); 18] = [
  ("ReadWriteAdminPermission", &["read", "write", "admin"]),
  ("ReadWritePermission", &["read", "write"]),
  ("ReadPermission", &["read"]),
  ("WritePermission", &["write"]),
  (
    "SquashMergeCommitMessage",
    &["PR_BODY", "COMMIT_MESSAGES", "BLANK"],
  ),
  ("MergeCommitMessage", &["PR_BODY", "PR_TITLE", "BLANK"]),
  ("MergeCommitTitle", &["PR_TITLE", "MERGE_MESSAGE"]),
  (
    "SquashMergeCommitTitle",
    &["PR_TITLE", "COMMIT_OR_PR_TITLE"],
  ),
  ("Severity", &["low", "medium", "high", "critical"]),
  ("Visibility", &["all", "private", "selected"]),
  (
    "Operator",
    &["starts_with", "ends_with", "contains", "regex"],
  ),
  ("IdentifiersType", &["CVE", "GHSA"]),
  ("RepositorySelection", &["all", "selected"]),
  (
    "DependabotAlertState",
    &["auto_dismissed", "dismissed", "fixed", "open"],
  ),
  (
    "CopilotOrganizationPolicy",
    &["enabled", "disabled", "unconfigured"],
  ),
  (
    "PackageType",
    &["npm", "maven", "rubygems", "docker", "nuget", "container"],
  ),
  (
    "DismissedReason",
    &[
      "fix_started",
      "inaccurate",
      "no_bandwidth",
      "not_used",
      "tolerable_risk",
    ],
  ),
  ("RequestRepositorySelection", &["none", "all", "subset"]),
];

impl SchemaParser {
  pub(super) fn parse_enum(&mut self, ctx: &mut ParseContext, schema: &Schema) -> ParsedData {
    let enum_name = self.prefixs.join("_");
    let mut enum_ = Enum::new(&enum_name);
    let mut is_optional = false;

    if let Some(description) = &schema.description {
      enum_.set_description(description);
    }

    if let Some(enum_values) = &schema.enum_ {
      if let Some(enum_) = self.check_if_preset_enum(enum_values) {
        return enum_;
      }

      for enum_value in enum_values {
        if let Some(enum_value) = &enum_value {
          match enum_value {
            StringOrBool::String(value) => {
              let enum_field = EnumField::new(value);

              enum_.add_field(enum_field);
            }
            StringOrBool::Bool(_) => {
              let mut enum_field = EnumField::new("Boolean");

              enum_field.set_type_name("bool");

              enum_.add_field(enum_field);
            }
          }
        } else {
          is_optional = true;
        }
      }
    }

    self.add_reference(ctx, &enum_.name, ParsedData::Enum(enum_.clone()));

    if is_optional {
      return ParsedData::Type(Type::new_with_reference(
        &format!("Option<{}>", enum_.name),
        &enum_.name,
      ));
    }

    ParsedData::Enum(enum_)
  }

  fn check_if_preset_enum(&self, enums: &[Option<StringOrBool>]) -> Option<ParsedData> {
    let no_null_enums: Vec<_> = enums.iter().filter_map(|x| x.as_ref()).collect();

    let has_null = enums.len() != no_null_enums.len();

    for (enum_name, values) in PRESET_ENUMS.iter() {
      if no_null_enums.len() == values.len() {
        let mut is_preset = true;

        for (index, value) in values.iter().enumerate() {
          if let StringOrBool::String(enum_value) = &no_null_enums[index] {
            if enum_value != value {
              is_preset = false;
              break;
            }
          } else {
            is_preset = false;
            break;
          }
        }

        if is_preset {
          if has_null {
            return Some(ParsedData::Type(Type::new_with_reference(
              &format!("Option<{}>", enum_name),
              enum_name,
            )));
          }

          return Some(ParsedData::Type(Type::new(enum_name)));
        }
      }
    }

    None
  }
}

#[cfg(test)]
mod schema_parser_enums_tests {
  use super::*;
  use crate::schemas::schema::SchemaDefinition;

  #[test]
  fn test_schema_parser_with_enum() {
    let json = r#"{
      "type": "object",
      "properties": {
        "state": {
          "type": "string",
          "enum": [
            "open",
            "closed",
            null
          ]
        }
      }
    }"#;

    let schema: Schema = serde_json::from_str(json).unwrap();

    let mut parser = SchemaParser::new();

    let mut ctx = ParseContext::default();

    let generated = parser.parse(
      &mut ctx,
      &"Response".to_string(),
      &SchemaDefinition::Schema(schema),
    );

    if let ParsedData::Struct(struct_) = generated {
      assert_eq!(struct_.name.to_string(), "Response");
      assert_eq!(struct_.fields.len(), 1);

      let field = &struct_.fields[0];
      assert_eq!(field.name, "state");
      assert_eq!(field.type_name, "Option<ResponseState>");

      let reference = &field.reference.clone().unwrap();

      assert_eq!(reference, "ResponseState");
    } else {
      panic!("Expected struct");
    }
  }
}
