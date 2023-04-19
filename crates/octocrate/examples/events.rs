use octocrate::{test_utils, GithubWebhookEvent, WebhookServer};

#[tokio::main]
async fn main() {
  let envs = test_utils::load_test_envs().unwrap();

  let mut server = WebhookServer::builder()
    .app_id(envs.github_app_id)
    .private_key(envs.github_app_private_key)
    .build()
    .unwrap();

  server
    .on_webhook_event(|event, _api| {
      match event {
        GithubWebhookEvent::PullRequest(evt) => {
          println!("Pull request {:#?}", evt);
        }
        GithubWebhookEvent::Push(evt) => {
          println!("Push {:#?}", evt);
        }
        GithubWebhookEvent::IssueComment(evt) => {
          println!("Issue comment {:#?}", evt);
        }
        GithubWebhookEvent::WorkflowJob(evt) => {
          println!("Workflow job {:#?}", evt);
        }
        GithubWebhookEvent::Unsupported {
          installation: _,
          payload,
        } => {
          println!("Unsupported {:#?}", payload);
        }
      };

      Ok(())
    })
    .start()
    .await
    .unwrap();
}
