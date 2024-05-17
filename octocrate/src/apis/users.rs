use octocrate_core::*;
#[allow(unused_imports)]
use octocrate_types::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use typed_builder::TypedBuilder;

pub mod get_authenticated {
  #[allow(unused_imports)]
  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize)]
  #[serde(untagged)]
  pub enum Response {
    /// Private User
    PrivateUser(PrivateUser),
    /// Public User
    PublicUser(PublicUser),
  }
}

pub mod update_authenticated {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = PrivateUser;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The new short biography of the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub bio: Option<String>,
    /// The new blog URL of the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub blog: Option<String>,
    /// The new company of the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub company: Option<String>,
    /// The publicly visible email address of the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub email: Option<String>,
    /// The new hiring availability of the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub hireable: Option<bool>,
    /// The new location of the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub location: Option<String>,
    /// The new name of the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub name: Option<String>,
    /// The new Twitter username of the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub twitter_username: Option<String>,
  }
}

pub mod list_blocked_by_authenticated_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<SimpleUser>;

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

pub mod check_blocked {
  #[allow(unused_imports)]
  use super::*;
}

pub mod block {
  #[allow(unused_imports)]
  use super::*;
}

pub mod unblock {
  #[allow(unused_imports)]
  use super::*;
}

pub mod set_primary_email_visibility_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Email>;

  /// Denotes whether an email is publicly visible.
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum RequestVisibility {
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "private")]
    Private,
  }

  impl ToString for RequestVisibility {
    fn to_string(&self) -> String {
      match self {
        RequestVisibility::Public => "public".to_string(),
        RequestVisibility::Private => "private".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// Denotes whether an email is publicly visible.
    pub visibility: RequestVisibility,
  }
}

pub mod list_emails_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Email>;

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

pub mod add_email_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Email>;

  #[derive(Debug, Clone, Serialize, Deserialize)]
  #[serde(untagged)]
  pub enum Request {
    RequestItem1(RequestItem1),
    StringArray(Vec<String>),
    String(String),
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct RequestItem1 {
    /// Adds one or more email addresses to your GitHub account. Must contain at least one email address. **Note:** Alternatively, you can pass a single email address or an `array` of emails addresses directly, but we recommend that you pass an object using the `emails` key.
    pub emails: Vec<String>,
  }
}

pub mod delete_email_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize)]
  #[serde(untagged)]
  pub enum Request {
    /// Deletes one or more email addresses from your GitHub account. Must contain at least one email address. **Note:** Alternatively, you can pass a single email address or an `array` of emails addresses directly, but we recommend that you pass an object using the `emails` key.
    RequestItem1(RequestItem1),
    StringArray(Vec<String>),
    String(String),
  }

  /// Deletes one or more email addresses from your GitHub account. Must contain at least one email address. **Note:** Alternatively, you can pass a single email address or an `array` of emails addresses directly, but we recommend that you pass an object using the `emails` key.
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct RequestItem1 {
    /// Email addresses associated with the GitHub user account.
    pub emails: Vec<String>,
  }
}

pub mod list_followers_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<SimpleUser>;

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

pub mod list_followed_by_authenticated_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<SimpleUser>;

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

pub mod check_person_is_followed_by_authenticated {
  #[allow(unused_imports)]
  use super::*;
}

pub mod follow {
  #[allow(unused_imports)]
  use super::*;
}

pub mod unfollow {
  #[allow(unused_imports)]
  use super::*;
}

pub mod list_gpg_keys_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<GpgKey>;

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

pub mod create_gpg_key_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = GpgKey;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// A GPG key in ASCII-armored format.
    pub armored_public_key: String,
    /// A descriptive name for the new key.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub name: Option<String>,
  }
}

pub mod get_gpg_key_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = GpgKey;
}

pub mod delete_gpg_key_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;
}

pub mod list_public_ssh_keys_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Key>;

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

pub mod create_public_ssh_key_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Key;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The public SSH key to add to your GitHub account.
    pub key: String,
    /// A descriptive name for the new key.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub title: Option<String>,
  }
}

pub mod get_public_ssh_key_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Key;
}

pub mod delete_public_ssh_key_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;
}

pub mod list_public_emails_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Email>;

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

pub mod list_social_accounts_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<SocialAccount>;

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

pub mod add_social_account_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<SocialAccount>;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// Full URLs for the social media profiles to add.
    pub account_urls: Vec<String>,
  }
}

pub mod delete_social_account_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// Full URLs for the social media profiles to delete.
    pub account_urls: Vec<String>,
  }
}

pub mod list_ssh_signing_keys_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<SshSigningKey>;

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

pub mod create_ssh_signing_key_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = SshSigningKey;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The public SSH key to add to your GitHub account. For more information, see "[Checking for existing SSH keys](https://docs.github.com/authentication/connecting-to-github-with-ssh/checking-for-existing-ssh-keys)."
    pub key: String,
    /// A descriptive name for the new key.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub title: Option<String>,
  }
}

pub mod get_ssh_signing_key_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = SshSigningKey;
}

pub mod delete_ssh_signing_key_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;
}

pub mod list {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<SimpleUser>;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// A user ID. Only return users with an ID greater than this ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub since: Option<i64>,
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
  }
}

pub mod get_by_username {
  #[allow(unused_imports)]
  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize)]
  #[serde(untagged)]
  pub enum Response {
    /// Private User
    PrivateUser(PrivateUser),
    /// Public User
    PublicUser(PublicUser),
  }
}

pub mod list_followers_for_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<SimpleUser>;

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

pub mod list_following_for_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<SimpleUser>;

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

pub mod check_following_for_user {
  #[allow(unused_imports)]
  use super::*;
}

pub mod list_gpg_keys_for_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<GpgKey>;

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

pub mod get_context_for_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Hovercard;

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QuerySubjectType {
    #[serde(rename = "organization")]
    Organization,
    #[serde(rename = "repository")]
    Repository,
    #[serde(rename = "issue")]
    Issue,
    #[serde(rename = "pull_request")]
    PullRequest,
  }

  impl ToString for QuerySubjectType {
    fn to_string(&self) -> String {
      match self {
        QuerySubjectType::Organization => "organization".to_string(),
        QuerySubjectType::Repository => "repository".to_string(),
        QuerySubjectType::Issue => "issue".to_string(),
        QuerySubjectType::PullRequest => "pull_request".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// Identifies which additional information you'd like to receive about the person's hovercard. Can be `organization`, `repository`, `issue`, `pull_request`. **Required** when using `subject_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub subject_type: Option<QuerySubjectType>,
    /// Uses the ID for the `subject_type` you specified. **Required** when using `subject_type`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub subject_id: Option<String>,
  }
}

pub mod list_public_keys_for_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<KeySimple>;

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

pub mod list_social_accounts_for_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<SocialAccount>;

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

pub mod list_ssh_signing_keys_for_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<SshSigningKey>;

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

/// Interact with and view information about users and also current user.
pub struct GitHubUsersAPI {
  config: SharedAPIConfig,
}

impl GitHubUsersAPI {
  pub fn new(config: &SharedAPIConfig) -> Self {
    Self {
      config: config.clone(),
    }
  }

  /// **Get the authenticated user**
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `user` scope in order for the response to include private profile information.
  ///
  /// *Documentation*: [https://docs.github.com/rest/users/users#get-the-authenticated-user](https://docs.github.com/rest/users/users#get-the-authenticated-user)
  pub fn get_authenticated(&self) -> Request<(), (), get_authenticated::Response> {
    let url = format!("/user");

    Request::<(), (), get_authenticated::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Update the authenticated user**
  ///
  /// **Note:** If your email is set to private and you send an `email` parameter as part of this request to update your profile, your privacy settings are still enforced: the email address will not be displayed on your public profile or via the API.
  ///
  /// *Documentation*: [https://docs.github.com/rest/users/users#update-the-authenticated-user](https://docs.github.com/rest/users/users#update-the-authenticated-user)
  pub fn update_authenticated(
    &self,
  ) -> Request<update_authenticated::Request, (), update_authenticated::Response> {
    let url = format!("/user");

    Request::<update_authenticated::Request, (), update_authenticated::Response>::builder(
      &self.config,
    )
    .patch(url)
    .build()
  }

  /// **List users blocked by the authenticated user**
  ///
  /// List the users you've blocked on your personal account.
  ///
  /// *Documentation*: [https://docs.github.com/rest/users/blocking#list-users-blocked-by-the-authenticated-user](https://docs.github.com/rest/users/blocking#list-users-blocked-by-the-authenticated-user)
  pub fn list_blocked_by_authenticated_user(
    &self,
  ) -> Request<
    (),
    list_blocked_by_authenticated_user::Query,
    list_blocked_by_authenticated_user::Response,
  > {
    let url = format!("/user/blocks");

    Request::<
      (),
      list_blocked_by_authenticated_user::Query,
      list_blocked_by_authenticated_user::Response,
    >::builder(&self.config)
    .get(url)
    .build()
  }

  /// **Check if a user is blocked by the authenticated user**
  ///
  /// Returns a 204 if the given user is blocked by the authenticated user. Returns a 404 if the given user is not blocked by the authenticated user, or if the given user account has been identified as spam by GitHub.
  ///
  /// *Documentation*: [https://docs.github.com/rest/users/blocking#check-if-a-user-is-blocked-by-the-authenticated-user](https://docs.github.com/rest/users/blocking#check-if-a-user-is-blocked-by-the-authenticated-user)
  pub fn check_blocked(&self, username: impl Into<String>) -> NoContentRequest<(), ()> {
    let username = username.into();
    let url = format!("/user/blocks/{username}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Block a user**
  ///
  /// Blocks the given user and returns a 204. If the authenticated user cannot block the given user a 422 is returned.
  ///
  /// *Documentation*: [https://docs.github.com/rest/users/blocking#block-a-user](https://docs.github.com/rest/users/blocking#block-a-user)
  pub fn block(&self, username: impl Into<String>) -> NoContentRequest<(), ()> {
    let username = username.into();
    let url = format!("/user/blocks/{username}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **Unblock a user**
  ///
  /// Unblocks the given user and returns a 204.
  ///
  /// *Documentation*: [https://docs.github.com/rest/users/blocking#unblock-a-user](https://docs.github.com/rest/users/blocking#unblock-a-user)
  pub fn unblock(&self, username: impl Into<String>) -> NoContentRequest<(), ()> {
    let username = username.into();
    let url = format!("/user/blocks/{username}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Set primary email visibility for the authenticated user**
  ///
  /// Sets the visibility for your primary email addresses.
  ///
  /// *Documentation*: [https://docs.github.com/rest/users/emails#set-primary-email-visibility-for-the-authenticated-user](https://docs.github.com/rest/users/emails#set-primary-email-visibility-for-the-authenticated-user)
  pub fn set_primary_email_visibility_for_authenticated_user(
    &self,
  ) -> Request<
    set_primary_email_visibility_for_authenticated_user::Request,
    (),
    set_primary_email_visibility_for_authenticated_user::Response,
  > {
    let url = format!("/user/email/visibility");

    Request::<
      set_primary_email_visibility_for_authenticated_user::Request,
      (),
      set_primary_email_visibility_for_authenticated_user::Response,
    >::builder(&self.config)
    .patch(url)
    .build()
  }

  /// **List email addresses for the authenticated user**
  ///
  /// Lists all of your email addresses, and specifies which one is visible
  /// to the public.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `user:email` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/users/emails#list-email-addresses-for-the-authenticated-user](https://docs.github.com/rest/users/emails#list-email-addresses-for-the-authenticated-user)
  pub fn list_emails_for_authenticated_user(
    &self,
  ) -> Request<
    (),
    list_emails_for_authenticated_user::Query,
    list_emails_for_authenticated_user::Response,
  > {
    let url = format!("/user/emails");

    Request::<
      (),
      list_emails_for_authenticated_user::Query,
      list_emails_for_authenticated_user::Response,
    >::builder(&self.config)
    .get(url)
    .build()
  }

  /// **Add an email address for the authenticated user**
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `user` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/users/emails#add-an-email-address-for-the-authenticated-user](https://docs.github.com/rest/users/emails#add-an-email-address-for-the-authenticated-user)
  pub fn add_email_for_authenticated_user(
    &self,
  ) -> Request<
    add_email_for_authenticated_user::Request,
    (),
    add_email_for_authenticated_user::Response,
  > {
    let url = format!("/user/emails");

    Request::<
      add_email_for_authenticated_user::Request,
      (),
      add_email_for_authenticated_user::Response,
    >::builder(&self.config)
    .post(url)
    .build()
  }

  /// **Delete an email address for the authenticated user**
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `user` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/users/emails#delete-an-email-address-for-the-authenticated-user](https://docs.github.com/rest/users/emails#delete-an-email-address-for-the-authenticated-user)
  pub fn delete_email_for_authenticated_user(
    &self,
  ) -> NoContentRequest<delete_email_for_authenticated_user::Request, ()> {
    let url = format!("/user/emails");

    NoContentRequest::<delete_email_for_authenticated_user::Request, ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List followers of the authenticated user**
  ///
  /// Lists the people following the authenticated user.
  ///
  /// *Documentation*: [https://docs.github.com/rest/users/followers#list-followers-of-the-authenticated-user](https://docs.github.com/rest/users/followers#list-followers-of-the-authenticated-user)
  pub fn list_followers_for_authenticated_user(
    &self,
  ) -> Request<
    (),
    list_followers_for_authenticated_user::Query,
    list_followers_for_authenticated_user::Response,
  > {
    let url = format!("/user/followers");

    Request::<
      (),
      list_followers_for_authenticated_user::Query,
      list_followers_for_authenticated_user::Response,
    >::builder(&self.config)
    .get(url)
    .build()
  }

  /// **List the people the authenticated user follows**
  ///
  /// Lists the people who the authenticated user follows.
  ///
  /// *Documentation*: [https://docs.github.com/rest/users/followers#list-the-people-the-authenticated-user-follows](https://docs.github.com/rest/users/followers#list-the-people-the-authenticated-user-follows)
  pub fn list_followed_by_authenticated_user(
    &self,
  ) -> Request<
    (),
    list_followed_by_authenticated_user::Query,
    list_followed_by_authenticated_user::Response,
  > {
    let url = format!("/user/following");

    Request::<
      (),
      list_followed_by_authenticated_user::Query,
      list_followed_by_authenticated_user::Response,
    >::builder(&self.config)
    .get(url)
    .build()
  }

  /// **Check if a person is followed by the authenticated user**
  ///
  ///
  /// *Documentation*: [https://docs.github.com/rest/users/followers#check-if-a-person-is-followed-by-the-authenticated-user](https://docs.github.com/rest/users/followers#check-if-a-person-is-followed-by-the-authenticated-user)
  pub fn check_person_is_followed_by_authenticated(
    &self,
    username: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let username = username.into();
    let url = format!("/user/following/{username}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Follow a user**
  ///
  /// Note that you'll need to set `Content-Length` to zero when calling out to this endpoint. For more information, see "[HTTP verbs](https://docs.github.com/rest/guides/getting-started-with-the-rest-api#http-method)."
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `user:follow` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/users/followers#follow-a-user](https://docs.github.com/rest/users/followers#follow-a-user)
  pub fn follow(&self, username: impl Into<String>) -> NoContentRequest<(), ()> {
    let username = username.into();
    let url = format!("/user/following/{username}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **Unfollow a user**
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `user:follow` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/users/followers#unfollow-a-user](https://docs.github.com/rest/users/followers#unfollow-a-user)
  pub fn unfollow(&self, username: impl Into<String>) -> NoContentRequest<(), ()> {
    let username = username.into();
    let url = format!("/user/following/{username}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List GPG keys for the authenticated user**
  ///
  /// Lists the current user's GPG keys.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `read:gpg_key` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/users/gpg-keys#list-gpg-keys-for-the-authenticated-user](https://docs.github.com/rest/users/gpg-keys#list-gpg-keys-for-the-authenticated-user)
  pub fn list_gpg_keys_for_authenticated_user(
    &self,
  ) -> Request<
    (),
    list_gpg_keys_for_authenticated_user::Query,
    list_gpg_keys_for_authenticated_user::Response,
  > {
    let url = format!("/user/gpg_keys");

    Request::<
      (),
      list_gpg_keys_for_authenticated_user::Query,
      list_gpg_keys_for_authenticated_user::Response,
    >::builder(&self.config)
    .get(url)
    .build()
  }

  /// **Create a GPG key for the authenticated user**
  ///
  /// Adds a GPG key to the authenticated user's GitHub account.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `write:gpg_key` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/users/gpg-keys#create-a-gpg-key-for-the-authenticated-user](https://docs.github.com/rest/users/gpg-keys#create-a-gpg-key-for-the-authenticated-user)
  pub fn create_gpg_key_for_authenticated_user(
    &self,
  ) -> Request<
    create_gpg_key_for_authenticated_user::Request,
    (),
    create_gpg_key_for_authenticated_user::Response,
  > {
    let url = format!("/user/gpg_keys");

    Request::<
      create_gpg_key_for_authenticated_user::Request,
      (),
      create_gpg_key_for_authenticated_user::Response,
    >::builder(&self.config)
    .post(url)
    .build()
  }

  /// **Get a GPG key for the authenticated user**
  ///
  /// View extended details for a single GPG key.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `read:gpg_key` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/users/gpg-keys#get-a-gpg-key-for-the-authenticated-user](https://docs.github.com/rest/users/gpg-keys#get-a-gpg-key-for-the-authenticated-user)
  pub fn get_gpg_key_for_authenticated_user(
    &self,
    gpg_key_id: impl Into<i64>,
  ) -> Request<(), (), get_gpg_key_for_authenticated_user::Response> {
    let gpg_key_id = gpg_key_id.into();
    let url = format!("/user/gpg_keys/{gpg_key_id}");

    Request::<(), (), get_gpg_key_for_authenticated_user::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Delete a GPG key for the authenticated user**
  ///
  /// Removes a GPG key from the authenticated user's GitHub account.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:gpg_key` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/users/gpg-keys#delete-a-gpg-key-for-the-authenticated-user](https://docs.github.com/rest/users/gpg-keys#delete-a-gpg-key-for-the-authenticated-user)
  pub fn delete_gpg_key_for_authenticated_user(
    &self,
    gpg_key_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let gpg_key_id = gpg_key_id.into();
    let url = format!("/user/gpg_keys/{gpg_key_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List public SSH keys for the authenticated user**
  ///
  /// Lists the public SSH keys for the authenticated user's GitHub account.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `read:public_key` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/users/keys#list-public-ssh-keys-for-the-authenticated-user](https://docs.github.com/rest/users/keys#list-public-ssh-keys-for-the-authenticated-user)
  pub fn list_public_ssh_keys_for_authenticated_user(
    &self,
  ) -> Request<
    (),
    list_public_ssh_keys_for_authenticated_user::Query,
    list_public_ssh_keys_for_authenticated_user::Response,
  > {
    let url = format!("/user/keys");

    Request::<
      (),
      list_public_ssh_keys_for_authenticated_user::Query,
      list_public_ssh_keys_for_authenticated_user::Response,
    >::builder(&self.config)
    .get(url)
    .build()
  }

  /// **Create a public SSH key for the authenticated user**
  ///
  /// Adds a public SSH key to the authenticated user's GitHub account.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `write:gpg_key` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/users/keys#create-a-public-ssh-key-for-the-authenticated-user](https://docs.github.com/rest/users/keys#create-a-public-ssh-key-for-the-authenticated-user)
  pub fn create_public_ssh_key_for_authenticated_user(
    &self,
  ) -> Request<
    create_public_ssh_key_for_authenticated_user::Request,
    (),
    create_public_ssh_key_for_authenticated_user::Response,
  > {
    let url = format!("/user/keys");

    Request::<
      create_public_ssh_key_for_authenticated_user::Request,
      (),
      create_public_ssh_key_for_authenticated_user::Response,
    >::builder(&self.config)
    .post(url)
    .build()
  }

  /// **Get a public SSH key for the authenticated user**
  ///
  /// View extended details for a single public SSH key.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `read:public_key` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/users/keys#get-a-public-ssh-key-for-the-authenticated-user](https://docs.github.com/rest/users/keys#get-a-public-ssh-key-for-the-authenticated-user)
  pub fn get_public_ssh_key_for_authenticated_user(
    &self,
    key_id: impl Into<i64>,
  ) -> Request<(), (), get_public_ssh_key_for_authenticated_user::Response> {
    let key_id = key_id.into();
    let url = format!("/user/keys/{key_id}");

    Request::<(), (), get_public_ssh_key_for_authenticated_user::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Delete a public SSH key for the authenticated user**
  ///
  /// Removes a public SSH key from the authenticated user's GitHub account.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:public_key` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/users/keys#delete-a-public-ssh-key-for-the-authenticated-user](https://docs.github.com/rest/users/keys#delete-a-public-ssh-key-for-the-authenticated-user)
  pub fn delete_public_ssh_key_for_authenticated_user(
    &self,
    key_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let key_id = key_id.into();
    let url = format!("/user/keys/{key_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List public email addresses for the authenticated user**
  ///
  /// Lists your publicly visible email address, which you can set with the
  /// [Set primary email visibility for the authenticated user](https://docs.github.com/rest/users/emails#set-primary-email-visibility-for-the-authenticated-user)
  /// endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `user:email` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/users/emails#list-public-email-addresses-for-the-authenticated-user](https://docs.github.com/rest/users/emails#list-public-email-addresses-for-the-authenticated-user)
  pub fn list_public_emails_for_authenticated_user(
    &self,
  ) -> Request<
    (),
    list_public_emails_for_authenticated_user::Query,
    list_public_emails_for_authenticated_user::Response,
  > {
    let url = format!("/user/public_emails");

    Request::<
      (),
      list_public_emails_for_authenticated_user::Query,
      list_public_emails_for_authenticated_user::Response,
    >::builder(&self.config)
    .get(url)
    .build()
  }

  /// **List social accounts for the authenticated user**
  ///
  /// Lists all of your social accounts.
  ///
  /// *Documentation*: [https://docs.github.com/rest/users/social-accounts#list-social-accounts-for-the-authenticated-user](https://docs.github.com/rest/users/social-accounts#list-social-accounts-for-the-authenticated-user)
  pub fn list_social_accounts_for_authenticated_user(
    &self,
  ) -> Request<
    (),
    list_social_accounts_for_authenticated_user::Query,
    list_social_accounts_for_authenticated_user::Response,
  > {
    let url = format!("/user/social_accounts");

    Request::<
      (),
      list_social_accounts_for_authenticated_user::Query,
      list_social_accounts_for_authenticated_user::Response,
    >::builder(&self.config)
    .get(url)
    .build()
  }

  /// **Add social accounts for the authenticated user**
  ///
  /// Add one or more social accounts to the authenticated user's profile.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `user` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/users/social-accounts#add-social-accounts-for-the-authenticated-user](https://docs.github.com/rest/users/social-accounts#add-social-accounts-for-the-authenticated-user)
  pub fn add_social_account_for_authenticated_user(
    &self,
  ) -> Request<
    add_social_account_for_authenticated_user::Request,
    (),
    add_social_account_for_authenticated_user::Response,
  > {
    let url = format!("/user/social_accounts");

    Request::<
      add_social_account_for_authenticated_user::Request,
      (),
      add_social_account_for_authenticated_user::Response,
    >::builder(&self.config)
    .post(url)
    .build()
  }

  /// **Delete social accounts for the authenticated user**
  ///
  /// Deletes one or more social accounts from the authenticated user's profile.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `user` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/users/social-accounts#delete-social-accounts-for-the-authenticated-user](https://docs.github.com/rest/users/social-accounts#delete-social-accounts-for-the-authenticated-user)
  pub fn delete_social_account_for_authenticated_user(
    &self,
  ) -> NoContentRequest<delete_social_account_for_authenticated_user::Request, ()> {
    let url = format!("/user/social_accounts");

    NoContentRequest::<delete_social_account_for_authenticated_user::Request, ()>::builder(
      &self.config,
    )
    .delete(url)
    .build()
  }

  /// **List SSH signing keys for the authenticated user**
  ///
  /// Lists the SSH signing keys for the authenticated user's GitHub account.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `read:ssh_signing_key` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/users/ssh-signing-keys#list-ssh-signing-keys-for-the-authenticated-user](https://docs.github.com/rest/users/ssh-signing-keys#list-ssh-signing-keys-for-the-authenticated-user)
  pub fn list_ssh_signing_keys_for_authenticated_user(
    &self,
  ) -> Request<
    (),
    list_ssh_signing_keys_for_authenticated_user::Query,
    list_ssh_signing_keys_for_authenticated_user::Response,
  > {
    let url = format!("/user/ssh_signing_keys");

    Request::<
      (),
      list_ssh_signing_keys_for_authenticated_user::Query,
      list_ssh_signing_keys_for_authenticated_user::Response,
    >::builder(&self.config)
    .get(url)
    .build()
  }

  /// **Create a SSH signing key for the authenticated user**
  ///
  /// Creates an SSH signing key for the authenticated user's GitHub account.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `write:ssh_signing_key` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/users/ssh-signing-keys#create-a-ssh-signing-key-for-the-authenticated-user](https://docs.github.com/rest/users/ssh-signing-keys#create-a-ssh-signing-key-for-the-authenticated-user)
  pub fn create_ssh_signing_key_for_authenticated_user(
    &self,
  ) -> Request<
    create_ssh_signing_key_for_authenticated_user::Request,
    (),
    create_ssh_signing_key_for_authenticated_user::Response,
  > {
    let url = format!("/user/ssh_signing_keys");

    Request::<
      create_ssh_signing_key_for_authenticated_user::Request,
      (),
      create_ssh_signing_key_for_authenticated_user::Response,
    >::builder(&self.config)
    .post(url)
    .build()
  }

  /// **Get an SSH signing key for the authenticated user**
  ///
  /// Gets extended details for an SSH signing key.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `read:ssh_signing_key` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/users/ssh-signing-keys#get-an-ssh-signing-key-for-the-authenticated-user](https://docs.github.com/rest/users/ssh-signing-keys#get-an-ssh-signing-key-for-the-authenticated-user)
  pub fn get_ssh_signing_key_for_authenticated_user(
    &self,
    ssh_signing_key_id: impl Into<i64>,
  ) -> Request<(), (), get_ssh_signing_key_for_authenticated_user::Response> {
    let ssh_signing_key_id = ssh_signing_key_id.into();
    let url = format!("/user/ssh_signing_keys/{ssh_signing_key_id}");

    Request::<(), (), get_ssh_signing_key_for_authenticated_user::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Delete an SSH signing key for the authenticated user**
  ///
  /// Deletes an SSH signing key from the authenticated user's GitHub account.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `admin:ssh_signing_key` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/users/ssh-signing-keys#delete-an-ssh-signing-key-for-the-authenticated-user](https://docs.github.com/rest/users/ssh-signing-keys#delete-an-ssh-signing-key-for-the-authenticated-user)
  pub fn delete_ssh_signing_key_for_authenticated_user(
    &self,
    ssh_signing_key_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let ssh_signing_key_id = ssh_signing_key_id.into();
    let url = format!("/user/ssh_signing_keys/{ssh_signing_key_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List users**
  ///
  /// Lists all users, in the order that they signed up on GitHub. This list includes personal user accounts and organization accounts.
  ///
  /// Note: Pagination is powered exclusively by the `since` parameter. Use the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers) to get the URL for the next page of users.
  ///
  /// *Documentation*: [https://docs.github.com/rest/users/users#list-users](https://docs.github.com/rest/users/users#list-users)
  pub fn list(&self) -> Request<(), list::Query, list::Response> {
    let url = format!("/users");

    Request::<(), list::Query, list::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get a user**
  ///
  /// Provides publicly available information about someone with a GitHub account.
  ///
  /// The `email` key in the following response is the publicly visible email address from your GitHub [profile page](https://github.com/settings/profile). When setting up your profile, you can select a primary email address to be “public” which provides an email entry for this endpoint. If you do not set a public email address for `email`, then it will have a value of `null`. You only see publicly visible email addresses when authenticated with GitHub. For more information, see [Authentication](https://docs.github.com/rest/guides/getting-started-with-the-rest-api#authentication).
  ///
  /// The Emails API enables you to list all of your email addresses, and toggle a primary email to be visible publicly. For more information, see "[Emails API](https://docs.github.com/rest/users/emails)".
  ///
  /// *Documentation*: [https://docs.github.com/rest/users/users#get-a-user](https://docs.github.com/rest/users/users#get-a-user)
  pub fn get_by_username(
    &self,
    username: impl Into<String>,
  ) -> Request<(), (), get_by_username::Response> {
    let username = username.into();
    let url = format!("/users/{username}");

    Request::<(), (), get_by_username::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List followers of a user**
  ///
  /// Lists the people following the specified user.
  ///
  /// *Documentation*: [https://docs.github.com/rest/users/followers#list-followers-of-a-user](https://docs.github.com/rest/users/followers#list-followers-of-a-user)
  pub fn list_followers_for_user(
    &self,
    username: impl Into<String>,
  ) -> Request<(), list_followers_for_user::Query, list_followers_for_user::Response> {
    let username = username.into();
    let url = format!("/users/{username}/followers");

    Request::<(), list_followers_for_user::Query, list_followers_for_user::Response>::builder(
      &self.config,
    )
    .get(url)
    .build()
  }

  /// **List the people a user follows**
  ///
  /// Lists the people who the specified user follows.
  ///
  /// *Documentation*: [https://docs.github.com/rest/users/followers#list-the-people-a-user-follows](https://docs.github.com/rest/users/followers#list-the-people-a-user-follows)
  pub fn list_following_for_user(
    &self,
    username: impl Into<String>,
  ) -> Request<(), list_following_for_user::Query, list_following_for_user::Response> {
    let username = username.into();
    let url = format!("/users/{username}/following");

    Request::<(), list_following_for_user::Query, list_following_for_user::Response>::builder(
      &self.config,
    )
    .get(url)
    .build()
  }

  /// **Check if a user follows another user**
  ///
  ///
  /// *Documentation*: [https://docs.github.com/rest/users/followers#check-if-a-user-follows-another-user](https://docs.github.com/rest/users/followers#check-if-a-user-follows-another-user)
  pub fn check_following_for_user(
    &self,
    username: impl Into<String>,
    target_user: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let username = username.into();
    let target_user = target_user.into();
    let url = format!("/users/{username}/following/{target_user}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List GPG keys for a user**
  ///
  /// Lists the GPG keys for a user. This information is accessible by anyone.
  ///
  /// *Documentation*: [https://docs.github.com/rest/users/gpg-keys#list-gpg-keys-for-a-user](https://docs.github.com/rest/users/gpg-keys#list-gpg-keys-for-a-user)
  pub fn list_gpg_keys_for_user(
    &self,
    username: impl Into<String>,
  ) -> Request<(), list_gpg_keys_for_user::Query, list_gpg_keys_for_user::Response> {
    let username = username.into();
    let url = format!("/users/{username}/gpg_keys");

    Request::<(), list_gpg_keys_for_user::Query, list_gpg_keys_for_user::Response>::builder(
      &self.config,
    )
    .get(url)
    .build()
  }

  /// **Get contextual information for a user**
  ///
  /// Provides hovercard information. You can find out more about someone in relation to their pull requests, issues, repositories, and organizations.
  ///
  ///   The `subject_type` and `subject_id` parameters provide context for the person's hovercard, which returns more information than without the parameters. For example, if you wanted to find out more about `octocat` who owns the `Spoon-Knife` repository, you would use a `subject_type` value of `repository` and a `subject_id` value of `1300192` (the ID of the `Spoon-Knife` repository).
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/users/users#get-contextual-information-for-a-user](https://docs.github.com/rest/users/users#get-contextual-information-for-a-user)
  pub fn get_context_for_user(
    &self,
    username: impl Into<String>,
  ) -> Request<(), get_context_for_user::Query, get_context_for_user::Response> {
    let username = username.into();
    let url = format!("/users/{username}/hovercard");

    Request::<(), get_context_for_user::Query, get_context_for_user::Response>::builder(
      &self.config,
    )
    .get(url)
    .build()
  }

  /// **List public keys for a user**
  ///
  /// Lists the _verified_ public SSH keys for a user. This is accessible by anyone.
  ///
  /// *Documentation*: [https://docs.github.com/rest/users/keys#list-public-keys-for-a-user](https://docs.github.com/rest/users/keys#list-public-keys-for-a-user)
  pub fn list_public_keys_for_user(
    &self,
    username: impl Into<String>,
  ) -> Request<(), list_public_keys_for_user::Query, list_public_keys_for_user::Response> {
    let username = username.into();
    let url = format!("/users/{username}/keys");

    Request::<(), list_public_keys_for_user::Query, list_public_keys_for_user::Response>::builder(
      &self.config,
    )
    .get(url)
    .build()
  }

  /// **List social accounts for a user**
  ///
  /// Lists social media accounts for a user. This endpoint is accessible by anyone.
  ///
  /// *Documentation*: [https://docs.github.com/rest/users/social-accounts#list-social-accounts-for-a-user](https://docs.github.com/rest/users/social-accounts#list-social-accounts-for-a-user)
  pub fn list_social_accounts_for_user(
    &self,
    username: impl Into<String>,
  ) -> Request<(), list_social_accounts_for_user::Query, list_social_accounts_for_user::Response>
  {
    let username = username.into();
    let url = format!("/users/{username}/social_accounts");

    Request::<(), list_social_accounts_for_user::Query, list_social_accounts_for_user::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List SSH signing keys for a user**
  ///
  /// Lists the SSH signing keys for a user. This operation is accessible by anyone.
  ///
  /// *Documentation*: [https://docs.github.com/rest/users/ssh-signing-keys#list-ssh-signing-keys-for-a-user](https://docs.github.com/rest/users/ssh-signing-keys#list-ssh-signing-keys-for-a-user)
  pub fn list_ssh_signing_keys_for_user(
    &self,
    username: impl Into<String>,
  ) -> Request<(), list_ssh_signing_keys_for_user::Query, list_ssh_signing_keys_for_user::Response>
  {
    let username = username.into();
    let url = format!("/users/{username}/ssh_signing_keys");

    Request::<(), list_ssh_signing_keys_for_user::Query, list_ssh_signing_keys_for_user::Response>::builder(&self.config)
      .get(url)
      .build()
  }
}
