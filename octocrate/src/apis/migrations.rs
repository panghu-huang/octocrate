#[allow(unused_imports)]
use crate::types::*;
use octocrate_core::*;

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
  ) -> Request<(), (), Import> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/import");

    Request::<(), (), Import>::builder(&self.config)
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
  ) -> Request<MigrationsStartImportRequest, (), Import> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/import");

    Request::<MigrationsStartImportRequest, (), Import>::builder(&self.config)
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
  ) -> Request<MigrationsUpdateImportRequest, (), Import> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/import");

    Request::<MigrationsUpdateImportRequest, (), Import>::builder(&self.config)
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

  /// **List repositories in an organization migration**
  ///
  /// List all the repositories for this organization migration.
  ///
  /// *Documentation*: [https://docs.github.com/rest/migrations/orgs#list-repositories-in-an-organization-migration](https://docs.github.com/rest/migrations/orgs#list-repositories-in-an-organization-migration)
  pub fn list_repos_for_org(
    &self,
    org: impl Into<String>,
    migration_id: impl Into<i64>,
  ) -> Request<(), MigrationsListReposForOrgQuery, MinimalRepositoryArray> {
    let org = org.into();
    let migration_id = migration_id.into();
    let url = format!("/orgs/{org}/migrations/{migration_id}/repositories");

    Request::<(), MigrationsListReposForOrgQuery, MinimalRepositoryArray>::builder(&self.config)
      .get(url)
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
  ) -> Request<(), (), PorterLargeFileArray> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/import/large_files");

    Request::<(), (), PorterLargeFileArray>::builder(&self.config)
      .get(url)
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
  ) -> Request<(), MigrationsGetCommitAuthorsQuery, PorterAuthorArray> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/import/authors");

    Request::<(), MigrationsGetCommitAuthorsQuery, PorterAuthorArray>::builder(&self.config)
      .get(url)
      .build()
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
  ) -> Request<(), MigrationsListForOrgQuery, MigrationArray> {
    let org = org.into();
    let url = format!("/orgs/{org}/migrations");

    Request::<(), MigrationsListForOrgQuery, MigrationArray>::builder(&self.config)
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
  ) -> Request<MigrationsStartForOrgRequest, (), Migration> {
    let org = org.into();
    let url = format!("/orgs/{org}/migrations");

    Request::<MigrationsStartForOrgRequest, (), Migration>::builder(&self.config)
      .post(url)
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

  /// **List repositories for a user migration**
  ///
  /// Lists all the repositories for this user migration.
  ///
  /// *Documentation*: [https://docs.github.com/rest/migrations/users#list-repositories-for-a-user-migration](https://docs.github.com/rest/migrations/users#list-repositories-for-a-user-migration)
  pub fn list_repos_for_authenticated_user(
    &self,
    migration_id: impl Into<i64>,
  ) -> Request<(), MigrationsListReposForAuthenticatedUserQuery, MinimalRepositoryArray> {
    let migration_id = migration_id.into();
    let url = format!("/user/migrations/{migration_id}/repositories");

    Request::<(), MigrationsListReposForAuthenticatedUserQuery, MinimalRepositoryArray>::builder(
      &self.config,
    )
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

  /// **List user migrations**
  ///
  /// Lists all migrations a user has started.
  ///
  /// *Documentation*: [https://docs.github.com/rest/migrations/users#list-user-migrations](https://docs.github.com/rest/migrations/users#list-user-migrations)
  pub fn list_for_authenticated_user(
    &self,
  ) -> Request<(), MigrationsListForAuthenticatedUserQuery, MigrationArray> {
    let url = format!("/user/migrations");

    Request::<(), MigrationsListForAuthenticatedUserQuery, MigrationArray>::builder(&self.config)
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
  ) -> Request<MigrationsStartForAuthenticatedUserRequest, (), Migration> {
    let url = format!("/user/migrations");

    Request::<MigrationsStartForAuthenticatedUserRequest, (), Migration>::builder(&self.config)
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
  ) -> Request<(), MigrationsGetStatusForOrgQuery, Migration> {
    let org = org.into();
    let migration_id = migration_id.into();
    let url = format!("/orgs/{org}/migrations/{migration_id}");

    Request::<(), MigrationsGetStatusForOrgQuery, Migration>::builder(&self.config)
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
  ) -> Request<MigrationsMapCommitAuthorRequest, (), PorterAuthor> {
    let owner = owner.into();
    let repo = repo.into();
    let author_id = author_id.into();
    let url = format!("/repos/{owner}/{repo}/import/authors/{author_id}");

    Request::<MigrationsMapCommitAuthorRequest, (), PorterAuthor>::builder(&self.config)
      .patch(url)
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
  ) -> Request<MigrationsSetLfsPreferenceRequest, (), Import> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/import/lfs");

    Request::<MigrationsSetLfsPreferenceRequest, (), Import>::builder(&self.config)
      .patch(url)
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
  ) -> Request<(), MigrationsGetStatusForAuthenticatedUserQuery, Migration> {
    let migration_id = migration_id.into();
    let url = format!("/user/migrations/{migration_id}");

    Request::<(), MigrationsGetStatusForAuthenticatedUserQuery, Migration>::builder(&self.config)
      .get(url)
      .build()
  }
}
