use octocrate_core::*;
#[allow(unused_imports)]
use octocrate_types::*;

/// Manage packages for authenticated users and organizations.
pub struct GitHubPackagesAPI {
  config: SharedAPIConfig,
}

impl GitHubPackagesAPI {
  pub fn new(config: &SharedAPIConfig) -> Self {
    Self {
      config: config.clone(),
    }
  }

  /// **Get list of conflicting packages during Docker migration for organization**
  ///
  /// Lists all packages that are in a specific organization, are readable by the requesting user, and that encountered a conflict during a Docker migration.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `read:packages` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/packages/packages#get-list-of-conflicting-packages-during-docker-migration-for-organization](https://docs.github.com/rest/packages/packages#get-list-of-conflicting-packages-during-docker-migration-for-organization)
  pub fn list_docker_migration_conflicting_packages_for_organization(
    &self,
    org: impl Into<String>,
  ) -> Request<(), (), PackageArray> {
    let org = org.into();
    let url = format!("/orgs/{org}/docker/conflicts");

    Request::<(), (), PackageArray>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List packages for an organization**
  ///
  /// Lists packages in an organization readable by the user.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `read:packages` scope to use this endpoint. If the `package_type` belongs to a GitHub Packages registry that only supports repository-scoped permissions, the `repo` scope is also required. For the list of these registries, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages)."
  ///
  /// *Documentation*: [https://docs.github.com/rest/packages/packages#list-packages-for-an-organization](https://docs.github.com/rest/packages/packages#list-packages-for-an-organization)
  pub fn list_packages_for_organization(
    &self,
    org: impl Into<String>,
  ) -> Request<(), PackagesListPackagesForOrganizationQuery, PackageArray> {
    let org = org.into();
    let url = format!("/orgs/{org}/packages");

    Request::<(), PackagesListPackagesForOrganizationQuery, PackageArray>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get a package for an organization**
  ///
  /// Gets a specific package in an organization.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `read:packages` scope to use this endpoint. If the `package_type` belongs to a GitHub Packages registry that only supports repository-scoped permissions, the `repo` scope is also required. For the list of these registries, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages)."
  ///
  /// *Documentation*: [https://docs.github.com/rest/packages/packages#get-a-package-for-an-organization](https://docs.github.com/rest/packages/packages#get-a-package-for-an-organization)
  pub fn get_package_for_organization(
    &self,
    package_type: impl Into<PackagesGetPackageForOrganizationParametersPackageType>,
    package_name: impl Into<String>,
    org: impl Into<String>,
  ) -> Request<(), (), Package> {
    let package_type = package_type.into();
    let package_name = package_name.into();
    let org = org.into();
    let package_type = package_type.to_string();
    let url = format!("/orgs/{org}/packages/{package_type}/{package_name}");

    Request::<(), (), Package>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Delete a package for an organization**
  ///
  /// Deletes an entire package in an organization. You cannot delete a public package if any version of the package has more than 5,000 downloads. In this scenario, contact GitHub support for further assistance.
  ///
  /// The authenticated user must have admin permissions in the organization to use this endpoint. If the `package_type` belongs to a GitHub Packages registry that supports granular permissions, the authenticated user must also have admin permissions to the package. For the list of these registries, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#granular-permissions-for-userorganization-scoped-packages)."
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `read:packages` and `delete:packages` scopes to use this endpoint. If the `package_type` belongs to a GitHub Packages registry that only supports repository-scoped permissions, the `repo` scope is also required. For the list of these registries, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages)."
  ///
  /// *Documentation*: [https://docs.github.com/rest/packages/packages#delete-a-package-for-an-organization](https://docs.github.com/rest/packages/packages#delete-a-package-for-an-organization)
  pub fn delete_package_for_org(
    &self,
    package_type: impl Into<PackagesDeletePackageForOrgParametersPackageType>,
    package_name: impl Into<String>,
    org: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let package_type = package_type.into();
    let package_name = package_name.into();
    let org = org.into();
    let package_type = package_type.to_string();
    let url = format!("/orgs/{org}/packages/{package_type}/{package_name}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Restore a package for an organization**
  ///
  /// Restores an entire package in an organization.
  ///
  /// You can restore a deleted package under the following conditions:
  ///   - The package was deleted within the last 30 days.
  ///   - The same package namespace and version is still available and not reused for a new package. If the same package namespace is not available, you will not be able to restore your package. In this scenario, to restore the deleted package, you must delete the new package that uses the deleted package's namespace first.
  ///
  /// The authenticated user must have admin permissions in the organization to use this endpoint. If the `package_type` belongs to a GitHub Packages registry that supports granular permissions, the authenticated user must also have admin permissions to the package. For the list of these registries, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#granular-permissions-for-userorganization-scoped-packages)."
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `read:packages` and `write:packages` scopes to use this endpoint. If the `package_type` belongs to a GitHub Packages registry that only supports repository-scoped permissions, the `repo` scope is also required. For the list of these registries, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages)."
  ///
  /// *Documentation*: [https://docs.github.com/rest/packages/packages#restore-a-package-for-an-organization](https://docs.github.com/rest/packages/packages#restore-a-package-for-an-organization)
  pub fn restore_package_for_org(
    &self,
    package_type: impl Into<PackagesRestorePackageForOrgParametersPackageType>,
    package_name: impl Into<String>,
    org: impl Into<String>,
  ) -> NoContentRequest<(), PackagesRestorePackageForOrgQuery> {
    let package_type = package_type.into();
    let package_name = package_name.into();
    let org = org.into();
    let package_type = package_type.to_string();
    let url = format!("/orgs/{org}/packages/{package_type}/{package_name}/restore");

    NoContentRequest::<(), PackagesRestorePackageForOrgQuery>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **List package versions for a package owned by an organization**
  ///
  /// Lists package versions for a package owned by an organization.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint if the `package_type` belongs to a GitHub Packages registry that only supports repository-scoped permissions. For the list of these registries, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages)."
  ///
  /// *Documentation*: [https://docs.github.com/rest/packages/packages#list-package-versions-for-a-package-owned-by-an-organization](https://docs.github.com/rest/packages/packages#list-package-versions-for-a-package-owned-by-an-organization)
  pub fn get_all_package_versions_for_package_owned_by_org(
    &self,
    package_type: impl Into<PackagesGetAllPackageVersionsForPackageOwnedByOrgParametersPackageType>,
    package_name: impl Into<String>,
    org: impl Into<String>,
  ) -> Request<(), PackagesGetAllPackageVersionsForPackageOwnedByOrgQuery, PackageVersionArray> {
    let package_type = package_type.into();
    let package_name = package_name.into();
    let org = org.into();
    let package_type = package_type.to_string();
    let url = format!("/orgs/{org}/packages/{package_type}/{package_name}/versions");

    Request::<(), PackagesGetAllPackageVersionsForPackageOwnedByOrgQuery, PackageVersionArray>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get a package version for an organization**
  ///
  /// Gets a specific package version in an organization.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `read:packages` scope to use this endpoint. If the `package_type` belongs to a GitHub Packages registry that only supports repository-scoped permissions, the `repo` scope is also required. For the list of these registries, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages)."
  ///
  /// *Documentation*: [https://docs.github.com/rest/packages/packages#get-a-package-version-for-an-organization](https://docs.github.com/rest/packages/packages#get-a-package-version-for-an-organization)
  pub fn get_package_version_for_organization(
    &self,
    package_type: impl Into<PackagesGetPackageVersionForOrganizationParametersPackageType>,
    package_name: impl Into<String>,
    org: impl Into<String>,
    package_version_id: impl Into<i64>,
  ) -> Request<(), (), PackageVersion> {
    let package_type = package_type.into();
    let package_name = package_name.into();
    let org = org.into();
    let package_version_id = package_version_id.into();
    let package_type = package_type.to_string();
    let url =
      format!("/orgs/{org}/packages/{package_type}/{package_name}/versions/{package_version_id}");

    Request::<(), (), PackageVersion>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Delete package version for an organization**
  ///
  /// Deletes a specific package version in an organization. If the package is public and the package version has more than 5,000 downloads, you cannot delete the package version. In this scenario, contact GitHub support for further assistance.
  ///
  /// The authenticated user must have admin permissions in the organization to use this endpoint. If the `package_type` belongs to a GitHub Packages registry that supports granular permissions, the authenticated user must also have admin permissions to the package. For the list of these registries, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#granular-permissions-for-userorganization-scoped-packages)."
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `read:packages` and `delete:packages` scopes to use this endpoint. If the `package_type` belongs to a GitHub Packages registry that only supports repository-scoped permissions, the `repo` scope is also required. For the list of these registries, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages)."
  ///
  /// *Documentation*: [https://docs.github.com/rest/packages/packages#delete-package-version-for-an-organization](https://docs.github.com/rest/packages/packages#delete-package-version-for-an-organization)
  pub fn delete_package_version_for_org(
    &self,
    package_type: impl Into<PackagesDeletePackageVersionForOrgParametersPackageType>,
    package_name: impl Into<String>,
    org: impl Into<String>,
    package_version_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let package_type = package_type.into();
    let package_name = package_name.into();
    let org = org.into();
    let package_version_id = package_version_id.into();
    let package_type = package_type.to_string();
    let url =
      format!("/orgs/{org}/packages/{package_type}/{package_name}/versions/{package_version_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Restore package version for an organization**
  ///
  /// Restores a specific package version in an organization.
  ///
  /// You can restore a deleted package under the following conditions:
  ///   - The package was deleted within the last 30 days.
  ///   - The same package namespace and version is still available and not reused for a new package. If the same package namespace is not available, you will not be able to restore your package. In this scenario, to restore the deleted package, you must delete the new package that uses the deleted package's namespace first.
  ///
  /// The authenticated user must have admin permissions in the organization to use this endpoint. If the `package_type` belongs to a GitHub Packages registry that supports granular permissions, the authenticated user must also have admin permissions to the package. For the list of these registries, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#granular-permissions-for-userorganization-scoped-packages)."
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `read:packages` and `write:packages` scopes to use this endpoint. If the `package_type` belongs to a GitHub Packages registry that only supports repository-scoped permissions, the `repo` scope is also required. For the list of these registries, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages)."
  ///
  /// *Documentation*: [https://docs.github.com/rest/packages/packages#restore-package-version-for-an-organization](https://docs.github.com/rest/packages/packages#restore-package-version-for-an-organization)
  pub fn restore_package_version_for_org(
    &self,
    package_type: impl Into<PackagesRestorePackageVersionForOrgParametersPackageType>,
    package_name: impl Into<String>,
    org: impl Into<String>,
    package_version_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let package_type = package_type.into();
    let package_name = package_name.into();
    let org = org.into();
    let package_version_id = package_version_id.into();
    let package_type = package_type.to_string();
    let url = format!(
      "/orgs/{org}/packages/{package_type}/{package_name}/versions/{package_version_id}/restore"
    );

    NoContentRequest::<(), ()>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Get list of conflicting packages during Docker migration for authenticated-user**
  ///
  /// Lists all packages that are owned by the authenticated user within the user's namespace, and that encountered a conflict during a Docker migration.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `read:packages` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/packages/packages#get-list-of-conflicting-packages-during-docker-migration-for-authenticated-user](https://docs.github.com/rest/packages/packages#get-list-of-conflicting-packages-during-docker-migration-for-authenticated-user)
  pub fn list_docker_migration_conflicting_packages_for_authenticated_user(
    &self,
  ) -> Request<(), (), PackageArray> {
    let url = format!("/user/docker/conflicts");

    Request::<(), (), PackageArray>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List packages for the authenticated user's namespace**
  ///
  /// Lists packages owned by the authenticated user within the user's namespace.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `read:packages` scope to use this endpoint. If the `package_type` belongs to a GitHub Packages registry that only supports repository-scoped permissions, the `repo` scope is also required. For the list of these registries, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages)."
  ///
  /// *Documentation*: [https://docs.github.com/rest/packages/packages#list-packages-for-the-authenticated-users-namespace](https://docs.github.com/rest/packages/packages#list-packages-for-the-authenticated-users-namespace)
  pub fn list_packages_for_authenticated_user(
    &self,
  ) -> Request<(), PackagesListPackagesForAuthenticatedUserQuery, PackageArray> {
    let url = format!("/user/packages");

    Request::<(), PackagesListPackagesForAuthenticatedUserQuery, PackageArray>::builder(
      &self.config,
    )
    .get(url)
    .build()
  }

  /// **Get a package for the authenticated user**
  ///
  /// Gets a specific package for a package owned by the authenticated user.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `read:packages` scope to use this endpoint. If the `package_type` belongs to a GitHub Packages registry that only supports repository-scoped permissions, the `repo` scope is also required. For the list of these registries, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages)."
  ///
  /// *Documentation*: [https://docs.github.com/rest/packages/packages#get-a-package-for-the-authenticated-user](https://docs.github.com/rest/packages/packages#get-a-package-for-the-authenticated-user)
  pub fn get_package_for_authenticated_user(
    &self,
    package_type: impl Into<PackagesGetPackageForAuthenticatedUserParametersPackageType>,
    package_name: impl Into<String>,
  ) -> Request<(), (), Package> {
    let package_type = package_type.into();
    let package_name = package_name.into();
    let package_type = package_type.to_string();
    let url = format!("/user/packages/{package_type}/{package_name}");

    Request::<(), (), Package>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Delete a package for the authenticated user**
  ///
  /// Deletes a package owned by the authenticated user. You cannot delete a public package if any version of the package has more than 5,000 downloads. In this scenario, contact GitHub support for further assistance.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `read:packages` and `delete:packages` scopes to use this endpoint. If the `package_type` belongs to a GitHub Packages registry that only supports repository-scoped permissions, `repo` scope is also required. For the list these registries, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages)."
  ///
  /// *Documentation*: [https://docs.github.com/rest/packages/packages#delete-a-package-for-the-authenticated-user](https://docs.github.com/rest/packages/packages#delete-a-package-for-the-authenticated-user)
  pub fn delete_package_for_authenticated_user(
    &self,
    package_type: impl Into<PackagesDeletePackageForAuthenticatedUserParametersPackageType>,
    package_name: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let package_type = package_type.into();
    let package_name = package_name.into();
    let package_type = package_type.to_string();
    let url = format!("/user/packages/{package_type}/{package_name}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Restore a package for the authenticated user**
  ///
  /// Restores a package owned by the authenticated user.
  ///
  /// You can restore a deleted package under the following conditions:
  ///   - The package was deleted within the last 30 days.
  ///   - The same package namespace and version is still available and not reused for a new package. If the same package namespace is not available, you will not be able to restore your package. In this scenario, to restore the deleted package, you must delete the new package that uses the deleted package's namespace first.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `read:packages` and `write:packages` scopes to use this endpoint. If the `package_type` belongs to a GitHub Packages registry that only supports repository-scoped permissions, the `repo` scope is also required. For the list of these registries, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages)."
  ///
  /// *Documentation*: [https://docs.github.com/rest/packages/packages#restore-a-package-for-the-authenticated-user](https://docs.github.com/rest/packages/packages#restore-a-package-for-the-authenticated-user)
  pub fn restore_package_for_authenticated_user(
    &self,
    package_type: impl Into<PackagesRestorePackageForAuthenticatedUserParametersPackageType>,
    package_name: impl Into<String>,
  ) -> NoContentRequest<(), PackagesRestorePackageForAuthenticatedUserQuery> {
    let package_type = package_type.into();
    let package_name = package_name.into();
    let package_type = package_type.to_string();
    let url = format!("/user/packages/{package_type}/{package_name}/restore");

    NoContentRequest::<(), PackagesRestorePackageForAuthenticatedUserQuery>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **List package versions for a package owned by the authenticated user**
  ///
  /// Lists package versions for a package owned by the authenticated user.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `read:packages` scope to use this endpoint. If the `package_type` belongs to a GitHub Packages registry that only supports repository-scoped permissions, the `repo` scope is also required. For the list of these registries, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages)."
  ///
  /// *Documentation*: [https://docs.github.com/rest/packages/packages#list-package-versions-for-a-package-owned-by-the-authenticated-user](https://docs.github.com/rest/packages/packages#list-package-versions-for-a-package-owned-by-the-authenticated-user)
  pub fn get_all_package_versions_for_package_owned_by_authenticated_user(
    &self,
    package_type: impl Into<
      PackagesGetAllPackageVersionsForPackageOwnedByAuthenticatedUserParametersPackageType,
    >,
    package_name: impl Into<String>,
  ) -> Request<
    (),
    PackagesGetAllPackageVersionsForPackageOwnedByAuthenticatedUserQuery,
    PackageVersionArray,
  > {
    let package_type = package_type.into();
    let package_name = package_name.into();
    let package_type = package_type.to_string();
    let url = format!("/user/packages/{package_type}/{package_name}/versions");

    Request::<
      (),
      PackagesGetAllPackageVersionsForPackageOwnedByAuthenticatedUserQuery,
      PackageVersionArray,
    >::builder(&self.config)
    .get(url)
    .build()
  }

  /// **Get a package version for the authenticated user**
  ///
  /// Gets a specific package version for a package owned by the authenticated user.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `read:packages` scope to use this endpoint. If the `package_type` belongs to a GitHub Packages registry that only supports repository-scoped permissions, the `repo` scope is also required. For the list of these registries, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages)."
  ///
  /// *Documentation*: [https://docs.github.com/rest/packages/packages#get-a-package-version-for-the-authenticated-user](https://docs.github.com/rest/packages/packages#get-a-package-version-for-the-authenticated-user)
  pub fn get_package_version_for_authenticated_user(
    &self,
    package_type: impl Into<PackagesGetPackageVersionForAuthenticatedUserParametersPackageType>,
    package_name: impl Into<String>,
    package_version_id: impl Into<i64>,
  ) -> Request<(), (), PackageVersion> {
    let package_type = package_type.into();
    let package_name = package_name.into();
    let package_version_id = package_version_id.into();
    let package_type = package_type.to_string();
    let url = format!("/user/packages/{package_type}/{package_name}/versions/{package_version_id}");

    Request::<(), (), PackageVersion>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Delete a package version for the authenticated user**
  ///
  /// Deletes a specific package version for a package owned by the authenticated user.  If the package is public and the package version has more than 5,000 downloads, you cannot delete the package version. In this scenario, contact GitHub support for further assistance.
  ///
  /// The authenticated user must have admin permissions in the organization to use this endpoint.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `read:packages` and `delete:packages` scopes to use this endpoint. If the `package_type` belongs to a GitHub Packages registry that only supports repository-scoped permissions, the `repo` scope is also required. For the list of these registries, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages)."
  ///
  /// *Documentation*: [https://docs.github.com/rest/packages/packages#delete-a-package-version-for-the-authenticated-user](https://docs.github.com/rest/packages/packages#delete-a-package-version-for-the-authenticated-user)
  pub fn delete_package_version_for_authenticated_user(
    &self,
    package_type: impl Into<PackagesDeletePackageVersionForAuthenticatedUserParametersPackageType>,
    package_name: impl Into<String>,
    package_version_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let package_type = package_type.into();
    let package_name = package_name.into();
    let package_version_id = package_version_id.into();
    let package_type = package_type.to_string();
    let url = format!("/user/packages/{package_type}/{package_name}/versions/{package_version_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Restore a package version for the authenticated user**
  ///
  /// Restores a package version owned by the authenticated user.
  ///
  /// You can restore a deleted package version under the following conditions:
  ///   - The package was deleted within the last 30 days.
  ///   - The same package namespace and version is still available and not reused for a new package. If the same package namespace is not available, you will not be able to restore your package. In this scenario, to restore the deleted package, you must delete the new package that uses the deleted package's namespace first.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `read:packages` and `write:packages` scopes to use this endpoint. If the `package_type` belongs to a GitHub Packages registry that only supports repository-scoped permissions, the `repo` scope is also required. For the list of these registries, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages)."
  ///
  /// *Documentation*: [https://docs.github.com/rest/packages/packages#restore-a-package-version-for-the-authenticated-user](https://docs.github.com/rest/packages/packages#restore-a-package-version-for-the-authenticated-user)
  pub fn restore_package_version_for_authenticated_user(
    &self,
    package_type: impl Into<PackagesRestorePackageVersionForAuthenticatedUserParametersPackageType>,
    package_name: impl Into<String>,
    package_version_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let package_type = package_type.into();
    let package_name = package_name.into();
    let package_version_id = package_version_id.into();
    let package_type = package_type.to_string();
    let url =
      format!("/user/packages/{package_type}/{package_name}/versions/{package_version_id}/restore");

    NoContentRequest::<(), ()>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Get list of conflicting packages during Docker migration for user**
  ///
  /// Lists all packages that are in a specific user's namespace, that the requesting user has access to, and that encountered a conflict during Docker migration.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `read:packages` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/packages/packages#get-list-of-conflicting-packages-during-docker-migration-for-user](https://docs.github.com/rest/packages/packages#get-list-of-conflicting-packages-during-docker-migration-for-user)
  pub fn list_docker_migration_conflicting_packages_for_user(
    &self,
    username: impl Into<String>,
  ) -> Request<(), (), PackageArray> {
    let username = username.into();
    let url = format!("/users/{username}/docker/conflicts");

    Request::<(), (), PackageArray>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List packages for a user**
  ///
  /// Lists all packages in a user's namespace for which the requesting user has access.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `read:packages` scope to use this endpoint. If the `package_type` belongs to a GitHub Packages registry that only supports repository-scoped permissions, the `repo` scope is also required. For the list of these registries, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages)."
  ///
  /// *Documentation*: [https://docs.github.com/rest/packages/packages#list-packages-for-a-user](https://docs.github.com/rest/packages/packages#list-packages-for-a-user)
  pub fn list_packages_for_user(
    &self,
    username: impl Into<String>,
  ) -> Request<(), PackagesListPackagesForUserQuery, PackageArray> {
    let username = username.into();
    let url = format!("/users/{username}/packages");

    Request::<(), PackagesListPackagesForUserQuery, PackageArray>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get a package for a user**
  ///
  /// Gets a specific package metadata for a public package owned by a user.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `read:packages` scope to use this endpoint. If the `package_type` belongs to a GitHub Packages registry that only supports repository-scoped permissions, the `repo` scope is also required. For the list of these registries, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages)."
  ///
  /// *Documentation*: [https://docs.github.com/rest/packages/packages#get-a-package-for-a-user](https://docs.github.com/rest/packages/packages#get-a-package-for-a-user)
  pub fn get_package_for_user(
    &self,
    package_type: impl Into<PackagesGetPackageForUserParametersPackageType>,
    package_name: impl Into<String>,
    username: impl Into<String>,
  ) -> Request<(), (), Package> {
    let package_type = package_type.into();
    let package_name = package_name.into();
    let username = username.into();
    let package_type = package_type.to_string();
    let url = format!("/users/{username}/packages/{package_type}/{package_name}");

    Request::<(), (), Package>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Delete a package for a user**
  ///
  /// Deletes an entire package for a user. You cannot delete a public package if any version of the package has more than 5,000 downloads. In this scenario, contact GitHub support for further assistance.
  ///
  /// If the `package_type` belongs to a GitHub Packages registry that supports granular permissions, the authenticated user must have admin permissions to the package. For the list of these registries, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#granular-permissions-for-userorganization-scoped-packages)."
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `read:packages` and `delete:packages` scopes to use this endpoint. If the `package_type` belongs to a GitHub Packages registry that only supports repository-scoped permissions, the `repo` scope is also required. For the list of these registries, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages)."
  ///
  /// *Documentation*: [https://docs.github.com/rest/packages/packages#delete-a-package-for-a-user](https://docs.github.com/rest/packages/packages#delete-a-package-for-a-user)
  pub fn delete_package_for_user(
    &self,
    package_type: impl Into<PackagesDeletePackageForUserParametersPackageType>,
    package_name: impl Into<String>,
    username: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let package_type = package_type.into();
    let package_name = package_name.into();
    let username = username.into();
    let package_type = package_type.to_string();
    let url = format!("/users/{username}/packages/{package_type}/{package_name}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Restore a package for a user**
  ///
  /// Restores an entire package for a user.
  ///
  /// You can restore a deleted package under the following conditions:
  ///   - The package was deleted within the last 30 days.
  ///   - The same package namespace and version is still available and not reused for a new package. If the same package namespace is not available, you will not be able to restore your package. In this scenario, to restore the deleted package, you must delete the new package that uses the deleted package's namespace first.
  ///
  /// If the `package_type` belongs to a GitHub Packages registry that supports granular permissions, the authenticated user must have admin permissions to the package. For the list of these registries, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#granular-permissions-for-userorganization-scoped-packages)."
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `read:packages` and `write:packages` scopes to use this endpoint. If the `package_type` belongs to a GitHub Packages registry that only supports repository-scoped permissions, the `repo` scope is also required. For the list of these registries, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages)."
  ///
  /// *Documentation*: [https://docs.github.com/rest/packages/packages#restore-a-package-for-a-user](https://docs.github.com/rest/packages/packages#restore-a-package-for-a-user)
  pub fn restore_package_for_user(
    &self,
    package_type: impl Into<PackagesRestorePackageForUserParametersPackageType>,
    package_name: impl Into<String>,
    username: impl Into<String>,
  ) -> NoContentRequest<(), PackagesRestorePackageForUserQuery> {
    let package_type = package_type.into();
    let package_name = package_name.into();
    let username = username.into();
    let package_type = package_type.to_string();
    let url = format!("/users/{username}/packages/{package_type}/{package_name}/restore");

    NoContentRequest::<(), PackagesRestorePackageForUserQuery>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **List package versions for a package owned by a user**
  ///
  /// Lists package versions for a public package owned by a specified user.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `read:packages` scope to use this endpoint. If the `package_type` belongs to a GitHub Packages registry that only supports repository-scoped permissions, the `repo` scope is also required. For the list of these registries, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages)."
  ///
  /// *Documentation*: [https://docs.github.com/rest/packages/packages#list-package-versions-for-a-package-owned-by-a-user](https://docs.github.com/rest/packages/packages#list-package-versions-for-a-package-owned-by-a-user)
  pub fn get_all_package_versions_for_package_owned_by_user(
    &self,
    package_type: impl Into<PackagesGetAllPackageVersionsForPackageOwnedByUserParametersPackageType>,
    package_name: impl Into<String>,
    username: impl Into<String>,
  ) -> Request<(), (), PackageVersionArray> {
    let package_type = package_type.into();
    let package_name = package_name.into();
    let username = username.into();
    let package_type = package_type.to_string();
    let url = format!("/users/{username}/packages/{package_type}/{package_name}/versions");

    Request::<(), (), PackageVersionArray>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get a package version for a user**
  ///
  /// Gets a specific package version for a public package owned by a specified user.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `read:packages` scope to use this endpoint. If the `package_type` belongs to a GitHub Packages registry that only supports repository-scoped permissions, the `repo` scope is also required. For the list of these registries, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages)."
  ///
  /// *Documentation*: [https://docs.github.com/rest/packages/packages#get-a-package-version-for-a-user](https://docs.github.com/rest/packages/packages#get-a-package-version-for-a-user)
  pub fn get_package_version_for_user(
    &self,
    package_type: impl Into<PackagesGetPackageVersionForUserParametersPackageType>,
    package_name: impl Into<String>,
    package_version_id: impl Into<i64>,
    username: impl Into<String>,
  ) -> Request<(), (), PackageVersion> {
    let package_type = package_type.into();
    let package_name = package_name.into();
    let package_version_id = package_version_id.into();
    let username = username.into();
    let package_type = package_type.to_string();
    let url = format!(
      "/users/{username}/packages/{package_type}/{package_name}/versions/{package_version_id}"
    );

    Request::<(), (), PackageVersion>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Delete package version for a user**
  ///
  /// Deletes a specific package version for a user. If the package is public and the package version has more than 5,000 downloads, you cannot delete the package version. In this scenario, contact GitHub support for further assistance.
  ///
  /// If the `package_type` belongs to a GitHub Packages registry that supports granular permissions, the authenticated user must have admin permissions to the package. For the list of these registries, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#granular-permissions-for-userorganization-scoped-packages)."
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `read:packages` and `delete:packages` scopes to use this endpoint. If the `package_type` belongs to a GitHub Packages registry that only supports repository-scoped permissions, the `repo` scope is also required. For the list of these registries, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages)."
  ///
  /// *Documentation*: [https://docs.github.com/rest/packages/packages#delete-package-version-for-a-user](https://docs.github.com/rest/packages/packages#delete-package-version-for-a-user)
  pub fn delete_package_version_for_user(
    &self,
    package_type: impl Into<PackagesDeletePackageVersionForUserParametersPackageType>,
    package_name: impl Into<String>,
    username: impl Into<String>,
    package_version_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let package_type = package_type.into();
    let package_name = package_name.into();
    let username = username.into();
    let package_version_id = package_version_id.into();
    let package_type = package_type.to_string();
    let url = format!(
      "/users/{username}/packages/{package_type}/{package_name}/versions/{package_version_id}"
    );

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Restore package version for a user**
  ///
  /// Restores a specific package version for a user.
  ///
  /// You can restore a deleted package under the following conditions:
  ///   - The package was deleted within the last 30 days.
  ///   - The same package namespace and version is still available and not reused for a new package. If the same package namespace is not available, you will not be able to restore your package. In this scenario, to restore the deleted package, you must delete the new package that uses the deleted package's namespace first.
  ///
  /// If the `package_type` belongs to a GitHub Packages registry that supports granular permissions, the authenticated user must have admin permissions to the package. For the list of these registries, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#granular-permissions-for-userorganization-scoped-packages)."
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `read:packages` and `write:packages` scopes to use this endpoint. If the `package_type` belongs to a GitHub Packages registry that only supports repository-scoped permissions, the `repo` scope is also required. For the list of these registries, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages)."
  ///
  /// *Documentation*: [https://docs.github.com/rest/packages/packages#restore-package-version-for-a-user](https://docs.github.com/rest/packages/packages#restore-package-version-for-a-user)
  pub fn restore_package_version_for_user(
    &self,
    package_type: impl Into<PackagesRestorePackageVersionForUserParametersPackageType>,
    package_name: impl Into<String>,
    username: impl Into<String>,
    package_version_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let package_type = package_type.into();
    let package_name = package_name.into();
    let username = username.into();
    let package_version_id = package_version_id.into();
    let package_type = package_type.to_string();
    let url = format!("/users/{username}/packages/{package_type}/{package_name}/versions/{package_version_id}/restore");

    NoContentRequest::<(), ()>::builder(&self.config)
      .post(url)
      .build()
  }
}
