use octocrate_core::*;
#[allow(unused_imports)]
use crate::types::*;

/// View gitignore templates
pub struct GitHubGitignoreAPI {
  config: SharedAPIConfig,
}

impl GitHubGitignoreAPI {
  pub fn new(config: &SharedAPIConfig) -> Self {
    Self {
      config: config.clone(),
    }
  }

  /// **Get a gitignore template**
  ///
  /// Get the content of a gitignore template.
  /// 
  /// This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
  /// 
  /// - **`application/vnd.github.raw+json`**: Returns the raw .gitignore contents.
  ///
  /// *Documentation*: [https://docs.github.com/rest/gitignore/gitignore#get-a-gitignore-template](https://docs.github.com/rest/gitignore/gitignore#get-a-gitignore-template)
  pub fn get_template(
    &self,
    name: impl Into<String>,
  ) -> Request<(), (), GitignoreTemplate> {
    let name = name.into();
    let url = format!("/gitignore/templates/{name}");

    Request::<(), (), GitignoreTemplate>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get all gitignore templates**
  ///
  /// List all templates available to pass as an option when [creating a repository](https://docs.github.com/rest/repos/repos#create-a-repository-for-the-authenticated-user).
  ///
  /// *Documentation*: [https://docs.github.com/rest/gitignore/gitignore#get-all-gitignore-templates](https://docs.github.com/rest/gitignore/gitignore#get-all-gitignore-templates)
  pub fn get_all_templates(
    &self,
  ) -> Request<(), (), StringArray> {
    let url = format!("/gitignore/templates");

    Request::<(), (), StringArray>::builder(&self.config)
      .get(url)
      .build()
  }


}
