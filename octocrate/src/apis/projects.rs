#[allow(unused_imports)]
use crate::types::*;
use octocrate_core::*;

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

  /// **List project columns**
  ///
  /// Lists the project columns in a project.
  ///
  /// *Documentation*: [https://docs.github.com/rest/projects/columns#list-project-columns](https://docs.github.com/rest/projects/columns#list-project-columns)
  pub fn list_columns(
    &self,
    project_id: impl Into<i64>,
  ) -> Request<(), ProjectsListColumnsQuery, ProjectColumnArray> {
    let project_id = project_id.into();
    let url = format!("/projects/{project_id}/columns");

    Request::<(), ProjectsListColumnsQuery, ProjectColumnArray>::builder(&self.config)
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
  ) -> Request<ProjectsCreateColumnRequest, (), ProjectColumn> {
    let project_id = project_id.into();
    let url = format!("/projects/{project_id}/columns");

    Request::<ProjectsCreateColumnRequest, (), ProjectColumn>::builder(&self.config)
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
  ) -> Request<(), ProjectsListForUserQuery, ProjectArray> {
    let username = username.into();
    let url = format!("/users/{username}/projects");

    Request::<(), ProjectsListForUserQuery, ProjectArray>::builder(&self.config)
      .get(url)
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
  ) -> Request<(), ProjectsListForRepoQuery, ProjectArray> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/projects");

    Request::<(), ProjectsListForRepoQuery, ProjectArray>::builder(&self.config)
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
  ) -> Request<ProjectsCreateForRepoRequest, (), Project> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/projects");

    Request::<ProjectsCreateForRepoRequest, (), Project>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Get a project column**
  ///
  /// Gets information about a project column.
  ///
  /// *Documentation*: [https://docs.github.com/rest/projects/columns#get-a-project-column](https://docs.github.com/rest/projects/columns#get-a-project-column)
  pub fn get_column(&self, column_id: impl Into<i64>) -> Request<(), (), ProjectColumn> {
    let column_id = column_id.into();
    let url = format!("/projects/columns/{column_id}");

    Request::<(), (), ProjectColumn>::builder(&self.config)
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
  ) -> Request<ProjectsUpdateColumnRequest, (), ProjectColumn> {
    let column_id = column_id.into();
    let url = format!("/projects/columns/{column_id}");

    Request::<ProjectsUpdateColumnRequest, (), ProjectColumn>::builder(&self.config)
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

  /// **Get a project**
  ///
  /// Gets a project by its `id`. Returns a `404 Not Found` status if projects are disabled. If you do not have sufficient privileges to perform this action, a `401 Unauthorized` or `410 Gone` status is returned.
  ///
  /// *Documentation*: [https://docs.github.com/rest/projects/projects#get-a-project](https://docs.github.com/rest/projects/projects#get-a-project)
  pub fn get(&self, project_id: impl Into<i64>) -> Request<(), (), Project> {
    let project_id = project_id.into();
    let url = format!("/projects/{project_id}");

    Request::<(), (), Project>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Update a project**
  ///
  /// Updates a project board's information. Returns a `404 Not Found` status if projects are disabled. If you do not have sufficient privileges to perform this action, a `401 Unauthorized` or `410 Gone` status is returned.
  ///
  /// *Documentation*: [https://docs.github.com/rest/projects/projects#update-a-project](https://docs.github.com/rest/projects/projects#update-a-project)
  pub fn update(&self, project_id: impl Into<i64>) -> Request<ProjectsUpdateRequest, (), Project> {
    let project_id = project_id.into();
    let url = format!("/projects/{project_id}");

    Request::<ProjectsUpdateRequest, (), Project>::builder(&self.config)
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

  /// **List project cards**
  ///
  /// Lists the project cards in a project.
  ///
  /// *Documentation*: [https://docs.github.com/rest/projects/cards#list-project-cards](https://docs.github.com/rest/projects/cards#list-project-cards)
  pub fn list_cards(
    &self,
    column_id: impl Into<i64>,
  ) -> Request<(), ProjectsListCardsQuery, ProjectCardArray> {
    let column_id = column_id.into();
    let url = format!("/projects/columns/{column_id}/cards");

    Request::<(), ProjectsListCardsQuery, ProjectCardArray>::builder(&self.config)
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
  ) -> Request<ProjectsCreateCardRequest, (), ProjectCard> {
    let column_id = column_id.into();
    let url = format!("/projects/columns/{column_id}/cards");

    Request::<ProjectsCreateCardRequest, (), ProjectCard>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Get a project card**
  ///
  /// Gets information about a project card.
  ///
  /// *Documentation*: [https://docs.github.com/rest/projects/cards#get-a-project-card](https://docs.github.com/rest/projects/cards#get-a-project-card)
  pub fn get_card(&self, card_id: impl Into<i64>) -> Request<(), (), ProjectCard> {
    let card_id = card_id.into();
    let url = format!("/projects/columns/cards/{card_id}");

    Request::<(), (), ProjectCard>::builder(&self.config)
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
  ) -> Request<ProjectsUpdateCardRequest, (), ProjectCard> {
    let card_id = card_id.into();
    let url = format!("/projects/columns/cards/{card_id}");

    Request::<ProjectsUpdateCardRequest, (), ProjectCard>::builder(&self.config)
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

  /// **Move a project column**
  ///
  ///
  /// *Documentation*: [https://docs.github.com/rest/projects/columns#move-a-project-column](https://docs.github.com/rest/projects/columns#move-a-project-column)
  pub fn move_column(
    &self,
    column_id: impl Into<i64>,
  ) -> Request<ProjectsMoveColumnRequest, (), ProjectsMoveColumnResponse> {
    let column_id = column_id.into();
    let url = format!("/projects/columns/{column_id}/moves");

    Request::<ProjectsMoveColumnRequest, (), ProjectsMoveColumnResponse>::builder(&self.config)
      .post(url)
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
  ) -> Request<(), (), ProjectCollaboratorPermission> {
    let project_id = project_id.into();
    let username = username.into();
    let url = format!("/projects/{project_id}/collaborators/{username}/permission");

    Request::<(), (), ProjectCollaboratorPermission>::builder(&self.config)
      .get(url)
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
  ) -> Request<(), ProjectsListCollaboratorsQuery, SimpleUserArray> {
    let project_id = project_id.into();
    let url = format!("/projects/{project_id}/collaborators");

    Request::<(), ProjectsListCollaboratorsQuery, SimpleUserArray>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Move a project card**
  ///
  ///
  /// *Documentation*: [https://docs.github.com/rest/projects/cards#move-a-project-card](https://docs.github.com/rest/projects/cards#move-a-project-card)
  pub fn move_card(
    &self,
    card_id: impl Into<i64>,
  ) -> Request<ProjectsMoveCardRequest, (), ProjectsMoveCardResponse> {
    let card_id = card_id.into();
    let url = format!("/projects/columns/cards/{card_id}/moves");

    Request::<ProjectsMoveCardRequest, (), ProjectsMoveCardResponse>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **List organization projects**
  ///
  /// Lists the projects in an organization. Returns a `404 Not Found` status if projects are disabled in the organization. If you do not have sufficient privileges to perform this action, a `401 Unauthorized` or `410 Gone` status is returned.
  ///
  /// *Documentation*: [https://docs.github.com/rest/projects/projects#list-organization-projects](https://docs.github.com/rest/projects/projects#list-organization-projects)
  pub fn list_for_org(
    &self,
    org: impl Into<String>,
  ) -> Request<(), ProjectsListForOrgQuery, ProjectArray> {
    let org = org.into();
    let url = format!("/orgs/{org}/projects");

    Request::<(), ProjectsListForOrgQuery, ProjectArray>::builder(&self.config)
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
  ) -> Request<ProjectsCreateForOrgRequest, (), Project> {
    let org = org.into();
    let url = format!("/orgs/{org}/projects");

    Request::<ProjectsCreateForOrgRequest, (), Project>::builder(&self.config)
      .post(url)
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
  ) -> NoContentRequest<ProjectsAddCollaboratorRequest, ()> {
    let project_id = project_id.into();
    let username = username.into();
    let url = format!("/projects/{project_id}/collaborators/{username}");

    NoContentRequest::<ProjectsAddCollaboratorRequest, ()>::builder(&self.config)
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

  /// **Create a user project**
  ///
  /// Creates a user project board. Returns a `410 Gone` status if the user does not have existing classic projects. If you do not have sufficient privileges to perform this action, a `401 Unauthorized` or `410 Gone` status is returned.
  ///
  /// *Documentation*: [https://docs.github.com/rest/projects/projects#create-a-user-project](https://docs.github.com/rest/projects/projects#create-a-user-project)
  pub fn create_for_authenticated_user(
    &self,
  ) -> Request<ProjectsCreateForAuthenticatedUserRequest, (), Project> {
    let url = format!("/user/projects");

    Request::<ProjectsCreateForAuthenticatedUserRequest, (), Project>::builder(&self.config)
      .post(url)
      .build()
  }
}
