use octocrate_core::*;
#[allow(unused_imports)]
use octocrate_types::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use typed_builder::TypedBuilder;

pub mod list_docker_migration_conflicting_packages_for_organization {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Package>;
}

pub mod list_packages_for_organization {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Package>;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The type of supported package. Packages in GitHub's Gradle registry have the type `maven`. Docker images pushed to GitHub's Container registry (`ghcr.io`) have the type `container`. You can use the type `docker` to find images that were pushed to GitHub's Docker registry (`docker.pkg.github.com`), even if these have now been migrated to the Container registry.
    pub package_type: PackageType,
    /// The selected visibility of the packages.  This parameter is optional and only filters an existing result set.
    ///
    /// The `internal` visibility is only supported for GitHub Packages registries that allow for granular permissions. For other ecosystems `internal` is synonymous with `private`.
    /// For the list of GitHub Packages registries that support granular permissions, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#granular-permissions-for-userorganization-scoped-packages)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub visibility: Option<parameters::PackageVisibility>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
  }
}

pub mod get_package_for_organization {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Package;
}

pub mod restore_package_for_org {
  #[allow(unused_imports)]
  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// package token
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub token: Option<String>,
  }
}

pub mod get_all_package_versions_for_package_owned_by_org {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<PackageVersion>;

  #[allow(clippy::large_enum_variant)]
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QueryState {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "deleted")]
    Deleted,
  }

  impl std::fmt::Display for QueryState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match self {
        QueryState::Active => write!(f, "active"),
        QueryState::Deleted => write!(f, "deleted"),
      }
    }
  }

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
    /// The state of the package, either active or deleted.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub state: Option<QueryState>,
  }
}

pub mod get_package_version_for_organization {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = PackageVersion;
}

pub mod list_docker_migration_conflicting_packages_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Package>;
}

pub mod list_packages_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Package>;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The type of supported package. Packages in GitHub's Gradle registry have the type `maven`. Docker images pushed to GitHub's Container registry (`ghcr.io`) have the type `container`. You can use the type `docker` to find images that were pushed to GitHub's Docker registry (`docker.pkg.github.com`), even if these have now been migrated to the Container registry.
    pub package_type: PackageType,
    /// The selected visibility of the packages.  This parameter is optional and only filters an existing result set.
    ///
    /// The `internal` visibility is only supported for GitHub Packages registries that allow for granular permissions. For other ecosystems `internal` is synonymous with `private`.
    /// For the list of GitHub Packages registries that support granular permissions, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#granular-permissions-for-userorganization-scoped-packages)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub visibility: Option<parameters::PackageVisibility>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
  }
}

pub mod get_package_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Package;
}

pub mod restore_package_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// package token
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub token: Option<String>,
  }
}

pub mod get_all_package_versions_for_package_owned_by_authenticated_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<PackageVersion>;

  #[allow(clippy::large_enum_variant)]
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QueryState {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "deleted")]
    Deleted,
  }

  impl std::fmt::Display for QueryState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match self {
        QueryState::Active => write!(f, "active"),
        QueryState::Deleted => write!(f, "deleted"),
      }
    }
  }

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
    /// The state of the package, either active or deleted.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub state: Option<QueryState>,
  }
}

pub mod get_package_version_for_authenticated_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = PackageVersion;
}

pub mod list_docker_migration_conflicting_packages_for_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Package>;
}

pub mod list_packages_for_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Package>;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The type of supported package. Packages in GitHub's Gradle registry have the type `maven`. Docker images pushed to GitHub's Container registry (`ghcr.io`) have the type `container`. You can use the type `docker` to find images that were pushed to GitHub's Docker registry (`docker.pkg.github.com`), even if these have now been migrated to the Container registry.
    pub package_type: PackageType,
    /// The selected visibility of the packages.  This parameter is optional and only filters an existing result set.
    ///
    /// The `internal` visibility is only supported for GitHub Packages registries that allow for granular permissions. For other ecosystems `internal` is synonymous with `private`.
    /// For the list of GitHub Packages registries that support granular permissions, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#granular-permissions-for-userorganization-scoped-packages)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub visibility: Option<parameters::PackageVisibility>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
  }
}

pub mod get_package_for_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Package;
}

pub mod restore_package_for_user {
  #[allow(unused_imports)]
  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// package token
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub token: Option<String>,
  }
}

pub mod get_all_package_versions_for_package_owned_by_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<PackageVersion>;
}

pub mod get_package_version_for_user {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = PackageVersion;
}

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
  ) -> Request<(), (), list_docker_migration_conflicting_packages_for_organization::Response> {
    let org = org.into();
    let url = format!("/orgs/{org}/docker/conflicts");

    Request::<(), (), list_docker_migration_conflicting_packages_for_organization::Response>::builder(&self.config)
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
  ) -> Request<(), list_packages_for_organization::Query, list_packages_for_organization::Response>
  {
    let org = org.into();
    let url = format!("/orgs/{org}/packages");

    Request::<(), list_packages_for_organization::Query, list_packages_for_organization::Response>::builder(&self.config)
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
    package_type: impl Into<PackageType>,
    package_name: impl Into<String>,
    org: impl Into<String>,
  ) -> Request<(), (), get_package_for_organization::Response> {
    let package_type = package_type.into();
    let package_name = package_name.into();
    let org = org.into();
    let url = format!("/orgs/{org}/packages/{package_type}/{package_name}");

    Request::<(), (), get_package_for_organization::Response>::builder(&self.config)
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
    package_type: impl Into<PackageType>,
    package_name: impl Into<String>,
    org: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let package_type = package_type.into();
    let package_name = package_name.into();
    let org = org.into();
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
    package_type: impl Into<PackageType>,
    package_name: impl Into<String>,
    org: impl Into<String>,
  ) -> NoContentRequest<(), restore_package_for_org::Query> {
    let package_type = package_type.into();
    let package_name = package_name.into();
    let org = org.into();
    let url = format!("/orgs/{org}/packages/{package_type}/{package_name}/restore");

    NoContentRequest::<(), restore_package_for_org::Query>::builder(&self.config)
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
    package_type: impl Into<PackageType>,
    package_name: impl Into<String>,
    org: impl Into<String>,
  ) -> Request<
    (),
    get_all_package_versions_for_package_owned_by_org::Query,
    get_all_package_versions_for_package_owned_by_org::Response,
  > {
    let package_type = package_type.into();
    let package_name = package_name.into();
    let org = org.into();
    let url = format!("/orgs/{org}/packages/{package_type}/{package_name}/versions");

    Request::<
      (),
      get_all_package_versions_for_package_owned_by_org::Query,
      get_all_package_versions_for_package_owned_by_org::Response,
    >::builder(&self.config)
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
    package_type: impl Into<PackageType>,
    package_name: impl Into<String>,
    org: impl Into<String>,
    package_version_id: impl Into<i64>,
  ) -> Request<(), (), get_package_version_for_organization::Response> {
    let package_type = package_type.into();
    let package_name = package_name.into();
    let org = org.into();
    let package_version_id = package_version_id.into();
    let url =
      format!("/orgs/{org}/packages/{package_type}/{package_name}/versions/{package_version_id}");

    Request::<(), (), get_package_version_for_organization::Response>::builder(&self.config)
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
    package_type: impl Into<PackageType>,
    package_name: impl Into<String>,
    org: impl Into<String>,
    package_version_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let package_type = package_type.into();
    let package_name = package_name.into();
    let org = org.into();
    let package_version_id = package_version_id.into();
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
    package_type: impl Into<PackageType>,
    package_name: impl Into<String>,
    org: impl Into<String>,
    package_version_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let package_type = package_type.into();
    let package_name = package_name.into();
    let org = org.into();
    let package_version_id = package_version_id.into();
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
  ) -> Request<(), (), list_docker_migration_conflicting_packages_for_authenticated_user::Response>
  {
    let url = format!("/user/docker/conflicts");

    Request::<(), (), list_docker_migration_conflicting_packages_for_authenticated_user::Response>::builder(&self.config)
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
  ) -> Request<
    (),
    list_packages_for_authenticated_user::Query,
    list_packages_for_authenticated_user::Response,
  > {
    let url = format!("/user/packages");

    Request::<
      (),
      list_packages_for_authenticated_user::Query,
      list_packages_for_authenticated_user::Response,
    >::builder(&self.config)
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
    package_type: impl Into<PackageType>,
    package_name: impl Into<String>,
  ) -> Request<(), (), get_package_for_authenticated_user::Response> {
    let package_type = package_type.into();
    let package_name = package_name.into();
    let url = format!("/user/packages/{package_type}/{package_name}");

    Request::<(), (), get_package_for_authenticated_user::Response>::builder(&self.config)
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
    package_type: impl Into<PackageType>,
    package_name: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let package_type = package_type.into();
    let package_name = package_name.into();
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
    package_type: impl Into<PackageType>,
    package_name: impl Into<String>,
  ) -> NoContentRequest<(), restore_package_for_authenticated_user::Query> {
    let package_type = package_type.into();
    let package_name = package_name.into();
    let url = format!("/user/packages/{package_type}/{package_name}/restore");

    NoContentRequest::<(), restore_package_for_authenticated_user::Query>::builder(&self.config)
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
    package_type: impl Into<PackageType>,
    package_name: impl Into<String>,
  ) -> Request<
    (),
    get_all_package_versions_for_package_owned_by_authenticated_user::Query,
    get_all_package_versions_for_package_owned_by_authenticated_user::Response,
  > {
    let package_type = package_type.into();
    let package_name = package_name.into();
    let url = format!("/user/packages/{package_type}/{package_name}/versions");

    Request::<
      (),
      get_all_package_versions_for_package_owned_by_authenticated_user::Query,
      get_all_package_versions_for_package_owned_by_authenticated_user::Response,
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
    package_type: impl Into<PackageType>,
    package_name: impl Into<String>,
    package_version_id: impl Into<i64>,
  ) -> Request<(), (), get_package_version_for_authenticated_user::Response> {
    let package_type = package_type.into();
    let package_name = package_name.into();
    let package_version_id = package_version_id.into();
    let url = format!("/user/packages/{package_type}/{package_name}/versions/{package_version_id}");

    Request::<(), (), get_package_version_for_authenticated_user::Response>::builder(&self.config)
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
    package_type: impl Into<PackageType>,
    package_name: impl Into<String>,
    package_version_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let package_type = package_type.into();
    let package_name = package_name.into();
    let package_version_id = package_version_id.into();
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
    package_type: impl Into<PackageType>,
    package_name: impl Into<String>,
    package_version_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let package_type = package_type.into();
    let package_name = package_name.into();
    let package_version_id = package_version_id.into();
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
  ) -> Request<(), (), list_docker_migration_conflicting_packages_for_user::Response> {
    let username = username.into();
    let url = format!("/users/{username}/docker/conflicts");

    Request::<(), (), list_docker_migration_conflicting_packages_for_user::Response>::builder(
      &self.config,
    )
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
  ) -> Request<(), list_packages_for_user::Query, list_packages_for_user::Response> {
    let username = username.into();
    let url = format!("/users/{username}/packages");

    Request::<(), list_packages_for_user::Query, list_packages_for_user::Response>::builder(
      &self.config,
    )
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
    package_type: impl Into<PackageType>,
    package_name: impl Into<String>,
    username: impl Into<String>,
  ) -> Request<(), (), get_package_for_user::Response> {
    let package_type = package_type.into();
    let package_name = package_name.into();
    let username = username.into();
    let url = format!("/users/{username}/packages/{package_type}/{package_name}");

    Request::<(), (), get_package_for_user::Response>::builder(&self.config)
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
    package_type: impl Into<PackageType>,
    package_name: impl Into<String>,
    username: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let package_type = package_type.into();
    let package_name = package_name.into();
    let username = username.into();
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
    package_type: impl Into<PackageType>,
    package_name: impl Into<String>,
    username: impl Into<String>,
  ) -> NoContentRequest<(), restore_package_for_user::Query> {
    let package_type = package_type.into();
    let package_name = package_name.into();
    let username = username.into();
    let url = format!("/users/{username}/packages/{package_type}/{package_name}/restore");

    NoContentRequest::<(), restore_package_for_user::Query>::builder(&self.config)
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
    package_type: impl Into<PackageType>,
    package_name: impl Into<String>,
    username: impl Into<String>,
  ) -> Request<(), (), get_all_package_versions_for_package_owned_by_user::Response> {
    let package_type = package_type.into();
    let package_name = package_name.into();
    let username = username.into();
    let url = format!("/users/{username}/packages/{package_type}/{package_name}/versions");

    Request::<(), (), get_all_package_versions_for_package_owned_by_user::Response>::builder(
      &self.config,
    )
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
    package_type: impl Into<PackageType>,
    package_name: impl Into<String>,
    package_version_id: impl Into<i64>,
    username: impl Into<String>,
  ) -> Request<(), (), get_package_version_for_user::Response> {
    let package_type = package_type.into();
    let package_name = package_name.into();
    let package_version_id = package_version_id.into();
    let username = username.into();
    let url = format!(
      "/users/{username}/packages/{package_type}/{package_name}/versions/{package_version_id}"
    );

    Request::<(), (), get_package_version_for_user::Response>::builder(&self.config)
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
    package_type: impl Into<PackageType>,
    package_name: impl Into<String>,
    username: impl Into<String>,
    package_version_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let package_type = package_type.into();
    let package_name = package_name.into();
    let username = username.into();
    let package_version_id = package_version_id.into();
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
    package_type: impl Into<PackageType>,
    package_name: impl Into<String>,
    username: impl Into<String>,
    package_version_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let package_type = package_type.into();
    let package_name = package_name.into();
    let username = username.into();
    let package_version_id = package_version_id.into();
    let url = format!("/users/{username}/packages/{package_type}/{package_name}/versions/{package_version_id}/restore");

    NoContentRequest::<(), ()>::builder(&self.config)
      .post(url)
      .build()
  }
}
