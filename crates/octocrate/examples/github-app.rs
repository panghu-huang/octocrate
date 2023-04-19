use octocrate::{test_utils, GithubApp};

#[tokio::main]
async fn main() {
  let envs = test_utils::load_test_envs().unwrap();

  let app = GithubApp::builder()
    .app_id(envs.github_app_id)
    .private_key(envs.github_app_private_key)
    .build()
    .unwrap();

  let installation = app
    .get_repository_installation(envs.repo_owner.clone(), envs.repo_name.clone())
    .await
    .unwrap();

  let api = app.get_api(installation.id).await.unwrap();

  let repository = api
    .repositories
    .get_repository(envs.repo_owner, envs.repo_name)
    .send()
    .await
    .unwrap();

  println!("Repository: {:?}", repository);
}
