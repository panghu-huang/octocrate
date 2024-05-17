use octocrate_core::*;
#[allow(unused_imports)]
use octocrate_types::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use typed_builder::TypedBuilder;

pub mod get_copilot_organization_details {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = CopilotOrganizationDetails;
}

pub mod list_copilot_seats {
  #[allow(unused_imports)]
  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub seats: Option<Vec<CopilotSeatDetails>>,
    /// Total number of Copilot seats for the organization currently being billed.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub total_seats: Option<i64>,
  }
}

pub mod add_copilot_seats_for_teams {
  #[allow(unused_imports)]
  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// List of team names within the organization to which to grant access to GitHub Copilot.
    pub selected_teams: Vec<String>,
  }

  /// The total number of seat assignments created.
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Response {
    pub seats_created: i64,
  }
}

pub mod cancel_copilot_seat_assignment_for_teams {
  #[allow(unused_imports)]
  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The names of teams from which to revoke access to GitHub Copilot.
    pub selected_teams: Vec<String>,
  }

  /// The total number of seat assignments cancelled.
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Response {
    pub seats_cancelled: i64,
  }
}

pub mod add_copilot_seats_for_users {
  #[allow(unused_imports)]
  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The usernames of the organization members to be granted access to GitHub Copilot.
    pub selected_usernames: Vec<String>,
  }

  /// The total number of seat assignments created.
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Response {
    pub seats_created: i64,
  }
}

pub mod cancel_copilot_seat_assignment_for_users {
  #[allow(unused_imports)]
  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The usernames of the organization members for which to revoke access to GitHub Copilot.
    pub selected_usernames: Vec<String>,
  }

  /// The total number of seat assignments cancelled.
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Response {
    pub seats_cancelled: i64,
  }
}

pub mod get_copilot_seat_details_for_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = CopilotSeatDetails;
}

/// Endpoints to manage Copilot using the REST API.
pub struct GitHubCopilotAPI {
  config: SharedAPIConfig,
}

impl GitHubCopilotAPI {
  pub fn new(config: &SharedAPIConfig) -> Self {
    Self {
      config: config.clone(),
    }
  }

  /// **Get Copilot seat information and settings for an organization**
  ///
  /// **Note**: This endpoint is in beta and is subject to change.
  ///
  /// Gets information about an organization's Copilot subscription, including seat breakdown
  /// and code matching policies. To configure these settings, go to your organization's settings on GitHub.com.
  /// For more information, see "[Managing policies for Copilot in your organization](https://docs.github.com/copilot/managing-copilot/managing-policies-for-copilot-business-in-your-organization)".
  ///
  /// Only organization owners can configure and view details about the organization's Copilot Business subscription.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `manage_billing:copilot` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/copilot/copilot-user-management#get-copilot-seat-information-and-settings-for-an-organization](https://docs.github.com/rest/copilot/copilot-user-management#get-copilot-seat-information-and-settings-for-an-organization)
  pub fn get_copilot_organization_details(
    &self,
    org: impl Into<String>,
  ) -> Request<(), (), get_copilot_organization_details::Response> {
    let org = org.into();
    let url = format!("/orgs/{org}/copilot/billing");

    Request::<(), (), get_copilot_organization_details::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List all Copilot seat assignments for an organization**
  ///
  /// **Note**: This endpoint is in beta and is subject to change.
  ///
  /// Lists all Copilot seat assignments for an organization that are currently being billed (either active or pending cancellation at the start of the next billing cycle).
  ///
  /// Only organization owners can configure and view details about the organization's Copilot Business or Enterprise subscription.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `manage_billing:copilot` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/copilot/copilot-user-management#list-all-copilot-seat-assignments-for-an-organization](https://docs.github.com/rest/copilot/copilot-user-management#list-all-copilot-seat-assignments-for-an-organization)
  pub fn list_copilot_seats(
    &self,
    org: impl Into<String>,
  ) -> Request<(), list_copilot_seats::Query, list_copilot_seats::Response> {
    let org = org.into();
    let url = format!("/orgs/{org}/copilot/billing/seats");

    Request::<(), list_copilot_seats::Query, list_copilot_seats::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Add teams to the Copilot subscription for an organization**
  ///
  /// **Note**: This endpoint is in beta and is subject to change.
  ///
  /// Purchases a GitHub Copilot seat for all users within each specified team.
  /// The organization will be billed accordingly. For more information about Copilot pricing, see "[Pricing for GitHub Copilot](https://docs.github.com/billing/managing-billing-for-github-copilot/about-billing-for-github-copilot#about-billing-for-github-copilot)".
  ///
  /// Only organization owners can configure GitHub Copilot in their organization.
  ///
  /// In order for an admin to use this endpoint, the organization must have a Copilot Business or Enterprise subscription and a configured suggestion matching policy.
  /// For more information about setting up a Copilot subscription, see "[Setting up a Copilot subscription for your organization](https://docs.github.com/billing/managing-billing-for-github-copilot/managing-your-github-copilot-subscription-for-your-organization-or-enterprise)".
  /// For more information about setting a suggestion matching policy, see "[Configuring suggestion matching policies for GitHub Copilot in your organization](https://docs.github.com/copilot/managing-copilot/managing-policies-for-github-copilot-in-your-organization#configuring-suggestion-matching-policies-for-github-copilot-in-your-organization)".
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `manage_billing:copilot` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/copilot/copilot-user-management#add-teams-to-the-copilot-subscription-for-an-organization](https://docs.github.com/rest/copilot/copilot-user-management#add-teams-to-the-copilot-subscription-for-an-organization)
  pub fn add_copilot_seats_for_teams(
    &self,
    org: impl Into<String>,
  ) -> Request<add_copilot_seats_for_teams::Request, (), add_copilot_seats_for_teams::Response> {
    let org = org.into();
    let url = format!("/orgs/{org}/copilot/billing/selected_teams");

    Request::<add_copilot_seats_for_teams::Request, (), add_copilot_seats_for_teams::Response>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Remove teams from the Copilot subscription for an organization**
  ///
  /// **Note**: This endpoint is in beta and is subject to change.
  ///
  /// Cancels the Copilot seat assignment for all members of each team specified.
  /// This will cause the members of the specified team(s) to lose access to GitHub Copilot at the end of the current billing cycle, and the organization will not be billed further for those users.
  ///
  /// For more information about Copilot pricing, see "[Pricing for GitHub Copilot](https://docs.github.com/billing/managing-billing-for-github-copilot/about-billing-for-github-copilot#about-billing-for-github-copilot)".
  ///
  /// For more information about disabling access to Copilot Business or Enterprise, see "[Revoking access to GitHub Copilot for specific users in your organization](https://docs.github.com/copilot/managing-copilot/managing-access-for-copilot-in-your-organization#revoking-access-to-github-copilot-for-specific-users-in-your-organization)".
  ///
  /// Only organization owners can configure GitHub Copilot in their organization.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `manage_billing:copilot` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/copilot/copilot-user-management#remove-teams-from-the-copilot-subscription-for-an-organization](https://docs.github.com/rest/copilot/copilot-user-management#remove-teams-from-the-copilot-subscription-for-an-organization)
  pub fn cancel_copilot_seat_assignment_for_teams(
    &self,
    org: impl Into<String>,
  ) -> Request<
    cancel_copilot_seat_assignment_for_teams::Request,
    (),
    cancel_copilot_seat_assignment_for_teams::Response,
  > {
    let org = org.into();
    let url = format!("/orgs/{org}/copilot/billing/selected_teams");

    Request::<
      cancel_copilot_seat_assignment_for_teams::Request,
      (),
      cancel_copilot_seat_assignment_for_teams::Response,
    >::builder(&self.config)
    .delete(url)
    .build()
  }

  /// **Add users to the Copilot subscription for an organization**
  ///
  /// **Note**: This endpoint is in beta and is subject to change.
  ///
  /// Purchases a GitHub Copilot seat for each user specified.
  /// The organization will be billed accordingly. For more information about Copilot pricing, see "[Pricing for GitHub Copilot](https://docs.github.com/billing/managing-billing-for-github-copilot/about-billing-for-github-copilot#about-billing-for-github-copilot)".
  ///
  /// Only organization owners can configure GitHub Copilot in their organization.
  ///
  /// In order for an admin to use this endpoint, the organization must have a Copilot Business or Enterprise subscription and a configured suggestion matching policy.
  /// For more information about setting up a Copilot subscription, see "[Setting up a Copilot subscription for your organization](https://docs.github.com/billing/managing-billing-for-github-copilot/managing-your-github-copilot-subscription-for-your-organization-or-enterprise)".
  /// For more information about setting a suggestion matching policy, see "[Configuring suggestion matching policies for GitHub Copilot in your organization](https://docs.github.com/copilot/managing-copilot/managing-policies-for-github-copilot-in-your-organization#configuring-suggestion-matching-policies-for-github-copilot-in-your-organization)".
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `manage_billing:copilot` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/copilot/copilot-user-management#add-users-to-the-copilot-subscription-for-an-organization](https://docs.github.com/rest/copilot/copilot-user-management#add-users-to-the-copilot-subscription-for-an-organization)
  pub fn add_copilot_seats_for_users(
    &self,
    org: impl Into<String>,
  ) -> Request<add_copilot_seats_for_users::Request, (), add_copilot_seats_for_users::Response> {
    let org = org.into();
    let url = format!("/orgs/{org}/copilot/billing/selected_users");

    Request::<add_copilot_seats_for_users::Request, (), add_copilot_seats_for_users::Response>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Remove users from the Copilot subscription for an organization**
  ///
  /// **Note**: This endpoint is in beta and is subject to change.
  ///
  /// Cancels the Copilot seat assignment for each user specified.
  /// This will cause the specified users to lose access to GitHub Copilot at the end of the current billing cycle, and the organization will not be billed further for those users.
  ///
  /// For more information about Copilot pricing, see "[Pricing for GitHub Copilot](https://docs.github.com/billing/managing-billing-for-github-copilot/about-billing-for-github-copilot#about-billing-for-github-copilot)".
  ///
  /// For more information about disabling access to Copilot Business or Enterprise, see "[Revoking access to GitHub Copilot for specific users in your organization](https://docs.github.com/copilot/managing-copilot/managing-access-for-copilot-in-your-organization#revoking-access-to-github-copilot-for-specific-users-in-your-organization)".
  ///
  /// Only organization owners can configure GitHub Copilot in their organization.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `manage_billing:copilot` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/copilot/copilot-user-management#remove-users-from-the-copilot-subscription-for-an-organization](https://docs.github.com/rest/copilot/copilot-user-management#remove-users-from-the-copilot-subscription-for-an-organization)
  pub fn cancel_copilot_seat_assignment_for_users(
    &self,
    org: impl Into<String>,
  ) -> Request<
    cancel_copilot_seat_assignment_for_users::Request,
    (),
    cancel_copilot_seat_assignment_for_users::Response,
  > {
    let org = org.into();
    let url = format!("/orgs/{org}/copilot/billing/selected_users");

    Request::<
      cancel_copilot_seat_assignment_for_users::Request,
      (),
      cancel_copilot_seat_assignment_for_users::Response,
    >::builder(&self.config)
    .delete(url)
    .build()
  }

  /// **Get Copilot seat assignment details for a user**
  ///
  /// **Note**: This endpoint is in beta and is subject to change.
  ///
  /// Gets the GitHub Copilot seat assignment details for a member of an organization who currently has access to GitHub Copilot.
  ///
  /// Organization owners can view GitHub Copilot seat assignment details for members in their organization.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `manage_billing:copilot` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/copilot/copilot-user-management#get-copilot-seat-assignment-details-for-a-user](https://docs.github.com/rest/copilot/copilot-user-management#get-copilot-seat-assignment-details-for-a-user)
  pub fn get_copilot_seat_details_for_user(
    &self,
    org: impl Into<String>,
    username: impl Into<String>,
  ) -> Request<(), (), get_copilot_seat_details_for_user::Response> {
    let org = org.into();
    let username = username.into();
    let url = format!("/orgs/{org}/members/{username}/copilot");

    Request::<(), (), get_copilot_seat_details_for_user::Response>::builder(&self.config)
      .get(url)
      .build()
  }
}
