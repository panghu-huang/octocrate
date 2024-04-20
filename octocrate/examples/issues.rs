use octocrate::{
  APIConfig, AuthorAssociation, GitHubAPI, IssuesCreateCommentRequest, PersonalAccessToken,
};

#[tokio::test]
async fn create_issue_comment() {
  dotenv::dotenv().ok();

  let personal_access_token =
    std::env::var("GITHUB_PERSONAL_ACCESS_TOKEN").expect("GITHUB_PERSONAL_ACCESS_TOKEN is not set");

  let personal_access_token = PersonalAccessToken::new(personal_access_token);

  // Use the personal access token to create issue comments
  let config = APIConfig::with_token(personal_access_token).shared();

  let api = GitHubAPI::new(&config);

  // https://github.com/panghu-huang/octocrate/pull/1#issuecomment-2041280635
  let comment = IssuesCreateCommentRequest {
    body: "Hello, world! (Created by octocrate)".to_string(),
  };

  let issue_comment = api
    .issues
    .create_comment("panghu-huang", "octocrate", 1)
    .body(&comment)
    .send()
    .await
    .unwrap();

  assert_eq!(
    issue_comment.body.unwrap(),
    "Hello, world! (Created by octocrate)"
  );

  let comments = api
    .issues
    .list_comments("panghu-huang", "octocrate", 1)
    .send()
    .await
    .unwrap();

  let first_comment = comments.first().clone().unwrap();

  assert_eq!(
    first_comment.body.as_ref().unwrap(),
    "Hello, world! (Created by octocrate)"
  );
  assert_eq!(first_comment.author_association, AuthorAssociation::Owner);
}
