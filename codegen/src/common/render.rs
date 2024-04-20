use handlebars::{handlebars_helper, no_escape, Handlebars};
use serde::Serialize;

pub fn render_template<T>(template: &str, data: &T) -> String
where
  T: Serialize,
{
  let mut handlebars = Handlebars::new();

  handlebars.register_escape_fn(no_escape);

  handlebars_helper!(split_lines: |input: str| {
    input.split("\n").map(|line| {
      line.to_string()
    }).collect::<Vec<String>>()
  });

  handlebars_helper!(is_optional_field: |input: str| {
    input.starts_with("Option<")
  });

  handlebars.register_helper("is_optional_field", Box::new(is_optional_field));
  handlebars.register_helper("split_lines", Box::new(split_lines));

  let rendered = handlebars.render_template(template, data).unwrap();

  rendered
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
}
