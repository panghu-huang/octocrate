use octocrate_core::*;
#[allow(unused_imports)]
use octocrate_types::*;

/// Interact with GitHub Teams.
pub struct GitHubTeamsAPI {
  config: SharedAPIConfig,
}

impl GitHubTeamsAPI {
  pub fn new(config: &SharedAPIConfig) -> Self {
    Self {
      config: config.clone(),
    }
  }

  /// **Get team member (Legacy)**
  ///
  /// The "Get team member" endpoint (described below) is deprecated.
  ///
  /// We recommend using the [Get team membership for a user](https://docs.github.com/rest/teams/members#get-team-membership-for-a-user) endpoint instead. It allows you to get both active and pending memberships.
  ///
  /// To list members in a team, the team must be visible to the authenticated user.
  ///
  /// *Documentation*: [https://docs.github.com/rest/teams/members#get-team-member-legacy](https://docs.github.com/rest/teams/members#get-team-member-legacy)
  pub fn get_member_legacy(
    &self,
    team_id: impl Into<i64>,
    username: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let team_id = team_id.into();
    let username = username.into();
    let url = format!("/teams/{team_id}/members/{username}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Add team member (Legacy)**
  ///
  /// The "Add team member" endpoint (described below) is deprecated.
  ///
  /// We recommend using the [Add or update team membership for a user](https://docs.github.com/rest/teams/members#add-or-update-team-membership-for-a-user) endpoint instead. It allows you to invite new organization members to your teams.
  ///
  /// Team synchronization is available for organizations using GitHub Enterprise Cloud. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
  ///
  /// To add someone to a team, the authenticated user must be an organization owner or a team maintainer in the team they're changing. The person being added to the team must be a member of the team's organization.
  ///
  /// **Note:** When you have team synchronization set up for a team with your organization's identity provider (IdP), you will see an error if you attempt to use the API for making changes to the team's membership. If you have access to manage group membership in your IdP, you can manage GitHub team membership through your identity provider, which automatically adds and removes team members in an organization. For more information, see "[Synchronizing teams between your identity provider and GitHub](https://docs.github.com/articles/synchronizing-teams-between-your-identity-provider-and-github/)."
  ///
  /// Note that you'll need to set `Content-Length` to zero when calling out to this endpoint. For more information, see "[HTTP method](https://docs.github.com/rest/guides/getting-started-with-the-rest-api#http-method)."
  ///
  /// *Documentation*: [https://docs.github.com/rest/teams/members#add-team-member-legacy](https://docs.github.com/rest/teams/members#add-team-member-legacy)
  pub fn add_member_legacy(
    &self,
    team_id: impl Into<i64>,
    username: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let team_id = team_id.into();
    let username = username.into();
    let url = format!("/teams/{team_id}/members/{username}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **Remove team member (Legacy)**
  ///
  /// The "Remove team member" endpoint (described below) is deprecated.
  ///
  /// We recommend using the [Remove team membership for a user](https://docs.github.com/rest/teams/members#remove-team-membership-for-a-user) endpoint instead. It allows you to remove both active and pending memberships.
  ///
  /// Team synchronization is available for organizations using GitHub Enterprise Cloud. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
  ///
  /// To remove a team member, the authenticated user must have 'admin' permissions to the team or be an owner of the org that the team is associated with. Removing a team member does not delete the user, it just removes them from the team.
  ///
  /// **Note:** When you have team synchronization set up for a team with your organization's identity provider (IdP), you will see an error if you attempt to use the API for making changes to the team's membership. If you have access to manage group membership in your IdP, you can manage GitHub team membership through your identity provider, which automatically adds and removes team members in an organization. For more information, see "[Synchronizing teams between your identity provider and GitHub](https://docs.github.com/articles/synchronizing-teams-between-your-identity-provider-and-github/)."
  ///
  /// *Documentation*: [https://docs.github.com/rest/teams/members#remove-team-member-legacy](https://docs.github.com/rest/teams/members#remove-team-member-legacy)
  pub fn remove_member_legacy(
    &self,
    team_id: impl Into<i64>,
    username: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let team_id = team_id.into();
    let username = username.into();
    let url = format!("/teams/{team_id}/members/{username}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Check team permissions for a repository (Legacy)**
  ///
  /// **Note**: Repositories inherited through a parent team will also be checked.
  ///
  /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [Check team permissions for a repository](https://docs.github.com/rest/teams/teams#check-team-permissions-for-a-repository) endpoint.
  ///
  /// You can also get information about the specified repository, including what permissions the team grants on it, by passing the following custom [media type](https://docs.github.com/rest/overview/media-types/) via the `Accept` header:
  ///
  /// *Documentation*: [https://docs.github.com/rest/teams/teams#check-team-permissions-for-a-repository-legacy](https://docs.github.com/rest/teams/teams#check-team-permissions-for-a-repository-legacy)
  pub fn check_permissions_for_repo_legacy(
    &self,
    team_id: impl Into<i64>,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), (), TeamRepository> {
    let team_id = team_id.into();
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/teams/{team_id}/repos/{owner}/{repo}");

    Request::<(), (), TeamRepository>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Add or update team repository permissions (Legacy)**
  ///
  /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new "[Add or update team repository permissions](https://docs.github.com/rest/teams/teams#add-or-update-team-repository-permissions)" endpoint.
  ///
  /// To add a repository to a team or update the team's permission on a repository, the authenticated user must have admin access to the repository, and must be able to see the team. The repository must be owned by the organization, or a direct fork of a repository owned by the organization. You will get a `422 Unprocessable Entity` status if you attempt to add a repository to a team that is not owned by the organization.
  ///
  /// Note that, if you choose not to pass any parameters, you'll need to set `Content-Length` to zero when calling out to this endpoint. For more information, see "[HTTP method](https://docs.github.com/rest/guides/getting-started-with-the-rest-api#http-method)."
  ///
  /// *Documentation*: [https://docs.github.com/rest/teams/teams#add-or-update-team-repository-permissions-legacy](https://docs.github.com/rest/teams/teams#add-or-update-team-repository-permissions-legacy)
  pub fn add_or_update_repo_permissions_legacy(
    &self,
    team_id: impl Into<i64>,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> NoContentRequest<TeamsAddOrUpdateRepoPermissionsLegacyRequest, ()> {
    let team_id = team_id.into();
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/teams/{team_id}/repos/{owner}/{repo}");

    NoContentRequest::<TeamsAddOrUpdateRepoPermissionsLegacyRequest, ()>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **Remove a repository from a team (Legacy)**
  ///
  /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [Remove a repository from a team](https://docs.github.com/rest/teams/teams#remove-a-repository-from-a-team) endpoint.
  ///
  /// If the authenticated user is an organization owner or a team maintainer, they can remove any repositories from the team. To remove a repository from a team as an organization member, the authenticated user must have admin access to the repository and must be able to see the team. NOTE: This does not delete the repository, it just removes it from the team.
  ///
  /// *Documentation*: [https://docs.github.com/rest/teams/teams#remove-a-repository-from-a-team-legacy](https://docs.github.com/rest/teams/teams#remove-a-repository-from-a-team-legacy)
  pub fn remove_repo_legacy(
    &self,
    team_id: impl Into<i64>,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let team_id = team_id.into();
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/teams/{team_id}/repos/{owner}/{repo}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Get a discussion (Legacy)**
  ///
  /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [Get a discussion](https://docs.github.com/rest/teams/discussions#get-a-discussion) endpoint.
  ///
  /// Get a specific discussion on a team's page.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `read:discussion` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/teams/discussions#get-a-discussion-legacy](https://docs.github.com/rest/teams/discussions#get-a-discussion-legacy)
  pub fn get_discussion_legacy(
    &self,
    team_id: impl Into<i64>,
    discussion_number: impl Into<i64>,
  ) -> Request<(), (), TeamDiscussion> {
    let team_id = team_id.into();
    let discussion_number = discussion_number.into();
    let url = format!("/teams/{team_id}/discussions/{discussion_number}");

    Request::<(), (), TeamDiscussion>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Update a discussion (Legacy)**
  ///
  /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [Update a discussion](https://docs.github.com/rest/teams/discussions#update-a-discussion) endpoint.
  ///
  /// Edits the title and body text of a discussion post. Only the parameters you provide are updated.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `write:discussion` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/teams/discussions#update-a-discussion-legacy](https://docs.github.com/rest/teams/discussions#update-a-discussion-legacy)
  pub fn update_discussion_legacy(
    &self,
    team_id: impl Into<i64>,
    discussion_number: impl Into<i64>,
  ) -> Request<TeamsUpdateDiscussionLegacyRequest, (), TeamDiscussion> {
    let team_id = team_id.into();
    let discussion_number = discussion_number.into();
    let url = format!("/teams/{team_id}/discussions/{discussion_number}");

    Request::<TeamsUpdateDiscussionLegacyRequest, (), TeamDiscussion>::builder(&self.config)
      .patch(url)
      .build()
  }

  /// **Delete a discussion (Legacy)**
  ///
  /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [`Delete a discussion`](https://docs.github.com/rest/teams/discussions#delete-a-discussion) endpoint.
  ///
  /// Delete a discussion from a team's page.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `write:discussion` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/teams/discussions#delete-a-discussion-legacy](https://docs.github.com/rest/teams/discussions#delete-a-discussion-legacy)
  pub fn delete_discussion_legacy(
    &self,
    team_id: impl Into<i64>,
    discussion_number: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let team_id = team_id.into();
    let discussion_number = discussion_number.into();
    let url = format!("/teams/{team_id}/discussions/{discussion_number}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List child teams (Legacy)**
  ///
  /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [`List child teams`](https://docs.github.com/rest/teams/teams#list-child-teams) endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/teams/teams#list-child-teams-legacy](https://docs.github.com/rest/teams/teams#list-child-teams-legacy)
  pub fn list_child_legacy(
    &self,
    team_id: impl Into<i64>,
  ) -> Request<(), TeamsListChildLegacyQuery, TeamArray> {
    let team_id = team_id.into();
    let url = format!("/teams/{team_id}/teams");

    Request::<(), TeamsListChildLegacyQuery, TeamArray>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List teams**
  ///
  /// Lists all teams in an organization that are visible to the authenticated user.
  ///
  /// *Documentation*: [https://docs.github.com/rest/teams/teams#list-teams](https://docs.github.com/rest/teams/teams#list-teams)
  pub fn list(&self, org: impl Into<String>) -> Request<(), TeamsListQuery, TeamArray> {
    let org = org.into();
    let url = format!("/orgs/{org}/teams");

    Request::<(), TeamsListQuery, TeamArray>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create a team**
  ///
  /// To create a team, the authenticated user must be a member or owner of `{org}`. By default, organization members can create teams. Organization owners can limit team creation to organization owners. For more information, see "[Setting team creation permissions](https://docs.github.com/articles/setting-team-creation-permissions-in-your-organization)."
  ///
  /// When you create a new team, you automatically become a team maintainer without explicitly adding yourself to the optional array of `maintainers`. For more information, see "[About teams](https://docs.github.com/github/setting-up-and-managing-organizations-and-teams/about-teams)".
  ///
  /// *Documentation*: [https://docs.github.com/rest/teams/teams#create-a-team](https://docs.github.com/rest/teams/teams#create-a-team)
  pub fn create(&self, org: impl Into<String>) -> Request<TeamsCreateRequest, (), FullTeam> {
    let org = org.into();
    let url = format!("/orgs/{org}/teams");

    Request::<TeamsCreateRequest, (), FullTeam>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Get team membership for a user**
  ///
  /// Team members will include the members of child teams.
  ///
  /// To get a user's membership with a team, the team must be visible to the authenticated user.
  ///
  /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}/memberships/{username}`.
  ///
  /// **Note:**
  /// The response contains the `state` of the membership and the member's `role`.
  ///
  /// The `role` for organization owners is set to `maintainer`. For more information about `maintainer` roles, see [Create a team](https://docs.github.com/rest/teams/teams#create-a-team).
  ///
  /// *Documentation*: [https://docs.github.com/rest/teams/members#get-team-membership-for-a-user](https://docs.github.com/rest/teams/members#get-team-membership-for-a-user)
  pub fn get_membership_for_user_in_org(
    &self,
    org: impl Into<String>,
    team_slug: impl Into<String>,
    username: impl Into<String>,
  ) -> Request<(), (), TeamMembership> {
    let org = org.into();
    let team_slug = team_slug.into();
    let username = username.into();
    let url = format!("/orgs/{org}/teams/{team_slug}/memberships/{username}");

    Request::<(), (), TeamMembership>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Add or update team membership for a user**
  ///
  /// Adds an organization member to a team. An authenticated organization owner or team maintainer can add organization members to a team.
  ///
  /// Team synchronization is available for organizations using GitHub Enterprise Cloud. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
  ///
  /// **Note:** When you have team synchronization set up for a team with your organization's identity provider (IdP), you will see an error if you attempt to use the API for making changes to the team's membership. If you have access to manage group membership in your IdP, you can manage GitHub team membership through your identity provider, which automatically adds and removes team members in an organization. For more information, see "[Synchronizing teams between your identity provider and GitHub](https://docs.github.com/articles/synchronizing-teams-between-your-identity-provider-and-github/)."
  ///
  /// An organization owner can add someone who is not part of the team's organization to a team. When an organization owner adds someone to a team who is not an organization member, this endpoint will send an invitation to the person via email. This newly-created membership will be in the "pending" state until the person accepts the invitation, at which point the membership will transition to the "active" state and the user will be added as a member of the team.
  ///
  /// If the user is already a member of the team, this endpoint will update the role of the team member's role. To update the membership of a team member, the authenticated user must be an organization owner or a team maintainer.
  ///
  /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `PUT /organizations/{org_id}/team/{team_id}/memberships/{username}`.
  ///
  /// *Documentation*: [https://docs.github.com/rest/teams/members#add-or-update-team-membership-for-a-user](https://docs.github.com/rest/teams/members#add-or-update-team-membership-for-a-user)
  pub fn add_or_update_membership_for_user_in_org(
    &self,
    org: impl Into<String>,
    team_slug: impl Into<String>,
    username: impl Into<String>,
  ) -> Request<TeamsAddOrUpdateMembershipForUserInOrgRequest, (), TeamMembership> {
    let org = org.into();
    let team_slug = team_slug.into();
    let username = username.into();
    let url = format!("/orgs/{org}/teams/{team_slug}/memberships/{username}");

    Request::<TeamsAddOrUpdateMembershipForUserInOrgRequest, (), TeamMembership>::builder(
      &self.config,
    )
    .put(url)
    .build()
  }

  /// **Remove team membership for a user**
  ///
  /// To remove a membership between a user and a team, the authenticated user must have 'admin' permissions to the team or be an owner of the organization that the team is associated with. Removing team membership does not delete the user, it just removes their membership from the team.
  ///
  /// Team synchronization is available for organizations using GitHub Enterprise Cloud. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
  ///
  /// **Note:** When you have team synchronization set up for a team with your organization's identity provider (IdP), you will see an error if you attempt to use the API for making changes to the team's membership. If you have access to manage group membership in your IdP, you can manage GitHub team membership through your identity provider, which automatically adds and removes team members in an organization. For more information, see "[Synchronizing teams between your identity provider and GitHub](https://docs.github.com/articles/synchronizing-teams-between-your-identity-provider-and-github/)."
  ///
  /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `DELETE /organizations/{org_id}/team/{team_id}/memberships/{username}`.
  ///
  /// *Documentation*: [https://docs.github.com/rest/teams/members#remove-team-membership-for-a-user](https://docs.github.com/rest/teams/members#remove-team-membership-for-a-user)
  pub fn remove_membership_for_user_in_org(
    &self,
    org: impl Into<String>,
    team_slug: impl Into<String>,
    username: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let team_slug = team_slug.into();
    let username = username.into();
    let url = format!("/orgs/{org}/teams/{team_slug}/memberships/{username}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List team projects (Legacy)**
  ///
  /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [`List team projects`](https://docs.github.com/rest/teams/teams#list-team-projects) endpoint.
  ///
  /// Lists the organization projects for a team.
  ///
  /// *Documentation*: [https://docs.github.com/rest/teams/teams#list-team-projects-legacy](https://docs.github.com/rest/teams/teams#list-team-projects-legacy)
  pub fn list_projects_legacy(
    &self,
    team_id: impl Into<i64>,
  ) -> Request<(), TeamsListProjectsLegacyQuery, TeamProjectArray> {
    let team_id = team_id.into();
    let url = format!("/teams/{team_id}/projects");

    Request::<(), TeamsListProjectsLegacyQuery, TeamProjectArray>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List teams for the authenticated user**
  ///
  /// List all of the teams across all of the organizations to which the authenticated
  /// user belongs.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `user`, `repo`, or `read:org` scope to use this endpoint.
  ///
  /// When using a fine-grained personal access token, the resource owner of the token must be a single organization, and the response will only include the teams from that organization.
  ///
  /// *Documentation*: [https://docs.github.com/rest/teams/teams#list-teams-for-the-authenticated-user](https://docs.github.com/rest/teams/teams#list-teams-for-the-authenticated-user)
  pub fn list_for_authenticated_user(
    &self,
  ) -> Request<(), TeamsListForAuthenticatedUserQuery, FullTeamArray> {
    let url = format!("/user/teams");

    Request::<(), TeamsListForAuthenticatedUserQuery, FullTeamArray>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get a team by name**
  ///
  /// Gets a team using the team's `slug`. To create the `slug`, GitHub replaces special characters in the `name` string, changes all words to lowercase, and replaces spaces with a `-` separator. For example, `"My TEam Näme"` would become `my-team-name`.
  ///
  /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}`.
  ///
  /// *Documentation*: [https://docs.github.com/rest/teams/teams#get-a-team-by-name](https://docs.github.com/rest/teams/teams#get-a-team-by-name)
  pub fn get_by_name(
    &self,
    org: impl Into<String>,
    team_slug: impl Into<String>,
  ) -> Request<(), (), FullTeam> {
    let org = org.into();
    let team_slug = team_slug.into();
    let url = format!("/orgs/{org}/teams/{team_slug}");

    Request::<(), (), FullTeam>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Update a team**
  ///
  /// To edit a team, the authenticated user must either be an organization owner or a team maintainer.
  ///
  /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `PATCH /organizations/{org_id}/team/{team_id}`.
  ///
  /// *Documentation*: [https://docs.github.com/rest/teams/teams#update-a-team](https://docs.github.com/rest/teams/teams#update-a-team)
  pub fn update_in_org(
    &self,
    org: impl Into<String>,
    team_slug: impl Into<String>,
  ) -> Request<TeamsUpdateInOrgRequest, (), FullTeam> {
    let org = org.into();
    let team_slug = team_slug.into();
    let url = format!("/orgs/{org}/teams/{team_slug}");

    Request::<TeamsUpdateInOrgRequest, (), FullTeam>::builder(&self.config)
      .patch(url)
      .build()
  }

  /// **Delete a team**
  ///
  /// To delete a team, the authenticated user must be an organization owner or team maintainer.
  ///
  /// If you are an organization owner, deleting a parent team will delete all of its child teams as well.
  ///
  /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `DELETE /organizations/{org_id}/team/{team_id}`.
  ///
  /// *Documentation*: [https://docs.github.com/rest/teams/teams#delete-a-team](https://docs.github.com/rest/teams/teams#delete-a-team)
  pub fn delete_in_org(
    &self,
    org: impl Into<String>,
    team_slug: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let team_slug = team_slug.into();
    let url = format!("/orgs/{org}/teams/{team_slug}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List discussions**
  ///
  /// List all discussions on a team's page.
  ///
  /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}/discussions`.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `read:discussion` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/teams/discussions#list-discussions](https://docs.github.com/rest/teams/discussions#list-discussions)
  pub fn list_discussions_in_org(
    &self,
    org: impl Into<String>,
    team_slug: impl Into<String>,
  ) -> Request<(), TeamsListDiscussionsInOrgQuery, TeamDiscussionArray> {
    let org = org.into();
    let team_slug = team_slug.into();
    let url = format!("/orgs/{org}/teams/{team_slug}/discussions");

    Request::<(), TeamsListDiscussionsInOrgQuery, TeamDiscussionArray>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create a discussion**
  ///
  /// Creates a new discussion post on a team's page.
  ///
  /// This endpoint triggers [notifications](https://docs.github.com/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. For more information, see "[Rate limits for the API](https://docs.github.com/rest/overview/rate-limits-for-the-rest-api#about-secondary-rate-limits)" and "[Best practices for using the REST API](https://docs.github.com/rest/guides/best-practices-for-using-the-rest-api)."
  ///
  /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `POST /organizations/{org_id}/team/{team_id}/discussions`.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `write:discussion` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/teams/discussions#create-a-discussion](https://docs.github.com/rest/teams/discussions#create-a-discussion)
  pub fn create_discussion_in_org(
    &self,
    org: impl Into<String>,
    team_slug: impl Into<String>,
  ) -> Request<TeamsCreateDiscussionInOrgRequest, (), TeamDiscussion> {
    let org = org.into();
    let team_slug = team_slug.into();
    let url = format!("/orgs/{org}/teams/{team_slug}/discussions");

    Request::<TeamsCreateDiscussionInOrgRequest, (), TeamDiscussion>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Get a discussion comment**
  ///
  /// Get a specific comment on a team discussion.
  ///
  /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}/discussions/{discussion_number}/comments/{comment_number}`.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `read:discussion` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/teams/discussion-comments#get-a-discussion-comment](https://docs.github.com/rest/teams/discussion-comments#get-a-discussion-comment)
  pub fn get_discussion_comment_in_org(
    &self,
    org: impl Into<String>,
    team_slug: impl Into<String>,
    discussion_number: impl Into<i64>,
    comment_number: impl Into<i64>,
  ) -> Request<(), (), TeamDiscussionComment> {
    let org = org.into();
    let team_slug = team_slug.into();
    let discussion_number = discussion_number.into();
    let comment_number = comment_number.into();
    let url = format!(
      "/orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/comments/{comment_number}"
    );

    Request::<(), (), TeamDiscussionComment>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Update a discussion comment**
  ///
  /// Edits the body text of a discussion comment.
  ///
  /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `PATCH /organizations/{org_id}/team/{team_id}/discussions/{discussion_number}/comments/{comment_number}`.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `write:discussion` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/teams/discussion-comments#update-a-discussion-comment](https://docs.github.com/rest/teams/discussion-comments#update-a-discussion-comment)
  pub fn update_discussion_comment_in_org(
    &self,
    org: impl Into<String>,
    team_slug: impl Into<String>,
    discussion_number: impl Into<i64>,
    comment_number: impl Into<i64>,
  ) -> Request<TeamsUpdateDiscussionCommentInOrgRequest, (), TeamDiscussionComment> {
    let org = org.into();
    let team_slug = team_slug.into();
    let discussion_number = discussion_number.into();
    let comment_number = comment_number.into();
    let url = format!(
      "/orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/comments/{comment_number}"
    );

    Request::<TeamsUpdateDiscussionCommentInOrgRequest, (), TeamDiscussionComment>::builder(
      &self.config,
    )
    .patch(url)
    .build()
  }

  /// **Delete a discussion comment**
  ///
  /// Deletes a comment on a team discussion.
  ///
  /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `DELETE /organizations/{org_id}/team/{team_id}/discussions/{discussion_number}/comments/{comment_number}`.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `write:discussion` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/teams/discussion-comments#delete-a-discussion-comment](https://docs.github.com/rest/teams/discussion-comments#delete-a-discussion-comment)
  pub fn delete_discussion_comment_in_org(
    &self,
    org: impl Into<String>,
    team_slug: impl Into<String>,
    discussion_number: impl Into<i64>,
    comment_number: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let team_slug = team_slug.into();
    let discussion_number = discussion_number.into();
    let comment_number = comment_number.into();
    let url = format!(
      "/orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/comments/{comment_number}"
    );

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Check team permissions for a project (Legacy)**
  ///
  /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [Check team permissions for a project](https://docs.github.com/rest/teams/teams#check-team-permissions-for-a-project) endpoint.
  ///
  /// Checks whether a team has `read`, `write`, or `admin` permissions for an organization project. The response includes projects inherited from a parent team.
  ///
  /// *Documentation*: [https://docs.github.com/rest/teams/teams#check-team-permissions-for-a-project-legacy](https://docs.github.com/rest/teams/teams#check-team-permissions-for-a-project-legacy)
  pub fn check_permissions_for_project_legacy(
    &self,
    team_id: impl Into<i64>,
    project_id: impl Into<i64>,
  ) -> Request<(), (), TeamProject> {
    let team_id = team_id.into();
    let project_id = project_id.into();
    let url = format!("/teams/{team_id}/projects/{project_id}");

    Request::<(), (), TeamProject>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Add or update team project permissions (Legacy)**
  ///
  /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [Add or update team project permissions](https://docs.github.com/rest/teams/teams#add-or-update-team-project-permissions) endpoint.
  ///
  /// Adds an organization project to a team. To add a project to a team or update the team's permission on a project, the authenticated user must have `admin` permissions for the project. The project and team must be part of the same organization.
  ///
  /// *Documentation*: [https://docs.github.com/rest/teams/teams#add-or-update-team-project-permissions-legacy](https://docs.github.com/rest/teams/teams#add-or-update-team-project-permissions-legacy)
  pub fn add_or_update_project_permissions_legacy(
    &self,
    team_id: impl Into<i64>,
    project_id: impl Into<i64>,
  ) -> NoContentRequest<TeamsAddOrUpdateProjectPermissionsLegacyRequest, ()> {
    let team_id = team_id.into();
    let project_id = project_id.into();
    let url = format!("/teams/{team_id}/projects/{project_id}");

    NoContentRequest::<TeamsAddOrUpdateProjectPermissionsLegacyRequest, ()>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **Remove a project from a team (Legacy)**
  ///
  /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [Remove a project from a team](https://docs.github.com/rest/teams/teams#remove-a-project-from-a-team) endpoint.
  ///
  /// Removes an organization project from a team. An organization owner or a team maintainer can remove any project from the team. To remove a project from a team as an organization member, the authenticated user must have `read` access to both the team and project, or `admin` access to the team or project. **Note:** This endpoint removes the project from the team, but does not delete it.
  ///
  /// *Documentation*: [https://docs.github.com/rest/teams/teams#remove-a-project-from-a-team-legacy](https://docs.github.com/rest/teams/teams#remove-a-project-from-a-team-legacy)
  pub fn remove_project_legacy(
    &self,
    team_id: impl Into<i64>,
    project_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let team_id = team_id.into();
    let project_id = project_id.into();
    let url = format!("/teams/{team_id}/projects/{project_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Check team permissions for a repository**
  ///
  /// Checks whether a team has `admin`, `push`, `maintain`, `triage`, or `pull` permission for a repository. Repositories inherited through a parent team will also be checked.
  ///
  /// You can also get information about the specified repository, including what permissions the team grants on it, by passing the following custom [media type](https://docs.github.com/rest/overview/media-types/) via the `application/vnd.github.v3.repository+json` accept header.
  ///
  /// If a team doesn't have permission for the repository, you will receive a `404 Not Found` response status.
  ///
  /// If the repository is private, you must have at least `read` permission for that repository, and your token must have the `repo` or `admin:org` scope. Otherwise, you will receive a `404 Not Found` response status.
  ///
  /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}/repos/{owner}/{repo}`.
  ///
  /// *Documentation*: [https://docs.github.com/rest/teams/teams#check-team-permissions-for-a-repository](https://docs.github.com/rest/teams/teams#check-team-permissions-for-a-repository)
  pub fn check_permissions_for_repo_in_org(
    &self,
    org: impl Into<String>,
    team_slug: impl Into<String>,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), (), TeamRepository> {
    let org = org.into();
    let team_slug = team_slug.into();
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/orgs/{org}/teams/{team_slug}/repos/{owner}/{repo}");

    Request::<(), (), TeamRepository>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Add or update team repository permissions**
  ///
  /// To add a repository to a team or update the team's permission on a repository, the authenticated user must have admin access to the repository, and must be able to see the team. The repository must be owned by the organization, or a direct fork of a repository owned by the organization. You will get a `422 Unprocessable Entity` status if you attempt to add a repository to a team that is not owned by the organization. Note that, if you choose not to pass any parameters, you'll need to set `Content-Length` to zero when calling out to this endpoint. For more information, see "[HTTP method](https://docs.github.com/rest/guides/getting-started-with-the-rest-api#http-method)."
  ///
  /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `PUT /organizations/{org_id}/team/{team_id}/repos/{owner}/{repo}`.
  ///
  /// For more information about the permission levels, see "[Repository permission levels for an organization](https://docs.github.com/github/setting-up-and-managing-organizations-and-teams/repository-permission-levels-for-an-organization#permission-levels-for-repositories-owned-by-an-organization)".
  ///
  /// *Documentation*: [https://docs.github.com/rest/teams/teams#add-or-update-team-repository-permissions](https://docs.github.com/rest/teams/teams#add-or-update-team-repository-permissions)
  pub fn add_or_update_repo_permissions_in_org(
    &self,
    org: impl Into<String>,
    team_slug: impl Into<String>,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> NoContentRequest<TeamsAddOrUpdateRepoPermissionsInOrgRequest, ()> {
    let org = org.into();
    let team_slug = team_slug.into();
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/orgs/{org}/teams/{team_slug}/repos/{owner}/{repo}");

    NoContentRequest::<TeamsAddOrUpdateRepoPermissionsInOrgRequest, ()>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **Remove a repository from a team**
  ///
  /// If the authenticated user is an organization owner or a team maintainer, they can remove any repositories from the team. To remove a repository from a team as an organization member, the authenticated user must have admin access to the repository and must be able to see the team. This does not delete the repository, it just removes it from the team.
  ///
  /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `DELETE /organizations/{org_id}/team/{team_id}/repos/{owner}/{repo}`.
  ///
  /// *Documentation*: [https://docs.github.com/rest/teams/teams#remove-a-repository-from-a-team](https://docs.github.com/rest/teams/teams#remove-a-repository-from-a-team)
  pub fn remove_repo_in_org(
    &self,
    org: impl Into<String>,
    team_slug: impl Into<String>,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let team_slug = team_slug.into();
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/orgs/{org}/teams/{team_slug}/repos/{owner}/{repo}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List discussions (Legacy)**
  ///
  /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [`List discussions`](https://docs.github.com/rest/teams/discussions#list-discussions) endpoint.
  ///
  /// List all discussions on a team's page.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `read:discussion` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/teams/discussions#list-discussions-legacy](https://docs.github.com/rest/teams/discussions#list-discussions-legacy)
  pub fn list_discussions_legacy(
    &self,
    team_id: impl Into<i64>,
  ) -> Request<(), TeamsListDiscussionsLegacyQuery, TeamDiscussionArray> {
    let team_id = team_id.into();
    let url = format!("/teams/{team_id}/discussions");

    Request::<(), TeamsListDiscussionsLegacyQuery, TeamDiscussionArray>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create a discussion (Legacy)**
  ///
  /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [`Create a discussion`](https://docs.github.com/rest/teams/discussions#create-a-discussion) endpoint.
  ///
  /// Creates a new discussion post on a team's page.
  ///
  /// This endpoint triggers [notifications](https://docs.github.com/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. For more information, see "[Rate limits for the API](https://docs.github.com/rest/overview/rate-limits-for-the-rest-api#about-secondary-rate-limits)" and "[Best practices for using the REST API](https://docs.github.com/rest/guides/best-practices-for-using-the-rest-api)."
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `write:discussion` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/teams/discussions#create-a-discussion-legacy](https://docs.github.com/rest/teams/discussions#create-a-discussion-legacy)
  pub fn create_discussion_legacy(
    &self,
    team_id: impl Into<i64>,
  ) -> Request<TeamsCreateDiscussionLegacyRequest, (), TeamDiscussion> {
    let team_id = team_id.into();
    let url = format!("/teams/{team_id}/discussions");

    Request::<TeamsCreateDiscussionLegacyRequest, (), TeamDiscussion>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **List child teams**
  ///
  /// Lists the child teams of the team specified by `{team_slug}`.
  ///
  /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}/teams`.
  ///
  /// *Documentation*: [https://docs.github.com/rest/teams/teams#list-child-teams](https://docs.github.com/rest/teams/teams#list-child-teams)
  pub fn list_child_in_org(
    &self,
    org: impl Into<String>,
    team_slug: impl Into<String>,
  ) -> Request<(), TeamsListChildInOrgQuery, TeamArray> {
    let org = org.into();
    let team_slug = team_slug.into();
    let url = format!("/orgs/{org}/teams/{team_slug}/teams");

    Request::<(), TeamsListChildInOrgQuery, TeamArray>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List team members**
  ///
  /// Team members will include the members of child teams.
  ///
  /// To list members in a team, the team must be visible to the authenticated user.
  ///
  /// *Documentation*: [https://docs.github.com/rest/teams/members#list-team-members](https://docs.github.com/rest/teams/members#list-team-members)
  pub fn list_members_in_org(
    &self,
    org: impl Into<String>,
    team_slug: impl Into<String>,
  ) -> Request<(), TeamsListMembersInOrgQuery, SimpleUserArray> {
    let org = org.into();
    let team_slug = team_slug.into();
    let url = format!("/orgs/{org}/teams/{team_slug}/members");

    Request::<(), TeamsListMembersInOrgQuery, SimpleUserArray>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List discussion comments (Legacy)**
  ///
  /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [List discussion comments](https://docs.github.com/rest/teams/discussion-comments#list-discussion-comments) endpoint.
  ///
  /// List all comments on a team discussion.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `read:discussion` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/teams/discussion-comments#list-discussion-comments-legacy](https://docs.github.com/rest/teams/discussion-comments#list-discussion-comments-legacy)
  pub fn list_discussion_comments_legacy(
    &self,
    team_id: impl Into<i64>,
    discussion_number: impl Into<i64>,
  ) -> Request<(), TeamsListDiscussionCommentsLegacyQuery, TeamDiscussionCommentArray> {
    let team_id = team_id.into();
    let discussion_number = discussion_number.into();
    let url = format!("/teams/{team_id}/discussions/{discussion_number}/comments");

    Request::<(), TeamsListDiscussionCommentsLegacyQuery, TeamDiscussionCommentArray>::builder(
      &self.config,
    )
    .get(url)
    .build()
  }

  /// **Create a discussion comment (Legacy)**
  ///
  /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [Create a discussion comment](https://docs.github.com/rest/teams/discussion-comments#create-a-discussion-comment) endpoint.
  ///
  /// Creates a new comment on a team discussion.
  ///
  /// This endpoint triggers [notifications](https://docs.github.com/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. For more information, see "[Rate limits for the API](https://docs.github.com/rest/overview/rate-limits-for-the-rest-api#about-secondary-rate-limits)" and "[Best practices for using the REST API](https://docs.github.com/rest/guides/best-practices-for-using-the-rest-api)."
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `write:discussion` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/teams/discussion-comments#create-a-discussion-comment-legacy](https://docs.github.com/rest/teams/discussion-comments#create-a-discussion-comment-legacy)
  pub fn create_discussion_comment_legacy(
    &self,
    team_id: impl Into<i64>,
    discussion_number: impl Into<i64>,
  ) -> Request<TeamsCreateDiscussionCommentLegacyRequest, (), TeamDiscussionComment> {
    let team_id = team_id.into();
    let discussion_number = discussion_number.into();
    let url = format!("/teams/{team_id}/discussions/{discussion_number}/comments");

    Request::<TeamsCreateDiscussionCommentLegacyRequest, (), TeamDiscussionComment>::builder(
      &self.config,
    )
    .post(url)
    .build()
  }

  /// **Get a discussion comment (Legacy)**
  ///
  /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [Get a discussion comment](https://docs.github.com/rest/teams/discussion-comments#get-a-discussion-comment) endpoint.
  ///
  /// Get a specific comment on a team discussion.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `read:discussion` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/teams/discussion-comments#get-a-discussion-comment-legacy](https://docs.github.com/rest/teams/discussion-comments#get-a-discussion-comment-legacy)
  pub fn get_discussion_comment_legacy(
    &self,
    team_id: impl Into<i64>,
    discussion_number: impl Into<i64>,
    comment_number: impl Into<i64>,
  ) -> Request<(), (), TeamDiscussionComment> {
    let team_id = team_id.into();
    let discussion_number = discussion_number.into();
    let comment_number = comment_number.into();
    let url = format!("/teams/{team_id}/discussions/{discussion_number}/comments/{comment_number}");

    Request::<(), (), TeamDiscussionComment>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Update a discussion comment (Legacy)**
  ///
  /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [Update a discussion comment](https://docs.github.com/rest/teams/discussion-comments#update-a-discussion-comment) endpoint.
  ///
  /// Edits the body text of a discussion comment.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `write:discussion` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/teams/discussion-comments#update-a-discussion-comment-legacy](https://docs.github.com/rest/teams/discussion-comments#update-a-discussion-comment-legacy)
  pub fn update_discussion_comment_legacy(
    &self,
    team_id: impl Into<i64>,
    discussion_number: impl Into<i64>,
    comment_number: impl Into<i64>,
  ) -> Request<TeamsUpdateDiscussionCommentLegacyRequest, (), TeamDiscussionComment> {
    let team_id = team_id.into();
    let discussion_number = discussion_number.into();
    let comment_number = comment_number.into();
    let url = format!("/teams/{team_id}/discussions/{discussion_number}/comments/{comment_number}");

    Request::<TeamsUpdateDiscussionCommentLegacyRequest, (), TeamDiscussionComment>::builder(
      &self.config,
    )
    .patch(url)
    .build()
  }

  /// **Delete a discussion comment (Legacy)**
  ///
  /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [Delete a discussion comment](https://docs.github.com/rest/teams/discussion-comments#delete-a-discussion-comment) endpoint.
  ///
  /// Deletes a comment on a team discussion.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `write:discussion` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/teams/discussion-comments#delete-a-discussion-comment-legacy](https://docs.github.com/rest/teams/discussion-comments#delete-a-discussion-comment-legacy)
  pub fn delete_discussion_comment_legacy(
    &self,
    team_id: impl Into<i64>,
    discussion_number: impl Into<i64>,
    comment_number: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let team_id = team_id.into();
    let discussion_number = discussion_number.into();
    let comment_number = comment_number.into();
    let url = format!("/teams/{team_id}/discussions/{discussion_number}/comments/{comment_number}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List team projects**
  ///
  /// Lists the organization projects for a team.
  ///
  /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}/projects`.
  ///
  /// *Documentation*: [https://docs.github.com/rest/teams/teams#list-team-projects](https://docs.github.com/rest/teams/teams#list-team-projects)
  pub fn list_projects_in_org(
    &self,
    org: impl Into<String>,
    team_slug: impl Into<String>,
  ) -> Request<(), TeamsListProjectsInOrgQuery, TeamProjectArray> {
    let org = org.into();
    let team_slug = team_slug.into();
    let url = format!("/orgs/{org}/teams/{team_slug}/projects");

    Request::<(), TeamsListProjectsInOrgQuery, TeamProjectArray>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List team repositories (Legacy)**
  ///
  /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [List team repositories](https://docs.github.com/rest/teams/teams#list-team-repositories) endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/teams/teams#list-team-repositories-legacy](https://docs.github.com/rest/teams/teams#list-team-repositories-legacy)
  pub fn list_repos_legacy(
    &self,
    team_id: impl Into<i64>,
  ) -> Request<(), TeamsListReposLegacyQuery, MinimalRepositoryArray> {
    let team_id = team_id.into();
    let url = format!("/teams/{team_id}/repos");

    Request::<(), TeamsListReposLegacyQuery, MinimalRepositoryArray>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get team membership for a user (Legacy)**
  ///
  /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [Get team membership for a user](https://docs.github.com/rest/teams/members#get-team-membership-for-a-user) endpoint.
  ///
  /// Team members will include the members of child teams.
  ///
  /// To get a user's membership with a team, the team must be visible to the authenticated user.
  ///
  /// **Note:**
  /// The response contains the `state` of the membership and the member's `role`.
  ///
  /// The `role` for organization owners is set to `maintainer`. For more information about `maintainer` roles, see [Create a team](https://docs.github.com/rest/teams/teams#create-a-team).
  ///
  /// *Documentation*: [https://docs.github.com/rest/teams/members#get-team-membership-for-a-user-legacy](https://docs.github.com/rest/teams/members#get-team-membership-for-a-user-legacy)
  pub fn get_membership_for_user_legacy(
    &self,
    team_id: impl Into<i64>,
    username: impl Into<String>,
  ) -> Request<(), (), TeamMembership> {
    let team_id = team_id.into();
    let username = username.into();
    let url = format!("/teams/{team_id}/memberships/{username}");

    Request::<(), (), TeamMembership>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Add or update team membership for a user (Legacy)**
  ///
  /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [Add or update team membership for a user](https://docs.github.com/rest/teams/members#add-or-update-team-membership-for-a-user) endpoint.
  ///
  /// Team synchronization is available for organizations using GitHub Enterprise Cloud. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
  ///
  /// If the user is already a member of the team's organization, this endpoint will add the user to the team. To add a membership between an organization member and a team, the authenticated user must be an organization owner or a team maintainer.
  ///
  /// **Note:** When you have team synchronization set up for a team with your organization's identity provider (IdP), you will see an error if you attempt to use the API for making changes to the team's membership. If you have access to manage group membership in your IdP, you can manage GitHub team membership through your identity provider, which automatically adds and removes team members in an organization. For more information, see "[Synchronizing teams between your identity provider and GitHub](https://docs.github.com/articles/synchronizing-teams-between-your-identity-provider-and-github/)."
  ///
  /// If the user is unaffiliated with the team's organization, this endpoint will send an invitation to the user via email. This newly-created membership will be in the "pending" state until the user accepts the invitation, at which point the membership will transition to the "active" state and the user will be added as a member of the team. To add a membership between an unaffiliated user and a team, the authenticated user must be an organization owner.
  ///
  /// If the user is already a member of the team, this endpoint will update the role of the team member's role. To update the membership of a team member, the authenticated user must be an organization owner or a team maintainer.
  ///
  /// *Documentation*: [https://docs.github.com/rest/teams/members#add-or-update-team-membership-for-a-user-legacy](https://docs.github.com/rest/teams/members#add-or-update-team-membership-for-a-user-legacy)
  pub fn add_or_update_membership_for_user_legacy(
    &self,
    team_id: impl Into<i64>,
    username: impl Into<String>,
  ) -> Request<TeamsAddOrUpdateMembershipForUserLegacyRequest, (), TeamMembership> {
    let team_id = team_id.into();
    let username = username.into();
    let url = format!("/teams/{team_id}/memberships/{username}");

    Request::<TeamsAddOrUpdateMembershipForUserLegacyRequest, (), TeamMembership>::builder(
      &self.config,
    )
    .put(url)
    .build()
  }

  /// **Remove team membership for a user (Legacy)**
  ///
  /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [Remove team membership for a user](https://docs.github.com/rest/teams/members#remove-team-membership-for-a-user) endpoint.
  ///
  /// Team synchronization is available for organizations using GitHub Enterprise Cloud. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
  ///
  /// To remove a membership between a user and a team, the authenticated user must have 'admin' permissions to the team or be an owner of the organization that the team is associated with. Removing team membership does not delete the user, it just removes their membership from the team.
  ///
  /// **Note:** When you have team synchronization set up for a team with your organization's identity provider (IdP), you will see an error if you attempt to use the API for making changes to the team's membership. If you have access to manage group membership in your IdP, you can manage GitHub team membership through your identity provider, which automatically adds and removes team members in an organization. For more information, see "[Synchronizing teams between your identity provider and GitHub](https://docs.github.com/articles/synchronizing-teams-between-your-identity-provider-and-github/)."
  ///
  /// *Documentation*: [https://docs.github.com/rest/teams/members#remove-team-membership-for-a-user-legacy](https://docs.github.com/rest/teams/members#remove-team-membership-for-a-user-legacy)
  pub fn remove_membership_for_user_legacy(
    &self,
    team_id: impl Into<i64>,
    username: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let team_id = team_id.into();
    let username = username.into();
    let url = format!("/teams/{team_id}/memberships/{username}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List discussion comments**
  ///
  /// List all comments on a team discussion.
  ///
  /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}/discussions/{discussion_number}/comments`.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `read:discussion` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/teams/discussion-comments#list-discussion-comments](https://docs.github.com/rest/teams/discussion-comments#list-discussion-comments)
  pub fn list_discussion_comments_in_org(
    &self,
    org: impl Into<String>,
    team_slug: impl Into<String>,
    discussion_number: impl Into<i64>,
  ) -> Request<(), TeamsListDiscussionCommentsInOrgQuery, TeamDiscussionCommentArray> {
    let org = org.into();
    let team_slug = team_slug.into();
    let discussion_number = discussion_number.into();
    let url = format!("/orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/comments");

    Request::<(), TeamsListDiscussionCommentsInOrgQuery, TeamDiscussionCommentArray>::builder(
      &self.config,
    )
    .get(url)
    .build()
  }

  /// **Create a discussion comment**
  ///
  /// Creates a new comment on a team discussion.
  ///
  /// This endpoint triggers [notifications](https://docs.github.com/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. For more information, see "[Rate limits for the API](https://docs.github.com/rest/overview/rate-limits-for-the-rest-api#about-secondary-rate-limits)" and "[Best practices for using the REST API](https://docs.github.com/rest/guides/best-practices-for-using-the-rest-api)."
  ///
  /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `POST /organizations/{org_id}/team/{team_id}/discussions/{discussion_number}/comments`.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `write:discussion` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/teams/discussion-comments#create-a-discussion-comment](https://docs.github.com/rest/teams/discussion-comments#create-a-discussion-comment)
  pub fn create_discussion_comment_in_org(
    &self,
    org: impl Into<String>,
    team_slug: impl Into<String>,
    discussion_number: impl Into<i64>,
  ) -> Request<TeamsCreateDiscussionCommentInOrgRequest, (), TeamDiscussionComment> {
    let org = org.into();
    let team_slug = team_slug.into();
    let discussion_number = discussion_number.into();
    let url = format!("/orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/comments");

    Request::<TeamsCreateDiscussionCommentInOrgRequest, (), TeamDiscussionComment>::builder(
      &self.config,
    )
    .post(url)
    .build()
  }

  /// **List pending team invitations (Legacy)**
  ///
  /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [`List pending team invitations`](https://docs.github.com/rest/teams/members#list-pending-team-invitations) endpoint.
  ///
  /// The return hash contains a `role` field which refers to the Organization Invitation role and will be one of the following values: `direct_member`, `admin`, `billing_manager`, `hiring_manager`, or `reinstate`. If the invitee is not a GitHub member, the `login` field in the return hash will be `null`.
  ///
  /// *Documentation*: [https://docs.github.com/rest/teams/members#list-pending-team-invitations-legacy](https://docs.github.com/rest/teams/members#list-pending-team-invitations-legacy)
  pub fn list_pending_invitations_legacy(
    &self,
    team_id: impl Into<i64>,
  ) -> Request<(), TeamsListPendingInvitationsLegacyQuery, OrganizationInvitationArray> {
    let team_id = team_id.into();
    let url = format!("/teams/{team_id}/invitations");

    Request::<(), TeamsListPendingInvitationsLegacyQuery, OrganizationInvitationArray>::builder(
      &self.config,
    )
    .get(url)
    .build()
  }

  /// **Check team permissions for a project**
  ///
  /// Checks whether a team has `read`, `write`, or `admin` permissions for an organization project. The response includes projects inherited from a parent team.
  ///
  /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}/projects/{project_id}`.
  ///
  /// *Documentation*: [https://docs.github.com/rest/teams/teams#check-team-permissions-for-a-project](https://docs.github.com/rest/teams/teams#check-team-permissions-for-a-project)
  pub fn check_permissions_for_project_in_org(
    &self,
    org: impl Into<String>,
    team_slug: impl Into<String>,
    project_id: impl Into<i64>,
  ) -> Request<(), (), TeamProject> {
    let org = org.into();
    let team_slug = team_slug.into();
    let project_id = project_id.into();
    let url = format!("/orgs/{org}/teams/{team_slug}/projects/{project_id}");

    Request::<(), (), TeamProject>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Add or update team project permissions**
  ///
  /// Adds an organization project to a team. To add a project to a team or update the team's permission on a project, the authenticated user must have `admin` permissions for the project. The project and team must be part of the same organization.
  ///
  /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `PUT /organizations/{org_id}/team/{team_id}/projects/{project_id}`.
  ///
  /// *Documentation*: [https://docs.github.com/rest/teams/teams#add-or-update-team-project-permissions](https://docs.github.com/rest/teams/teams#add-or-update-team-project-permissions)
  pub fn add_or_update_project_permissions_in_org(
    &self,
    org: impl Into<String>,
    team_slug: impl Into<String>,
    project_id: impl Into<i64>,
  ) -> NoContentRequest<TeamsAddOrUpdateProjectPermissionsInOrgRequest, ()> {
    let org = org.into();
    let team_slug = team_slug.into();
    let project_id = project_id.into();
    let url = format!("/orgs/{org}/teams/{team_slug}/projects/{project_id}");

    NoContentRequest::<TeamsAddOrUpdateProjectPermissionsInOrgRequest, ()>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **Remove a project from a team**
  ///
  /// Removes an organization project from a team. An organization owner or a team maintainer can remove any project from the team. To remove a project from a team as an organization member, the authenticated user must have `read` access to both the team and project, or `admin` access to the team or project. This endpoint removes the project from the team, but does not delete the project.
  ///
  /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `DELETE /organizations/{org_id}/team/{team_id}/projects/{project_id}`.
  ///
  /// *Documentation*: [https://docs.github.com/rest/teams/teams#remove-a-project-from-a-team](https://docs.github.com/rest/teams/teams#remove-a-project-from-a-team)
  pub fn remove_project_in_org(
    &self,
    org: impl Into<String>,
    team_slug: impl Into<String>,
    project_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let team_slug = team_slug.into();
    let project_id = project_id.into();
    let url = format!("/orgs/{org}/teams/{team_slug}/projects/{project_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List team members (Legacy)**
  ///
  /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [`List team members`](https://docs.github.com/rest/teams/members#list-team-members) endpoint.
  ///
  /// Team members will include the members of child teams.
  ///
  /// *Documentation*: [https://docs.github.com/rest/teams/members#list-team-members-legacy](https://docs.github.com/rest/teams/members#list-team-members-legacy)
  pub fn list_members_legacy(
    &self,
    team_id: impl Into<i64>,
  ) -> Request<(), TeamsListMembersLegacyQuery, SimpleUserArray> {
    let team_id = team_id.into();
    let url = format!("/teams/{team_id}/members");

    Request::<(), TeamsListMembersLegacyQuery, SimpleUserArray>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List pending team invitations**
  ///
  /// The return hash contains a `role` field which refers to the Organization Invitation role and will be one of the following values: `direct_member`, `admin`, `billing_manager`, `hiring_manager`, or `reinstate`. If the invitee is not a GitHub member, the `login` field in the return hash will be `null`.
  ///
  /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}/invitations`.
  ///
  /// *Documentation*: [https://docs.github.com/rest/teams/members#list-pending-team-invitations](https://docs.github.com/rest/teams/members#list-pending-team-invitations)
  pub fn list_pending_invitations_in_org(
    &self,
    org: impl Into<String>,
    team_slug: impl Into<String>,
  ) -> Request<(), TeamsListPendingInvitationsInOrgQuery, OrganizationInvitationArray> {
    let org = org.into();
    let team_slug = team_slug.into();
    let url = format!("/orgs/{org}/teams/{team_slug}/invitations");

    Request::<(), TeamsListPendingInvitationsInOrgQuery, OrganizationInvitationArray>::builder(
      &self.config,
    )
    .get(url)
    .build()
  }

  /// **Get a discussion**
  ///
  /// Get a specific discussion on a team's page.
  ///
  /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}/discussions/{discussion_number}`.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `read:discussion` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/teams/discussions#get-a-discussion](https://docs.github.com/rest/teams/discussions#get-a-discussion)
  pub fn get_discussion_in_org(
    &self,
    org: impl Into<String>,
    team_slug: impl Into<String>,
    discussion_number: impl Into<i64>,
  ) -> Request<(), (), TeamDiscussion> {
    let org = org.into();
    let team_slug = team_slug.into();
    let discussion_number = discussion_number.into();
    let url = format!("/orgs/{org}/teams/{team_slug}/discussions/{discussion_number}");

    Request::<(), (), TeamDiscussion>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Update a discussion**
  ///
  /// Edits the title and body text of a discussion post. Only the parameters you provide are updated.
  ///
  /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `PATCH /organizations/{org_id}/team/{team_id}/discussions/{discussion_number}`.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `write:discussion` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/teams/discussions#update-a-discussion](https://docs.github.com/rest/teams/discussions#update-a-discussion)
  pub fn update_discussion_in_org(
    &self,
    org: impl Into<String>,
    team_slug: impl Into<String>,
    discussion_number: impl Into<i64>,
  ) -> Request<TeamsUpdateDiscussionInOrgRequest, (), TeamDiscussion> {
    let org = org.into();
    let team_slug = team_slug.into();
    let discussion_number = discussion_number.into();
    let url = format!("/orgs/{org}/teams/{team_slug}/discussions/{discussion_number}");

    Request::<TeamsUpdateDiscussionInOrgRequest, (), TeamDiscussion>::builder(&self.config)
      .patch(url)
      .build()
  }

  /// **Delete a discussion**
  ///
  /// Delete a discussion from a team's page.
  ///
  /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `DELETE /organizations/{org_id}/team/{team_id}/discussions/{discussion_number}`.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `write:discussion` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/teams/discussions#delete-a-discussion](https://docs.github.com/rest/teams/discussions#delete-a-discussion)
  pub fn delete_discussion_in_org(
    &self,
    org: impl Into<String>,
    team_slug: impl Into<String>,
    discussion_number: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let team_slug = team_slug.into();
    let discussion_number = discussion_number.into();
    let url = format!("/orgs/{org}/teams/{team_slug}/discussions/{discussion_number}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Get a team (Legacy)**
  ///
  /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the [Get a team by name](https://docs.github.com/rest/teams/teams#get-a-team-by-name) endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/teams/teams#get-a-team-legacy](https://docs.github.com/rest/teams/teams#get-a-team-legacy)
  pub fn get_legacy(&self, team_id: impl Into<i64>) -> Request<(), (), FullTeam> {
    let team_id = team_id.into();
    let url = format!("/teams/{team_id}");

    Request::<(), (), FullTeam>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Update a team (Legacy)**
  ///
  /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [Update a team](https://docs.github.com/rest/teams/teams#update-a-team) endpoint.
  ///
  /// To edit a team, the authenticated user must either be an organization owner or a team maintainer.
  ///
  /// **Note:** With nested teams, the `privacy` for parent teams cannot be `secret`.
  ///
  /// *Documentation*: [https://docs.github.com/rest/teams/teams#update-a-team-legacy](https://docs.github.com/rest/teams/teams#update-a-team-legacy)
  pub fn update_legacy(
    &self,
    team_id: impl Into<i64>,
  ) -> Request<TeamsUpdateLegacyRequest, (), FullTeam> {
    let team_id = team_id.into();
    let url = format!("/teams/{team_id}");

    Request::<TeamsUpdateLegacyRequest, (), FullTeam>::builder(&self.config)
      .patch(url)
      .build()
  }

  /// **Delete a team (Legacy)**
  ///
  /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [Delete a team](https://docs.github.com/rest/teams/teams#delete-a-team) endpoint.
  ///
  /// To delete a team, the authenticated user must be an organization owner or team maintainer.
  ///
  /// If you are an organization owner, deleting a parent team will delete all of its child teams as well.
  ///
  /// *Documentation*: [https://docs.github.com/rest/teams/teams#delete-a-team-legacy](https://docs.github.com/rest/teams/teams#delete-a-team-legacy)
  pub fn delete_legacy(&self, team_id: impl Into<i64>) -> NoContentRequest<(), ()> {
    let team_id = team_id.into();
    let url = format!("/teams/{team_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List team repositories**
  ///
  /// Lists a team's repositories visible to the authenticated user.
  ///
  /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}/repos`.
  ///
  /// *Documentation*: [https://docs.github.com/rest/teams/teams#list-team-repositories](https://docs.github.com/rest/teams/teams#list-team-repositories)
  pub fn list_repos_in_org(
    &self,
    org: impl Into<String>,
    team_slug: impl Into<String>,
  ) -> Request<(), TeamsListReposInOrgQuery, MinimalRepositoryArray> {
    let org = org.into();
    let team_slug = team_slug.into();
    let url = format!("/orgs/{org}/teams/{team_slug}/repos");

    Request::<(), TeamsListReposInOrgQuery, MinimalRepositoryArray>::builder(&self.config)
      .get(url)
      .build()
  }
}
