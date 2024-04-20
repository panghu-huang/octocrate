use octocrate_core::*;
#[allow(unused_imports)]
use octocrate_types::*;

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

  /// **List public SSH keys for the authenticated user**
  ///
  /// Lists the public SSH keys for the authenticated user's GitHub account.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `read:public_key` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/users/keys#list-public-ssh-keys-for-the-authenticated-user](https://docs.github.com/rest/users/keys#list-public-ssh-keys-for-the-authenticated-user)
  pub fn list_public_ssh_keys_for_authenticated_user(
    &self,
  ) -> Request<(), UsersListPublicSshKeysForAuthenticatedUserQuery, KeyArray> {
    let url = format!("/user/keys");

    Request::<(), UsersListPublicSshKeysForAuthenticatedUserQuery, KeyArray>::builder(&self.config)
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
  ) -> Request<UsersCreatePublicSshKeyForAuthenticatedUserRequest, (), Key> {
    let url = format!("/user/keys");

    Request::<UsersCreatePublicSshKeyForAuthenticatedUserRequest, (), Key>::builder(&self.config)
      .post(url)
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
  ) -> Request<(), (), UsersGetByUsernameResponse> {
    let username = username.into();
    let url = format!("/users/{username}");

    Request::<(), (), UsersGetByUsernameResponse>::builder(&self.config)
      .get(url)
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
  ) -> Request<(), UsersListGpgKeysForAuthenticatedUserQuery, GpgKeyArray> {
    let url = format!("/user/gpg_keys");

    Request::<(), UsersListGpgKeysForAuthenticatedUserQuery, GpgKeyArray>::builder(&self.config)
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
  ) -> Request<UsersCreateGpgKeyForAuthenticatedUserRequest, (), GpgKey> {
    let url = format!("/user/gpg_keys");

    Request::<UsersCreateGpgKeyForAuthenticatedUserRequest, (), GpgKey>::builder(&self.config)
      .post(url)
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
  ) -> Request<(), UsersListGpgKeysForUserQuery, GpgKeyArray> {
    let username = username.into();
    let url = format!("/users/{username}/gpg_keys");

    Request::<(), UsersListGpgKeysForUserQuery, GpgKeyArray>::builder(&self.config)
      .get(url)
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
  ) -> Request<(), (), GpgKey> {
    let gpg_key_id = gpg_key_id.into();
    let url = format!("/user/gpg_keys/{gpg_key_id}");

    Request::<(), (), GpgKey>::builder(&self.config)
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
  ) -> Request<(), (), Key> {
    let key_id = key_id.into();
    let url = format!("/user/keys/{key_id}");

    Request::<(), (), Key>::builder(&self.config)
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

  /// **List social accounts for the authenticated user**
  ///
  /// Lists all of your social accounts.
  ///
  /// *Documentation*: [https://docs.github.com/rest/users/social-accounts#list-social-accounts-for-the-authenticated-user](https://docs.github.com/rest/users/social-accounts#list-social-accounts-for-the-authenticated-user)
  pub fn list_social_accounts_for_authenticated_user(
    &self,
  ) -> Request<(), UsersListSocialAccountsForAuthenticatedUserQuery, SocialAccountArray> {
    let url = format!("/user/social_accounts");

    Request::<(), UsersListSocialAccountsForAuthenticatedUserQuery, SocialAccountArray>::builder(
      &self.config,
    )
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
  ) -> Request<UsersAddSocialAccountForAuthenticatedUserRequest, (), SocialAccountArray> {
    let url = format!("/user/social_accounts");

    Request::<UsersAddSocialAccountForAuthenticatedUserRequest, (), SocialAccountArray>::builder(
      &self.config,
    )
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
  ) -> NoContentRequest<UsersDeleteSocialAccountForAuthenticatedUserRequest, ()> {
    let url = format!("/user/social_accounts");

    NoContentRequest::<UsersDeleteSocialAccountForAuthenticatedUserRequest, ()>::builder(
      &self.config,
    )
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
  pub fn list(&self) -> Request<(), UsersListQuery, SimpleUserArray> {
    let url = format!("/users");

    Request::<(), UsersListQuery, SimpleUserArray>::builder(&self.config)
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

  /// **List the people a user follows**
  ///
  /// Lists the people who the specified user follows.
  ///
  /// *Documentation*: [https://docs.github.com/rest/users/followers#list-the-people-a-user-follows](https://docs.github.com/rest/users/followers#list-the-people-a-user-follows)
  pub fn list_following_for_user(
    &self,
    username: impl Into<String>,
  ) -> Request<(), UsersListFollowingForUserQuery, SimpleUserArray> {
    let username = username.into();
    let url = format!("/users/{username}/following");

    Request::<(), UsersListFollowingForUserQuery, SimpleUserArray>::builder(&self.config)
      .get(url)
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
  ) -> Request<(), UsersListEmailsForAuthenticatedUserQuery, EmailArray> {
    let url = format!("/user/emails");

    Request::<(), UsersListEmailsForAuthenticatedUserQuery, EmailArray>::builder(&self.config)
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
  ) -> Request<UsersAddEmailForAuthenticatedUserRequest, (), EmailArray> {
    let url = format!("/user/emails");

    Request::<UsersAddEmailForAuthenticatedUserRequest, (), EmailArray>::builder(&self.config)
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
  ) -> NoContentRequest<UsersDeleteEmailForAuthenticatedUserRequest, ()> {
    let url = format!("/user/emails");

    NoContentRequest::<UsersDeleteEmailForAuthenticatedUserRequest, ()>::builder(&self.config)
      .delete(url)
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

  /// **List SSH signing keys for a user**
  ///
  /// Lists the SSH signing keys for a user. This operation is accessible by anyone.
  ///
  /// *Documentation*: [https://docs.github.com/rest/users/ssh-signing-keys#list-ssh-signing-keys-for-a-user](https://docs.github.com/rest/users/ssh-signing-keys#list-ssh-signing-keys-for-a-user)
  pub fn list_ssh_signing_keys_for_user(
    &self,
    username: impl Into<String>,
  ) -> Request<(), UsersListSshSigningKeysForUserQuery, SshSigningKeyArray> {
    let username = username.into();
    let url = format!("/users/{username}/ssh_signing_keys");

    Request::<(), UsersListSshSigningKeysForUserQuery, SshSigningKeyArray>::builder(&self.config)
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
  ) -> Request<(), UsersGetContextForUserQuery, Hovercard> {
    let username = username.into();
    let url = format!("/users/{username}/hovercard");

    Request::<(), UsersGetContextForUserQuery, Hovercard>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List followers of the authenticated user**
  ///
  /// Lists the people following the authenticated user.
  ///
  /// *Documentation*: [https://docs.github.com/rest/users/followers#list-followers-of-the-authenticated-user](https://docs.github.com/rest/users/followers#list-followers-of-the-authenticated-user)
  pub fn list_followers_for_authenticated_user(
    &self,
  ) -> Request<(), UsersListFollowersForAuthenticatedUserQuery, SimpleUserArray> {
    let url = format!("/user/followers");

    Request::<(), UsersListFollowersForAuthenticatedUserQuery, SimpleUserArray>::builder(
      &self.config,
    )
    .get(url)
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
  ) -> Request<(), (), SshSigningKey> {
    let ssh_signing_key_id = ssh_signing_key_id.into();
    let url = format!("/user/ssh_signing_keys/{ssh_signing_key_id}");

    Request::<(), (), SshSigningKey>::builder(&self.config)
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

  /// **List users blocked by the authenticated user**
  ///
  /// List the users you've blocked on your personal account.
  ///
  /// *Documentation*: [https://docs.github.com/rest/users/blocking#list-users-blocked-by-the-authenticated-user](https://docs.github.com/rest/users/blocking#list-users-blocked-by-the-authenticated-user)
  pub fn list_blocked_by_authenticated_user(
    &self,
  ) -> Request<(), UsersListBlockedByAuthenticatedUserQuery, SimpleUserArray> {
    let url = format!("/user/blocks");

    Request::<(), UsersListBlockedByAuthenticatedUserQuery, SimpleUserArray>::builder(&self.config)
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
  ) -> Request<(), UsersListFollowedByAuthenticatedUserQuery, SimpleUserArray> {
    let url = format!("/user/following");

    Request::<(), UsersListFollowedByAuthenticatedUserQuery, SimpleUserArray>::builder(&self.config)
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
  ) -> Request<(), UsersListPublicKeysForUserQuery, KeySimpleArray> {
    let username = username.into();
    let url = format!("/users/{username}/keys");

    Request::<(), UsersListPublicKeysForUserQuery, KeySimpleArray>::builder(&self.config)
      .get(url)
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
  ) -> Request<(), UsersListSshSigningKeysForAuthenticatedUserQuery, SshSigningKeyArray> {
    let url = format!("/user/ssh_signing_keys");

    Request::<(), UsersListSshSigningKeysForAuthenticatedUserQuery, SshSigningKeyArray>::builder(
      &self.config,
    )
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
  ) -> Request<UsersCreateSshSigningKeyForAuthenticatedUserRequest, (), SshSigningKey> {
    let url = format!("/user/ssh_signing_keys");

    Request::<UsersCreateSshSigningKeyForAuthenticatedUserRequest, (), SshSigningKey>::builder(
      &self.config,
    )
    .post(url)
    .build()
  }

  /// **Get the authenticated user**
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `user` scope in order for the response to include private profile information.
  ///
  /// *Documentation*: [https://docs.github.com/rest/users/users#get-the-authenticated-user](https://docs.github.com/rest/users/users#get-the-authenticated-user)
  pub fn get_authenticated(&self) -> Request<(), (), UsersGetAuthenticatedResponse> {
    let url = format!("/user");

    Request::<(), (), UsersGetAuthenticatedResponse>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Update the authenticated user**
  ///
  /// **Note:** If your email is set to private and you send an `email` parameter as part of this request to update your profile, your privacy settings are still enforced: the email address will not be displayed on your public profile or via the API.
  ///
  /// *Documentation*: [https://docs.github.com/rest/users/users#update-the-authenticated-user](https://docs.github.com/rest/users/users#update-the-authenticated-user)
  pub fn update_authenticated(&self) -> Request<UsersUpdateAuthenticatedRequest, (), PrivateUser> {
    let url = format!("/user");

    Request::<UsersUpdateAuthenticatedRequest, (), PrivateUser>::builder(&self.config)
      .patch(url)
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
  ) -> Request<(), UsersListSocialAccountsForUserQuery, SocialAccountArray> {
    let username = username.into();
    let url = format!("/users/{username}/social_accounts");

    Request::<(), UsersListSocialAccountsForUserQuery, SocialAccountArray>::builder(&self.config)
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
  ) -> Request<(), UsersListFollowersForUserQuery, SimpleUserArray> {
    let username = username.into();
    let url = format!("/users/{username}/followers");

    Request::<(), UsersListFollowersForUserQuery, SimpleUserArray>::builder(&self.config)
      .get(url)
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
  ) -> Request<(), UsersListPublicEmailsForAuthenticatedUserQuery, EmailArray> {
    let url = format!("/user/public_emails");

    Request::<(), UsersListPublicEmailsForAuthenticatedUserQuery, EmailArray>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Set primary email visibility for the authenticated user**
  ///
  /// Sets the visibility for your primary email addresses.
  ///
  /// *Documentation*: [https://docs.github.com/rest/users/emails#set-primary-email-visibility-for-the-authenticated-user](https://docs.github.com/rest/users/emails#set-primary-email-visibility-for-the-authenticated-user)
  pub fn set_primary_email_visibility_for_authenticated_user(
    &self,
  ) -> Request<UsersSetPrimaryEmailVisibilityForAuthenticatedUserRequest, (), EmailArray> {
    let url = format!("/user/email/visibility");

    Request::<UsersSetPrimaryEmailVisibilityForAuthenticatedUserRequest, (), EmailArray>::builder(
      &self.config,
    )
    .patch(url)
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
}
