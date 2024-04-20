#[allow(unused_imports)]
use crate::types::*;
use octocrate_core::*;

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
  ) -> Request<(), (), GitCommit> {
    let owner = owner.into();
    let repo = repo.into();
    let commit_sha = commit_sha.into();
    let url = format!("/repos/{owner}/{repo}/git/commits/{commit_sha}");

    Request::<(), (), GitCommit>::builder(&self.config)
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
  ) -> Request<(), (), GitReference> {
    let owner = owner.into();
    let repo = repo.into();
    let ref_ = ref_.into();
    let url = format!("/repos/{owner}/{repo}/git/ref/{ref_}");

    Request::<(), (), GitReference>::builder(&self.config)
      .get(url)
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
  ) -> Request<GitUpdateRefRequest, (), GitReference> {
    let owner = owner.into();
    let repo = repo.into();
    let ref_ = ref_.into();
    let url = format!("/repos/{owner}/{repo}/git/refs/{ref_}");

    Request::<GitUpdateRefRequest, (), GitReference>::builder(&self.config)
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
  ) -> Request<GitCreateCommitRequest, (), GitCommit> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/git/commits");

    Request::<GitCreateCommitRequest, (), GitCommit>::builder(&self.config)
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
  ) -> Request<(), GitGetTreeQuery, GitTree> {
    let owner = owner.into();
    let repo = repo.into();
    let tree_sha = tree_sha.into();
    let url = format!("/repos/{owner}/{repo}/git/trees/{tree_sha}");

    Request::<(), GitGetTreeQuery, GitTree>::builder(&self.config)
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
  ) -> Request<GitCreateRefRequest, (), GitReference> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/git/refs");

    Request::<GitCreateRefRequest, (), GitReference>::builder(&self.config)
      .post(url)
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
  ) -> Request<GitCreateTreeRequest, (), GitTree> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/git/trees");

    Request::<GitCreateTreeRequest, (), GitTree>::builder(&self.config)
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
  ) -> Request<(), (), Blob> {
    let owner = owner.into();
    let repo = repo.into();
    let file_sha = file_sha.into();
    let url = format!("/repos/{owner}/{repo}/git/blobs/{file_sha}");

    Request::<(), (), Blob>::builder(&self.config)
      .get(url)
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
  ) -> Request<(), (), GitTag> {
    let owner = owner.into();
    let repo = repo.into();
    let tag_sha = tag_sha.into();
    let url = format!("/repos/{owner}/{repo}/git/tags/{tag_sha}");

    Request::<(), (), GitTag>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create a blob**
  ///
  ///
  /// *Documentation*: [https://docs.github.com/rest/git/blobs#create-a-blob](https://docs.github.com/rest/git/blobs#create-a-blob)
  pub fn create_blob(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<GitCreateBlobRequest, (), ShortBlob> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/git/blobs");

    Request::<GitCreateBlobRequest, (), ShortBlob>::builder(&self.config)
      .post(url)
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
  ) -> Request<GitCreateTagRequest, (), GitTag> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/git/tags");

    Request::<GitCreateTagRequest, (), GitTag>::builder(&self.config)
      .post(url)
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
  ) -> Request<(), (), GitReferenceArray> {
    let owner = owner.into();
    let repo = repo.into();
    let ref_ = ref_.into();
    let url = format!("/repos/{owner}/{repo}/git/matching-refs/{ref_}");

    Request::<(), (), GitReferenceArray>::builder(&self.config)
      .get(url)
      .build()
  }
}
