use handlebars::{handlebars_helper, no_escape, Handlebars};
use serde::Serialize;

pub fn render_template<T>(template: &str, data: &T) -> String
where
  T: Serialize,
{
  let mut handlebars = Handlebars::new();

  handlebars.register_escape_fn(no_escape);

  handlebars_helper!(split_lines: |input: str| {
    input.split('\n').map(|line| {
      line.to_string()
    }).collect::<Vec<String>>()
  });

  handlebars_helper!(is_optional_field: |input: str| {
    input.starts_with("Option<")
  });

  // This helper is used to render a cfg attribute with a list of features
  handlebars_helper!(render_features_cfg: |tags: Vec<String>| {
    match tags.len() {
      0 => String::from(""),
      1 => format!("#[cfg(feature = \"{}\")]", tags[0]),
      _ => {
        let mut cfg = String::from("#[cfg(any(");
        cfg.push_str(&tags.iter().map(|f| format!("feature = \"{}\"", f)).collect::<Vec<String>>().join(", "));
        cfg.push_str("))]");
        cfg
      }
    }
  });

  handlebars.register_helper("is_optional_field", Box::new(is_optional_field));
  handlebars.register_helper("split_lines", Box::new(split_lines));
  handlebars.register_helper("render_features_cfg", Box::new(render_features_cfg));

  handlebars.render_template(template, data).unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_render_template() {
    let template = r#"
    {{#each (split_lines description)}}
    /// {{this}}
    {{/each}}
    {{#each fields}}
    {{#if (is_optional_field this)}}
    Optional field
    {{/if}}
    {{/each}}
      "#;

    let data = serde_json::json!({
      "description": "This is a description\nThis is a description",
      "fields": [
        "Option<String>",
        "String",
      ]
    });

    let rendered = render_template(template, &data);

    assert_eq!(
      rendered.trim(),
      "/// This is a description\n    /// This is a description\n    Optional field"
    );
  }

  #[test]
  fn test_render_features_cfg() {
    let template = r#"
    {{render_features_cfg features}}
      "#;

    let data = serde_json::json!({
      "features": ["feature1", "feature2"]
    });

    let rendered = render_template(template, &data);

    assert_eq!(
      rendered.trim(),
      "#[cfg(any(feature = \"feature1\", feature = \"feature2\"))]"
    );
  }
}
