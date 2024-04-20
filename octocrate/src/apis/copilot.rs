use octocrate_core::*;
#[allow(unused_imports)]
use crate::types::*;

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
  ) -> Request<(), (), CopilotBusinessOrganizationDetails> {
    let org = org.into();
    let url = format!("/orgs/{org}/copilot/billing");

    Request::<(), (), CopilotBusinessOrganizationDetails>::builder(&self.config)
      .get(url)
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
  ) -> Request<(), (), CopilotBusinessSeatDetail> {
    let org = org.into();
    let username = username.into();
    let url = format!("/orgs/{org}/members/{username}/copilot");

    Request::<(), (), CopilotBusinessSeatDetail>::builder(&self.config)
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
  ) -> Request<CopilotAddCopilotSeatsForTeamsRequest, (), CopilotAddCopilotSeatsForTeamsResponse> {
    let org = org.into();
    let url = format!("/orgs/{org}/copilot/billing/selected_teams");

    Request::<CopilotAddCopilotSeatsForTeamsRequest, (), CopilotAddCopilotSeatsForTeamsResponse>::builder(&self.config)
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
  ) -> Request<CopilotCancelCopilotSeatAssignmentForTeamsRequest, (), CopilotCancelCopilotSeatAssignmentForTeamsResponse> {
    let org = org.into();
    let url = format!("/orgs/{org}/copilot/billing/selected_teams");

    Request::<CopilotCancelCopilotSeatAssignmentForTeamsRequest, (), CopilotCancelCopilotSeatAssignmentForTeamsResponse>::builder(&self.config)
      .delete(url)
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
  ) -> Request<(), CopilotListCopilotSeatsQuery, CopilotListCopilotSeatsResponse> {
    let org = org.into();
    let url = format!("/orgs/{org}/copilot/billing/seats");

    Request::<(), CopilotListCopilotSeatsQuery, CopilotListCopilotSeatsResponse>::builder(&self.config)
      .get(url)
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
  ) -> Request<CopilotAddCopilotSeatsForUsersRequest, (), CopilotAddCopilotSeatsForUsersResponse> {
    let org = org.into();
    let url = format!("/orgs/{org}/copilot/billing/selected_users");

    Request::<CopilotAddCopilotSeatsForUsersRequest, (), CopilotAddCopilotSeatsForUsersResponse>::builder(&self.config)
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
  ) -> Request<CopilotCancelCopilotSeatAssignmentForUsersRequest, (), CopilotCancelCopilotSeatAssignmentForUsersResponse> {
    let org = org.into();
    let url = format!("/orgs/{org}/copilot/billing/selected_users");

    Request::<CopilotCancelCopilotSeatAssignmentForUsersRequest, (), CopilotCancelCopilotSeatAssignmentForUsersResponse>::builder(&self.config)
      .delete(url)
      .build()
  }


}
