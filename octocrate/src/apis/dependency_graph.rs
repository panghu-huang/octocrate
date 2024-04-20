use octocrate_core::*;
#[allow(unused_imports)]
use octocrate_types::*;

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
  ) -> Request<(), DependencyGraphDiffRangeQuery, DependencyGraphDiff> {
    let owner = owner.into();
    let repo = repo.into();
    let basehead = basehead.into();
    let url = format!("/repos/{owner}/{repo}/dependency-graph/compare/{basehead}");

    Request::<(), DependencyGraphDiffRangeQuery, DependencyGraphDiff>::builder(&self.config)
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
  ) -> Request<(), (), DependencyGraphSpdxSbom> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/dependency-graph/sbom");

    Request::<(), (), DependencyGraphSpdxSbom>::builder(&self.config)
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
  ) -> Request<Snapshot, (), DependencyGraphCreateRepositorySnapshotResponse> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/dependency-graph/snapshots");

    Request::<Snapshot, (), DependencyGraphCreateRepositorySnapshotResponse>::builder(&self.config)
      .post(url)
      .build()
  }
}
