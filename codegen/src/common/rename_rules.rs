use regex::Regex;

const CONFLICTS_KEYWORDS: [&str; 8] = [
  "type", "ref", "pub", "self", "super", "async", "default", "build",
];

const INVALID_FIELD_NAMES: [(&str, &str, &str); 10] = [
  ("+1", "PlusOne", "plus_one"),
  ("-1", "MinusOne", "minus_one"),
  ("2fa_disabled", "TwoFADisabled", "two_fa_disabled"),
  (
    "reactions--1",
    "ReactionsMinusMinusOne",
    "reactions_minus_minus_one",
  ),
  (
    "reactions-+1",
    "ReactionsMinusPlusOne",
    "reactions_minus_plus_one",
  ),
  ("reactions+1", "ReactionsPlusOne", "reactions_plus_one"),
  ("@timestamp", "AtTimestamp", "at_timestamp"),
  ("$ref", "Ref", "ref_"),
  ("/", "Slash", "slash"),
  ("*", "Asterisk", "asterisk"),
];

#[derive(Copy, Clone, PartialEq)]
pub enum RenameRule {
  VariantName,
  FieldName,
}

impl RenameRule {
  pub fn apply(self, field: &str) -> String {
    for (invalid, pascal_case, snake_case) in INVALID_FIELD_NAMES.iter() {
      if field == *invalid {
        return match self {
          RenameRule::FieldName => snake_case.to_string(),
          RenameRule::VariantName => pascal_case.to_string(),
        };
      }
    }

    let normalized = RenameRule::normalize(&field.to_string());
    let value = match self {
      RenameRule::FieldName => normalized.to_lowercase(),
      RenameRule::VariantName => {
        let mut pascal = String::new();
        let mut capitalize = true;
        for ch in normalized.chars() {
          if ch == '_' {
            capitalize = true;
          } else if capitalize {
            pascal.push(ch.to_ascii_uppercase());
            capitalize = false;
          } else {
            pascal.push(ch);
          }
        }
        pascal
      }
    };

    if CONFLICTS_KEYWORDS.contains(&value.as_str()) {
      return format!("{}_", value);
    }

    let regexp = Regex::new(r#"^[0-9]+"#).unwrap();

    if regexp.is_match(&value) {
      return format!("_{}", value);
    }

    value
  }

  fn normalize(original: &String) -> String {
    let regexp = Regex::new(r#"^([A-Z]|_)+$"#).unwrap();

    let mut value = vec![];

    for i in original.split_whitespace() {
      if regexp.is_match(i) {
        value.push(i.to_lowercase());
      } else {
        value.push(i.to_string());
      }
    }

    let value = value.join(" ");

    let regexp = r#"(\s|_|-|/|\(|\)|:|'|\.)+"#;

    let regexp = Regex::new(regexp).unwrap();

    let value = regexp.replace_all(value.trim(), "_").to_string();

    let regexp = r#"^_+|_+$"#;

    let regexp = Regex::new(regexp).unwrap();

    let value = regexp.replace_all(value.trim(), "").to_string();

    value
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_rename_rule() {
    assert_eq!(
      RenameRule::VariantName.apply("function_name"),
      "FunctionName"
    );
    assert_eq!(
      RenameRule::VariantName.apply("functionName"),
      "FunctionName"
    );
    assert_eq!(
      RenameRule::FieldName.apply("List_CODEOWNERS_errors"),
      "list_codeowners_errors"
    );
    assert_eq!(
      RenameRule::VariantName.apply("CODEOWNERS errors"),
      "CodeownersErrors"
    );

    // PR_TITLE
    assert_eq!(RenameRule::VariantName.apply("PR_TITLE"), "PrTitle");
  }
}
