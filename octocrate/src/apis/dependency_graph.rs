use octocrate_core::*;
#[allow(unused_imports)]
use octocrate_types::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use typed_builder::TypedBuilder;

pub mod diff_range {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = DependencyGraphDiff;

  /// Query for `Get a diff of the dependencies between commits`
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// The full path, relative to the repository root, of the dependency manifest file.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub name: Option<String>,
  }
}

pub mod export_sbom {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = DependencyGraphSpdxSbom;
}

pub mod create_repository_snapshot {
  #[allow(unused_imports)]
  use super::*;

  pub type Request = Snapshot;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Response {
    /// The time at which the snapshot was created.
    pub created_at: String,
    /// ID of the created snapshot.
    pub id: i64,
    /// A message providing further details about the result, such as why the dependencies were not updated.
    pub message: String,
    /// Either "SUCCESS", "ACCEPTED", or "INVALID". "SUCCESS" indicates that the snapshot was successfully created and the repository's dependencies were updated. "ACCEPTED" indicates that the snapshot was successfully created, but the repository's dependencies were not updated. "INVALID" indicates that the snapshot was malformed.
    pub result: String,
  }
}

pub struct GitHubDependencyGraphAPI {
  config: SharedAPIConfig,
}

impl GitHubDependencyGraphAPI {
  pub fn new(config: &SharedAPIConfig) -> Self {
    Self {
      config: config.clone(),
    }
  }

  /// **Get a diff of the dependencies between commits**
  ///
  /// Gets the diff of the dependency changes between two commits of a repository, based on the changes to the dependency manifests made in those commits.
  ///
  /// *Documentation*: [https://docs.github.com/rest/dependency-graph/dependency-review#get-a-diff-of-the-dependencies-between-commits](https://docs.github.com/rest/dependency-graph/dependency-review#get-a-diff-of-the-dependencies-between-commits)
  pub fn diff_range(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    basehead: impl Into<String>,
  ) -> Request<(), diff_range::Query, diff_range::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let basehead = basehead.into();
    let url = format!("/repos/{owner}/{repo}/dependency-graph/compare/{basehead}");

    Request::<(), diff_range::Query, diff_range::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Export a software bill of materials (SBOM) for a repository.**
  ///
  /// Exports the software bill of materials (SBOM) for a repository in SPDX JSON format.
  ///
  /// *Documentation*: [https://docs.github.com/rest/dependency-graph/sboms#export-a-software-bill-of-materials-sbom-for-a-repository](https://docs.github.com/rest/dependency-graph/sboms#export-a-software-bill-of-materials-sbom-for-a-repository)
  pub fn export_sbom(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), (), export_sbom::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/dependency-graph/sbom");

    Request::<(), (), export_sbom::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create a snapshot of dependencies for a repository**
  ///
  /// Create a new snapshot of a repository's dependencies.
  ///
  /// The authenticated user must have access to the repository.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/dependency-graph/dependency-submission#create-a-snapshot-of-dependencies-for-a-repository](https://docs.github.com/rest/dependency-graph/dependency-submission#create-a-snapshot-of-dependencies-for-a-repository)
  pub fn create_repository_snapshot(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<create_repository_snapshot::Request, (), create_repository_snapshot::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/dependency-graph/snapshots");

    Request::<create_repository_snapshot::Request, (), create_repository_snapshot::Response>::builder(&self.config)
      .post(url)
      .build()
  }
}
