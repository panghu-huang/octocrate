use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum BodyParameterType {
  String,
  Number,
  Integer,
  Boolean,
  Object,
  Array,
  #[serde(rename = "string or null")]
  StringOrNull,
  #[serde(rename = "object or null")]
  ObjectOrNull,
  #[serde(rename = "boolean or null")]
  BooleanOrNull,
  #[serde(rename = "number or null")]
  NumberOrNull,
  #[serde(rename = "integer or null")]
  IntegerOrNull,
  #[serde(rename = "array of strings")]
  ArrayOfStrings,
  #[serde(rename = "array of integers")]
  ArrayOfIntegers,
  #[serde(rename = "array of objects")]
  ArrayOfObjects,
  #[serde(rename = "string or number")]
  StringOrNumber,
  #[serde(rename = "string or integer")]
  StringOrInteger,
  #[serde(rename = "boolean or string")]
  BooleanOrString,
  #[serde(rename = "object or string")]
  ObjectOrString,
  #[serde(rename = "array of objects or null")]
  ArrayOfObjectsOrNull,
  #[serde(rename = "array of strings or null")]
  ArrayOfStringsOrNull,
  /// Option<StringOrNumber> or serde_json::Value
  #[serde(rename = "null or string or integer")]
  NullOrStringOrInteger,
  /// use serde_json::Value for this type
  #[serde(rename = "null or string or array")]
  NullOrStringOrArray,
  /// Only one. use serde_json::Value for this type
  #[serde(rename = "object or array or string")]
  ObjectOrArrayOrString,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum BodyParameterRequired {
  Bool(bool),
  Array(Vec<String>),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BodyParameter {
  pub name: String,
  pub description: String,
  #[serde(rename = "isRequired")]
  pub required: Option<BodyParameterRequired>,
  #[serde(rename = "type")]
  pub type_: BodyParameterType,
  #[serde(rename = "childParamsGroups")]
  pub child_params_group: Option<Vec<BodyParameter>>,
  #[serde(rename = "enum")]
  pub enum_: Option<Vec<Option<String>>>,
}

#[cfg(test)]
mod body_parameters_tests {
  use super::*;

  #[test]
  fn test_body_parameter() {
    let json = r#"[
      {
        "type": "string",
        "name": "name",
        "in": "body",
        "isRequired": true,
        "description": "<p>The name of the repository.</p>"
      },
      {
        "type": "object or null",
        "name": "security_and_analysis",
        "in": "body",
        "description": "<p>Specify which security and analysis features to enable or disable for the repository.</p>\n<p>To use this parameter, you must have admin permissions for the repository or be an owner or security manager for the organization that owns the repository. For more information, see \"<a href=\"https://docs.github.com/enterprise-cloud@latest//organizations/managing-peoples-access-to-your-organization-with-roles/managing-security-managers-in-your-organization\">Managing security managers in your organization</a>.\"</p>\n<p>For example, to enable GitHub Advanced Security, use this data in the body of the <code>PATCH</code> request:\n<code>{ \"security_and_analysis\": {\"advanced_security\": { \"status\": \"enabled\" } } }</code>.</p>\n<p>You can check which security and analysis features are currently enabled by using a <code>GET /repos/{owner}/{repo}</code> request.</p>",
        "childParamsGroups": [
          {
            "type": "object",
            "name": "advanced_security",
            "description": "<p>Use the <code>status</code> property to enable or disable GitHub Advanced Security for this repository. For more information, see \"<a href=\"/github/getting-started-with-github/learning-about-github/about-github-advanced-security\">About GitHub Advanced Security</a>.\"</p>",
            "childParamsGroups": [
              {
                "type": "string",
                "name": "status",
                "description": "<p>Can be <code>enabled</code> or <code>disabled</code>.</p>"
              }
            ]
          }
        ]
      },
      {
        "type": "boolean",
        "name": "has_issues",
        "in": "body",
        "description": "<p>Either <code>true</code> to enable issues for this repository or <code>false</code> to disable them.</p>",
        "default": true
      },
      {
        "type": "string",
        "name": "squash_merge_commit_title",
        "in": "body",
        "description": "<p>The default value for a squash merge commit title:</p>\n<ul>\n<li><code>PR_TITLE</code> - default to the pull request's title.</li>\n<li><code>COMMIT_OR_PR_TITLE</code> - default to the commit's title (if only one commit) or the pull request's title (when more than one commit).</li>\n</ul>",
        "enum": [
          "PR_TITLE",
          "COMMIT_OR_PR_TITLE"
        ]
      }
    ]"#;

    let body_parameters: Vec<BodyParameter> = serde_json::from_str(json).unwrap();

    assert_eq!(body_parameters.len(), 4);

    let body_parameter = &body_parameters[0];

    assert_eq!(body_parameter.name, "name");
    assert_eq!(
      body_parameter.description,
      "<p>The name of the repository.</p>"
    );
    assert_eq!(
      body_parameter.required,
      Some(BodyParameterRequired::Bool(true))
    );
    assert_eq!(body_parameter.type_, BodyParameterType::String);
    assert!(body_parameter.child_params_group.is_none());
    assert_eq!(body_parameter.enum_, None);

    let body_parameter = &body_parameters[1];

    assert_eq!(body_parameter.name, "security_and_analysis");
    assert_eq!(body_parameter.description, "<p>Specify which security and analysis features to enable or disable for the repository.</p>\n<p>To use this parameter, you must have admin permissions for the repository or be an owner or security manager for the organization that owns the repository. For more information, see \"<a href=\"https://docs.github.com/enterprise-cloud@latest//organizations/managing-peoples-access-to-your-organization-with-roles/managing-security-managers-in-your-organization\">Managing security managers in your organization</a>.\"</p>\n<p>For example, to enable GitHub Advanced Security, use this data in the body of the <code>PATCH</code> request:\n<code>{ \"security_and_analysis\": {\"advanced_security\": { \"status\": \"enabled\" } } }</code>.</p>\n<p>You can check which security and analysis features are currently enabled by using a <code>GET /repos/{owner}/{repo}</code> request.</p>");
    assert!(body_parameter.required.is_none());
    assert_eq!(body_parameter.type_, BodyParameterType::ObjectOrNull);
    assert_eq!(body_parameter.child_params_group.is_some(), true);
    assert_eq!(body_parameter.enum_, None);

    let child_params_group = body_parameter.child_params_group.as_ref().unwrap();

    assert_eq!(child_params_group.len(), 1);

    let child_params_group = &child_params_group[0];

    assert_eq!(child_params_group.name, "advanced_security");
    assert_eq!(child_params_group.description, "<p>Use the <code>status</code> property to enable or disable GitHub Advanced Security for this repository. For more information, see \"<a href=\"/github/getting-started-with-github/learning-about-github/about-github-advanced-security\">About GitHub Advanced Security</a>.\"</p>");
    assert!(body_parameter.required.is_none());
    assert_eq!(child_params_group.type_, BodyParameterType::Object);
    assert_eq!(child_params_group.child_params_group.is_some(), true);

    let child_params_group = child_params_group.child_params_group.as_ref().unwrap();

    assert_eq!(child_params_group.len(), 1);
    assert_eq!(child_params_group[0].name, "status");
    assert_eq!(
      child_params_group[0].description,
      "<p>Can be <code>enabled</code> or <code>disabled</code>.</p>"
    );
    assert!(body_parameter.required.is_none());
    assert_eq!(child_params_group[0].type_, BodyParameterType::String);
    assert_eq!(child_params_group[0].enum_, None);

    let body_parameter = &body_parameters[2];

    assert_eq!(body_parameter.name, "has_issues");
    assert_eq!(body_parameter.description, "<p>Either <code>true</code> to enable issues for this repository or <code>false</code> to disable them.</p>");
    assert!(body_parameter.required.is_none());
    assert_eq!(body_parameter.type_, BodyParameterType::Boolean);
    assert!(body_parameter.child_params_group.is_none());
    assert_eq!(body_parameter.enum_, None);

    let body_parameter = &body_parameters[3];

    assert_eq!(body_parameter.name, "squash_merge_commit_title");
    assert_eq!(body_parameter.description, "<p>The default value for a squash merge commit title:</p>\n<ul>\n<li><code>PR_TITLE</code> - default to the pull request's title.</li>\n<li><code>COMMIT_OR_PR_TITLE</code> - default to the commit's title (if only one commit) or the pull request's title (when more than one commit).</li>\n</ul>");
    assert!(body_parameter.required.is_none());
    assert_eq!(body_parameter.type_, BodyParameterType::String);
    assert!(body_parameter.child_params_group.is_none());
    assert_eq!(
      body_parameter.enum_,
      Some(vec![
        Some("PR_TITLE".to_string()),
        Some("COMMIT_OR_PR_TITLE".to_string())
      ])
    );
  }
}
