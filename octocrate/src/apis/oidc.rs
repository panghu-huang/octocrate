use octocrate_core::*;
#[allow(unused_imports)]
use octocrate_types::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use typed_builder::TypedBuilder;

pub mod get_oidc_custom_sub_template_for_org {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = OidcCustomSub;
}

pub mod update_oidc_custom_sub_template_for_org {
  #[allow(unused_imports)]
  use super::*;

  pub type Request = OidcCustomSub;
  pub type Response = EmptyObject;
}

/// Endpoints to manage GitHub OIDC configuration using the REST API.
pub struct GitHubOidcAPI {
  config: SharedAPIConfig,
}

impl GitHubOidcAPI {
  pub fn new(config: &SharedAPIConfig) -> Self {
    Self {
      config: config.clone(),
    }
  }

  /// **Get the customization template for an OIDC subject claim for an organization**
  ///
  /// Gets the customization template for an OpenID Connect (OIDC) subject claim.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `read:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/oidc#get-the-customization-template-for-an-oidc-subject-claim-for-an-organization](https://docs.github.com/rest/actions/oidc#get-the-customization-template-for-an-oidc-subject-claim-for-an-organization)
  pub fn get_oidc_custom_sub_template_for_org(
    &self,
    org: impl Into<String>,
  ) -> Request<(), (), get_oidc_custom_sub_template_for_org::Response> {
    let org = org.into();
    let url = format!("/orgs/{org}/actions/oidc/customization/sub");

    Request::<(), (), get_oidc_custom_sub_template_for_org::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Set the customization template for an OIDC subject claim for an organization**
  ///
  /// Creates or updates the customization template for an OpenID Connect (OIDC) subject claim.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `write:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/actions/oidc#set-the-customization-template-for-an-oidc-subject-claim-for-an-organization](https://docs.github.com/rest/actions/oidc#set-the-customization-template-for-an-oidc-subject-claim-for-an-organization)
  pub fn update_oidc_custom_sub_template_for_org(
    &self,
    org: impl Into<String>,
  ) -> Request<
    update_oidc_custom_sub_template_for_org::Request,
    (),
    update_oidc_custom_sub_template_for_org::Response,
  > {
    let org = org.into();
    let url = format!("/orgs/{org}/actions/oidc/customization/sub");

    Request::<
      update_oidc_custom_sub_template_for_org::Request,
      (),
      update_oidc_custom_sub_template_for_org::Response,
    >::builder(&self.config)
    .put(url)
    .build()
  }
}
