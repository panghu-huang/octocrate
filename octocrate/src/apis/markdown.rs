use octocrate_core::*;
#[allow(unused_imports)]
use octocrate_types::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use typed_builder::TypedBuilder;

pub mod render {
  #[allow(unused_imports)]
  use super::*;

  /// The rendering mode.
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum RequestMode {
    #[serde(rename = "markdown")]
    Markdown,
    #[serde(rename = "gfm")]
    Gfm,
  }

  impl ToString for RequestMode {
    fn to_string(&self) -> String {
      match self {
        RequestMode::Markdown => "markdown".to_string(),
        RequestMode::Gfm => "gfm".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The repository context to use when creating references in `gfm` mode.  For example, setting `context` to `octo-org/octo-repo` will change the text `#42` into an HTML link to issue 42 in the `octo-org/octo-repo` repository.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub context: Option<String>,
    /// The rendering mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub mode: Option<RequestMode>,
    /// The Markdown text to render in HTML.
    pub text: String,
  }
}

pub mod render_raw {
  #[allow(unused_imports)]
  use super::*;
}

/// Render GitHub flavored markdown
pub struct GitHubMarkdownAPI {
  config: SharedAPIConfig,
}

impl GitHubMarkdownAPI {
  pub fn new(config: &SharedAPIConfig) -> Self {
    Self {
      config: config.clone(),
    }
  }

  /// **Render a Markdown document**
  ///
  ///
  /// *Documentation*: [https://docs.github.com/rest/markdown/markdown#render-a-markdown-document](https://docs.github.com/rest/markdown/markdown#render-a-markdown-document)
  pub fn render(&self) -> NoContentRequest<render::Request, ()> {
    let url = format!("/markdown");

    NoContentRequest::<render::Request, ()>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Render a Markdown document in raw mode**
  ///
  /// You must send Markdown as plain text (using a `Content-Type` header of `text/plain` or `text/x-markdown`) to this endpoint, rather than using JSON format. In raw mode, [GitHub Flavored Markdown](https://github.github.com/gfm/) is not supported and Markdown will be rendered in plain format like a README.md file. Markdown content must be 400 KB or less.
  ///
  /// *Documentation*: [https://docs.github.com/rest/markdown/markdown#render-a-markdown-document-in-raw-mode](https://docs.github.com/rest/markdown/markdown#render-a-markdown-document-in-raw-mode)
  pub fn render_raw(&self) -> NoContentRequest<(), ()> {
    let url = format!("/markdown/raw");

    NoContentRequest::<(), ()>::builder(&self.config)
      .post(url)
      .build()
  }
}
