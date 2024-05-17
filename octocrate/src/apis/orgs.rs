use octocrate_core::*;
#[allow(unused_imports)]
use octocrate_types::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use typed_builder::TypedBuilder;

pub mod list {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<OrganizationSimple>;

  /// Query for `List organizations`
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// An organization ID. Only return organizations with an ID greater than this ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub since: Option<i64>,
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
  }
}

pub mod get {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = OrganizationFull;
}

pub mod update {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = OrganizationFull;

  /// Default permission level members have for organization repositories.
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum RequestDefaultRepositoryPermission {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "none")]
    None,
  }

  impl ToString for RequestDefaultRepositoryPermission {
    fn to_string(&self) -> String {
      match self {
        RequestDefaultRepositoryPermission::Read => "read".to_string(),
        RequestDefaultRepositoryPermission::Write => "write".to_string(),
        RequestDefaultRepositoryPermission::Admin => "admin".to_string(),
        RequestDefaultRepositoryPermission::None => "none".to_string(),
      }
    }
  }

  /// Specifies which types of repositories non-admin organization members can create. `private` is only available to repositories that are part of an organization on GitHub Enterprise Cloud.
  /// **Note:** This parameter is deprecated and will be removed in the future. Its return value ignores internal repositories. Using this parameter overrides values set in `members_can_create_repositories`. See the parameter deprecation notice in the operation description for details.
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum RequestMembersAllowedRepositoryCreationType {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "none")]
    None,
  }

  impl ToString for RequestMembersAllowedRepositoryCreationType {
    fn to_string(&self) -> String {
      match self {
        RequestMembersAllowedRepositoryCreationType::All => "all".to_string(),
        RequestMembersAllowedRepositoryCreationType::Private => "private".to_string(),
        RequestMembersAllowedRepositoryCreationType::None => "none".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// Whether GitHub Advanced Security is automatically enabled for new repositories.
    ///
    /// To use this parameter, you must have admin permissions for the repository or be an owner or security manager for the organization that owns the repository. For more information, see "[Managing security managers in your organization](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/managing-security-managers-in-your-organization)."
    ///
    /// You can check which security and analysis features are currently enabled by using a `GET /orgs/{org}` request.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub advanced_security_enabled_for_new_repositories: Option<bool>,
    /// Billing email address. This address is not publicized.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub billing_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub blog: Option<String>,
    /// The company name.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub company: Option<String>,
    /// Default permission level members have for organization repositories.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub default_repository_permission: Option<RequestDefaultRepositoryPermission>,
    /// Whether Dependabot alerts is automatically enabled for new repositories.
    ///
    /// To use this parameter, you must have admin permissions for the repository or be an owner or security manager for the organization that owns the repository. For more information, see "[Managing security managers in your organization](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/managing-security-managers-in-your-organization)."
    ///
    /// You can check which security and analysis features are currently enabled by using a `GET /orgs/{org}` request.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub dependabot_alerts_enabled_for_new_repositories: Option<bool>,
    /// Whether Dependabot security updates is automatically enabled for new repositories.
    ///
    /// To use this parameter, you must have admin permissions for the repository or be an owner or security manager for the organization that owns the repository. For more information, see "[Managing security managers in your organization](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/managing-security-managers-in-your-organization)."
    ///
    /// You can check which security and analysis features are currently enabled by using a `GET /orgs/{org}` request.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub dependabot_security_updates_enabled_for_new_repositories: Option<bool>,
    /// Whether dependency graph is automatically enabled for new repositories.
    ///
    /// To use this parameter, you must have admin permissions for the repository or be an owner or security manager for the organization that owns the repository. For more information, see "[Managing security managers in your organization](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/managing-security-managers-in-your-organization)."
    ///
    /// You can check which security and analysis features are currently enabled by using a `GET /orgs/{org}` request.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub dependency_graph_enabled_for_new_repositories: Option<bool>,
    /// The description of the company. The maximum size is 160 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub description: Option<String>,
    /// The publicly visible email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub email: Option<String>,
    /// Whether an organization can use organization projects.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub has_organization_projects: Option<bool>,
    /// Whether repositories that belong to the organization can use repository projects.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub has_repository_projects: Option<bool>,
    /// The location.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub location: Option<String>,
    /// Specifies which types of repositories non-admin organization members can create. `private` is only available to repositories that are part of an organization on GitHub Enterprise Cloud.
    /// **Note:** This parameter is deprecated and will be removed in the future. Its return value ignores internal repositories. Using this parameter overrides values set in `members_can_create_repositories`. See the parameter deprecation notice in the operation description for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub members_allowed_repository_creation_type:
      Option<RequestMembersAllowedRepositoryCreationType>,
    /// Whether organization members can create internal repositories, which are visible to all enterprise members. You can only allow members to create internal repositories if your organization is associated with an enterprise account using GitHub Enterprise Cloud or GitHub Enterprise Server 2.20+. For more information, see "[Restricting repository creation in your organization](https://docs.github.com/github/setting-up-and-managing-organizations-and-teams/restricting-repository-creation-in-your-organization)" in the GitHub Help documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub members_can_create_internal_repositories: Option<bool>,
    /// Whether organization members can create GitHub Pages sites. Existing published sites will not be impacted.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub members_can_create_pages: Option<bool>,
    /// Whether organization members can create private GitHub Pages sites. Existing published sites will not be impacted.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub members_can_create_private_pages: Option<bool>,
    /// Whether organization members can create private repositories, which are visible to organization members with permission. For more information, see "[Restricting repository creation in your organization](https://docs.github.com/github/setting-up-and-managing-organizations-and-teams/restricting-repository-creation-in-your-organization)" in the GitHub Help documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub members_can_create_private_repositories: Option<bool>,
    /// Whether organization members can create public GitHub Pages sites. Existing published sites will not be impacted.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub members_can_create_public_pages: Option<bool>,
    /// Whether organization members can create public repositories, which are visible to anyone. For more information, see "[Restricting repository creation in your organization](https://docs.github.com/github/setting-up-and-managing-organizations-and-teams/restricting-repository-creation-in-your-organization)" in the GitHub Help documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub members_can_create_public_repositories: Option<bool>,
    /// Whether of non-admin organization members can create repositories. **Note:** A parameter can override this parameter. See `members_allowed_repository_creation_type` in this table for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub members_can_create_repositories: Option<bool>,
    /// Whether organization members can fork private organization repositories.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub members_can_fork_private_repositories: Option<bool>,
    /// The shorthand name of the company.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub name: Option<String>,
    /// Whether secret scanning is automatically enabled for new repositories.
    ///
    /// To use this parameter, you must have admin permissions for the repository or be an owner or security manager for the organization that owns the repository. For more information, see "[Managing security managers in your organization](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/managing-security-managers-in-your-organization)."
    ///
    /// You can check which security and analysis features are currently enabled by using a `GET /orgs/{org}` request.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub secret_scanning_enabled_for_new_repositories: Option<bool>,
    /// If `secret_scanning_push_protection_custom_link_enabled` is true, the URL that will be displayed to contributors who are blocked from pushing a secret.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub secret_scanning_push_protection_custom_link: Option<String>,
    /// Whether a custom link is shown to contributors who are blocked from pushing a secret by push protection.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub secret_scanning_push_protection_custom_link_enabled: Option<bool>,
    /// Whether secret scanning push protection is automatically enabled for new repositories.
    ///
    /// To use this parameter, you must have admin permissions for the repository or be an owner or security manager for the organization that owns the repository. For more information, see "[Managing security managers in your organization](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/managing-security-managers-in-your-organization)."
    ///
    /// You can check which security and analysis features are currently enabled by using a `GET /orgs/{org}` request.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub secret_scanning_push_protection_enabled_for_new_repositories: Option<bool>,
    /// The Twitter username of the company.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub twitter_username: Option<String>,
    /// Whether contributors to organization repositories are required to sign off on commits they make through GitHub's web interface.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub web_commit_signoff_required: Option<bool>,
  }
}

pub mod delete {
  #[allow(unused_imports)]
  use super::*;
}

pub mod list_blocked_users {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<SimpleUser>;

  /// Query for `List users blocked by an organization`
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
  }
}

pub mod check_blocked_user {
  #[allow(unused_imports)]
  use super::*;
}

pub mod block_user {
  #[allow(unused_imports)]
  use super::*;
}

pub mod unblock_user {
  #[allow(unused_imports)]
  use super::*;
}

pub mod list_failed_invitations {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<OrganizationInvitation>;

  /// Query for `List failed organization invitations`
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
  }
}

pub mod list_webhooks {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<OrgHook>;

  /// Query for `List organization webhooks`
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
  }
}

pub mod create_webhook {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = OrgHook;

  /// Key/value pairs to provide settings for this webhook.
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct RequestConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub content_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub insecure_ssl: Option<StringOrNumber>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub secret: Option<String>,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub username: Option<String>,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// Determines if notifications are sent when the webhook is triggered. Set to `true` to send notifications.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub active: Option<bool>,
    /// Key/value pairs to provide settings for this webhook.
    pub config: RequestConfig,
    /// Determines what [events](https://docs.github.com/webhooks/event-payloads) the hook is triggered for. Set to `["*"]` to receive all possible events.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub events: Option<Vec<String>>,
    /// Must be passed as "web".
    pub name: String,
  }
}

pub mod get_webhook {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = OrgHook;
}

pub mod update_webhook {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = OrgHook;

  /// Key/value pairs to provide settings for this webhook.
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct RequestConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub content_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub insecure_ssl: Option<StringOrNumber>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub secret: Option<String>,
    pub url: String,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// Determines if notifications are sent when the webhook is triggered. Set to `true` to send notifications.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub active: Option<bool>,
    /// Key/value pairs to provide settings for this webhook.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub config: Option<RequestConfig>,
    /// Determines what [events](https://docs.github.com/webhooks/event-payloads) the hook is triggered for.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub events: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub name: Option<String>,
  }
}

pub mod delete_webhook {
  #[allow(unused_imports)]
  use super::*;
}

pub mod get_webhook_config_for_org {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = WebhookConfig;
}

pub mod update_webhook_config_for_org {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = WebhookConfig;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub content_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub insecure_ssl: Option<StringOrNumber>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub secret: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub url: Option<String>,
  }
}

pub mod list_webhook_deliveries {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<HookDeliveryItem>;

  /// Query for `List deliveries for an organization webhook`
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// Used for pagination: the starting delivery from which the page of deliveries is fetched. Refer to the `link` header for the next and previous page cursors.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub redelivery: Option<bool>,
  }
}

pub mod get_webhook_delivery {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = HookDelivery;
}

pub mod redeliver_webhook_delivery {
  #[allow(unused_imports)]
  use super::*;
}

pub mod ping_webhook {
  #[allow(unused_imports)]
  use super::*;
}

pub mod list_app_installations {
  #[allow(unused_imports)]
  use super::*;

  /// Query for `List app installations for an organization`
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Response {
    pub installations: Vec<Installation>,
    pub total_count: i64,
  }
}

pub mod list_pending_invitations {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<OrganizationInvitation>;

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QueryRole {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "direct_member")]
    DirectMember,
    #[serde(rename = "billing_manager")]
    BillingManager,
    #[serde(rename = "hiring_manager")]
    HiringManager,
  }

  impl ToString for QueryRole {
    fn to_string(&self) -> String {
      match self {
        QueryRole::All => "all".to_string(),
        QueryRole::Admin => "admin".to_string(),
        QueryRole::DirectMember => "direct_member".to_string(),
        QueryRole::BillingManager => "billing_manager".to_string(),
        QueryRole::HiringManager => "hiring_manager".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QueryInvitationSource {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "member")]
    Member,
    #[serde(rename = "scim")]
    Scim,
  }

  impl ToString for QueryInvitationSource {
    fn to_string(&self) -> String {
      match self {
        QueryInvitationSource::All => "all".to_string(),
        QueryInvitationSource::Member => "member".to_string(),
        QueryInvitationSource::Scim => "scim".to_string(),
      }
    }
  }

  /// Query for `List pending organization invitations`
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
    /// Filter invitations by their member role.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub role: Option<QueryRole>,
    /// Filter invitations by their invitation source.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub invitation_source: Option<QueryInvitationSource>,
  }
}

pub mod create_invitation {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = OrganizationInvitation;

  /// The role for the new member.
  ///  * `admin` - Organization owners with full administrative rights to the organization and complete access to all repositories and teams.  
  ///  * `direct_member` - Non-owner organization members with ability to see other members and join teams by invitation.  
  ///  * `billing_manager` - Non-owner organization members with ability to manage the billing settings of your organization.
  ///  * `reinstate` - The previous role assigned to the invitee before they were removed from your organization. Can be one of the roles listed above. Only works if the invitee was previously part of your organization.
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum RequestRole {
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "direct_member")]
    DirectMember,
    #[serde(rename = "billing_manager")]
    BillingManager,
    #[serde(rename = "reinstate")]
    Reinstate,
  }

  impl ToString for RequestRole {
    fn to_string(&self) -> String {
      match self {
        RequestRole::Admin => "admin".to_string(),
        RequestRole::DirectMember => "direct_member".to_string(),
        RequestRole::BillingManager => "billing_manager".to_string(),
        RequestRole::Reinstate => "reinstate".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// **Required unless you provide `invitee_id`**. Email address of the person you are inviting, which can be an existing GitHub user.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub email: Option<String>,
    /// **Required unless you provide `email`**. GitHub user ID for the person you are inviting.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub invitee_id: Option<i64>,
    /// The role for the new member.
    ///  * `admin` - Organization owners with full administrative rights to the organization and complete access to all repositories and teams.  
    ///  * `direct_member` - Non-owner organization members with ability to see other members and join teams by invitation.  
    ///  * `billing_manager` - Non-owner organization members with ability to manage the billing settings of your organization.
    ///  * `reinstate` - The previous role assigned to the invitee before they were removed from your organization. Can be one of the roles listed above. Only works if the invitee was previously part of your organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub role: Option<RequestRole>,
    /// Specify IDs for the teams you want to invite new members to.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub team_ids: Option<Vec<i64>>,
  }
}

pub mod cancel_invitation {
  #[allow(unused_imports)]
  use super::*;
}

pub mod list_invitation_teams {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Team>;

  /// Query for `List organization invitation teams`
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
  }
}

pub mod list_members {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<SimpleUser>;

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QueryFilter {
    #[serde(rename = "2fa_disabled")]
    TwoFADisabled,
    #[serde(rename = "all")]
    All,
  }

  impl ToString for QueryFilter {
    fn to_string(&self) -> String {
      match self {
        QueryFilter::TwoFADisabled => "2fa_disabled".to_string(),
        QueryFilter::All => "all".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QueryRole {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "member")]
    Member,
  }

  impl ToString for QueryRole {
    fn to_string(&self) -> String {
      match self {
        QueryRole::All => "all".to_string(),
        QueryRole::Admin => "admin".to_string(),
        QueryRole::Member => "member".to_string(),
      }
    }
  }

  /// Query for `List organization members`
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// Filter members returned in the list. `2fa_disabled` means that only members without [two-factor authentication](https://github.com/blog/1614-two-factor-authentication) enabled will be returned. This options is only available for organization owners.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub filter: Option<QueryFilter>,
    /// Filter members returned by their role.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub role: Option<QueryRole>,
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
  }
}

pub mod check_membership_for_user {
  #[allow(unused_imports)]
  use super::*;
}

pub mod remove_member {
  #[allow(unused_imports)]
  use super::*;
}

pub mod get_membership_for_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = OrgMembership;
}

pub mod set_membership_for_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = OrgMembership;

  /// The role to give the user in the organization. Can be one of:  
  ///  * `admin` - The user will become an owner of the organization.  
  ///  * `member` - The user will become a non-owner member of the organization.
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum RequestRole {
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "member")]
    Member,
  }

  impl ToString for RequestRole {
    fn to_string(&self) -> String {
      match self {
        RequestRole::Admin => "admin".to_string(),
        RequestRole::Member => "member".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The role to give the user in the organization. Can be one of:  
    ///  * `admin` - The user will become an owner of the organization.  
    ///  * `member` - The user will become a non-owner member of the organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub role: Option<RequestRole>,
  }
}

pub mod remove_membership_for_user {
  #[allow(unused_imports)]
  use super::*;
}

pub mod list_organization_fine_grained_permissions {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<OrganizationFineGrainedPermission>;
}

pub mod list_org_roles {
  #[allow(unused_imports)]
  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Response {
    /// The list of organization roles available to the organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub roles: Option<Vec<OrganizationRole>>,
    /// The total number of organization roles available to the organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub total_count: Option<i64>,
  }
}

pub mod create_custom_organization_role {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = OrganizationRole;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// A short description about the intended usage of this role or what permissions it grants.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub description: Option<String>,
    /// The name of the custom role.
    pub name: String,
    /// A list of additional permissions included in this role.
    pub permissions: Vec<String>,
  }
}

pub mod revoke_all_org_roles_team {
  #[allow(unused_imports)]
  use super::*;
}

pub mod assign_team_to_org_role {
  #[allow(unused_imports)]
  use super::*;
}

pub mod revoke_org_role_team {
  #[allow(unused_imports)]
  use super::*;
}

pub mod revoke_all_org_roles_user {
  #[allow(unused_imports)]
  use super::*;
}

pub mod assign_user_to_org_role {
  #[allow(unused_imports)]
  use super::*;
}

pub mod revoke_org_role_user {
  #[allow(unused_imports)]
  use super::*;
}

pub mod get_org_role {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = OrganizationRole;
}

pub mod patch_custom_organization_role {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = OrganizationRole;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// A short description about the intended usage of this role or what permissions it grants.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub description: Option<String>,
    /// The name of the custom role.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub name: Option<String>,
    /// A list of additional permissions included in this role.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub permissions: Option<Vec<String>>,
  }
}

pub mod delete_custom_organization_role {
  #[allow(unused_imports)]
  use super::*;
}

pub mod list_org_role_teams {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Team>;

  /// Query for `List teams that are assigned to an organization role`
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
  }
}

pub mod list_org_role_users {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<SimpleUser>;

  /// Query for `List users that are assigned to an organization role`
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
  }
}

pub mod list_outside_collaborators {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<SimpleUser>;

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QueryFilter {
    #[serde(rename = "2fa_disabled")]
    TwoFADisabled,
    #[serde(rename = "all")]
    All,
  }

  impl ToString for QueryFilter {
    fn to_string(&self) -> String {
      match self {
        QueryFilter::TwoFADisabled => "2fa_disabled".to_string(),
        QueryFilter::All => "all".to_string(),
      }
    }
  }

  /// Query for `List outside collaborators for an organization`
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// Filter the list of outside collaborators. `2fa_disabled` means that only outside collaborators without [two-factor authentication](https://github.com/blog/1614-two-factor-authentication) enabled will be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub filter: Option<QueryFilter>,
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
  }
}

pub mod convert_member_to_outside_collaborator {
  #[allow(unused_imports)]
  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// When set to `true`, the request will be performed asynchronously. Returns a 202 status code when the job is successfully queued.
    #[serde(rename = "async")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub async_: Option<bool>,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Response {}
}

pub mod remove_outside_collaborator {
  #[allow(unused_imports)]
  use super::*;
}

pub mod list_pat_grant_requests {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<OrganizationProgrammaticAccessGrantRequest>;

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QuerySort {
    #[serde(rename = "created_at")]
    CreatedAt,
  }

  impl ToString for QuerySort {
    fn to_string(&self) -> String {
      match self {
        QuerySort::CreatedAt => "created_at".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QueryDirection {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
  }

  impl ToString for QueryDirection {
    fn to_string(&self) -> String {
      match self {
        QueryDirection::Asc => "asc".to_string(),
        QueryDirection::Desc => "desc".to_string(),
      }
    }
  }

  /// Query for `List requests to access organization resources with fine-grained personal access tokens`
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
    /// The property by which to sort the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub sort: Option<QuerySort>,
    /// The direction to sort the results by.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub direction: Option<QueryDirection>,
    /// A list of owner usernames to use to filter the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub owner: Option<Vec<String>>,
    /// The name of the repository to use to filter the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub repository: Option<String>,
    /// The permission to use to filter the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub permission: Option<String>,
    /// Only show fine-grained personal access tokens used before the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub last_used_before: Option<String>,
    /// Only show fine-grained personal access tokens used after the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub last_used_after: Option<String>,
  }
}

pub mod review_pat_grant_requests_in_bulk {
  #[allow(unused_imports)]
  use super::*;

  /// Action to apply to the requests.
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum RequestAction {
    #[serde(rename = "approve")]
    Approve,
    #[serde(rename = "deny")]
    Deny,
  }

  impl ToString for RequestAction {
    fn to_string(&self) -> String {
      match self {
        RequestAction::Approve => "approve".to_string(),
        RequestAction::Deny => "deny".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// Action to apply to the requests.
    pub action: RequestAction,
    /// Unique identifiers of the requests for access via fine-grained personal access token. Must be formed of between 1 and 100 `pat_request_id` values.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub pat_request_ids: Option<Vec<i64>>,
    /// Reason for approving or denying the requests. Max 1024 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub reason: Option<String>,
  }
}

pub mod review_pat_grant_request {
  #[allow(unused_imports)]
  use super::*;

  /// Action to apply to the request.
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum RequestAction {
    #[serde(rename = "approve")]
    Approve,
    #[serde(rename = "deny")]
    Deny,
  }

  impl ToString for RequestAction {
    fn to_string(&self) -> String {
      match self {
        RequestAction::Approve => "approve".to_string(),
        RequestAction::Deny => "deny".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// Action to apply to the request.
    pub action: RequestAction,
    /// Reason for approving or denying the request. Max 1024 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub reason: Option<String>,
  }
}

pub mod list_pat_grant_request_repositories {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<MinimalRepository>;

  /// Query for `List repositories requested to be accessed by a fine-grained personal access token`
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
  }
}

pub mod list_pat_grants {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<OrganizationProgrammaticAccessGrant>;

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QuerySort {
    #[serde(rename = "created_at")]
    CreatedAt,
  }

  impl ToString for QuerySort {
    fn to_string(&self) -> String {
      match self {
        QuerySort::CreatedAt => "created_at".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QueryDirection {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
  }

  impl ToString for QueryDirection {
    fn to_string(&self) -> String {
      match self {
        QueryDirection::Asc => "asc".to_string(),
        QueryDirection::Desc => "desc".to_string(),
      }
    }
  }

  /// Query for `List fine-grained personal access tokens with access to organization resources`
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
    /// The property by which to sort the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub sort: Option<QuerySort>,
    /// The direction to sort the results by.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub direction: Option<QueryDirection>,
    /// A list of owner usernames to use to filter the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub owner: Option<Vec<String>>,
    /// The name of the repository to use to filter the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub repository: Option<String>,
    /// The permission to use to filter the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub permission: Option<String>,
    /// Only show fine-grained personal access tokens used before the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub last_used_before: Option<String>,
    /// Only show fine-grained personal access tokens used after the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub last_used_after: Option<String>,
  }
}

pub mod update_pat_accesses {
  #[allow(unused_imports)]
  use super::*;

  /// Action to apply to the fine-grained personal access token.
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum RequestAction {
    #[serde(rename = "revoke")]
    Revoke,
  }

  impl ToString for RequestAction {
    fn to_string(&self) -> String {
      match self {
        RequestAction::Revoke => "revoke".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// Action to apply to the fine-grained personal access token.
    pub action: RequestAction,
    /// The IDs of the fine-grained personal access tokens.
    pub pat_ids: Vec<i64>,
  }
}

pub mod update_pat_access {
  #[allow(unused_imports)]
  use super::*;

  /// Action to apply to the fine-grained personal access token.
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum RequestAction {
    #[serde(rename = "revoke")]
    Revoke,
  }

  impl ToString for RequestAction {
    fn to_string(&self) -> String {
      match self {
        RequestAction::Revoke => "revoke".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// Action to apply to the fine-grained personal access token.
    pub action: RequestAction,
  }
}

pub mod list_pat_grant_repositories {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<MinimalRepository>;

  /// Query for `List repositories a fine-grained personal access token has access to`
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
  }
}

pub mod get_all_custom_properties {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<OrgCustomProperty>;
}

pub mod create_or_update_custom_properties {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<OrgCustomProperty>;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The array of custom properties to create or update.
    pub properties: Vec<OrgCustomProperty>,
  }
}

pub mod get_custom_property {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = OrgCustomProperty;
}

pub mod create_or_update_custom_property {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = OrgCustomProperty;

  /// The type of the value for the property
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum RequestValueType {
    #[serde(rename = "string")]
    String,
    #[serde(rename = "single_select")]
    SingleSelect,
  }

  impl ToString for RequestValueType {
    fn to_string(&self) -> String {
      match self {
        RequestValueType::String => "string".to_string(),
        RequestValueType::SingleSelect => "single_select".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// An ordered list of the allowed values of the property.
    /// The property can have up to 200 allowed values.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub allowed_values: Option<Vec<String>>,
    /// Default value of the property
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub default_value: Option<Vec<String>>,
    /// Short description of the property
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub description: Option<String>,
    /// Whether the property is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub required: Option<bool>,
    /// The type of the value for the property
    pub value_type: RequestValueType,
  }
}

pub mod remove_custom_property {
  #[allow(unused_imports)]
  use super::*;
}

pub mod list_custom_properties_values_for_repos {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<OrgRepoCustomPropertyValues>;

  /// Query for `List custom property values for organization repositories`
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
    /// Finds repositories in the organization with a query containing one or more search keywords and qualifiers. Qualifiers allow you to limit your search to specific areas of GitHub. The REST API supports the same qualifiers as the web interface for GitHub. To learn more about the format of the query, see [Constructing a search query](https://docs.github.com/rest/search/search#constructing-a-search-query). See "[Searching for repositories](https://docs.github.com/articles/searching-for-repositories/)" for a detailed list of qualifiers.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub repository_query: Option<String>,
  }
}

pub mod create_or_update_custom_properties_values_for_repos {
  #[allow(unused_imports)]
  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// List of custom property names and associated values to apply to the repositories.
    pub properties: Vec<CustomPropertyValue>,
    /// The names of repositories that the custom property values will be applied to.
    pub repository_names: Vec<String>,
  }
}

pub mod list_public_members {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<SimpleUser>;

  /// Query for `List public organization members`
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
  }
}

pub mod check_public_membership_for_user {
  #[allow(unused_imports)]
  use super::*;
}

pub mod set_public_membership_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;
}

pub mod remove_public_membership_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;
}

pub mod list_security_manager_teams {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<TeamSimple>;
}

pub mod add_security_manager_team {
  #[allow(unused_imports)]
  use super::*;
}

pub mod remove_security_manager_team {
  #[allow(unused_imports)]
  use super::*;
}

pub mod enable_or_disable_security_product_on_all_org_repos {
  #[allow(unused_imports)]
  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum ParametersSecurityProduct {
    #[serde(rename = "dependency_graph")]
    DependencyGraph,
    #[serde(rename = "dependabot_alerts")]
    DependabotAlerts,
    #[serde(rename = "dependabot_security_updates")]
    DependabotSecurityUpdates,
    #[serde(rename = "advanced_security")]
    AdvancedSecurity,
    #[serde(rename = "code_scanning_default_setup")]
    CodeScanningDefaultSetup,
    #[serde(rename = "secret_scanning")]
    SecretScanning,
    #[serde(rename = "secret_scanning_push_protection")]
    SecretScanningPushProtection,
  }

  impl ToString for ParametersSecurityProduct {
    fn to_string(&self) -> String {
      match self {
        ParametersSecurityProduct::DependencyGraph => "dependency_graph".to_string(),
        ParametersSecurityProduct::DependabotAlerts => "dependabot_alerts".to_string(),
        ParametersSecurityProduct::DependabotSecurityUpdates => {
          "dependabot_security_updates".to_string()
        }
        ParametersSecurityProduct::AdvancedSecurity => "advanced_security".to_string(),
        ParametersSecurityProduct::CodeScanningDefaultSetup => {
          "code_scanning_default_setup".to_string()
        }
        ParametersSecurityProduct::SecretScanning => "secret_scanning".to_string(),
        ParametersSecurityProduct::SecretScanningPushProtection => {
          "secret_scanning_push_protection".to_string()
        }
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum ParametersEnablement {
    #[serde(rename = "enable_all")]
    EnableAll,
    #[serde(rename = "disable_all")]
    DisableAll,
  }

  impl ToString for ParametersEnablement {
    fn to_string(&self) -> String {
      match self {
        ParametersEnablement::EnableAll => "enable_all".to_string(),
        ParametersEnablement::DisableAll => "disable_all".to_string(),
      }
    }
  }

  /// CodeQL query suite to be used. If you specify the `query_suite` parameter, the default setup will be configured with this query suite only on all repositories that didn't have default setup already configured. It will not change the query suite on repositories that already have default setup configured.
  /// If you don't specify any `query_suite` in your request, the preferred query suite of the organization will be applied.
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum RequestQuerySuite {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "extended")]
    Extended,
  }

  impl ToString for RequestQuerySuite {
    fn to_string(&self) -> String {
      match self {
        RequestQuerySuite::Default => "default".to_string(),
        RequestQuerySuite::Extended => "extended".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// CodeQL query suite to be used. If you specify the `query_suite` parameter, the default setup will be configured with this query suite only on all repositories that didn't have default setup already configured. It will not change the query suite on repositories that already have default setup configured.
    /// If you don't specify any `query_suite` in your request, the preferred query suite of the organization will be applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub query_suite: Option<RequestQuerySuite>,
  }
}

pub mod list_memberships_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<OrgMembership>;

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QueryState {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "pending")]
    Pending,
  }

  impl ToString for QueryState {
    fn to_string(&self) -> String {
      match self {
        QueryState::Active => "active".to_string(),
        QueryState::Pending => "pending".to_string(),
      }
    }
  }

  /// Query for `List organization memberships for the authenticated user`
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// Indicates the state of the memberships to return. If not specified, the API returns both active and pending memberships.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub state: Option<QueryState>,
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
  }
}

pub mod get_membership_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = OrgMembership;
}

pub mod update_membership_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = OrgMembership;

  /// The state that the membership should be in. Only `"active"` will be accepted.
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum RequestState {
    #[serde(rename = "active")]
    Active,
  }

  impl ToString for RequestState {
    fn to_string(&self) -> String {
      match self {
        RequestState::Active => "active".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The state that the membership should be in. Only `"active"` will be accepted.
    pub state: RequestState,
  }
}

pub mod list_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<OrganizationSimple>;

  /// Query for `List organizations for the authenticated user`
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
  }
}

pub mod list_for_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<OrganizationSimple>;

  /// Query for `List organizations for a user`
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
  }
}

/// Interact with GitHub Orgs.
pub struct GitHubOrgsAPI {
  config: SharedAPIConfig,
}

impl GitHubOrgsAPI {
  pub fn new(config: &SharedAPIConfig) -> Self {
    Self {
      config: config.clone(),
    }
  }

  /// **List organizations**
  ///
  /// Lists all organizations, in the order that they were created.
  ///
  /// **Note:** Pagination is powered exclusively by the `since` parameter. Use the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers) to get the URL for the next page of organizations.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/orgs#list-organizations](https://docs.github.com/rest/orgs/orgs#list-organizations)
  pub fn list(&self) -> Request<(), list::Query, list::Response> {
    let url = format!("/organizations");

    Request::<(), list::Query, list::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get an organization**
  ///
  /// Gets information about an organization.
  ///
  /// When the value of `two_factor_requirement_enabled` is `true`, the organization requires all members, billing managers, and outside collaborators to enable [two-factor authentication](https://docs.github.com/articles/securing-your-account-with-two-factor-authentication-2fa/).
  ///
  /// To see the full details about an organization, the authenticated user must be an organization owner.
  ///
  /// The values returned by this endpoint are set by the "Update an organization" endpoint. If your organization set a default security configuration (beta), the following values retrieved from the "Update an organization" endpoint have been overwritten by that configuration:
  ///
  /// - advanced_security_enabled_for_new_repositories
  /// - dependabot_alerts_enabled_for_new_repositories
  /// - dependabot_security_updates_enabled_for_new_repositories
  /// - dependency_graph_enabled_for_new_repositories
  /// - secret_scanning_enabled_for_new_repositories
  /// - secret_scanning_push_protection_enabled_for_new_repositories
  ///
  /// For more information on security configurations, see "[Enabling security features at scale](https://docs.github.com/code-security/securing-your-organization/introduction-to-securing-your-organization-at-scale/about-enabling-security-features-at-scale)."
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to see the full details about an organization.
  ///
  /// To see information about an organization's GitHub plan, GitHub Apps need the `Organization plan` permission.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/orgs#get-an-organization](https://docs.github.com/rest/orgs/orgs#get-an-organization)
  pub fn get(&self, org: impl Into<String>) -> Request<(), (), get::Response> {
    let org = org.into();
    let url = format!("/orgs/{org}");

    Request::<(), (), get::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Update an organization**
  ///
  /// **Parameter Deprecation Notice:** GitHub will replace and discontinue `members_allowed_repository_creation_type` in favor of more granular permissions. The new input parameters are `members_can_create_public_repositories`, `members_can_create_private_repositories` for all organizations and `members_can_create_internal_repositories` for organizations associated with an enterprise account using GitHub Enterprise Cloud or GitHub Enterprise Server 2.20+. For more information, see the [blog post](https://developer.github.com/changes/2019-12-03-internal-visibility-changes).
  ///
  /// Updates the organization's profile and member privileges.
  ///
  /// With security configurations (beta), your organization can choose a default security configuration which will automatically apply a set of security enablement settings to new repositories in your organization based on their visibility. For targeted repositories, the following attributes will be overridden by the default security configuration:
  ///
  /// - advanced_security_enabled_for_new_repositories
  /// - dependabot_alerts_enabled_for_new_repositories
  /// - dependabot_security_updates_enabled_for_new_repositories
  /// - dependency_graph_enabled_for_new_repositories
  /// - secret_scanning_enabled_for_new_repositories
  /// - secret_scanning_push_protection_enabled_for_new_repositories
  ///
  /// For more information on setting a default security configuration, see "[Enabling security features at scale](https://docs.github.com/code-security/securing-your-organization/introduction-to-securing-your-organization-at-scale/about-enabling-security-features-at-scale)."
  ///
  /// The authenticated user must be an organization owner to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` or `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/orgs#update-an-organization](https://docs.github.com/rest/orgs/orgs#update-an-organization)
  pub fn update(&self, org: impl Into<String>) -> Request<update::Request, (), update::Response> {
    let org = org.into();
    let url = format!("/orgs/{org}");

    Request::<update::Request, (), update::Response>::builder(&self.config)
      .patch(url)
      .build()
  }

  /// **Delete an organization**
  ///
  /// Deletes an organization and all its repositories.
  ///
  /// The organization login will be unavailable for 90 days after deletion.
  ///
  /// Please review the Terms of Service regarding account deletion before using this endpoint:
  ///
  /// https://docs.github.com/site-policy/github-terms/github-terms-of-service
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/orgs#delete-an-organization](https://docs.github.com/rest/orgs/orgs#delete-an-organization)
  pub fn delete(&self, org: impl Into<String>) -> NoContentRequest<(), ()> {
    let org = org.into();
    let url = format!("/orgs/{org}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List users blocked by an organization**
  ///
  /// List the users blocked by an organization.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/blocking#list-users-blocked-by-an-organization](https://docs.github.com/rest/orgs/blocking#list-users-blocked-by-an-organization)
  pub fn list_blocked_users(
    &self,
    org: impl Into<String>,
  ) -> Request<(), list_blocked_users::Query, list_blocked_users::Response> {
    let org = org.into();
    let url = format!("/orgs/{org}/blocks");

    Request::<(), list_blocked_users::Query, list_blocked_users::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Check if a user is blocked by an organization**
  ///
  /// Returns a 204 if the given user is blocked by the given organization. Returns a 404 if the organization is not blocking the user, or if the user account has been identified as spam by GitHub.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/blocking#check-if-a-user-is-blocked-by-an-organization](https://docs.github.com/rest/orgs/blocking#check-if-a-user-is-blocked-by-an-organization)
  pub fn check_blocked_user(
    &self,
    org: impl Into<String>,
    username: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let username = username.into();
    let url = format!("/orgs/{org}/blocks/{username}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Block a user from an organization**
  ///
  /// Blocks the given user on behalf of the specified organization and returns a 204. If the organization cannot block the given user a 422 is returned.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/blocking#block-a-user-from-an-organization](https://docs.github.com/rest/orgs/blocking#block-a-user-from-an-organization)
  pub fn block_user(
    &self,
    org: impl Into<String>,
    username: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let username = username.into();
    let url = format!("/orgs/{org}/blocks/{username}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **Unblock a user from an organization**
  ///
  /// Unblocks the given user on behalf of the specified organization.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/blocking#unblock-a-user-from-an-organization](https://docs.github.com/rest/orgs/blocking#unblock-a-user-from-an-organization)
  pub fn unblock_user(
    &self,
    org: impl Into<String>,
    username: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let username = username.into();
    let url = format!("/orgs/{org}/blocks/{username}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List failed organization invitations**
  ///
  /// The return hash contains `failed_at` and `failed_reason` fields which represent the time at which the invitation failed and the reason for the failure.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/members#list-failed-organization-invitations](https://docs.github.com/rest/orgs/members#list-failed-organization-invitations)
  pub fn list_failed_invitations(
    &self,
    org: impl Into<String>,
  ) -> Request<(), list_failed_invitations::Query, list_failed_invitations::Response> {
    let org = org.into();
    let url = format!("/orgs/{org}/failed_invitations");

    Request::<(), list_failed_invitations::Query, list_failed_invitations::Response>::builder(
      &self.config,
    )
    .get(url)
    .build()
  }

  /// **List organization webhooks**
  ///
  /// You must be an organization owner to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need `admin:org_hook` scope. OAuth apps cannot list, view, or edit
  /// webhooks that they did not create and users cannot list, view, or edit webhooks that were created by OAuth apps.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/webhooks#list-organization-webhooks](https://docs.github.com/rest/orgs/webhooks#list-organization-webhooks)
  pub fn list_webhooks(
    &self,
    org: impl Into<String>,
  ) -> Request<(), list_webhooks::Query, list_webhooks::Response> {
    let org = org.into();
    let url = format!("/orgs/{org}/hooks");

    Request::<(), list_webhooks::Query, list_webhooks::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create an organization webhook**
  ///
  /// Create a hook that posts payloads in JSON format.
  ///
  /// You must be an organization owner to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need `admin:org_hook` scope. OAuth apps cannot list, view, or
  /// edit webhooks that they did not create and users cannot list, view, or edit webhooks that were created by OAuth apps.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/webhooks#create-an-organization-webhook](https://docs.github.com/rest/orgs/webhooks#create-an-organization-webhook)
  pub fn create_webhook(
    &self,
    org: impl Into<String>,
  ) -> Request<create_webhook::Request, (), create_webhook::Response> {
    let org = org.into();
    let url = format!("/orgs/{org}/hooks");

    Request::<create_webhook::Request, (), create_webhook::Response>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Get an organization webhook**
  ///
  /// Returns a webhook configured in an organization. To get only the webhook
  /// `config` properties, see "[Get a webhook configuration for an organization](/rest/orgs/webhooks#get-a-webhook-configuration-for-an-organization).
  ///
  /// You must be an organization owner to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need `admin:org_hook` scope. OAuth apps cannot list, view, or edit
  /// webhooks that they did not create and users cannot list, view, or edit webhooks that were created by OAuth apps.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/webhooks#get-an-organization-webhook](https://docs.github.com/rest/orgs/webhooks#get-an-organization-webhook)
  pub fn get_webhook(
    &self,
    org: impl Into<String>,
    hook_id: impl Into<i64>,
  ) -> Request<(), (), get_webhook::Response> {
    let org = org.into();
    let hook_id = hook_id.into();
    let url = format!("/orgs/{org}/hooks/{hook_id}");

    Request::<(), (), get_webhook::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Update an organization webhook**
  ///
  /// Updates a webhook configured in an organization. When you update a webhook,
  /// the `secret` will be overwritten. If you previously had a `secret` set, you must
  /// provide the same `secret` or set a new `secret` or the secret will be removed. If
  /// you are only updating individual webhook `config` properties, use "[Update a webhook
  /// configuration for an organization](/rest/orgs/webhooks#update-a-webhook-configuration-for-an-organization)".
  ///
  /// You must be an organization owner to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need `admin:org_hook` scope. OAuth apps cannot list, view, or edit
  /// webhooks that they did not create and users cannot list, view, or edit webhooks that were created by OAuth apps.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/webhooks#update-an-organization-webhook](https://docs.github.com/rest/orgs/webhooks#update-an-organization-webhook)
  pub fn update_webhook(
    &self,
    org: impl Into<String>,
    hook_id: impl Into<i64>,
  ) -> Request<update_webhook::Request, (), update_webhook::Response> {
    let org = org.into();
    let hook_id = hook_id.into();
    let url = format!("/orgs/{org}/hooks/{hook_id}");

    Request::<update_webhook::Request, (), update_webhook::Response>::builder(&self.config)
      .patch(url)
      .build()
  }

  /// **Delete an organization webhook**
  ///
  /// You must be an organization owner to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need `admin:org_hook` scope. OAuth apps cannot list, view, or edit
  /// webhooks that they did not create and users cannot list, view, or edit webhooks that were created by OAuth apps.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/webhooks#delete-an-organization-webhook](https://docs.github.com/rest/orgs/webhooks#delete-an-organization-webhook)
  pub fn delete_webhook(
    &self,
    org: impl Into<String>,
    hook_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let hook_id = hook_id.into();
    let url = format!("/orgs/{org}/hooks/{hook_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Get a webhook configuration for an organization**
  ///
  /// Returns the webhook configuration for an organization. To get more information about the webhook, including the `active` state and `events`, use "[Get an organization webhook ](/rest/orgs/webhooks#get-an-organization-webhook)."
  ///
  /// You must be an organization owner to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need `admin:org_hook` scope. OAuth apps cannot list, view, or edit
  /// webhooks that they did not create and users cannot list, view, or edit webhooks that were created by OAuth apps.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/webhooks#get-a-webhook-configuration-for-an-organization](https://docs.github.com/rest/orgs/webhooks#get-a-webhook-configuration-for-an-organization)
  pub fn get_webhook_config_for_org(
    &self,
    org: impl Into<String>,
    hook_id: impl Into<i64>,
  ) -> Request<(), (), get_webhook_config_for_org::Response> {
    let org = org.into();
    let hook_id = hook_id.into();
    let url = format!("/orgs/{org}/hooks/{hook_id}/config");

    Request::<(), (), get_webhook_config_for_org::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Update a webhook configuration for an organization**
  ///
  /// Updates the webhook configuration for an organization. To update more information about the webhook, including the `active` state and `events`, use "[Update an organization webhook ](/rest/orgs/webhooks#update-an-organization-webhook)."
  ///
  /// You must be an organization owner to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need `admin:org_hook` scope. OAuth apps cannot list, view, or edit
  /// webhooks that they did not create and users cannot list, view, or edit webhooks that were created by OAuth apps.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/webhooks#update-a-webhook-configuration-for-an-organization](https://docs.github.com/rest/orgs/webhooks#update-a-webhook-configuration-for-an-organization)
  pub fn update_webhook_config_for_org(
    &self,
    org: impl Into<String>,
    hook_id: impl Into<i64>,
  ) -> Request<update_webhook_config_for_org::Request, (), update_webhook_config_for_org::Response>
  {
    let org = org.into();
    let hook_id = hook_id.into();
    let url = format!("/orgs/{org}/hooks/{hook_id}/config");

    Request::<update_webhook_config_for_org::Request, (), update_webhook_config_for_org::Response>::builder(&self.config)
      .patch(url)
      .build()
  }

  /// **List deliveries for an organization webhook**
  ///
  /// Returns a list of webhook deliveries for a webhook configured in an organization.
  ///
  /// You must be an organization owner to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need `admin:org_hook` scope. OAuth apps cannot list, view, or edit
  /// webhooks that they did not create and users cannot list, view, or edit webhooks that were created by OAuth apps.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/webhooks#list-deliveries-for-an-organization-webhook](https://docs.github.com/rest/orgs/webhooks#list-deliveries-for-an-organization-webhook)
  pub fn list_webhook_deliveries(
    &self,
    org: impl Into<String>,
    hook_id: impl Into<i64>,
  ) -> Request<(), list_webhook_deliveries::Query, list_webhook_deliveries::Response> {
    let org = org.into();
    let hook_id = hook_id.into();
    let url = format!("/orgs/{org}/hooks/{hook_id}/deliveries");

    Request::<(), list_webhook_deliveries::Query, list_webhook_deliveries::Response>::builder(
      &self.config,
    )
    .get(url)
    .build()
  }

  /// **Get a webhook delivery for an organization webhook**
  ///
  /// Returns a delivery for a webhook configured in an organization.
  ///
  /// You must be an organization owner to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need `admin:org_hook` scope. OAuth apps cannot list, view, or edit
  /// webhooks that they did not create and users cannot list, view, or edit webhooks that were created by OAuth apps.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/webhooks#get-a-webhook-delivery-for-an-organization-webhook](https://docs.github.com/rest/orgs/webhooks#get-a-webhook-delivery-for-an-organization-webhook)
  pub fn get_webhook_delivery(
    &self,
    org: impl Into<String>,
    hook_id: impl Into<i64>,
    delivery_id: impl Into<i64>,
  ) -> Request<(), (), get_webhook_delivery::Response> {
    let org = org.into();
    let hook_id = hook_id.into();
    let delivery_id = delivery_id.into();
    let url = format!("/orgs/{org}/hooks/{hook_id}/deliveries/{delivery_id}");

    Request::<(), (), get_webhook_delivery::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Redeliver a delivery for an organization webhook**
  ///
  /// Redeliver a delivery for a webhook configured in an organization.
  ///
  /// You must be an organization owner to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need `admin:org_hook` scope. OAuth apps cannot list, view, or edit
  /// webhooks that they did not create and users cannot list, view, or edit webhooks that were created by OAuth apps.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/webhooks#redeliver-a-delivery-for-an-organization-webhook](https://docs.github.com/rest/orgs/webhooks#redeliver-a-delivery-for-an-organization-webhook)
  pub fn redeliver_webhook_delivery(
    &self,
    org: impl Into<String>,
    hook_id: impl Into<i64>,
    delivery_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let hook_id = hook_id.into();
    let delivery_id = delivery_id.into();
    let url = format!("/orgs/{org}/hooks/{hook_id}/deliveries/{delivery_id}/attempts");

    NoContentRequest::<(), ()>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Ping an organization webhook**
  ///
  /// This will trigger a [ping event](https://docs.github.com/webhooks/#ping-event)
  /// to be sent to the hook.
  ///
  /// You must be an organization owner to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need `admin:org_hook` scope. OAuth apps cannot list, view, or edit
  /// webhooks that they did not create and users cannot list, view, or edit webhooks that were created by OAuth apps.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/webhooks#ping-an-organization-webhook](https://docs.github.com/rest/orgs/webhooks#ping-an-organization-webhook)
  pub fn ping_webhook(
    &self,
    org: impl Into<String>,
    hook_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let hook_id = hook_id.into();
    let url = format!("/orgs/{org}/hooks/{hook_id}/pings");

    NoContentRequest::<(), ()>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **List app installations for an organization**
  ///
  /// Lists all GitHub Apps in an organization. The installation count includes
  /// all GitHub Apps installed on repositories in the organization.
  ///
  /// The authenticated user must be an organization owner to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:read` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/orgs#list-app-installations-for-an-organization](https://docs.github.com/rest/orgs/orgs#list-app-installations-for-an-organization)
  pub fn list_app_installations(
    &self,
    org: impl Into<String>,
  ) -> Request<(), list_app_installations::Query, list_app_installations::Response> {
    let org = org.into();
    let url = format!("/orgs/{org}/installations");

    Request::<(), list_app_installations::Query, list_app_installations::Response>::builder(
      &self.config,
    )
    .get(url)
    .build()
  }

  /// **List pending organization invitations**
  ///
  /// The return hash contains a `role` field which refers to the Organization Invitation role and will be one of the following values: `direct_member`, `admin`, `billing_manager`, or `hiring_manager`. If the invitee is not a GitHub member, the `login` field in the return hash will be `null`.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/members#list-pending-organization-invitations](https://docs.github.com/rest/orgs/members#list-pending-organization-invitations)
  pub fn list_pending_invitations(
    &self,
    org: impl Into<String>,
  ) -> Request<(), list_pending_invitations::Query, list_pending_invitations::Response> {
    let org = org.into();
    let url = format!("/orgs/{org}/invitations");

    Request::<(), list_pending_invitations::Query, list_pending_invitations::Response>::builder(
      &self.config,
    )
    .get(url)
    .build()
  }

  /// **Create an organization invitation**
  ///
  /// Invite people to an organization by using their GitHub user ID or their email address. In order to create invitations in an organization, the authenticated user must be an organization owner.
  ///
  /// This endpoint triggers [notifications](https://docs.github.com/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. For more information, see "[Rate limits for the API](https://docs.github.com/rest/overview/rate-limits-for-the-rest-api#about-secondary-rate-limits)"
  /// and "[Best practices for using the REST API](https://docs.github.com/rest/guides/best-practices-for-using-the-rest-api)."
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/members#create-an-organization-invitation](https://docs.github.com/rest/orgs/members#create-an-organization-invitation)
  pub fn create_invitation(
    &self,
    org: impl Into<String>,
  ) -> Request<create_invitation::Request, (), create_invitation::Response> {
    let org = org.into();
    let url = format!("/orgs/{org}/invitations");

    Request::<create_invitation::Request, (), create_invitation::Response>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Cancel an organization invitation**
  ///
  /// Cancel an organization invitation. In order to cancel an organization invitation, the authenticated user must be an organization owner.
  ///
  /// This endpoint triggers [notifications](https://docs.github.com/github/managing-subscriptions-and-notifications-on-github/about-notifications).
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/members#cancel-an-organization-invitation](https://docs.github.com/rest/orgs/members#cancel-an-organization-invitation)
  pub fn cancel_invitation(
    &self,
    org: impl Into<String>,
    invitation_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let invitation_id = invitation_id.into();
    let url = format!("/orgs/{org}/invitations/{invitation_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List organization invitation teams**
  ///
  /// List all teams associated with an invitation. In order to see invitations in an organization, the authenticated user must be an organization owner.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/members#list-organization-invitation-teams](https://docs.github.com/rest/orgs/members#list-organization-invitation-teams)
  pub fn list_invitation_teams(
    &self,
    org: impl Into<String>,
    invitation_id: impl Into<i64>,
  ) -> Request<(), list_invitation_teams::Query, list_invitation_teams::Response> {
    let org = org.into();
    let invitation_id = invitation_id.into();
    let url = format!("/orgs/{org}/invitations/{invitation_id}/teams");

    Request::<(), list_invitation_teams::Query, list_invitation_teams::Response>::builder(
      &self.config,
    )
    .get(url)
    .build()
  }

  /// **List organization members**
  ///
  /// List all users who are members of an organization. If the authenticated user is also a member of this organization then both concealed and public members will be returned.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/members#list-organization-members](https://docs.github.com/rest/orgs/members#list-organization-members)
  pub fn list_members(
    &self,
    org: impl Into<String>,
  ) -> Request<(), list_members::Query, list_members::Response> {
    let org = org.into();
    let url = format!("/orgs/{org}/members");

    Request::<(), list_members::Query, list_members::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Check organization membership for a user**
  ///
  /// Check if a user is, publicly or privately, a member of the organization.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/members#check-organization-membership-for-a-user](https://docs.github.com/rest/orgs/members#check-organization-membership-for-a-user)
  pub fn check_membership_for_user(
    &self,
    org: impl Into<String>,
    username: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let username = username.into();
    let url = format!("/orgs/{org}/members/{username}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Remove an organization member**
  ///
  /// Removing a user from this list will remove them from all teams and they will no longer have any access to the organization's repositories.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/members#remove-an-organization-member](https://docs.github.com/rest/orgs/members#remove-an-organization-member)
  pub fn remove_member(
    &self,
    org: impl Into<String>,
    username: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let username = username.into();
    let url = format!("/orgs/{org}/members/{username}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Get organization membership for a user**
  ///
  /// In order to get a user's membership with an organization, the authenticated user must be an organization member. The `state` parameter in the response can be used to identify the user's membership status.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/members#get-organization-membership-for-a-user](https://docs.github.com/rest/orgs/members#get-organization-membership-for-a-user)
  pub fn get_membership_for_user(
    &self,
    org: impl Into<String>,
    username: impl Into<String>,
  ) -> Request<(), (), get_membership_for_user::Response> {
    let org = org.into();
    let username = username.into();
    let url = format!("/orgs/{org}/memberships/{username}");

    Request::<(), (), get_membership_for_user::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Set organization membership for a user**
  ///
  /// Only authenticated organization owners can add a member to the organization or update the member's role.
  ///
  /// *   If the authenticated user is _adding_ a member to the organization, the invited user will receive an email inviting them to the organization. The user's [membership status](https://docs.github.com/rest/orgs/members#get-organization-membership-for-a-user) will be `pending` until they accept the invitation.
  ///     
  /// *   Authenticated users can _update_ a user's membership by passing the `role` parameter. If the authenticated user changes a member's role to `admin`, the affected user will receive an email notifying them that they've been made an organization owner. If the authenticated user changes an owner's role to `member`, no email will be sent.
  ///
  /// **Rate limits**
  ///
  /// To prevent abuse, the authenticated user is limited to 50 organization invitations per 24 hour period. If the organization is more than one month old or on a paid plan, the limit is 500 invitations per 24 hour period.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/members#set-organization-membership-for-a-user](https://docs.github.com/rest/orgs/members#set-organization-membership-for-a-user)
  pub fn set_membership_for_user(
    &self,
    org: impl Into<String>,
    username: impl Into<String>,
  ) -> Request<set_membership_for_user::Request, (), set_membership_for_user::Response> {
    let org = org.into();
    let username = username.into();
    let url = format!("/orgs/{org}/memberships/{username}");

    Request::<set_membership_for_user::Request, (), set_membership_for_user::Response>::builder(
      &self.config,
    )
    .put(url)
    .build()
  }

  /// **Remove organization membership for a user**
  ///
  /// In order to remove a user's membership with an organization, the authenticated user must be an organization owner.
  ///
  /// If the specified user is an active member of the organization, this will remove them from the organization. If the specified user has been invited to the organization, this will cancel their invitation. The specified user will receive an email notification in both cases.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/members#remove-organization-membership-for-a-user](https://docs.github.com/rest/orgs/members#remove-organization-membership-for-a-user)
  pub fn remove_membership_for_user(
    &self,
    org: impl Into<String>,
    username: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let username = username.into();
    let url = format!("/orgs/{org}/memberships/{username}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List organization fine-grained permissions for an organization**
  ///
  /// Lists the fine-grained permissions that can be used in custom organization roles for an organization. For more information, see "[Managing people's access to your organization with roles](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/about-custom-organization-roles)."
  ///
  /// To list the fine-grained permissions that can be used in custom repository roles for an organization, see "[List repository fine-grained permissions for an organization](https://docs.github.com/rest/orgs/organization-roles#list-repository-fine-grained-permissions-for-an-organization)."
  ///
  /// To use this endpoint, the authenticated user must be one of:
  ///
  /// - An administrator for the organization.
  /// - A user, or a user on a team, with the fine-grained permissions of `read_organization_custom_org_role` in the organization.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/organization-roles#list-organization-fine-grained-permissions-for-an-organization](https://docs.github.com/rest/orgs/organization-roles#list-organization-fine-grained-permissions-for-an-organization)
  pub fn list_organization_fine_grained_permissions(
    &self,
    org: impl Into<String>,
  ) -> Request<(), (), list_organization_fine_grained_permissions::Response> {
    let org = org.into();
    let url = format!("/orgs/{org}/organization-fine-grained-permissions");

    Request::<(), (), list_organization_fine_grained_permissions::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get all organization roles for an organization**
  ///
  /// Lists the organization roles available in this organization. For more information on organization roles, see "[Managing people's access to your organization with roles](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/about-custom-organization-roles)."
  ///
  /// To use this endpoint, the authenticated user must be one of:
  ///
  /// - An administrator for the organization.
  /// - A user, or a user on a team, with the fine-grained permissions of `read_organization_custom_org_role` in the organization.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/organization-roles#get-all-organization-roles-for-an-organization](https://docs.github.com/rest/orgs/organization-roles#get-all-organization-roles-for-an-organization)
  pub fn list_org_roles(
    &self,
    org: impl Into<String>,
  ) -> Request<(), (), list_org_roles::Response> {
    let org = org.into();
    let url = format!("/orgs/{org}/organization-roles");

    Request::<(), (), list_org_roles::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create a custom organization role**
  ///
  /// Creates a custom organization role that can be assigned to users and teams, granting them specific permissions over the organization. For more information on custom organization roles, see "[Managing people's access to your organization with roles](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/about-custom-organization-roles)."
  ///
  /// To use this endpoint, the authenticated user must be one of:
  ///
  /// - An administrator for the organization.
  /// - A user, or a user on a team, with the fine-grained permissions of `write_organization_custom_org_role` in the organization.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/organization-roles#create-a-custom-organization-role](https://docs.github.com/rest/orgs/organization-roles#create-a-custom-organization-role)
  pub fn create_custom_organization_role(
    &self,
    org: impl Into<String>,
  ) -> Request<
    create_custom_organization_role::Request,
    (),
    create_custom_organization_role::Response,
  > {
    let org = org.into();
    let url = format!("/orgs/{org}/organization-roles");

    Request::<create_custom_organization_role::Request, (), create_custom_organization_role::Response>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Remove all organization roles for a team**
  ///
  /// Removes all assigned organization roles from a team. For more information on organization roles, see "[Managing people's access to your organization with roles](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/about-custom-organization-roles)."
  ///
  /// The authenticated user must be an administrator for the organization to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/organization-roles#remove-all-organization-roles-for-a-team](https://docs.github.com/rest/orgs/organization-roles#remove-all-organization-roles-for-a-team)
  pub fn revoke_all_org_roles_team(
    &self,
    org: impl Into<String>,
    team_slug: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let team_slug = team_slug.into();
    let url = format!("/orgs/{org}/organization-roles/teams/{team_slug}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Assign an organization role to a team**
  ///
  /// Assigns an organization role to a team in an organization. For more information on organization roles, see "[Managing people's access to your organization with roles](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/about-custom-organization-roles)."
  ///
  /// The authenticated user must be an administrator for the organization to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/organization-roles#assign-an-organization-role-to-a-team](https://docs.github.com/rest/orgs/organization-roles#assign-an-organization-role-to-a-team)
  pub fn assign_team_to_org_role(
    &self,
    org: impl Into<String>,
    team_slug: impl Into<String>,
    role_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let team_slug = team_slug.into();
    let role_id = role_id.into();
    let url = format!("/orgs/{org}/organization-roles/teams/{team_slug}/{role_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **Remove an organization role from a team**
  ///
  /// Removes an organization role from a team. For more information on organization roles, see "[Managing people's access to your organization with roles](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/about-custom-organization-roles)."
  ///
  /// The authenticated user must be an administrator for the organization to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/organization-roles#remove-an-organization-role-from-a-team](https://docs.github.com/rest/orgs/organization-roles#remove-an-organization-role-from-a-team)
  pub fn revoke_org_role_team(
    &self,
    org: impl Into<String>,
    team_slug: impl Into<String>,
    role_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let team_slug = team_slug.into();
    let role_id = role_id.into();
    let url = format!("/orgs/{org}/organization-roles/teams/{team_slug}/{role_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Remove all organization roles for a user**
  ///
  /// Revokes all assigned organization roles from a user. For more information on organization roles, see "[Managing people's access to your organization with roles](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/about-custom-organization-roles)."
  ///
  /// The authenticated user must be an administrator for the organization to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/organization-roles#remove-all-organization-roles-for-a-user](https://docs.github.com/rest/orgs/organization-roles#remove-all-organization-roles-for-a-user)
  pub fn revoke_all_org_roles_user(
    &self,
    org: impl Into<String>,
    username: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let username = username.into();
    let url = format!("/orgs/{org}/organization-roles/users/{username}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Assign an organization role to a user**
  ///
  /// Assigns an organization role to a member of an organization. For more information on organization roles, see "[Managing people's access to your organization with roles](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/about-custom-organization-roles)."
  ///
  /// The authenticated user must be an administrator for the organization to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/organization-roles#assign-an-organization-role-to-a-user](https://docs.github.com/rest/orgs/organization-roles#assign-an-organization-role-to-a-user)
  pub fn assign_user_to_org_role(
    &self,
    org: impl Into<String>,
    username: impl Into<String>,
    role_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let username = username.into();
    let role_id = role_id.into();
    let url = format!("/orgs/{org}/organization-roles/users/{username}/{role_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **Remove an organization role from a user**
  ///
  /// Remove an organization role from a user. For more information on organization roles, see "[Managing people's access to your organization with roles](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/about-custom-organization-roles)."
  ///
  /// The authenticated user must be an administrator for the organization to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/organization-roles#remove-an-organization-role-from-a-user](https://docs.github.com/rest/orgs/organization-roles#remove-an-organization-role-from-a-user)
  pub fn revoke_org_role_user(
    &self,
    org: impl Into<String>,
    username: impl Into<String>,
    role_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let username = username.into();
    let role_id = role_id.into();
    let url = format!("/orgs/{org}/organization-roles/users/{username}/{role_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Get an organization role**
  ///
  /// Gets an organization role that is available to this organization. For more information on organization roles, see "[Managing people's access to your organization with roles](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/about-custom-organization-roles)."
  ///
  /// To use this endpoint, the authenticated user must be one of:
  ///
  /// - An administrator for the organization.
  /// - A user, or a user on a team, with the fine-grained permissions of `read_organization_custom_org_role` in the organization.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/organization-roles#get-an-organization-role](https://docs.github.com/rest/orgs/organization-roles#get-an-organization-role)
  pub fn get_org_role(
    &self,
    org: impl Into<String>,
    role_id: impl Into<i64>,
  ) -> Request<(), (), get_org_role::Response> {
    let org = org.into();
    let role_id = role_id.into();
    let url = format!("/orgs/{org}/organization-roles/{role_id}");

    Request::<(), (), get_org_role::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Update a custom organization role**
  ///
  /// Updates an existing custom organization role. Permission changes will apply to all assignees. For more information on custom organization roles, see "[Managing people's access to your organization with roles](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/about-custom-organization-roles)."
  ///
  ///
  /// To use this endpoint, the authenticated user must be one of:
  ///
  /// - An administrator for the organization.
  /// - A user, or a user on a team, with the fine-grained permissions of `write_organization_custom_org_role` in the organization.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/organization-roles#update-a-custom-organization-role](https://docs.github.com/rest/orgs/organization-roles#update-a-custom-organization-role)
  pub fn patch_custom_organization_role(
    &self,
    org: impl Into<String>,
    role_id: impl Into<i64>,
  ) -> Request<patch_custom_organization_role::Request, (), patch_custom_organization_role::Response>
  {
    let org = org.into();
    let role_id = role_id.into();
    let url = format!("/orgs/{org}/organization-roles/{role_id}");

    Request::<patch_custom_organization_role::Request, (), patch_custom_organization_role::Response>::builder(&self.config)
      .patch(url)
      .build()
  }

  /// **Delete a custom organization role.**
  ///
  /// Deletes a custom organization role. For more information on custom organization roles, see "[Managing people's access to your organization with roles](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/about-custom-organization-roles)."
  ///
  /// To use this endpoint, the authenticated user must be one of:
  ///
  /// - An administrator for the organization.
  /// - A user, or a user on a team, with the fine-grained permissions of `write_organization_custom_org_role` in the organization.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/organization-roles#delete-a-custom-organization-role](https://docs.github.com/rest/orgs/organization-roles#delete-a-custom-organization-role)
  pub fn delete_custom_organization_role(
    &self,
    org: impl Into<String>,
    role_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let role_id = role_id.into();
    let url = format!("/orgs/{org}/organization-roles/{role_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List teams that are assigned to an organization role**
  ///
  /// Lists the teams that are assigned to an organization role. For more information on organization roles, see "[Managing people's access to your organization with roles](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/about-custom-organization-roles)."
  ///
  /// To use this endpoint, you must be an administrator for the organization.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/organization-roles#list-teams-that-are-assigned-to-an-organization-role](https://docs.github.com/rest/orgs/organization-roles#list-teams-that-are-assigned-to-an-organization-role)
  pub fn list_org_role_teams(
    &self,
    org: impl Into<String>,
    role_id: impl Into<i64>,
  ) -> Request<(), list_org_role_teams::Query, list_org_role_teams::Response> {
    let org = org.into();
    let role_id = role_id.into();
    let url = format!("/orgs/{org}/organization-roles/{role_id}/teams");

    Request::<(), list_org_role_teams::Query, list_org_role_teams::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List users that are assigned to an organization role**
  ///
  /// Lists organization members that are assigned to an organization role. For more information on organization roles, see "[Managing people's access to your organization with roles](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/about-custom-organization-roles)."
  ///
  /// To use this endpoint, you must be an administrator for the organization.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/organization-roles#list-users-that-are-assigned-to-an-organization-role](https://docs.github.com/rest/orgs/organization-roles#list-users-that-are-assigned-to-an-organization-role)
  pub fn list_org_role_users(
    &self,
    org: impl Into<String>,
    role_id: impl Into<i64>,
  ) -> Request<(), list_org_role_users::Query, list_org_role_users::Response> {
    let org = org.into();
    let role_id = role_id.into();
    let url = format!("/orgs/{org}/organization-roles/{role_id}/users");

    Request::<(), list_org_role_users::Query, list_org_role_users::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List outside collaborators for an organization**
  ///
  /// List all users who are outside collaborators of an organization.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/outside-collaborators#list-outside-collaborators-for-an-organization](https://docs.github.com/rest/orgs/outside-collaborators#list-outside-collaborators-for-an-organization)
  pub fn list_outside_collaborators(
    &self,
    org: impl Into<String>,
  ) -> Request<(), list_outside_collaborators::Query, list_outside_collaborators::Response> {
    let org = org.into();
    let url = format!("/orgs/{org}/outside_collaborators");

    Request::<(), list_outside_collaborators::Query, list_outside_collaborators::Response>::builder(
      &self.config,
    )
    .get(url)
    .build()
  }

  /// **Convert an organization member to outside collaborator**
  ///
  /// When an organization member is converted to an outside collaborator, they'll only have access to the repositories that their current team membership allows. The user will no longer be a member of the organization. For more information, see "[Converting an organization member to an outside collaborator](https://docs.github.com/articles/converting-an-organization-member-to-an-outside-collaborator/)". Converting an organization member to an outside collaborator may be restricted by enterprise administrators. For more information, see "[Enforcing repository management policies in your enterprise](https://docs.github.com/admin/policies/enforcing-policies-for-your-enterprise/enforcing-repository-management-policies-in-your-enterprise#enforcing-a-policy-for-inviting-outside-collaborators-to-repositories)."
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/outside-collaborators#convert-an-organization-member-to-outside-collaborator](https://docs.github.com/rest/orgs/outside-collaborators#convert-an-organization-member-to-outside-collaborator)
  pub fn convert_member_to_outside_collaborator(
    &self,
    org: impl Into<String>,
    username: impl Into<String>,
  ) -> Request<
    convert_member_to_outside_collaborator::Request,
    (),
    convert_member_to_outside_collaborator::Response,
  > {
    let org = org.into();
    let username = username.into();
    let url = format!("/orgs/{org}/outside_collaborators/{username}");

    Request::<
      convert_member_to_outside_collaborator::Request,
      (),
      convert_member_to_outside_collaborator::Response,
    >::builder(&self.config)
    .put(url)
    .build()
  }

  /// **Remove outside collaborator from an organization**
  ///
  /// Removing a user from this list will remove them from all the organization's repositories.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/outside-collaborators#remove-outside-collaborator-from-an-organization](https://docs.github.com/rest/orgs/outside-collaborators#remove-outside-collaborator-from-an-organization)
  pub fn remove_outside_collaborator(
    &self,
    org: impl Into<String>,
    username: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let username = username.into();
    let url = format!("/orgs/{org}/outside_collaborators/{username}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List requests to access organization resources with fine-grained personal access tokens**
  ///
  /// Lists requests from organization members to access organization resources with a fine-grained personal access token.
  ///
  /// Only GitHub Apps can use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/personal-access-tokens#list-requests-to-access-organization-resources-with-fine-grained-personal-access-tokens](https://docs.github.com/rest/orgs/personal-access-tokens#list-requests-to-access-organization-resources-with-fine-grained-personal-access-tokens)
  pub fn list_pat_grant_requests(
    &self,
    org: impl Into<String>,
  ) -> Request<(), list_pat_grant_requests::Query, list_pat_grant_requests::Response> {
    let org = org.into();
    let url = format!("/orgs/{org}/personal-access-token-requests");

    Request::<(), list_pat_grant_requests::Query, list_pat_grant_requests::Response>::builder(
      &self.config,
    )
    .get(url)
    .build()
  }

  /// **Review requests to access organization resources with fine-grained personal access tokens**
  ///
  /// Approves or denies multiple pending requests to access organization resources via a fine-grained personal access token.
  ///
  /// Only GitHub Apps can use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/personal-access-tokens#review-requests-to-access-organization-resources-with-fine-grained-personal-access-tokens](https://docs.github.com/rest/orgs/personal-access-tokens#review-requests-to-access-organization-resources-with-fine-grained-personal-access-tokens)
  pub fn review_pat_grant_requests_in_bulk(
    &self,
    org: impl Into<String>,
  ) -> NoContentRequest<review_pat_grant_requests_in_bulk::Request, ()> {
    let org = org.into();
    let url = format!("/orgs/{org}/personal-access-token-requests");

    NoContentRequest::<review_pat_grant_requests_in_bulk::Request, ()>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Review a request to access organization resources with a fine-grained personal access token**
  ///
  /// Approves or denies a pending request to access organization resources via a fine-grained personal access token.
  ///
  /// Only GitHub Apps can use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/personal-access-tokens#review-a-request-to-access-organization-resources-with-a-fine-grained-personal-access-token](https://docs.github.com/rest/orgs/personal-access-tokens#review-a-request-to-access-organization-resources-with-a-fine-grained-personal-access-token)
  pub fn review_pat_grant_request(
    &self,
    org: impl Into<String>,
    pat_request_id: impl Into<i64>,
  ) -> NoContentRequest<review_pat_grant_request::Request, ()> {
    let org = org.into();
    let pat_request_id = pat_request_id.into();
    let url = format!("/orgs/{org}/personal-access-token-requests/{pat_request_id}");

    NoContentRequest::<review_pat_grant_request::Request, ()>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **List repositories requested to be accessed by a fine-grained personal access token**
  ///
  /// Lists the repositories a fine-grained personal access token request is requesting access to.
  ///
  /// Only GitHub Apps can use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/personal-access-tokens#list-repositories-requested-to-be-accessed-by-a-fine-grained-personal-access-token](https://docs.github.com/rest/orgs/personal-access-tokens#list-repositories-requested-to-be-accessed-by-a-fine-grained-personal-access-token)
  pub fn list_pat_grant_request_repositories(
    &self,
    org: impl Into<String>,
    pat_request_id: impl Into<i64>,
  ) -> Request<
    (),
    list_pat_grant_request_repositories::Query,
    list_pat_grant_request_repositories::Response,
  > {
    let org = org.into();
    let pat_request_id = pat_request_id.into();
    let url = format!("/orgs/{org}/personal-access-token-requests/{pat_request_id}/repositories");

    Request::<
      (),
      list_pat_grant_request_repositories::Query,
      list_pat_grant_request_repositories::Response,
    >::builder(&self.config)
    .get(url)
    .build()
  }

  /// **List fine-grained personal access tokens with access to organization resources**
  ///
  /// Lists approved fine-grained personal access tokens owned by organization members that can access organization resources.
  ///
  /// Only GitHub Apps can use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/personal-access-tokens#list-fine-grained-personal-access-tokens-with-access-to-organization-resources](https://docs.github.com/rest/orgs/personal-access-tokens#list-fine-grained-personal-access-tokens-with-access-to-organization-resources)
  pub fn list_pat_grants(
    &self,
    org: impl Into<String>,
  ) -> Request<(), list_pat_grants::Query, list_pat_grants::Response> {
    let org = org.into();
    let url = format!("/orgs/{org}/personal-access-tokens");

    Request::<(), list_pat_grants::Query, list_pat_grants::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Update the access to organization resources via fine-grained personal access tokens**
  ///
  /// Updates the access organization members have to organization resources via fine-grained personal access tokens. Limited to revoking a token's existing access.
  ///
  /// Only GitHub Apps can use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/personal-access-tokens#update-the-access-to-organization-resources-via-fine-grained-personal-access-tokens](https://docs.github.com/rest/orgs/personal-access-tokens#update-the-access-to-organization-resources-via-fine-grained-personal-access-tokens)
  pub fn update_pat_accesses(
    &self,
    org: impl Into<String>,
  ) -> NoContentRequest<update_pat_accesses::Request, ()> {
    let org = org.into();
    let url = format!("/orgs/{org}/personal-access-tokens");

    NoContentRequest::<update_pat_accesses::Request, ()>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Update the access a fine-grained personal access token has to organization resources**
  ///
  /// Updates the access an organization member has to organization resources via a fine-grained personal access token. Limited to revoking the token's existing access. Limited to revoking a token's existing access.
  ///
  /// Only GitHub Apps can use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/personal-access-tokens#update-the-access-a-fine-grained-personal-access-token-has-to-organization-resources](https://docs.github.com/rest/orgs/personal-access-tokens#update-the-access-a-fine-grained-personal-access-token-has-to-organization-resources)
  pub fn update_pat_access(
    &self,
    org: impl Into<String>,
    pat_id: impl Into<i64>,
  ) -> NoContentRequest<update_pat_access::Request, ()> {
    let org = org.into();
    let pat_id = pat_id.into();
    let url = format!("/orgs/{org}/personal-access-tokens/{pat_id}");

    NoContentRequest::<update_pat_access::Request, ()>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **List repositories a fine-grained personal access token has access to**
  ///
  /// Lists the repositories a fine-grained personal access token has access to.
  ///
  /// Only GitHub Apps can use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/personal-access-tokens#list-repositories-a-fine-grained-personal-access-token-has-access-to](https://docs.github.com/rest/orgs/personal-access-tokens#list-repositories-a-fine-grained-personal-access-token-has-access-to)
  pub fn list_pat_grant_repositories(
    &self,
    org: impl Into<String>,
    pat_id: impl Into<i64>,
  ) -> Request<(), list_pat_grant_repositories::Query, list_pat_grant_repositories::Response> {
    let org = org.into();
    let pat_id = pat_id.into();
    let url = format!("/orgs/{org}/personal-access-tokens/{pat_id}/repositories");

    Request::<(), list_pat_grant_repositories::Query, list_pat_grant_repositories::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get all custom properties for an organization**
  ///
  /// Gets all custom properties defined for an organization.
  /// Organization members can read these properties.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/custom-properties#get-all-custom-properties-for-an-organization](https://docs.github.com/rest/orgs/custom-properties#get-all-custom-properties-for-an-organization)
  pub fn get_all_custom_properties(
    &self,
    org: impl Into<String>,
  ) -> Request<(), (), get_all_custom_properties::Response> {
    let org = org.into();
    let url = format!("/orgs/{org}/properties/schema");

    Request::<(), (), get_all_custom_properties::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create or update custom properties for an organization**
  ///
  /// Creates new or updates existing custom properties defined for an organization in a batch.
  ///
  /// To use this endpoint, the authenticated user must be one of:
  ///   - An administrator for the organization.
  ///   - A user, or a user on a team, with the fine-grained permission of `custom_properties_org_definitions_manager` in the organization.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/custom-properties#create-or-update-custom-properties-for-an-organization](https://docs.github.com/rest/orgs/custom-properties#create-or-update-custom-properties-for-an-organization)
  pub fn create_or_update_custom_properties(
    &self,
    org: impl Into<String>,
  ) -> Request<
    create_or_update_custom_properties::Request,
    (),
    create_or_update_custom_properties::Response,
  > {
    let org = org.into();
    let url = format!("/orgs/{org}/properties/schema");

    Request::<
      create_or_update_custom_properties::Request,
      (),
      create_or_update_custom_properties::Response,
    >::builder(&self.config)
    .patch(url)
    .build()
  }

  /// **Get a custom property for an organization**
  ///
  /// Gets a custom property that is defined for an organization.
  /// Organization members can read these properties.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/custom-properties#get-a-custom-property-for-an-organization](https://docs.github.com/rest/orgs/custom-properties#get-a-custom-property-for-an-organization)
  pub fn get_custom_property(
    &self,
    org: impl Into<String>,
    custom_property_name: impl Into<String>,
  ) -> Request<(), (), get_custom_property::Response> {
    let org = org.into();
    let custom_property_name = custom_property_name.into();
    let url = format!("/orgs/{org}/properties/schema/{custom_property_name}");

    Request::<(), (), get_custom_property::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create or update a custom property for an organization**
  ///
  /// Creates a new or updates an existing custom property that is defined for an organization.
  ///
  /// To use this endpoint, the authenticated user must be one of:
  /// - An administrator for the organization.
  /// - A user, or a user on a team, with the fine-grained permission of `custom_properties_org_definitions_manager` in the organization.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/custom-properties#create-or-update-a-custom-property-for-an-organization](https://docs.github.com/rest/orgs/custom-properties#create-or-update-a-custom-property-for-an-organization)
  pub fn create_or_update_custom_property(
    &self,
    org: impl Into<String>,
    custom_property_name: impl Into<String>,
  ) -> Request<
    create_or_update_custom_property::Request,
    (),
    create_or_update_custom_property::Response,
  > {
    let org = org.into();
    let custom_property_name = custom_property_name.into();
    let url = format!("/orgs/{org}/properties/schema/{custom_property_name}");

    Request::<
      create_or_update_custom_property::Request,
      (),
      create_or_update_custom_property::Response,
    >::builder(&self.config)
    .put(url)
    .build()
  }

  /// **Remove a custom property for an organization**
  ///
  /// Removes a custom property that is defined for an organization.
  ///
  /// To use this endpoint, the authenticated user must be one of:
  ///   - An administrator for the organization.
  ///   - A user, or a user on a team, with the fine-grained permission of `custom_properties_org_definitions_manager` in the organization.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/custom-properties#remove-a-custom-property-for-an-organization](https://docs.github.com/rest/orgs/custom-properties#remove-a-custom-property-for-an-organization)
  pub fn remove_custom_property(
    &self,
    org: impl Into<String>,
    custom_property_name: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let custom_property_name = custom_property_name.into();
    let url = format!("/orgs/{org}/properties/schema/{custom_property_name}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List custom property values for organization repositories**
  ///
  /// Lists organization repositories with all of their custom property values.
  /// Organization members can read these properties.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/custom-properties#list-custom-property-values-for-organization-repositories](https://docs.github.com/rest/orgs/custom-properties#list-custom-property-values-for-organization-repositories)
  pub fn list_custom_properties_values_for_repos(
    &self,
    org: impl Into<String>,
  ) -> Request<
    (),
    list_custom_properties_values_for_repos::Query,
    list_custom_properties_values_for_repos::Response,
  > {
    let org = org.into();
    let url = format!("/orgs/{org}/properties/values");

    Request::<
      (),
      list_custom_properties_values_for_repos::Query,
      list_custom_properties_values_for_repos::Response,
    >::builder(&self.config)
    .get(url)
    .build()
  }

  /// **Create or update custom property values for organization repositories**
  ///
  /// Create new or update existing custom property values for repositories in a batch that belong to an organization.
  /// Each target repository will have its custom property values updated to match the values provided in the request.
  ///
  /// A maximum of 30 repositories can be updated in a single request.
  ///
  /// Using a value of `null` for a custom property will remove or 'unset' the property value from the repository.
  ///
  /// To use this endpoint, the authenticated user must be one of:
  ///   - An administrator for the organization.
  ///   - A user, or a user on a team, with the fine-grained permission of `custom_properties_org_values_editor` in the organization.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/custom-properties#create-or-update-custom-property-values-for-organization-repositories](https://docs.github.com/rest/orgs/custom-properties#create-or-update-custom-property-values-for-organization-repositories)
  pub fn create_or_update_custom_properties_values_for_repos(
    &self,
    org: impl Into<String>,
  ) -> NoContentRequest<create_or_update_custom_properties_values_for_repos::Request, ()> {
    let org = org.into();
    let url = format!("/orgs/{org}/properties/values");

    NoContentRequest::<create_or_update_custom_properties_values_for_repos::Request, ()>::builder(
      &self.config,
    )
    .patch(url)
    .build()
  }

  /// **List public organization members**
  ///
  /// Members of an organization can choose to have their membership publicized or not.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/members#list-public-organization-members](https://docs.github.com/rest/orgs/members#list-public-organization-members)
  pub fn list_public_members(
    &self,
    org: impl Into<String>,
  ) -> Request<(), list_public_members::Query, list_public_members::Response> {
    let org = org.into();
    let url = format!("/orgs/{org}/public_members");

    Request::<(), list_public_members::Query, list_public_members::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Check public organization membership for a user**
  ///
  /// Check if the provided user is a public member of the organization.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/members#check-public-organization-membership-for-a-user](https://docs.github.com/rest/orgs/members#check-public-organization-membership-for-a-user)
  pub fn check_public_membership_for_user(
    &self,
    org: impl Into<String>,
    username: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let username = username.into();
    let url = format!("/orgs/{org}/public_members/{username}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Set public organization membership for the authenticated user**
  ///
  /// The user can publicize their own membership. (A user cannot publicize the membership for another user.)
  ///
  /// Note that you'll need to set `Content-Length` to zero when calling out to this endpoint. For more information, see "[HTTP method](https://docs.github.com/rest/guides/getting-started-with-the-rest-api#http-method)."
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/members#set-public-organization-membership-for-the-authenticated-user](https://docs.github.com/rest/orgs/members#set-public-organization-membership-for-the-authenticated-user)
  pub fn set_public_membership_for_authenticated_user(
    &self,
    org: impl Into<String>,
    username: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let username = username.into();
    let url = format!("/orgs/{org}/public_members/{username}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **Remove public organization membership for the authenticated user**
  ///
  /// Removes the public membership for the authenticated user from the specified organization, unless public visibility is enforced by default.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/members#remove-public-organization-membership-for-the-authenticated-user](https://docs.github.com/rest/orgs/members#remove-public-organization-membership-for-the-authenticated-user)
  pub fn remove_public_membership_for_authenticated_user(
    &self,
    org: impl Into<String>,
    username: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let username = username.into();
    let url = format!("/orgs/{org}/public_members/{username}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List security manager teams**
  ///
  /// Lists teams that are security managers for an organization. For more information, see "[Managing security managers in your organization](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/managing-security-managers-in-your-organization)."
  ///
  /// The authenticated user must be an administrator or security manager for the organization to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `read:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/security-managers#list-security-manager-teams](https://docs.github.com/rest/orgs/security-managers#list-security-manager-teams)
  pub fn list_security_manager_teams(
    &self,
    org: impl Into<String>,
  ) -> Request<(), (), list_security_manager_teams::Response> {
    let org = org.into();
    let url = format!("/orgs/{org}/security-managers");

    Request::<(), (), list_security_manager_teams::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Add a security manager team**
  ///
  /// Adds a team as a security manager for an organization. For more information, see "[Managing security for an organization](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/managing-security-managers-in-your-organization) for an organization."
  ///
  /// The authenticated user must be an administrator for the organization to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `write:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/security-managers#add-a-security-manager-team](https://docs.github.com/rest/orgs/security-managers#add-a-security-manager-team)
  pub fn add_security_manager_team(
    &self,
    org: impl Into<String>,
    team_slug: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let team_slug = team_slug.into();
    let url = format!("/orgs/{org}/security-managers/teams/{team_slug}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **Remove a security manager team**
  ///
  /// Removes the security manager role from a team for an organization. For more information, see "[Managing security managers in your organization](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/managing-security-managers-in-your-organization) team from an organization."
  ///
  /// The authenticated user must be an administrator for the organization to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/security-managers#remove-a-security-manager-team](https://docs.github.com/rest/orgs/security-managers#remove-a-security-manager-team)
  pub fn remove_security_manager_team(
    &self,
    org: impl Into<String>,
    team_slug: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let team_slug = team_slug.into();
    let url = format!("/orgs/{org}/security-managers/teams/{team_slug}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Enable or disable a security feature for an organization**
  ///
  /// Enables or disables the specified security feature for all eligible repositories in an organization. For more information, see "[Managing security managers in your organization](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/managing-security-managers-in-your-organization)."
  ///
  /// The authenticated user must be an organization owner or be member of a team with the security manager role to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `write:org` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/orgs#enable-or-disable-a-security-feature-for-an-organization](https://docs.github.com/rest/orgs/orgs#enable-or-disable-a-security-feature-for-an-organization)
  pub fn enable_or_disable_security_product_on_all_org_repos(
    &self,
    org: impl Into<String>,
    security_product: impl Into<
      enable_or_disable_security_product_on_all_org_repos::ParametersSecurityProduct,
    >,
    enablement: impl Into<enable_or_disable_security_product_on_all_org_repos::ParametersEnablement>,
  ) -> NoContentRequest<enable_or_disable_security_product_on_all_org_repos::Request, ()> {
    let org = org.into();
    let security_product = security_product.into();
    let enablement = enablement.into();
    let security_product = security_product.to_string();
    let enablement = enablement.to_string();
    let url = format!("/orgs/{org}/{security_product}/{enablement}");

    NoContentRequest::<enable_or_disable_security_product_on_all_org_repos::Request, ()>::builder(
      &self.config,
    )
    .post(url)
    .build()
  }

  /// **List organization memberships for the authenticated user**
  ///
  /// Lists all of the authenticated user's organization memberships.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/members#list-organization-memberships-for-the-authenticated-user](https://docs.github.com/rest/orgs/members#list-organization-memberships-for-the-authenticated-user)
  pub fn list_memberships_for_authenticated_user(
    &self,
  ) -> Request<
    (),
    list_memberships_for_authenticated_user::Query,
    list_memberships_for_authenticated_user::Response,
  > {
    let url = format!("/user/memberships/orgs");

    Request::<
      (),
      list_memberships_for_authenticated_user::Query,
      list_memberships_for_authenticated_user::Response,
    >::builder(&self.config)
    .get(url)
    .build()
  }

  /// **Get an organization membership for the authenticated user**
  ///
  /// If the authenticated user is an active or pending member of the organization, this endpoint will return the user's membership. If the authenticated user is not affiliated with the organization, a `404` is returned. This endpoint will return a `403` if the request is made by a GitHub App that is blocked by the organization.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/members#get-an-organization-membership-for-the-authenticated-user](https://docs.github.com/rest/orgs/members#get-an-organization-membership-for-the-authenticated-user)
  pub fn get_membership_for_authenticated_user(
    &self,
    org: impl Into<String>,
  ) -> Request<(), (), get_membership_for_authenticated_user::Response> {
    let org = org.into();
    let url = format!("/user/memberships/orgs/{org}");

    Request::<(), (), get_membership_for_authenticated_user::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Update an organization membership for the authenticated user**
  ///
  /// Converts the authenticated user to an active member of the organization, if that user has a pending invitation from the organization.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/members#update-an-organization-membership-for-the-authenticated-user](https://docs.github.com/rest/orgs/members#update-an-organization-membership-for-the-authenticated-user)
  pub fn update_membership_for_authenticated_user(
    &self,
    org: impl Into<String>,
  ) -> Request<
    update_membership_for_authenticated_user::Request,
    (),
    update_membership_for_authenticated_user::Response,
  > {
    let org = org.into();
    let url = format!("/user/memberships/orgs/{org}");

    Request::<
      update_membership_for_authenticated_user::Request,
      (),
      update_membership_for_authenticated_user::Response,
    >::builder(&self.config)
    .patch(url)
    .build()
  }

  /// **List organizations for the authenticated user**
  ///
  /// List organizations for the authenticated user.
  ///
  /// For OAuth app tokens and personal access tokens (classic), this endpoint only lists organizations that your authorization allows you to operate on in some way (e.g., you can list teams with `read:org` scope, you can publicize your organization membership with `user` scope, etc.). Therefore, this API requires at least `user` or `read:org` scope for OAuth app tokens and personal access tokens (classic). Requests with insufficient scope will receive a `403 Forbidden` response.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/orgs#list-organizations-for-the-authenticated-user](https://docs.github.com/rest/orgs/orgs#list-organizations-for-the-authenticated-user)
  pub fn list_for_authenticated_user(
    &self,
  ) -> Request<(), list_for_authenticated_user::Query, list_for_authenticated_user::Response> {
    let url = format!("/user/orgs");

    Request::<(), list_for_authenticated_user::Query, list_for_authenticated_user::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List organizations for a user**
  ///
  /// List [public organization memberships](https://docs.github.com/articles/publicizing-or-concealing-organization-membership) for the specified user.
  ///
  /// This method only lists _public_ memberships, regardless of authentication. If you need to fetch all of the organization memberships (public and private) for the authenticated user, use the [List organizations for the authenticated user](https://docs.github.com/rest/orgs/orgs#list-organizations-for-the-authenticated-user) API instead.
  ///
  /// *Documentation*: [https://docs.github.com/rest/orgs/orgs#list-organizations-for-a-user](https://docs.github.com/rest/orgs/orgs#list-organizations-for-a-user)
  pub fn list_for_user(
    &self,
    username: impl Into<String>,
  ) -> Request<(), list_for_user::Query, list_for_user::Response> {
    let username = username.into();
    let url = format!("/users/{username}/orgs");

    Request::<(), list_for_user::Query, list_for_user::Response>::builder(&self.config)
      .get(url)
      .build()
  }
}
