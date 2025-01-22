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

  pub type Response = Vec<Migration>;

  #[allow(clippy::large_enum_variant)]
  /// Allowed values that can be passed to the exclude param.
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QueryExclude {
    #[serde(rename = "repositories")]
    Repositories,
  }

  impl std::fmt::Display for QueryExclude {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match self {
        QueryExclude::Repositories => write!(f, "repositories"),
      }
    }
  }

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
    /// Exclude attributes from the API response to improve performance
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub exclude: Option<Vec<QueryExclude>>,
  }
}

pub mod start_for_org {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Migration;

  #[allow(clippy::large_enum_variant)]
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum RequestExclude {
    #[serde(rename = "repositories")]
    Repositories,
  }

  impl std::fmt::Display for RequestExclude {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match self {
        RequestExclude::Repositories => write!(f, "repositories"),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// Exclude related items from being returned in the response in order to improve performance of the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub exclude: Option<Vec<RequestExclude>>,
    /// Indicates whether attachments should be excluded from the migration (to reduce migration archive file size).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub exclude_attachments: Option<bool>,
    /// Indicates whether the repository git data should be excluded from the migration.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub exclude_git_data: Option<bool>,
    /// Indicates whether metadata should be excluded and only git source should be included for the migration.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub exclude_metadata: Option<bool>,
    /// Indicates whether projects owned by the organization or users should be excluded. from the migration.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub exclude_owner_projects: Option<bool>,
    /// Indicates whether releases should be excluded from the migration (to reduce migration archive file size).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub exclude_releases: Option<bool>,
    /// Indicates whether repositories should be locked (to prevent manipulation) while migrating data.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub lock_repositories: Option<bool>,
    /// Indicates whether this should only include organization metadata (repositories array should be empty and will ignore other flags).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub org_metadata_only: Option<bool>,
    /// A list of arrays indicating which repositories should be migrated.
    pub repositories: Vec<String>,
  }
}

pub mod get_status_for_org {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Migration;

  #[allow(clippy::large_enum_variant)]
  /// Allowed values that can be passed to the exclude param.
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QueryExclude {
    #[serde(rename = "repositories")]
    Repositories,
  }

  impl std::fmt::Display for QueryExclude {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match self {
        QueryExclude::Repositories => write!(f, "repositories"),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// Exclude attributes from the API response to improve performance
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub exclude: Option<Vec<QueryExclude>>,
  }
}

pub mod list_repos_for_org {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<MinimalRepository>;

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

pub mod get_import_status {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Import;
}

pub mod start_import {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Import;

  #[allow(clippy::large_enum_variant)]
  /// The originating VCS type. Without this parameter, the import job will take additional time to detect the VCS type before beginning the import. This detection step will be reflected in the response.
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum RequestVcs {
    #[serde(rename = "subversion")]
    Subversion,
    #[serde(rename = "git")]
    Git,
    #[serde(rename = "mercurial")]
    Mercurial,
    #[serde(rename = "tfvc")]
    Tfvc,
  }

  impl std::fmt::Display for RequestVcs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match self {
        RequestVcs::Subversion => write!(f, "subversion"),
        RequestVcs::Git => write!(f, "git"),
        RequestVcs::Mercurial => write!(f, "mercurial"),
        RequestVcs::Tfvc => write!(f, "tfvc"),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// For a tfvc import, the name of the project that is being imported.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub tfvc_project: Option<String>,
    /// The originating VCS type. Without this parameter, the import job will take additional time to detect the VCS type before beginning the import. This detection step will be reflected in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub vcs: Option<RequestVcs>,
    /// If authentication is required, the password to provide to `vcs_url`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub vcs_password: Option<String>,
    /// The URL of the originating repository.
    pub vcs_url: String,
    /// If authentication is required, the username to provide to `vcs_url`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub vcs_username: Option<String>,
  }
}

pub mod update_import {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Import;

  #[allow(clippy::large_enum_variant)]
  /// The type of version control system you are migrating from.
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum RequestVcs {
    #[serde(rename = "subversion")]
    Subversion,
    #[serde(rename = "tfvc")]
    Tfvc,
    #[serde(rename = "git")]
    Git,
    #[serde(rename = "mercurial")]
    Mercurial,
  }

  impl std::fmt::Display for RequestVcs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match self {
        RequestVcs::Subversion => write!(f, "subversion"),
        RequestVcs::Tfvc => write!(f, "tfvc"),
        RequestVcs::Git => write!(f, "git"),
        RequestVcs::Mercurial => write!(f, "mercurial"),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// For a tfvc import, the name of the project that is being imported.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub tfvc_project: Option<String>,
    /// The type of version control system you are migrating from.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub vcs: Option<RequestVcs>,
    /// The password to provide to the originating repository.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub vcs_password: Option<String>,
    /// The username to provide to the originating repository.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub vcs_username: Option<String>,
  }
}

pub mod get_commit_authors {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<PorterAuthor>;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// A user ID. Only return users with an ID greater than this ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub since: Option<i64>,
  }
}

pub mod map_commit_author {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = PorterAuthor;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The new Git author email.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub email: Option<String>,
    /// The new Git author name.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub name: Option<String>,
  }
}

pub mod get_large_files {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<PorterLargeFile>;
}

pub mod set_lfs_preference {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Import;

  #[allow(clippy::large_enum_variant)]
  /// Whether to store large files during the import. `opt_in` means large files will be stored using Git LFS. `opt_out` means large files will be removed during the import.
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum RequestUseLfs {
    #[serde(rename = "opt_in")]
    OptIn,
    #[serde(rename = "opt_out")]
    OptOut,
  }

  impl std::fmt::Display for RequestUseLfs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match self {
        RequestUseLfs::OptIn => write!(f, "opt_in"),
        RequestUseLfs::OptOut => write!(f, "opt_out"),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// Whether to store large files during the import. `opt_in` means large files will be stored using Git LFS. `opt_out` means large files will be removed during the import.
    pub use_lfs: RequestUseLfs,
  }
}

pub mod list_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Migration>;

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

pub mod start_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Migration;

  #[allow(clippy::large_enum_variant)]
  /// Allowed values that can be passed to the exclude param.
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum RequestExclude {
    #[serde(rename = "repositories")]
    Repositories,
  }

  impl std::fmt::Display for RequestExclude {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match self {
        RequestExclude::Repositories => write!(f, "repositories"),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// Exclude attributes from the API response to improve performance
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub exclude: Option<Vec<RequestExclude>>,
    /// Do not include attachments in the migration
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub exclude_attachments: Option<bool>,
    /// Indicates whether the repository git data should be excluded from the migration.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub exclude_git_data: Option<bool>,
    /// Indicates whether metadata should be excluded and only git source should be included for the migration.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub exclude_metadata: Option<bool>,
    /// Indicates whether projects owned by the organization or users should be excluded.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub exclude_owner_projects: Option<bool>,
    /// Do not include releases in the migration
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub exclude_releases: Option<bool>,
    /// Lock the repositories being migrated at the start of the migration
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub lock_repositories: Option<bool>,
    /// Indicates whether this should only include organization metadata (repositories array should be empty and will ignore other flags).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub org_metadata_only: Option<bool>,
    pub repositories: Vec<String>,
  }
}

pub mod get_status_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Migration;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub exclude: Option<Vec<String>>,
  }
}

pub mod list_repos_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<MinimalRepository>;

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

/// Move projects to or from GitHub.
pub struct GitHubMigrationsAPI {
  config: SharedAPIConfig,
}

impl GitHubMigrationsAPI {
  pub fn new(config: &SharedAPIConfig) -> Self {
    Self {
      config: config.clone(),
    }
  }

  /// **List organization migrations**
  ///
  /// Lists the most recent migrations, including both exports (which can be started through the REST API) and imports (which cannot be started using the REST API).
  ///
  /// A list of `repositories` is only returned for export migrations.
  ///
  /// *Documentation*: [https://docs.github.com/rest/migrations/orgs#list-organization-migrations](https://docs.github.com/rest/migrations/orgs#list-organization-migrations)
  pub fn list_for_org(
    &self,
    org: impl Into<String>,
  ) -> Request<(), list_for_org::Query, list_for_org::Response> {
    let org = org.into();
    let url = format!("/orgs/{org}/migrations");

    Request::<(), list_for_org::Query, list_for_org::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Start an organization migration**
  ///
  /// Initiates the generation of a migration archive.
  ///
  /// *Documentation*: [https://docs.github.com/rest/migrations/orgs#start-an-organization-migration](https://docs.github.com/rest/migrations/orgs#start-an-organization-migration)
  pub fn start_for_org(
    &self,
    org: impl Into<String>,
  ) -> Request<start_for_org::Request, (), start_for_org::Response> {
    let org = org.into();
    let url = format!("/orgs/{org}/migrations");

    Request::<start_for_org::Request, (), start_for_org::Response>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Get an organization migration status**
  ///
  /// Fetches the status of a migration.
  ///
  /// The `state` of a migration can be one of the following values:
  ///
  /// *   `pending`, which means the migration hasn't started yet.
  /// *   `exporting`, which means the migration is in progress.
  /// *   `exported`, which means the migration finished successfully.
  /// *   `failed`, which means the migration failed.
  ///
  /// *Documentation*: [https://docs.github.com/rest/migrations/orgs#get-an-organization-migration-status](https://docs.github.com/rest/migrations/orgs#get-an-organization-migration-status)
  pub fn get_status_for_org(
    &self,
    org: impl Into<String>,
    migration_id: impl Into<i64>,
  ) -> Request<(), get_status_for_org::Query, get_status_for_org::Response> {
    let org = org.into();
    let migration_id = migration_id.into();
    let url = format!("/orgs/{org}/migrations/{migration_id}");

    Request::<(), get_status_for_org::Query, get_status_for_org::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Download an organization migration archive**
  ///
  /// Fetches the URL to a migration archive.
  ///
  /// *Documentation*: [https://docs.github.com/rest/migrations/orgs#download-an-organization-migration-archive](https://docs.github.com/rest/migrations/orgs#download-an-organization-migration-archive)
  pub fn download_archive_for_org(
    &self,
    org: impl Into<String>,
    migration_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let migration_id = migration_id.into();
    let url = format!("/orgs/{org}/migrations/{migration_id}/archive");

    NoContentRequest::<(), ()>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Delete an organization migration archive**
  ///
  /// Deletes a previous migration archive. Migration archives are automatically deleted after seven days.
  ///
  /// *Documentation*: [https://docs.github.com/rest/migrations/orgs#delete-an-organization-migration-archive](https://docs.github.com/rest/migrations/orgs#delete-an-organization-migration-archive)
  pub fn delete_archive_for_org(
    &self,
    org: impl Into<String>,
    migration_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let migration_id = migration_id.into();
    let url = format!("/orgs/{org}/migrations/{migration_id}/archive");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Unlock an organization repository**
  ///
  /// Unlocks a repository that was locked for migration. You should unlock each migrated repository and [delete them](https://docs.github.com/rest/repos/repos#delete-a-repository) when the migration is complete and you no longer need the source data.
  ///
  /// *Documentation*: [https://docs.github.com/rest/migrations/orgs#unlock-an-organization-repository](https://docs.github.com/rest/migrations/orgs#unlock-an-organization-repository)
  pub fn unlock_repo_for_org(
    &self,
    org: impl Into<String>,
    migration_id: impl Into<i64>,
    repo_name: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let migration_id = migration_id.into();
    let repo_name = repo_name.into();
    let url = format!("/orgs/{org}/migrations/{migration_id}/repos/{repo_name}/lock");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List repositories in an organization migration**
  ///
  /// List all the repositories for this organization migration.
  ///
  /// *Documentation*: [https://docs.github.com/rest/migrations/orgs#list-repositories-in-an-organization-migration](https://docs.github.com/rest/migrations/orgs#list-repositories-in-an-organization-migration)
  pub fn list_repos_for_org(
    &self,
    org: impl Into<String>,
    migration_id: impl Into<i64>,
  ) -> Request<(), list_repos_for_org::Query, list_repos_for_org::Response> {
    let org = org.into();
    let migration_id = migration_id.into();
    let url = format!("/orgs/{org}/migrations/{migration_id}/repositories");

    Request::<(), list_repos_for_org::Query, list_repos_for_org::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get an import status**
  ///
  /// View the progress of an import.
  ///
  /// **Warning:** Due to very low levels of usage and available alternatives, this endpoint is deprecated and will no longer be available from 00:00 UTC on April 12, 2024. For more details and alternatives, see the [changelog](https://gh.io/source-imports-api-deprecation).
  ///
  /// **Import status**
  ///
  /// This section includes details about the possible values of the `status` field of the Import Progress response.
  ///
  /// An import that does not have errors will progress through these steps:
  ///
  /// *   `detecting` - the "detection" step of the import is in progress because the request did not include a `vcs` parameter. The import is identifying the type of source control present at the URL.
  /// *   `importing` - the "raw" step of the import is in progress. This is where commit data is fetched from the original repository. The import progress response will include `commit_count` (the total number of raw commits that will be imported) and `percent` (0 - 100, the current progress through the import).
  /// *   `mapping` - the "rewrite" step of the import is in progress. This is where SVN branches are converted to Git branches, and where author updates are applied. The import progress response does not include progress information.
  /// *   `pushing` - the "push" step of the import is in progress. This is where the importer updates the repository on GitHub. The import progress response will include `push_percent`, which is the percent value reported by `git push` when it is "Writing objects".
  /// *   `complete` - the import is complete, and the repository is ready on GitHub.
  ///
  /// If there are problems, you will see one of these in the `status` field:
  ///
  /// *   `auth_failed` - the import requires authentication in order to connect to the original repository. To update authentication for the import, please see the [Update an import](https://docs.github.com/rest/migrations/source-imports#update-an-import) section.
  /// *   `error` - the import encountered an error. The import progress response will include the `failed_step` and an error message. Contact [GitHub Support](https://support.github.com/contact?tags=dotcom-rest-api) for more information.
  /// *   `detection_needs_auth` - the importer requires authentication for the originating repository to continue detection. To update authentication for the import, please see the [Update an import](https://docs.github.com/rest/migrations/source-imports#update-an-import) section.
  /// *   `detection_found_nothing` - the importer didn't recognize any source control at the URL. To resolve, [Cancel the import](https://docs.github.com/rest/migrations/source-imports#cancel-an-import) and [retry](https://docs.github.com/rest/migrations/source-imports#start-an-import) with the correct URL.
  /// *   `detection_found_multiple` - the importer found several projects or repositories at the provided URL. When this is the case, the Import Progress response will also include a `project_choices` field with the possible project choices as values. To update project choice, please see the [Update an import](https://docs.github.com/rest/migrations/source-imports#update-an-import) section.
  ///
  /// **The project_choices field**
  ///
  /// When multiple projects are found at the provided URL, the response hash will include a `project_choices` field, the value of which is an array of hashes each representing a project choice. The exact key/value pairs of the project hashes will differ depending on the version control type.
  ///
  /// **Git LFS related fields**
  ///
  /// This section includes details about Git LFS related fields that may be present in the Import Progress response.
  ///
  /// *   `use_lfs` - describes whether the import has been opted in or out of using Git LFS. The value can be `opt_in`, `opt_out`, or `undecided` if no action has been taken.
  /// *   `has_large_files` - the boolean value describing whether files larger than 100MB were found during the `importing` step.
  /// *   `large_files_size` - the total size in gigabytes of files larger than 100MB found in the originating repository.
  /// *   `large_files_count` - the total number of files larger than 100MB found in the originating repository. To see a list of these files, make a "Get Large Files" request.
  ///
  /// *Documentation*: [https://docs.github.com/rest/migrations/source-imports#get-an-import-status](https://docs.github.com/rest/migrations/source-imports#get-an-import-status)
  pub fn get_import_status(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), (), get_import_status::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/import");

    Request::<(), (), get_import_status::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Start an import**
  ///
  /// Start a source import to a GitHub repository using GitHub Importer.
  /// Importing into a GitHub repository with GitHub Actions enabled is not supported and will
  /// return a status `422 Unprocessable Entity` response.
  ///
  /// **Warning:** Due to very low levels of usage and available alternatives, this endpoint is deprecated and will no longer be available from 00:00 UTC on April 12, 2024. For more details and alternatives, see the [changelog](https://gh.io/source-imports-api-deprecation).
  ///
  ///
  /// *Documentation*: [https://docs.github.com/rest/migrations/source-imports#start-an-import](https://docs.github.com/rest/migrations/source-imports#start-an-import)
  pub fn start_import(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<start_import::Request, (), start_import::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/import");

    Request::<start_import::Request, (), start_import::Response>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **Update an import**
  ///
  /// An import can be updated with credentials or a project choice by passing in the appropriate parameters in this API
  /// request. If no parameters are provided, the import will be restarted.
  ///
  /// Some servers (e.g. TFS servers) can have several projects at a single URL. In those cases the import progress will
  /// have the status `detection_found_multiple` and the Import Progress response will include a `project_choices` array.
  /// You can select the project to import by providing one of the objects in the `project_choices` array in the update request.
  ///
  /// **Warning:** Due to very low levels of usage and available alternatives, this endpoint is deprecated and will no longer be available from 00:00 UTC on April 12, 2024. For more details and alternatives, see the [changelog](https://gh.io/source-imports-api-deprecation).
  ///
  /// *Documentation*: [https://docs.github.com/rest/migrations/source-imports#update-an-import](https://docs.github.com/rest/migrations/source-imports#update-an-import)
  pub fn update_import(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<update_import::Request, (), update_import::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/import");

    Request::<update_import::Request, (), update_import::Response>::builder(&self.config)
      .patch(url)
      .build()
  }

  /// **Cancel an import**
  ///
  /// Stop an import for a repository.
  ///
  /// **Warning:** Due to very low levels of usage and available alternatives, this endpoint is deprecated and will no longer be available from 00:00 UTC on April 12, 2024. For more details and alternatives, see the [changelog](https://gh.io/source-imports-api-deprecation).
  ///
  ///
  /// *Documentation*: [https://docs.github.com/rest/migrations/source-imports#cancel-an-import](https://docs.github.com/rest/migrations/source-imports#cancel-an-import)
  pub fn cancel_import(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/import");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Get commit authors**
  ///
  /// Each type of source control system represents authors in a different way. For example, a Git commit author has a display name and an email address, but a Subversion commit author just has a username. The GitHub Importer will make the author information valid, but the author might not be correct. For example, it will change the bare Subversion username `hubot` into something like `hubot <hubot@12341234-abab-fefe-8787-fedcba987654>`.
  ///
  /// This endpoint and the [Map a commit author](https://docs.github.com/rest/migrations/source-imports#map-a-commit-author) endpoint allow you to provide correct Git author information.
  ///
  /// **Warning:** Due to very low levels of usage and available alternatives, this endpoint is deprecated and will no longer be available from 00:00 UTC on April 12, 2024. For more details and alternatives, see the [changelog](https://gh.io/source-imports-api-deprecation).
  ///
  /// *Documentation*: [https://docs.github.com/rest/migrations/source-imports#get-commit-authors](https://docs.github.com/rest/migrations/source-imports#get-commit-authors)
  pub fn get_commit_authors(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), get_commit_authors::Query, get_commit_authors::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/import/authors");

    Request::<(), get_commit_authors::Query, get_commit_authors::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Map a commit author**
  ///
  /// Update an author's identity for the import. Your application can continue updating authors any time before you push
  /// new commits to the repository.
  ///
  /// **Warning:** Due to very low levels of usage and available alternatives, this endpoint is deprecated and will no longer be available from 00:00 UTC on April 12, 2024. For more details and alternatives, see the [changelog](https://gh.io/source-imports-api-deprecation).
  ///
  ///
  /// *Documentation*: [https://docs.github.com/rest/migrations/source-imports#map-a-commit-author](https://docs.github.com/rest/migrations/source-imports#map-a-commit-author)
  pub fn map_commit_author(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    author_id: impl Into<i64>,
  ) -> Request<map_commit_author::Request, (), map_commit_author::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let author_id = author_id.into();
    let url = format!("/repos/{owner}/{repo}/import/authors/{author_id}");

    Request::<map_commit_author::Request, (), map_commit_author::Response>::builder(&self.config)
      .patch(url)
      .build()
  }

  /// **Get large files**
  ///
  /// List files larger than 100MB found during the import
  ///
  /// **Warning:** Due to very low levels of usage and available alternatives, this endpoint is deprecated and will no longer be available from 00:00 UTC on April 12, 2024. For more details and alternatives, see the [changelog](https://gh.io/source-imports-api-deprecation).
  ///
  ///
  /// *Documentation*: [https://docs.github.com/rest/migrations/source-imports#get-large-files](https://docs.github.com/rest/migrations/source-imports#get-large-files)
  pub fn get_large_files(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), (), get_large_files::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/import/large_files");

    Request::<(), (), get_large_files::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Update Git LFS preference**
  ///
  /// You can import repositories from Subversion, Mercurial, and TFS that include files larger than 100MB. This ability
  /// is powered by [Git LFS](https://git-lfs.com).
  ///
  /// You can learn more about our LFS feature and working with large files [on our help
  /// site](https://docs.github.com/repositories/working-with-files/managing-large-files).
  ///
  /// **Warning:** Due to very low levels of usage and available alternatives, this endpoint is deprecated and will no longer be available from 00:00 UTC on April 12, 2024. For more details and alternatives, see the [changelog](https://gh.io/source-imports-api-deprecation).
  ///
  ///
  /// *Documentation*: [https://docs.github.com/rest/migrations/source-imports#update-git-lfs-preference](https://docs.github.com/rest/migrations/source-imports#update-git-lfs-preference)
  pub fn set_lfs_preference(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<set_lfs_preference::Request, (), set_lfs_preference::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/import/lfs");

    Request::<set_lfs_preference::Request, (), set_lfs_preference::Response>::builder(&self.config)
      .patch(url)
      .build()
  }

  /// **List user migrations**
  ///
  /// Lists all migrations a user has started.
  ///
  /// *Documentation*: [https://docs.github.com/rest/migrations/users#list-user-migrations](https://docs.github.com/rest/migrations/users#list-user-migrations)
  pub fn list_for_authenticated_user(
    &self,
  ) -> Request<(), list_for_authenticated_user::Query, list_for_authenticated_user::Response> {
    let url = format!("/user/migrations");

    Request::<(), list_for_authenticated_user::Query, list_for_authenticated_user::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Start a user migration**
  ///
  /// Initiates the generation of a user migration archive.
  ///
  /// *Documentation*: [https://docs.github.com/rest/migrations/users#start-a-user-migration](https://docs.github.com/rest/migrations/users#start-a-user-migration)
  pub fn start_for_authenticated_user(
    &self,
  ) -> Request<start_for_authenticated_user::Request, (), start_for_authenticated_user::Response>
  {
    let url = format!("/user/migrations");

    Request::<start_for_authenticated_user::Request, (), start_for_authenticated_user::Response>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Get a user migration status**
  ///
  /// Fetches a single user migration. The response includes the `state` of the migration, which can be one of the following values:
  ///
  /// *   `pending` - the migration hasn't started yet.
  /// *   `exporting` - the migration is in progress.
  /// *   `exported` - the migration finished successfully.
  /// *   `failed` - the migration failed.
  ///
  /// Once the migration has been `exported` you can [download the migration archive](https://docs.github.com/rest/migrations/users#download-a-user-migration-archive).
  ///
  /// *Documentation*: [https://docs.github.com/rest/migrations/users#get-a-user-migration-status](https://docs.github.com/rest/migrations/users#get-a-user-migration-status)
  pub fn get_status_for_authenticated_user(
    &self,
    migration_id: impl Into<i64>,
  ) -> Request<
    (),
    get_status_for_authenticated_user::Query,
    get_status_for_authenticated_user::Response,
  > {
    let migration_id = migration_id.into();
    let url = format!("/user/migrations/{migration_id}");

    Request::<
      (),
      get_status_for_authenticated_user::Query,
      get_status_for_authenticated_user::Response,
    >::builder(&self.config)
    .get(url)
    .build()
  }

  /// **Download a user migration archive**
  ///
  /// Fetches the URL to download the migration archive as a `tar.gz` file. Depending on the resources your repository uses, the migration archive can contain JSON files with data for these objects:
  ///
  /// *   attachments
  /// *   bases
  /// *   commit\_comments
  /// *   issue\_comments
  /// *   issue\_events
  /// *   issues
  /// *   milestones
  /// *   organizations
  /// *   projects
  /// *   protected\_branches
  /// *   pull\_request\_reviews
  /// *   pull\_requests
  /// *   releases
  /// *   repositories
  /// *   review\_comments
  /// *   schema
  /// *   users
  ///
  /// The archive will also contain an `attachments` directory that includes all attachment files uploaded to GitHub.com and a `repositories` directory that contains the repository's Git data.
  ///
  /// *Documentation*: [https://docs.github.com/rest/migrations/users#download-a-user-migration-archive](https://docs.github.com/rest/migrations/users#download-a-user-migration-archive)
  pub fn get_archive_for_authenticated_user(
    &self,
    migration_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let migration_id = migration_id.into();
    let url = format!("/user/migrations/{migration_id}/archive");

    NoContentRequest::<(), ()>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Delete a user migration archive**
  ///
  /// Deletes a previous migration archive. Downloadable migration archives are automatically deleted after seven days. Migration metadata, which is returned in the [List user migrations](https://docs.github.com/rest/migrations/users#list-user-migrations) and [Get a user migration status](https://docs.github.com/rest/migrations/users#get-a-user-migration-status) endpoints, will continue to be available even after an archive is deleted.
  ///
  /// *Documentation*: [https://docs.github.com/rest/migrations/users#delete-a-user-migration-archive](https://docs.github.com/rest/migrations/users#delete-a-user-migration-archive)
  pub fn delete_archive_for_authenticated_user(
    &self,
    migration_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let migration_id = migration_id.into();
    let url = format!("/user/migrations/{migration_id}/archive");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Unlock a user repository**
  ///
  /// Unlocks a repository. You can lock repositories when you [start a user migration](https://docs.github.com/rest/migrations/users#start-a-user-migration). Once the migration is complete you can unlock each repository to begin using it again or [delete the repository](https://docs.github.com/rest/repos/repos#delete-a-repository) if you no longer need the source data. Returns a status of `404 Not Found` if the repository is not locked.
  ///
  /// *Documentation*: [https://docs.github.com/rest/migrations/users#unlock-a-user-repository](https://docs.github.com/rest/migrations/users#unlock-a-user-repository)
  pub fn unlock_repo_for_authenticated_user(
    &self,
    migration_id: impl Into<i64>,
    repo_name: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let migration_id = migration_id.into();
    let repo_name = repo_name.into();
    let url = format!("/user/migrations/{migration_id}/repos/{repo_name}/lock");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List repositories for a user migration**
  ///
  /// Lists all the repositories for this user migration.
  ///
  /// *Documentation*: [https://docs.github.com/rest/migrations/users#list-repositories-for-a-user-migration](https://docs.github.com/rest/migrations/users#list-repositories-for-a-user-migration)
  pub fn list_repos_for_authenticated_user(
    &self,
    migration_id: impl Into<i64>,
  ) -> Request<
    (),
    list_repos_for_authenticated_user::Query,
    list_repos_for_authenticated_user::Response,
  > {
    let migration_id = migration_id.into();
    let url = format!("/user/migrations/{migration_id}/repositories");

    Request::<
      (),
      list_repos_for_authenticated_user::Query,
      list_repos_for_authenticated_user::Response,
    >::builder(&self.config)
    .get(url)
    .build()
  }
}
