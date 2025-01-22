use octocrate_core::*;
#[allow(unused_imports)]
use octocrate_types::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use typed_builder::TypedBuilder;

pub mod create_blob {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = ShortBlob;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The new blob's content.
    pub content: String,
    /// The encoding used for `content`. Currently, `"utf-8"` and `"base64"` are supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub encoding: Option<String>,
  }
}

pub mod get_blob {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Blob;
}

pub mod create_commit {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = GitCommit;

  /// Information about the author of the commit. By default, the `author` will be the authenticated user and the current date. See the `author` and `committer` object below for details.
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct RequestAuthor {
    /// Indicates when this commit was authored (or committed). This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub date: Option<String>,
    /// The email of the author (or committer) of the commit
    pub email: String,
    /// The name of the author (or committer) of the commit
    pub name: String,
  }

  /// Information about the person who is making the commit. By default, `committer` will use the information set in `author`. See the `author` and `committer` object below for details.
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct RequestCommitter {
    /// Indicates when this commit was authored (or committed). This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub date: Option<String>,
    /// The email of the author (or committer) of the commit
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub email: Option<String>,
    /// The name of the author (or committer) of the commit
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub name: Option<String>,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// Information about the author of the commit. By default, the `author` will be the authenticated user and the current date. See the `author` and `committer` object below for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub author: Option<RequestAuthor>,
    /// Information about the person who is making the commit. By default, `committer` will use the information set in `author`. See the `author` and `committer` object below for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub committer: Option<RequestCommitter>,
    /// The commit message
    pub message: String,
    /// The SHAs of the commits that were the parents of this commit. If omitted or empty, the commit will be written as a root commit. For a single parent, an array of one SHA should be provided; for a merge commit, an array of more than one should be provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub parents: Option<Vec<String>>,
    /// The [PGP signature](https://en.wikipedia.org/wiki/Pretty_Good_Privacy) of the commit. GitHub adds the signature to the `gpgsig` header of the created commit. For a commit signature to be verifiable by Git or GitHub, it must be an ASCII-armored detached PGP signature over the string commit as it would be written to the object database. To pass a `signature` parameter, you need to first manually create a valid PGP signature, which can be complicated. You may find it easier to [use the command line](https://git-scm.com/book/id/v2/Git-Tools-Signing-Your-Work) to create signed commits.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub signature: Option<String>,
    /// The SHA of the tree object this commit points to
    pub tree: String,
  }
}

pub mod get_commit {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = GitCommit;
}

pub mod list_matching_refs {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<GitRef>;
}

pub mod get_ref {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = GitRef;
}

pub mod create_ref {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = GitRef;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The name of the fully qualified reference (ie: `refs/heads/master`). If it doesn't start with 'refs' and have at least two slashes, it will be rejected.
    #[serde(rename = "ref")]
    pub ref_: String,
    /// The SHA1 value for this reference.
    pub sha: String,
  }
}

pub mod update_ref {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = GitRef;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// Indicates whether to force the update or to make sure the update is a fast-forward update. Leaving this out or setting it to `false` will make sure you're not overwriting work.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub force: Option<bool>,
    /// The SHA1 value to set this reference to
    pub sha: String,
  }
}

pub mod create_tag {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = GitTag;

  #[allow(clippy::large_enum_variant)]
  /// The type of the object we're tagging. Normally this is a `commit` but it can also be a `tree` or a `blob`.
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum RequestType {
    #[serde(rename = "commit")]
    Commit,
    #[serde(rename = "tree")]
    Tree,
    #[serde(rename = "blob")]
    Blob,
  }

  impl std::fmt::Display for RequestType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match self {
        RequestType::Commit => write!(f, "commit"),
        RequestType::Tree => write!(f, "tree"),
        RequestType::Blob => write!(f, "blob"),
      }
    }
  }

  /// An object with information about the individual creating the tag.
  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct RequestTagger {
    /// When this object was tagged. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub date: Option<String>,
    /// The email of the author of the tag
    pub email: String,
    /// The name of the author of the tag
    pub name: String,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The tag message.
    pub message: String,
    /// The SHA of the git object this is tagging.
    pub object: String,
    /// The tag's name. This is typically a version (e.g., "v0.0.1").
    pub tag: String,
    /// An object with information about the individual creating the tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub tagger: Option<RequestTagger>,
    /// The type of the object we're tagging. Normally this is a `commit` but it can also be a `tree` or a `blob`.
    #[serde(rename = "type")]
    pub type_: RequestType,
  }
}

pub mod get_tag {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = GitTag;
}

pub mod create_tree {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = GitTree;

  #[allow(clippy::large_enum_variant)]
  /// The file mode; one of `100644` for file (blob), `100755` for executable (blob), `040000` for subdirectory (tree), `160000` for submodule (commit), or `120000` for a blob that specifies the path of a symlink.
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum RequestTreeMode {
    #[serde(rename = "100644")]
    _100644,
    #[serde(rename = "100755")]
    _100755,
    #[serde(rename = "040000")]
    _040000,
    #[serde(rename = "160000")]
    _160000,
    #[serde(rename = "120000")]
    _120000,
  }

  impl std::fmt::Display for RequestTreeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match self {
        RequestTreeMode::_100644 => write!(f, "100644"),
        RequestTreeMode::_100755 => write!(f, "100755"),
        RequestTreeMode::_040000 => write!(f, "040000"),
        RequestTreeMode::_160000 => write!(f, "160000"),
        RequestTreeMode::_120000 => write!(f, "120000"),
      }
    }
  }

  #[allow(clippy::large_enum_variant)]
  /// Either `blob`, `tree`, or `commit`.
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum RequestTreeType {
    #[serde(rename = "blob")]
    Blob,
    #[serde(rename = "tree")]
    Tree,
    #[serde(rename = "commit")]
    Commit,
  }

  impl std::fmt::Display for RequestTreeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match self {
        RequestTreeType::Blob => write!(f, "blob"),
        RequestTreeType::Tree => write!(f, "tree"),
        RequestTreeType::Commit => write!(f, "commit"),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct RequestTree {
    /// The content you want this file to have. GitHub will write this blob out and use that SHA for this entry. Use either this, or `tree.sha`.  
    ///   
    /// **Note:** Use either `tree.sha` or `content` to specify the contents of the entry. Using both `tree.sha` and `content` will return an error.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub content: Option<String>,
    /// The file mode; one of `100644` for file (blob), `100755` for executable (blob), `040000` for subdirectory (tree), `160000` for submodule (commit), or `120000` for a blob that specifies the path of a symlink.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub mode: Option<RequestTreeMode>,
    /// The file referenced in the tree.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub path: Option<String>,
    /// The SHA1 checksum ID of the object in the tree. Also called `tree.sha`. If the value is `null` then the file will be deleted.  
    ///   
    /// **Note:** Use either `tree.sha` or `content` to specify the contents of the entry. Using both `tree.sha` and `content` will return an error.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub sha: Option<String>,
    /// Either `blob`, `tree`, or `commit`.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub type_: Option<RequestTreeType>,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The SHA1 of an existing Git tree object which will be used as the base for the new tree. If provided, a new Git tree object will be created from entries in the Git tree object pointed to by `base_tree` and entries defined in the `tree` parameter. Entries defined in the `tree` parameter will overwrite items from `base_tree` with the same `path`. If you're creating new changes on a branch, then normally you'd set `base_tree` to the SHA1 of the Git tree object of the current latest commit on the branch you're working on.
    /// If not provided, GitHub will create a new Git tree object from only the entries defined in the `tree` parameter. If you create a new commit pointing to such a tree, then all files which were a part of the parent commit's tree and were not defined in the `tree` parameter will be listed as deleted by the new commit.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub base_tree: Option<String>,
    /// Objects (of `path`, `mode`, `type`, and `sha`) specifying a tree structure.
    pub tree: Vec<RequestTree>,
  }
}

pub mod get_tree {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = GitTree;

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// Setting this parameter to any value returns the objects or subtrees referenced by the tree specified in `:tree_sha`. For example, setting `recursive` to any of the following will enable returning objects or subtrees: `0`, `1`, `"true"`, and `"false"`. Omit this parameter to prevent recursively returning objects or subtrees.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub recursive: Option<String>,
  }
}

/// Raw Git functionality.
pub struct GitHubGitAPI {
  config: SharedAPIConfig,
}

impl GitHubGitAPI {
  pub fn new(config: &SharedAPIConfig) -> Self {
    Self {
      config: config.clone(),
    }
  }

  /// **Create a blob**
  ///
  ///
  /// *Documentation*: [https://docs.github.com/rest/git/blobs#create-a-blob](https://docs.github.com/rest/git/blobs#create-a-blob)
  pub fn create_blob(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<create_blob::Request, (), create_blob::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/git/blobs");

    Request::<create_blob::Request, (), create_blob::Response>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Get a blob**
  ///
  /// The `content` in the response will always be Base64 encoded.
  ///
  /// This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
  ///
  /// - **`application/vnd.github.raw+json`**: Returns the raw blob data.
  /// - **`application/vnd.github+json`**: Returns a JSON representation of the blob with `content` as a base64 encoded string. This is the default if no media type is specified.
  ///
  /// **Note** This endpoint supports blobs up to 100 megabytes in size.
  ///
  /// *Documentation*: [https://docs.github.com/rest/git/blobs#get-a-blob](https://docs.github.com/rest/git/blobs#get-a-blob)
  pub fn get_blob(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    file_sha: impl Into<String>,
  ) -> Request<(), (), get_blob::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let file_sha = file_sha.into();
    let url = format!("/repos/{owner}/{repo}/git/blobs/{file_sha}");

    Request::<(), (), get_blob::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create a commit**
  ///
  /// Creates a new Git [commit object](https://git-scm.com/book/en/v2/Git-Internals-Git-Objects).
  ///
  /// **Signature verification object**
  ///
  /// The response will include a `verification` object that describes the result of verifying the commit's signature. The following fields are included in the `verification` object:
  ///
  /// | Name | Type | Description |
  /// | ---- | ---- | ----------- |
  /// | `verified` | `boolean` | Indicates whether GitHub considers the signature in this commit to be verified. |
  /// | `reason` | `string` | The reason for verified value. Possible values and their meanings are enumerated in the table below. |
  /// | `signature` | `string` | The signature that was extracted from the commit. |
  /// | `payload` | `string` | The value that was signed. |
  ///
  /// These are the possible values for `reason` in the `verification` object:
  ///
  /// | Value | Description |
  /// | ----- | ----------- |
  /// | `expired_key` | The key that made the signature is expired. |
  /// | `not_signing_key` | The "signing" flag is not among the usage flags in the GPG key that made the signature. |
  /// | `gpgverify_error` | There was an error communicating with the signature verification service. |
  /// | `gpgverify_unavailable` | The signature verification service is currently unavailable. |
  /// | `unsigned` | The object does not include a signature. |
  /// | `unknown_signature_type` | A non-PGP signature was found in the commit. |
  /// | `no_user` | No user was associated with the `committer` email address in the commit. |
  /// | `unverified_email` | The `committer` email address in the commit was associated with a user, but the email address is not verified on their account. |
  /// | `bad_email` | The `committer` email address in the commit is not included in the identities of the PGP key that made the signature. |
  /// | `unknown_key` | The key that made the signature has not been registered with any user's account. |
  /// | `malformed_signature` | There was an error parsing the signature. |
  /// | `invalid` | The signature could not be cryptographically verified using the key whose key-id was found in the signature. |
  /// | `valid` | None of the above errors applied, so the signature is considered to be verified. |
  ///
  /// *Documentation*: [https://docs.github.com/rest/git/commits#create-a-commit](https://docs.github.com/rest/git/commits#create-a-commit)
  pub fn create_commit(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<create_commit::Request, (), create_commit::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/git/commits");

    Request::<create_commit::Request, (), create_commit::Response>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Get a commit object**
  ///
  /// Gets a Git [commit object](https://git-scm.com/book/en/v2/Git-Internals-Git-Objects).
  ///
  /// To get the contents of a commit, see "[Get a commit](/rest/commits/commits#get-a-commit)."
  ///
  /// **Signature verification object**
  ///
  /// The response will include a `verification` object that describes the result of verifying the commit's signature. The following fields are included in the `verification` object:
  ///
  /// | Name | Type | Description |
  /// | ---- | ---- | ----------- |
  /// | `verified` | `boolean` | Indicates whether GitHub considers the signature in this commit to be verified. |
  /// | `reason` | `string` | The reason for verified value. Possible values and their meanings are enumerated in the table below. |
  /// | `signature` | `string` | The signature that was extracted from the commit. |
  /// | `payload` | `string` | The value that was signed. |
  ///
  /// These are the possible values for `reason` in the `verification` object:
  ///
  /// | Value | Description |
  /// | ----- | ----------- |
  /// | `expired_key` | The key that made the signature is expired. |
  /// | `not_signing_key` | The "signing" flag is not among the usage flags in the GPG key that made the signature. |
  /// | `gpgverify_error` | There was an error communicating with the signature verification service. |
  /// | `gpgverify_unavailable` | The signature verification service is currently unavailable. |
  /// | `unsigned` | The object does not include a signature. |
  /// | `unknown_signature_type` | A non-PGP signature was found in the commit. |
  /// | `no_user` | No user was associated with the `committer` email address in the commit. |
  /// | `unverified_email` | The `committer` email address in the commit was associated with a user, but the email address is not verified on their account. |
  /// | `bad_email` | The `committer` email address in the commit is not included in the identities of the PGP key that made the signature. |
  /// | `unknown_key` | The key that made the signature has not been registered with any user's account. |
  /// | `malformed_signature` | There was an error parsing the signature. |
  /// | `invalid` | The signature could not be cryptographically verified using the key whose key-id was found in the signature. |
  /// | `valid` | None of the above errors applied, so the signature is considered to be verified. |
  ///
  /// *Documentation*: [https://docs.github.com/rest/git/commits#get-a-commit-object](https://docs.github.com/rest/git/commits#get-a-commit-object)
  pub fn get_commit(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    commit_sha: impl Into<String>,
  ) -> Request<(), (), get_commit::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let commit_sha = commit_sha.into();
    let url = format!("/repos/{owner}/{repo}/git/commits/{commit_sha}");

    Request::<(), (), get_commit::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List matching references**
  ///
  /// Returns an array of references from your Git database that match the supplied name. The `:ref` in the URL must be formatted as `heads/<branch name>` for branches and `tags/<tag name>` for tags. If the `:ref` doesn't exist in the repository, but existing refs start with `:ref`, they will be returned as an array.
  ///
  /// When you use this endpoint without providing a `:ref`, it will return an array of all the references from your Git database, including notes and stashes if they exist on the server. Anything in the namespace is returned, not just `heads` and `tags`.
  ///
  /// **Note:** You need to explicitly [request a pull request](https://docs.github.com/rest/pulls/pulls#get-a-pull-request) to trigger a test merge commit, which checks the mergeability of pull requests. For more information, see "[Checking mergeability of pull requests](https://docs.github.com/rest/guides/getting-started-with-the-git-database-api#checking-mergeability-of-pull-requests)".
  ///
  /// If you request matching references for a branch named `feature` but the branch `feature` doesn't exist, the response can still include other matching head refs that start with the word `feature`, such as `featureA` and `featureB`.
  ///
  /// *Documentation*: [https://docs.github.com/rest/git/refs#list-matching-references](https://docs.github.com/rest/git/refs#list-matching-references)
  pub fn list_matching_refs(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    ref_: impl Into<String>,
  ) -> Request<(), (), list_matching_refs::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let ref_ = ref_.into();
    let url = format!("/repos/{owner}/{repo}/git/matching-refs/{ref_}");

    Request::<(), (), list_matching_refs::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get a reference**
  ///
  /// Returns a single reference from your Git database. The `:ref` in the URL must be formatted as `heads/<branch name>` for branches and `tags/<tag name>` for tags. If the `:ref` doesn't match an existing ref, a `404` is returned.
  ///
  /// **Note:** You need to explicitly [request a pull request](https://docs.github.com/rest/pulls/pulls#get-a-pull-request) to trigger a test merge commit, which checks the mergeability of pull requests. For more information, see "[Checking mergeability of pull requests](https://docs.github.com/rest/guides/getting-started-with-the-git-database-api#checking-mergeability-of-pull-requests)".
  ///
  /// *Documentation*: [https://docs.github.com/rest/git/refs#get-a-reference](https://docs.github.com/rest/git/refs#get-a-reference)
  pub fn get_ref(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    ref_: impl Into<String>,
  ) -> Request<(), (), get_ref::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let ref_ = ref_.into();
    let url = format!("/repos/{owner}/{repo}/git/ref/{ref_}");

    Request::<(), (), get_ref::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create a reference**
  ///
  /// Creates a reference for your repository. You are unable to create new references for empty repositories, even if the commit SHA-1 hash used exists. Empty repositories are repositories without branches.
  ///
  /// *Documentation*: [https://docs.github.com/rest/git/refs#create-a-reference](https://docs.github.com/rest/git/refs#create-a-reference)
  pub fn create_ref(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<create_ref::Request, (), create_ref::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/git/refs");

    Request::<create_ref::Request, (), create_ref::Response>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Update a reference**
  ///
  /// Updates the provided reference to point to a new SHA. For more information, see "[Git References](https://git-scm.com/book/en/v2/Git-Internals-Git-References)" in the Git documentation.
  ///
  /// *Documentation*: [https://docs.github.com/rest/git/refs#update-a-reference](https://docs.github.com/rest/git/refs#update-a-reference)
  pub fn update_ref(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    ref_: impl Into<String>,
  ) -> Request<update_ref::Request, (), update_ref::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let ref_ = ref_.into();
    let url = format!("/repos/{owner}/{repo}/git/refs/{ref_}");

    Request::<update_ref::Request, (), update_ref::Response>::builder(&self.config)
      .patch(url)
      .build()
  }

  /// **Delete a reference**
  ///
  /// Deletes the provided reference.
  ///
  /// *Documentation*: [https://docs.github.com/rest/git/refs#delete-a-reference](https://docs.github.com/rest/git/refs#delete-a-reference)
  pub fn delete_ref(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    ref_: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let ref_ = ref_.into();
    let url = format!("/repos/{owner}/{repo}/git/refs/{ref_}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Create a tag object**
  ///
  /// Note that creating a tag object does not create the reference that makes a tag in Git. If you want to create an annotated tag in Git, you have to do this call to create the tag object, and then [create](https://docs.github.com/rest/git/refs#create-a-reference) the `refs/tags/[tag]` reference. If you want to create a lightweight tag, you only have to [create](https://docs.github.com/rest/git/refs#create-a-reference) the tag reference - this call would be unnecessary.
  ///
  /// **Signature verification object**
  ///
  /// The response will include a `verification` object that describes the result of verifying the commit's signature. The following fields are included in the `verification` object:
  ///
  /// | Name | Type | Description |
  /// | ---- | ---- | ----------- |
  /// | `verified` | `boolean` | Indicates whether GitHub considers the signature in this commit to be verified. |
  /// | `reason` | `string` | The reason for verified value. Possible values and their meanings are enumerated in table below. |
  /// | `signature` | `string` | The signature that was extracted from the commit. |
  /// | `payload` | `string` | The value that was signed. |
  ///
  /// These are the possible values for `reason` in the `verification` object:
  ///
  /// | Value | Description |
  /// | ----- | ----------- |
  /// | `expired_key` | The key that made the signature is expired. |
  /// | `not_signing_key` | The "signing" flag is not among the usage flags in the GPG key that made the signature. |
  /// | `gpgverify_error` | There was an error communicating with the signature verification service. |
  /// | `gpgverify_unavailable` | The signature verification service is currently unavailable. |
  /// | `unsigned` | The object does not include a signature. |
  /// | `unknown_signature_type` | A non-PGP signature was found in the commit. |
  /// | `no_user` | No user was associated with the `committer` email address in the commit. |
  /// | `unverified_email` | The `committer` email address in the commit was associated with a user, but the email address is not verified on their account. |
  /// | `bad_email` | The `committer` email address in the commit is not included in the identities of the PGP key that made the signature. |
  /// | `unknown_key` | The key that made the signature has not been registered with any user's account. |
  /// | `malformed_signature` | There was an error parsing the signature. |
  /// | `invalid` | The signature could not be cryptographically verified using the key whose key-id was found in the signature. |
  /// | `valid` | None of the above errors applied, so the signature is considered to be verified. |
  ///
  /// *Documentation*: [https://docs.github.com/rest/git/tags#create-a-tag-object](https://docs.github.com/rest/git/tags#create-a-tag-object)
  pub fn create_tag(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<create_tag::Request, (), create_tag::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/git/tags");

    Request::<create_tag::Request, (), create_tag::Response>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Get a tag**
  ///
  /// **Signature verification object**
  ///
  /// The response will include a `verification` object that describes the result of verifying the commit's signature. The following fields are included in the `verification` object:
  ///
  /// | Name | Type | Description |
  /// | ---- | ---- | ----------- |
  /// | `verified` | `boolean` | Indicates whether GitHub considers the signature in this commit to be verified. |
  /// | `reason` | `string` | The reason for verified value. Possible values and their meanings are enumerated in table below. |
  /// | `signature` | `string` | The signature that was extracted from the commit. |
  /// | `payload` | `string` | The value that was signed. |
  ///
  /// These are the possible values for `reason` in the `verification` object:
  ///
  /// | Value | Description |
  /// | ----- | ----------- |
  /// | `expired_key` | The key that made the signature is expired. |
  /// | `not_signing_key` | The "signing" flag is not among the usage flags in the GPG key that made the signature. |
  /// | `gpgverify_error` | There was an error communicating with the signature verification service. |
  /// | `gpgverify_unavailable` | The signature verification service is currently unavailable. |
  /// | `unsigned` | The object does not include a signature. |
  /// | `unknown_signature_type` | A non-PGP signature was found in the commit. |
  /// | `no_user` | No user was associated with the `committer` email address in the commit. |
  /// | `unverified_email` | The `committer` email address in the commit was associated with a user, but the email address is not verified on their account. |
  /// | `bad_email` | The `committer` email address in the commit is not included in the identities of the PGP key that made the signature. |
  /// | `unknown_key` | The key that made the signature has not been registered with any user's account. |
  /// | `malformed_signature` | There was an error parsing the signature. |
  /// | `invalid` | The signature could not be cryptographically verified using the key whose key-id was found in the signature. |
  /// | `valid` | None of the above errors applied, so the signature is considered to be verified. |
  ///
  /// *Documentation*: [https://docs.github.com/rest/git/tags#get-a-tag](https://docs.github.com/rest/git/tags#get-a-tag)
  pub fn get_tag(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    tag_sha: impl Into<String>,
  ) -> Request<(), (), get_tag::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let tag_sha = tag_sha.into();
    let url = format!("/repos/{owner}/{repo}/git/tags/{tag_sha}");

    Request::<(), (), get_tag::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create a tree**
  ///
  /// The tree creation API accepts nested entries. If you specify both a tree and a nested path modifying that tree, this endpoint will overwrite the contents of the tree with the new path contents, and create a new tree structure.
  ///
  /// If you use this endpoint to add, delete, or modify the file contents in a tree, you will need to commit the tree and then update a branch to point to the commit. For more information see "[Create a commit](https://docs.github.com/rest/git/commits#create-a-commit)" and "[Update a reference](https://docs.github.com/rest/git/refs#update-a-reference)."
  ///
  /// Returns an error if you try to delete a file that does not exist.
  ///
  /// *Documentation*: [https://docs.github.com/rest/git/trees#create-a-tree](https://docs.github.com/rest/git/trees#create-a-tree)
  pub fn create_tree(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<create_tree::Request, (), create_tree::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/git/trees");

    Request::<create_tree::Request, (), create_tree::Response>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Get a tree**
  ///
  /// Returns a single tree using the SHA1 value or ref name for that tree.
  ///
  /// If `truncated` is `true` in the response then the number of items in the `tree` array exceeded our maximum limit. If you need to fetch more items, use the non-recursive method of fetching trees, and fetch one sub-tree at a time.
  ///
  ///
  /// **Note**: The limit for the `tree` array is 100,000 entries with a maximum size of 7 MB when using the `recursive` parameter.
  ///
  /// *Documentation*: [https://docs.github.com/rest/git/trees#get-a-tree](https://docs.github.com/rest/git/trees#get-a-tree)
  pub fn get_tree(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    tree_sha: impl Into<String>,
  ) -> Request<(), get_tree::Query, get_tree::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let tree_sha = tree_sha.into();
    let url = format!("/repos/{owner}/{repo}/git/trees/{tree_sha}");

    Request::<(), get_tree::Query, get_tree::Response>::builder(&self.config)
      .get(url)
      .build()
  }
}
