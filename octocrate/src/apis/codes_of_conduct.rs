use octocrate_core::*;
#[allow(unused_imports)]
use octocrate_types::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use typed_builder::TypedBuilder;

pub mod get_all_codes_of_conduct {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<CodeOfConduct>;
}

pub mod get_conduct_code {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = CodeOfConduct;
}

pub struct GitHubCodesOfConductAPI {
  config: SharedAPIConfig,
}

impl GitHubCodesOfConductAPI {
  pub fn new(config: &SharedAPIConfig) -> Self {
    Self {
      config: config.clone(),
    }
  }

  /// **Get all codes of conduct**
  ///
  /// Returns array of all GitHub's codes of conduct.
  ///
  /// *Documentation*: [https://docs.github.com/rest/codes-of-conduct/codes-of-conduct#get-all-codes-of-conduct](https://docs.github.com/rest/codes-of-conduct/codes-of-conduct#get-all-codes-of-conduct)
  pub fn get_all_codes_of_conduct(&self) -> Request<(), (), get_all_codes_of_conduct::Response> {
    let url = format!("/codes_of_conduct");

    Request::<(), (), get_all_codes_of_conduct::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get a code of conduct**
  ///
  /// Returns information about the specified GitHub code of conduct.
  ///
  /// *Documentation*: [https://docs.github.com/rest/codes-of-conduct/codes-of-conduct#get-a-code-of-conduct](https://docs.github.com/rest/codes-of-conduct/codes-of-conduct#get-a-code-of-conduct)
  pub fn get_conduct_code(
    &self,
    key: impl Into<String>,
  ) -> Request<(), (), get_conduct_code::Response> {
    let key = key.into();
    let url = format!("/codes_of_conduct/{key}");

    Request::<(), (), get_conduct_code::Response>::builder(&self.config)
      .get(url)
      .build()
  }
}
