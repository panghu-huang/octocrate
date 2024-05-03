#[cfg(any(
  feature = "full",
  feature = "webhook_event",
  feature = "webhook_push",
  feature = "webhook_pull_request",
  feature = "webhook_workflow_run",
  feature = "webhook_create"
))]
use octocrate_types::webhooks::*;

#[cfg(any(
  feature = "full",
  feature = "webhook_event",
  feature = "webhook_push",
  feature = "webhook_pull_request",
  feature = "webhook_workflow_run",
  feature = "webhook_create"
))]
fn parse_webhook_event(event: &str, data: &str) -> WebhookEvent {
  match event {
    #[cfg(feature = "webhook_push")]
    "push" => {
      let push_evnet: WebhookPush = serde_json::from_str(data).unwrap();

      WebhookEvent::Push(push_evnet)
    }
    #[cfg(feature = "webhook_pull_request")]
    "pull_request" => {
      let pull_request: WebhookPullRequestEvent = serde_json::from_str(data).unwrap();

      WebhookEvent::PullRequest(pull_request)
    }
    #[cfg(feature = "webhook_workflow_run")]
    "workflow_run" => {
      let workflow_run: WebhookWorkflowRunEvent = serde_json::from_str(data).unwrap();

      WebhookEvent::WorkflowRun(workflow_run)
    }
    #[cfg(feature = "webhook_create")]
    "create" => {
      let create: WebhookCreate = serde_json::from_str(data).unwrap();

      WebhookEvent::Create(create)
    }
    _ => {
      panic!("Unsupported webhook event: {}", event);
    }
  }
}

#[cfg(any(feature = "full", feature = "webhook_event", feature = "webhook_push"))]
#[test]
fn test_webhook_push_event() {
  let push = include_str!("./webhooks/push.json");

  let push_event = parse_webhook_event("push", push);

  if let WebhookEvent::Push(push_event) = push_event {
    assert_eq!(push_event.ref_, "refs/heads/main");
    assert_eq!(
      push_event.before,
      "76c1db59d428fc2cbbf54b72b08fdbca656c78b3"
    );
    assert_eq!(push_event.after, "c833dfd0e1dda0ac60b8d3dfd75a25e341e7eda2");
    assert_eq!(push_event.repository.full_name, "panghu-huang/octocrate");
    assert_eq!(push_event.pusher.name, "panghu-huang");
  } else {
    panic!("Expected push event");
  }
}

#[cfg(any(
  feature = "full",
  feature = "webhook_event",
  feature = "webhook_pull_request",
))]
#[test]
fn test_webhook_pull_request_opened_event() {
  let pull_request = include_str!("./webhooks/pull_request.json");

  let pull_request_event = parse_webhook_event("pull_request", pull_request);

  if let WebhookEvent::PullRequest(WebhookPullRequestEvent::Opened(pull_request_event)) =
    pull_request_event
  {
    assert_eq!(pull_request_event.action.to_string(), "opened");
    assert_eq!(pull_request_event.number, 23);
    if let WebhookPullRequestOpenedPullRequest::PullRequest(pull_request) =
      pull_request_event.pull_request
    {
      assert_eq!(pull_request.title, "feat: add webhooks feature");
      assert_eq!(pull_request.user.login, "panghu-huang");
    } else {
      panic!("Expected pull request");
    }
  } else {
    panic!("Expected pull request opened event");
  }
}

#[cfg(any(
  feature = "full",
  feature = "webhook_event",
  feature = "webhook_workflow_run",
))]
#[test]
fn test_webhook_workflow_run_completed_event() {
  let workflow_run = include_str!("./webhooks/workflow_run.json");

  let workflow_run_event = parse_webhook_event("workflow_run", workflow_run);

  if let WebhookEvent::WorkflowRun(WebhookWorkflowRunEvent::Completed(workflow_run_event)) =
    workflow_run_event
  {
    assert_eq!(workflow_run_event.workflow.as_ref().unwrap().name, "CI");
    assert_eq!(workflow_run_event.action.to_string(), "completed");
  } else {
    panic!("Expected workflow run completed event");
  }
}

#[cfg(any(
  feature = "full",
  feature = "webhook_event",
  feature = "webhook_create"
))]
#[test]
fn test_webhook_create_event() {
  let create = include_str!("./webhooks/create.json");

  let create_event = parse_webhook_event("create", create);

  if let WebhookEvent::Create(create_event) = create_event {
    assert_eq!(create_event.ref_, "webhooks");
    assert_eq!(create_event.ref_type.to_string(), "branch");
    assert_eq!(create_event.master_branch, "main");
    assert_eq!(create_event.repository.full_name, "panghu-huang/octocrate");
  } else {
    panic!("Expected create event");
  }
}
