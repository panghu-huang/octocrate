use octocrate_core::*;
#[allow(unused_imports)]
use octocrate_types::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use typed_builder::TypedBuilder;

pub mod list_for_org {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Project>;

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QueryState {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "closed")]
    Closed,
    #[serde(rename = "all")]
    All,
  }

  impl ToString for QueryState {
    fn to_string(&self) -> String {
      match self {
        QueryState::Open => "open".to_string(),
        QueryState::Closed => "closed".to_string(),
        QueryState::All => "all".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// Indicates the state of the projects to return.
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

pub mod create_for_org {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Project;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The description of the project.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub body: Option<String>,
    /// The name of the project.
    pub name: String,
  }
}

pub mod get_card {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = ProjectCard;
}

pub mod update_card {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = ProjectCard;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// Whether or not the card is archived
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub archived: Option<bool>,
    /// The project card's note
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub note: Option<String>,
  }
}

pub mod delete_card {
  #[allow(unused_imports)]
  use super::*;
}

pub mod move_card {
  #[allow(unused_imports)]
  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The unique identifier of the column the card should be moved to
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub column_id: Option<i64>,
    /// The position of the card in a column. Can be one of: `top`, `bottom`, or `after:<card_id>` to place after the specified card.
    pub position: String,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Response {}
}

pub mod get_column {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = ProjectColumn;
}

pub mod update_column {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = ProjectColumn;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// Name of the project column
    pub name: String,
  }
}

pub mod delete_column {
  #[allow(unused_imports)]
  use super::*;
}

pub mod list_cards {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<ProjectCard>;

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QueryArchivedState {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "archived")]
    Archived,
    #[serde(rename = "not_archived")]
    NotArchived,
  }

  impl ToString for QueryArchivedState {
    fn to_string(&self) -> String {
      match self {
        QueryArchivedState::All => "all".to_string(),
        QueryArchivedState::Archived => "archived".to_string(),
        QueryArchivedState::NotArchived => "not_archived".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// Filters the project cards that are returned by the card's state.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub archived_state: Option<QueryArchivedState>,
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

pub mod create_card {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = ProjectCard;

  #[derive(Debug, Clone, Serialize, Deserialize)]
  #[serde(untagged)]
  pub enum Request {
    RequestItem1(RequestItem1),
    RequestItem2(RequestItem2),
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct RequestItem1 {
    /// The project card's note
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub note: Option<String>,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct RequestItem2 {
    /// The unique identifier of the content associated with the card
    pub content_id: i64,
    /// The piece of content associated with the card
    pub content_type: String,
  }
}

pub mod move_column {
  #[allow(unused_imports)]
  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The position of the column in a project. Can be one of: `first`, `last`, or `after:<column_id>` to place after the specified column.
    pub position: String,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Response {}
}

pub mod get {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Project;
}

pub mod update {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Project;

  /// The baseline permission that all organization members have on this project
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum RequestOrganizationPermission {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "none")]
    None,
  }

  impl ToString for RequestOrganizationPermission {
    fn to_string(&self) -> String {
      match self {
        RequestOrganizationPermission::Read => "read".to_string(),
        RequestOrganizationPermission::Write => "write".to_string(),
        RequestOrganizationPermission::Admin => "admin".to_string(),
        RequestOrganizationPermission::None => "none".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// Body of the project
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub body: Option<String>,
    /// Name of the project
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub name: Option<String>,
    /// The baseline permission that all organization members have on this project
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub organization_permission: Option<RequestOrganizationPermission>,
    /// Whether or not this project can be seen by everyone.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub private: Option<bool>,
    /// State of the project; either 'open' or 'closed'
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub state: Option<String>,
  }
}

pub mod delete {
  #[allow(unused_imports)]
  use super::*;
}

pub mod list_collaborators {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<SimpleUser>;

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QueryAffiliation {
    #[serde(rename = "outside")]
    Outside,
    #[serde(rename = "direct")]
    Direct,
    #[serde(rename = "all")]
    All,
  }

  impl ToString for QueryAffiliation {
    fn to_string(&self) -> String {
      match self {
        QueryAffiliation::Outside => "outside".to_string(),
        QueryAffiliation::Direct => "direct".to_string(),
        QueryAffiliation::All => "all".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// Filters the collaborators by their affiliation. `outside` means outside collaborators of a project that are not a member of the project's organization. `direct` means collaborators with permissions to a project, regardless of organization membership status. `all` means all collaborators the authenticated user can see.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub affiliation: Option<QueryAffiliation>,
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

pub mod add_collaborator {
  #[allow(unused_imports)]
  use super::*;

  /// The permission to grant the collaborator.
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum RequestPermission {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
    #[serde(rename = "admin")]
    Admin,
  }

  impl ToString for RequestPermission {
    fn to_string(&self) -> String {
      match self {
        RequestPermission::Read => "read".to_string(),
        RequestPermission::Write => "write".to_string(),
        RequestPermission::Admin => "admin".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The permission to grant the collaborator.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub permission: Option<RequestPermission>,
  }
}

pub mod remove_collaborator {
  #[allow(unused_imports)]
  use super::*;
}

pub mod get_permission_for_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = ProjectCollaboratorPermission;
}

pub mod list_columns {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<ProjectColumn>;

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

pub mod create_column {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = ProjectColumn;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// Name of the project column
    pub name: String,
  }
}

pub mod list_for_repo {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Project>;

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QueryState {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "closed")]
    Closed,
    #[serde(rename = "all")]
    All,
  }

  impl ToString for QueryState {
    fn to_string(&self) -> String {
      match self {
        QueryState::Open => "open".to_string(),
        QueryState::Closed => "closed".to_string(),
        QueryState::All => "all".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// Indicates the state of the projects to return.
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

pub mod create_for_repo {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Project;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The description of the project.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub body: Option<String>,
    /// The name of the project.
    pub name: String,
  }
}

pub mod create_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Project;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// Body of the project
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub body: Option<String>,
    /// Name of the project
    pub name: String,
  }
}

pub mod list_for_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Project>;

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QueryState {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "closed")]
    Closed,
    #[serde(rename = "all")]
    All,
  }

  impl ToString for QueryState {
    fn to_string(&self) -> String {
      match self {
        QueryState::Open => "open".to_string(),
        QueryState::Closed => "closed".to_string(),
        QueryState::All => "all".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// Indicates the state of the projects to return.
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

/// Interact with GitHub Projects.
pub struct GitHubProjectsAPI {
  config: SharedAPIConfig,
}

impl GitHubProjectsAPI {
  pub fn new(config: &SharedAPIConfig) -> Self {
    Self {
      config: config.clone(),
    }
  }

  /// **List organization projects**
  ///
  /// Lists the projects in an organization. Returns a `404 Not Found` status if projects are disabled in the organization. If you do not have sufficient privileges to perform this action, a `401 Unauthorized` or `410 Gone` status is returned.
  ///
  /// *Documentation*: [https://docs.github.com/rest/projects/projects#list-organization-projects](https://docs.github.com/rest/projects/projects#list-organization-projects)
  pub fn list_for_org(
    &self,
    org: impl Into<String>,
  ) -> Request<(), list_for_org::Query, list_for_org::Response> {
    let org = org.into();
    let url = format!("/orgs/{org}/projects");

    Request::<(), list_for_org::Query, list_for_org::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create an organization project**
  ///
  /// Creates an organization project board. Returns a `410 Gone` status if projects are disabled in the organization or if the organization does not have existing classic projects. If you do not have sufficient privileges to perform this action, a `401 Unauthorized` or `410 Gone` status is returned.
  ///
  /// *Documentation*: [https://docs.github.com/rest/projects/projects#create-an-organization-project](https://docs.github.com/rest/projects/projects#create-an-organization-project)
  pub fn create_for_org(
    &self,
    org: impl Into<String>,
  ) -> Request<create_for_org::Request, (), create_for_org::Response> {
    let org = org.into();
    let url = format!("/orgs/{org}/projects");

    Request::<create_for_org::Request, (), create_for_org::Response>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Get a project card**
  ///
  /// Gets information about a project card.
  ///
  /// *Documentation*: [https://docs.github.com/rest/projects/cards#get-a-project-card](https://docs.github.com/rest/projects/cards#get-a-project-card)
  pub fn get_card(&self, card_id: impl Into<i64>) -> Request<(), (), get_card::Response> {
    let card_id = card_id.into();
    let url = format!("/projects/columns/cards/{card_id}");

    Request::<(), (), get_card::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Update an existing project card**
  ///
  ///
  /// *Documentation*: [https://docs.github.com/rest/projects/cards#update-an-existing-project-card](https://docs.github.com/rest/projects/cards#update-an-existing-project-card)
  pub fn update_card(
    &self,
    card_id: impl Into<i64>,
  ) -> Request<update_card::Request, (), update_card::Response> {
    let card_id = card_id.into();
    let url = format!("/projects/columns/cards/{card_id}");

    Request::<update_card::Request, (), update_card::Response>::builder(&self.config)
      .patch(url)
      .build()
  }

  /// **Delete a project card**
  ///
  /// Deletes a project card
  ///
  /// *Documentation*: [https://docs.github.com/rest/projects/cards#delete-a-project-card](https://docs.github.com/rest/projects/cards#delete-a-project-card)
  pub fn delete_card(&self, card_id: impl Into<i64>) -> NoContentRequest<(), ()> {
    let card_id = card_id.into();
    let url = format!("/projects/columns/cards/{card_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Move a project card**
  ///
  ///
  /// *Documentation*: [https://docs.github.com/rest/projects/cards#move-a-project-card](https://docs.github.com/rest/projects/cards#move-a-project-card)
  pub fn move_card(
    &self,
    card_id: impl Into<i64>,
  ) -> Request<move_card::Request, (), move_card::Response> {
    let card_id = card_id.into();
    let url = format!("/projects/columns/cards/{card_id}/moves");

    Request::<move_card::Request, (), move_card::Response>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Get a project column**
  ///
  /// Gets information about a project column.
  ///
  /// *Documentation*: [https://docs.github.com/rest/projects/columns#get-a-project-column](https://docs.github.com/rest/projects/columns#get-a-project-column)
  pub fn get_column(&self, column_id: impl Into<i64>) -> Request<(), (), get_column::Response> {
    let column_id = column_id.into();
    let url = format!("/projects/columns/{column_id}");

    Request::<(), (), get_column::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Update an existing project column**
  ///
  ///
  /// *Documentation*: [https://docs.github.com/rest/projects/columns#update-an-existing-project-column](https://docs.github.com/rest/projects/columns#update-an-existing-project-column)
  pub fn update_column(
    &self,
    column_id: impl Into<i64>,
  ) -> Request<update_column::Request, (), update_column::Response> {
    let column_id = column_id.into();
    let url = format!("/projects/columns/{column_id}");

    Request::<update_column::Request, (), update_column::Response>::builder(&self.config)
      .patch(url)
      .build()
  }

  /// **Delete a project column**
  ///
  /// Deletes a project column.
  ///
  /// *Documentation*: [https://docs.github.com/rest/projects/columns#delete-a-project-column](https://docs.github.com/rest/projects/columns#delete-a-project-column)
  pub fn delete_column(&self, column_id: impl Into<i64>) -> NoContentRequest<(), ()> {
    let column_id = column_id.into();
    let url = format!("/projects/columns/{column_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List project cards**
  ///
  /// Lists the project cards in a project.
  ///
  /// *Documentation*: [https://docs.github.com/rest/projects/cards#list-project-cards](https://docs.github.com/rest/projects/cards#list-project-cards)
  pub fn list_cards(
    &self,
    column_id: impl Into<i64>,
  ) -> Request<(), list_cards::Query, list_cards::Response> {
    let column_id = column_id.into();
    let url = format!("/projects/columns/{column_id}/cards");

    Request::<(), list_cards::Query, list_cards::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create a project card**
  ///
  ///
  /// *Documentation*: [https://docs.github.com/rest/projects/cards#create-a-project-card](https://docs.github.com/rest/projects/cards#create-a-project-card)
  pub fn create_card(
    &self,
    column_id: impl Into<i64>,
  ) -> Request<create_card::Request, (), create_card::Response> {
    let column_id = column_id.into();
    let url = format!("/projects/columns/{column_id}/cards");

    Request::<create_card::Request, (), create_card::Response>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Move a project column**
  ///
  ///
  /// *Documentation*: [https://docs.github.com/rest/projects/columns#move-a-project-column](https://docs.github.com/rest/projects/columns#move-a-project-column)
  pub fn move_column(
    &self,
    column_id: impl Into<i64>,
  ) -> Request<move_column::Request, (), move_column::Response> {
    let column_id = column_id.into();
    let url = format!("/projects/columns/{column_id}/moves");

    Request::<move_column::Request, (), move_column::Response>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Get a project**
  ///
  /// Gets a project by its `id`. Returns a `404 Not Found` status if projects are disabled. If you do not have sufficient privileges to perform this action, a `401 Unauthorized` or `410 Gone` status is returned.
  ///
  /// *Documentation*: [https://docs.github.com/rest/projects/projects#get-a-project](https://docs.github.com/rest/projects/projects#get-a-project)
  pub fn get(&self, project_id: impl Into<i64>) -> Request<(), (), get::Response> {
    let project_id = project_id.into();
    let url = format!("/projects/{project_id}");

    Request::<(), (), get::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Update a project**
  ///
  /// Updates a project board's information. Returns a `404 Not Found` status if projects are disabled. If you do not have sufficient privileges to perform this action, a `401 Unauthorized` or `410 Gone` status is returned.
  ///
  /// *Documentation*: [https://docs.github.com/rest/projects/projects#update-a-project](https://docs.github.com/rest/projects/projects#update-a-project)
  pub fn update(
    &self,
    project_id: impl Into<i64>,
  ) -> Request<update::Request, (), update::Response> {
    let project_id = project_id.into();
    let url = format!("/projects/{project_id}");

    Request::<update::Request, (), update::Response>::builder(&self.config)
      .patch(url)
      .build()
  }

  /// **Delete a project**
  ///
  /// Deletes a project board. Returns a `404 Not Found` status if projects are disabled.
  ///
  /// *Documentation*: [https://docs.github.com/rest/projects/projects#delete-a-project](https://docs.github.com/rest/projects/projects#delete-a-project)
  pub fn delete(&self, project_id: impl Into<i64>) -> NoContentRequest<(), ()> {
    let project_id = project_id.into();
    let url = format!("/projects/{project_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List project collaborators**
  ///
  /// Lists the collaborators for an organization project. For a project, the list of collaborators includes outside collaborators, organization members that are direct collaborators, organization members with access through team memberships, organization members with access through default organization permissions, and organization owners. You must be an organization owner or a project `admin` to list collaborators.
  ///
  /// *Documentation*: [https://docs.github.com/rest/projects/collaborators#list-project-collaborators](https://docs.github.com/rest/projects/collaborators#list-project-collaborators)
  pub fn list_collaborators(
    &self,
    project_id: impl Into<i64>,
  ) -> Request<(), list_collaborators::Query, list_collaborators::Response> {
    let project_id = project_id.into();
    let url = format!("/projects/{project_id}/collaborators");

    Request::<(), list_collaborators::Query, list_collaborators::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Add project collaborator**
  ///
  /// Adds a collaborator to an organization project and sets their permission level. You must be an organization owner or a project `admin` to add a collaborator.
  ///
  /// *Documentation*: [https://docs.github.com/rest/projects/collaborators#add-project-collaborator](https://docs.github.com/rest/projects/collaborators#add-project-collaborator)
  pub fn add_collaborator(
    &self,
    project_id: impl Into<i64>,
    username: impl Into<String>,
  ) -> NoContentRequest<add_collaborator::Request, ()> {
    let project_id = project_id.into();
    let username = username.into();
    let url = format!("/projects/{project_id}/collaborators/{username}");

    NoContentRequest::<add_collaborator::Request, ()>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **Remove user as a collaborator**
  ///
  /// Removes a collaborator from an organization project. You must be an organization owner or a project `admin` to remove a collaborator.
  ///
  /// *Documentation*: [https://docs.github.com/rest/projects/collaborators#remove-user-as-a-collaborator](https://docs.github.com/rest/projects/collaborators#remove-user-as-a-collaborator)
  pub fn remove_collaborator(
    &self,
    project_id: impl Into<i64>,
    username: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let project_id = project_id.into();
    let username = username.into();
    let url = format!("/projects/{project_id}/collaborators/{username}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Get project permission for a user**
  ///
  /// Returns the collaborator's permission level for an organization project. Possible values for the `permission` key: `admin`, `write`, `read`, `none`. You must be an organization owner or a project `admin` to review a user's permission level.
  ///
  /// *Documentation*: [https://docs.github.com/rest/projects/collaborators#get-project-permission-for-a-user](https://docs.github.com/rest/projects/collaborators#get-project-permission-for-a-user)
  pub fn get_permission_for_user(
    &self,
    project_id: impl Into<i64>,
    username: impl Into<String>,
  ) -> Request<(), (), get_permission_for_user::Response> {
    let project_id = project_id.into();
    let username = username.into();
    let url = format!("/projects/{project_id}/collaborators/{username}/permission");

    Request::<(), (), get_permission_for_user::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List project columns**
  ///
  /// Lists the project columns in a project.
  ///
  /// *Documentation*: [https://docs.github.com/rest/projects/columns#list-project-columns](https://docs.github.com/rest/projects/columns#list-project-columns)
  pub fn list_columns(
    &self,
    project_id: impl Into<i64>,
  ) -> Request<(), list_columns::Query, list_columns::Response> {
    let project_id = project_id.into();
    let url = format!("/projects/{project_id}/columns");

    Request::<(), list_columns::Query, list_columns::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create a project column**
  ///
  /// Creates a new project column.
  ///
  /// *Documentation*: [https://docs.github.com/rest/projects/columns#create-a-project-column](https://docs.github.com/rest/projects/columns#create-a-project-column)
  pub fn create_column(
    &self,
    project_id: impl Into<i64>,
  ) -> Request<create_column::Request, (), create_column::Response> {
    let project_id = project_id.into();
    let url = format!("/projects/{project_id}/columns");

    Request::<create_column::Request, (), create_column::Response>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **List repository projects**
  ///
  /// Lists the projects in a repository. Returns a `404 Not Found` status if projects are disabled in the repository. If you do not have sufficient privileges to perform this action, a `401 Unauthorized` or `410 Gone` status is returned.
  ///
  /// *Documentation*: [https://docs.github.com/rest/projects/projects#list-repository-projects](https://docs.github.com/rest/projects/projects#list-repository-projects)
  pub fn list_for_repo(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), list_for_repo::Query, list_for_repo::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/projects");

    Request::<(), list_for_repo::Query, list_for_repo::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create a repository project**
  ///
  /// Creates a repository project board. Returns a `410 Gone` status if projects are disabled in the repository or if the repository does not have existing classic projects. If you do not have sufficient privileges to perform this action, a `401 Unauthorized` or `410 Gone` status is returned.
  ///
  /// *Documentation*: [https://docs.github.com/rest/projects/projects#create-a-repository-project](https://docs.github.com/rest/projects/projects#create-a-repository-project)
  pub fn create_for_repo(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<create_for_repo::Request, (), create_for_repo::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/projects");

    Request::<create_for_repo::Request, (), create_for_repo::Response>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Create a user project**
  ///
  /// Creates a user project board. Returns a `410 Gone` status if the user does not have existing classic projects. If you do not have sufficient privileges to perform this action, a `401 Unauthorized` or `410 Gone` status is returned.
  ///
  /// *Documentation*: [https://docs.github.com/rest/projects/projects#create-a-user-project](https://docs.github.com/rest/projects/projects#create-a-user-project)
  pub fn create_for_authenticated_user(
    &self,
  ) -> Request<create_for_authenticated_user::Request, (), create_for_authenticated_user::Response>
  {
    let url = format!("/user/projects");

    Request::<create_for_authenticated_user::Request, (), create_for_authenticated_user::Response>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **List user projects**
  ///
  /// Lists projects for a user.
  ///
  /// *Documentation*: [https://docs.github.com/rest/projects/projects#list-user-projects](https://docs.github.com/rest/projects/projects#list-user-projects)
  pub fn list_for_user(
    &self,
    username: impl Into<String>,
  ) -> Request<(), list_for_user::Query, list_for_user::Response> {
    let username = username.into();
    let url = format!("/users/{username}/projects");

    Request::<(), list_for_user::Query, list_for_user::Response>::builder(&self.config)
      .get(url)
      .build()
  }
}
