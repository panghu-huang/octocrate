use octocrate_core::*;
#[allow(unused_imports)]
use crate::types::*;

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
  pub fn render(
    &self,
  ) -> NoContentRequest<MarkdownRenderRequest, ()> {
    let url = format!("/markdown");

    NoContentRequest::<MarkdownRenderRequest, ()>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Render a Markdown document in raw mode**
  ///
  /// You must send Markdown as plain text (using a `Content-Type` header of `text/plain` or `text/x-markdown`) to this endpoint, rather than using JSON format. In raw mode, [GitHub Flavored Markdown](https://github.github.com/gfm/) is not supported and Markdown will be rendered in plain format like a README.md file. Markdown content must be 400 KB or less.
  ///
  /// *Documentation*: [https://docs.github.com/rest/markdown/markdown#render-a-markdown-document-in-raw-mode](https://docs.github.com/rest/markdown/markdown#render-a-markdown-document-in-raw-mode)
  pub fn render_raw(
    &self,
  ) -> NoContentRequest<(), ()> {
    let url = format!("/markdown/raw");

    NoContentRequest::<(), ()>::builder(&self.config)
      .post(url)
      .build()
  }


}
