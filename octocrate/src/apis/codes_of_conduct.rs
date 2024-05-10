use octocrate_core::*;
#[allow(unused_imports)]
use octocrate_types::*;

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
  pub fn get_all_codes_of_conduct(&self) -> Request<(), (), Vec<CodeOfConduct>> {
    let url = format!("/codes_of_conduct");

    Request::<(), (), Vec<CodeOfConduct>>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get a code of conduct**
  ///
  /// Returns information about the specified GitHub code of conduct.
  ///
  /// *Documentation*: [https://docs.github.com/rest/codes-of-conduct/codes-of-conduct#get-a-code-of-conduct](https://docs.github.com/rest/codes-of-conduct/codes-of-conduct#get-a-code-of-conduct)
  pub fn get_conduct_code(&self, key: impl Into<String>) -> Request<(), (), CodeOfConduct> {
    let key = key.into();
    let url = format!("/codes_of_conduct/{key}");

    Request::<(), (), CodeOfConduct>::builder(&self.config)
      .get(url)
      .build()
  }
}
