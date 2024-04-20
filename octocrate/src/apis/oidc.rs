#[allow(unused_imports)]
use crate::types::*;
use octocrate_core::*;

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
  ) -> Request<(), (), ActionsOidcSubjectCustomization> {
    let org = org.into();
    let url = format!("/orgs/{org}/actions/oidc/customization/sub");

    Request::<(), (), ActionsOidcSubjectCustomization>::builder(&self.config)
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
  ) -> Request<ActionsOidcSubjectCustomization, (), EmptyObject> {
    let org = org.into();
    let url = format!("/orgs/{org}/actions/oidc/customization/sub");

    Request::<ActionsOidcSubjectCustomization, (), EmptyObject>::builder(&self.config)
      .put(url)
      .build()
  }
}
