use octocrate_core::*;
#[allow(unused_imports)]
use octocrate_types::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use typed_builder::TypedBuilder;

pub mod list_for_team_discussion_comment_in_org {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Reaction>;

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QueryContent {
    #[serde(rename = "+1")]
    PlusOne,
    #[serde(rename = "-1")]
    MinusOne,
    #[serde(rename = "laugh")]
    Laugh,
    #[serde(rename = "confused")]
    Confused,
    #[serde(rename = "heart")]
    Heart,
    #[serde(rename = "hooray")]
    Hooray,
    #[serde(rename = "rocket")]
    Rocket,
    #[serde(rename = "eyes")]
    Eyes,
  }

  impl ToString for QueryContent {
    fn to_string(&self) -> String {
      match self {
        QueryContent::PlusOne => "+1".to_string(),
        QueryContent::MinusOne => "-1".to_string(),
        QueryContent::Laugh => "laugh".to_string(),
        QueryContent::Confused => "confused".to_string(),
        QueryContent::Heart => "heart".to_string(),
        QueryContent::Hooray => "hooray".to_string(),
        QueryContent::Rocket => "rocket".to_string(),
        QueryContent::Eyes => "eyes".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// Returns a single [reaction type](https://docs.github.com/rest/reactions/reactions#about-reactions). Omit this parameter to list all reactions to a team discussion comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub content: Option<QueryContent>,
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
  }
}

pub mod create_for_team_discussion_comment_in_org {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Reaction;

  /// The [reaction type](https://docs.github.com/rest/reactions/reactions#about-reactions) to add to the team discussion comment.
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum RequestContent {
    #[serde(rename = "+1")]
    PlusOne,
    #[serde(rename = "-1")]
    MinusOne,
    #[serde(rename = "laugh")]
    Laugh,
    #[serde(rename = "confused")]
    Confused,
    #[serde(rename = "heart")]
    Heart,
    #[serde(rename = "hooray")]
    Hooray,
    #[serde(rename = "rocket")]
    Rocket,
    #[serde(rename = "eyes")]
    Eyes,
  }

  impl ToString for RequestContent {
    fn to_string(&self) -> String {
      match self {
        RequestContent::PlusOne => "+1".to_string(),
        RequestContent::MinusOne => "-1".to_string(),
        RequestContent::Laugh => "laugh".to_string(),
        RequestContent::Confused => "confused".to_string(),
        RequestContent::Heart => "heart".to_string(),
        RequestContent::Hooray => "hooray".to_string(),
        RequestContent::Rocket => "rocket".to_string(),
        RequestContent::Eyes => "eyes".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The [reaction type](https://docs.github.com/rest/reactions/reactions#about-reactions) to add to the team discussion comment.
    pub content: RequestContent,
  }
}

pub mod delete_for_team_discussion_comment {
  #[allow(unused_imports)]
  use super::*;
}

pub mod list_for_team_discussion_in_org {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Reaction>;

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QueryContent {
    #[serde(rename = "+1")]
    PlusOne,
    #[serde(rename = "-1")]
    MinusOne,
    #[serde(rename = "laugh")]
    Laugh,
    #[serde(rename = "confused")]
    Confused,
    #[serde(rename = "heart")]
    Heart,
    #[serde(rename = "hooray")]
    Hooray,
    #[serde(rename = "rocket")]
    Rocket,
    #[serde(rename = "eyes")]
    Eyes,
  }

  impl ToString for QueryContent {
    fn to_string(&self) -> String {
      match self {
        QueryContent::PlusOne => "+1".to_string(),
        QueryContent::MinusOne => "-1".to_string(),
        QueryContent::Laugh => "laugh".to_string(),
        QueryContent::Confused => "confused".to_string(),
        QueryContent::Heart => "heart".to_string(),
        QueryContent::Hooray => "hooray".to_string(),
        QueryContent::Rocket => "rocket".to_string(),
        QueryContent::Eyes => "eyes".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// Returns a single [reaction type](https://docs.github.com/rest/reactions/reactions#about-reactions). Omit this parameter to list all reactions to a team discussion.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub content: Option<QueryContent>,
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
  }
}

pub mod create_for_team_discussion_in_org {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Reaction;

  /// The [reaction type](https://docs.github.com/rest/reactions/reactions#about-reactions) to add to the team discussion.
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum RequestContent {
    #[serde(rename = "+1")]
    PlusOne,
    #[serde(rename = "-1")]
    MinusOne,
    #[serde(rename = "laugh")]
    Laugh,
    #[serde(rename = "confused")]
    Confused,
    #[serde(rename = "heart")]
    Heart,
    #[serde(rename = "hooray")]
    Hooray,
    #[serde(rename = "rocket")]
    Rocket,
    #[serde(rename = "eyes")]
    Eyes,
  }

  impl ToString for RequestContent {
    fn to_string(&self) -> String {
      match self {
        RequestContent::PlusOne => "+1".to_string(),
        RequestContent::MinusOne => "-1".to_string(),
        RequestContent::Laugh => "laugh".to_string(),
        RequestContent::Confused => "confused".to_string(),
        RequestContent::Heart => "heart".to_string(),
        RequestContent::Hooray => "hooray".to_string(),
        RequestContent::Rocket => "rocket".to_string(),
        RequestContent::Eyes => "eyes".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The [reaction type](https://docs.github.com/rest/reactions/reactions#about-reactions) to add to the team discussion.
    pub content: RequestContent,
  }
}

pub mod delete_for_team_discussion {
  #[allow(unused_imports)]
  use super::*;
}

pub mod list_for_commit_comment {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Reaction>;

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QueryContent {
    #[serde(rename = "+1")]
    PlusOne,
    #[serde(rename = "-1")]
    MinusOne,
    #[serde(rename = "laugh")]
    Laugh,
    #[serde(rename = "confused")]
    Confused,
    #[serde(rename = "heart")]
    Heart,
    #[serde(rename = "hooray")]
    Hooray,
    #[serde(rename = "rocket")]
    Rocket,
    #[serde(rename = "eyes")]
    Eyes,
  }

  impl ToString for QueryContent {
    fn to_string(&self) -> String {
      match self {
        QueryContent::PlusOne => "+1".to_string(),
        QueryContent::MinusOne => "-1".to_string(),
        QueryContent::Laugh => "laugh".to_string(),
        QueryContent::Confused => "confused".to_string(),
        QueryContent::Heart => "heart".to_string(),
        QueryContent::Hooray => "hooray".to_string(),
        QueryContent::Rocket => "rocket".to_string(),
        QueryContent::Eyes => "eyes".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// Returns a single [reaction type](https://docs.github.com/rest/reactions/reactions#about-reactions). Omit this parameter to list all reactions to a commit comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub content: Option<QueryContent>,
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
  }
}

pub mod create_for_commit_comment {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Reaction;

  /// The [reaction type](https://docs.github.com/rest/reactions/reactions#about-reactions) to add to the commit comment.
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum RequestContent {
    #[serde(rename = "+1")]
    PlusOne,
    #[serde(rename = "-1")]
    MinusOne,
    #[serde(rename = "laugh")]
    Laugh,
    #[serde(rename = "confused")]
    Confused,
    #[serde(rename = "heart")]
    Heart,
    #[serde(rename = "hooray")]
    Hooray,
    #[serde(rename = "rocket")]
    Rocket,
    #[serde(rename = "eyes")]
    Eyes,
  }

  impl ToString for RequestContent {
    fn to_string(&self) -> String {
      match self {
        RequestContent::PlusOne => "+1".to_string(),
        RequestContent::MinusOne => "-1".to_string(),
        RequestContent::Laugh => "laugh".to_string(),
        RequestContent::Confused => "confused".to_string(),
        RequestContent::Heart => "heart".to_string(),
        RequestContent::Hooray => "hooray".to_string(),
        RequestContent::Rocket => "rocket".to_string(),
        RequestContent::Eyes => "eyes".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The [reaction type](https://docs.github.com/rest/reactions/reactions#about-reactions) to add to the commit comment.
    pub content: RequestContent,
  }
}

pub mod delete_for_commit_comment {
  #[allow(unused_imports)]
  use super::*;
}

pub mod list_for_issue_comment {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Reaction>;

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QueryContent {
    #[serde(rename = "+1")]
    PlusOne,
    #[serde(rename = "-1")]
    MinusOne,
    #[serde(rename = "laugh")]
    Laugh,
    #[serde(rename = "confused")]
    Confused,
    #[serde(rename = "heart")]
    Heart,
    #[serde(rename = "hooray")]
    Hooray,
    #[serde(rename = "rocket")]
    Rocket,
    #[serde(rename = "eyes")]
    Eyes,
  }

  impl ToString for QueryContent {
    fn to_string(&self) -> String {
      match self {
        QueryContent::PlusOne => "+1".to_string(),
        QueryContent::MinusOne => "-1".to_string(),
        QueryContent::Laugh => "laugh".to_string(),
        QueryContent::Confused => "confused".to_string(),
        QueryContent::Heart => "heart".to_string(),
        QueryContent::Hooray => "hooray".to_string(),
        QueryContent::Rocket => "rocket".to_string(),
        QueryContent::Eyes => "eyes".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// Returns a single [reaction type](https://docs.github.com/rest/reactions/reactions#about-reactions). Omit this parameter to list all reactions to an issue comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub content: Option<QueryContent>,
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
  }
}

pub mod create_for_issue_comment {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Reaction;

  /// The [reaction type](https://docs.github.com/rest/reactions/reactions#about-reactions) to add to the issue comment.
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum RequestContent {
    #[serde(rename = "+1")]
    PlusOne,
    #[serde(rename = "-1")]
    MinusOne,
    #[serde(rename = "laugh")]
    Laugh,
    #[serde(rename = "confused")]
    Confused,
    #[serde(rename = "heart")]
    Heart,
    #[serde(rename = "hooray")]
    Hooray,
    #[serde(rename = "rocket")]
    Rocket,
    #[serde(rename = "eyes")]
    Eyes,
  }

  impl ToString for RequestContent {
    fn to_string(&self) -> String {
      match self {
        RequestContent::PlusOne => "+1".to_string(),
        RequestContent::MinusOne => "-1".to_string(),
        RequestContent::Laugh => "laugh".to_string(),
        RequestContent::Confused => "confused".to_string(),
        RequestContent::Heart => "heart".to_string(),
        RequestContent::Hooray => "hooray".to_string(),
        RequestContent::Rocket => "rocket".to_string(),
        RequestContent::Eyes => "eyes".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The [reaction type](https://docs.github.com/rest/reactions/reactions#about-reactions) to add to the issue comment.
    pub content: RequestContent,
  }
}

pub mod delete_for_issue_comment {
  #[allow(unused_imports)]
  use super::*;
}

pub mod list_for_issue {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Reaction>;

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QueryContent {
    #[serde(rename = "+1")]
    PlusOne,
    #[serde(rename = "-1")]
    MinusOne,
    #[serde(rename = "laugh")]
    Laugh,
    #[serde(rename = "confused")]
    Confused,
    #[serde(rename = "heart")]
    Heart,
    #[serde(rename = "hooray")]
    Hooray,
    #[serde(rename = "rocket")]
    Rocket,
    #[serde(rename = "eyes")]
    Eyes,
  }

  impl ToString for QueryContent {
    fn to_string(&self) -> String {
      match self {
        QueryContent::PlusOne => "+1".to_string(),
        QueryContent::MinusOne => "-1".to_string(),
        QueryContent::Laugh => "laugh".to_string(),
        QueryContent::Confused => "confused".to_string(),
        QueryContent::Heart => "heart".to_string(),
        QueryContent::Hooray => "hooray".to_string(),
        QueryContent::Rocket => "rocket".to_string(),
        QueryContent::Eyes => "eyes".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// Returns a single [reaction type](https://docs.github.com/rest/reactions/reactions#about-reactions). Omit this parameter to list all reactions to an issue.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub content: Option<QueryContent>,
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
  }
}

pub mod create_for_issue {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Reaction;

  /// The [reaction type](https://docs.github.com/rest/reactions/reactions#about-reactions) to add to the issue.
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum RequestContent {
    #[serde(rename = "+1")]
    PlusOne,
    #[serde(rename = "-1")]
    MinusOne,
    #[serde(rename = "laugh")]
    Laugh,
    #[serde(rename = "confused")]
    Confused,
    #[serde(rename = "heart")]
    Heart,
    #[serde(rename = "hooray")]
    Hooray,
    #[serde(rename = "rocket")]
    Rocket,
    #[serde(rename = "eyes")]
    Eyes,
  }

  impl ToString for RequestContent {
    fn to_string(&self) -> String {
      match self {
        RequestContent::PlusOne => "+1".to_string(),
        RequestContent::MinusOne => "-1".to_string(),
        RequestContent::Laugh => "laugh".to_string(),
        RequestContent::Confused => "confused".to_string(),
        RequestContent::Heart => "heart".to_string(),
        RequestContent::Hooray => "hooray".to_string(),
        RequestContent::Rocket => "rocket".to_string(),
        RequestContent::Eyes => "eyes".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The [reaction type](https://docs.github.com/rest/reactions/reactions#about-reactions) to add to the issue.
    pub content: RequestContent,
  }
}

pub mod delete_for_issue {
  #[allow(unused_imports)]
  use super::*;
}

pub mod list_for_pull_request_review_comment {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Reaction>;

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QueryContent {
    #[serde(rename = "+1")]
    PlusOne,
    #[serde(rename = "-1")]
    MinusOne,
    #[serde(rename = "laugh")]
    Laugh,
    #[serde(rename = "confused")]
    Confused,
    #[serde(rename = "heart")]
    Heart,
    #[serde(rename = "hooray")]
    Hooray,
    #[serde(rename = "rocket")]
    Rocket,
    #[serde(rename = "eyes")]
    Eyes,
  }

  impl ToString for QueryContent {
    fn to_string(&self) -> String {
      match self {
        QueryContent::PlusOne => "+1".to_string(),
        QueryContent::MinusOne => "-1".to_string(),
        QueryContent::Laugh => "laugh".to_string(),
        QueryContent::Confused => "confused".to_string(),
        QueryContent::Heart => "heart".to_string(),
        QueryContent::Hooray => "hooray".to_string(),
        QueryContent::Rocket => "rocket".to_string(),
        QueryContent::Eyes => "eyes".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// Returns a single [reaction type](https://docs.github.com/rest/reactions/reactions#about-reactions). Omit this parameter to list all reactions to a pull request review comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub content: Option<QueryContent>,
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
  }
}

pub mod create_for_pull_request_review_comment {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Reaction;

  /// The [reaction type](https://docs.github.com/rest/reactions/reactions#about-reactions) to add to the pull request review comment.
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum RequestContent {
    #[serde(rename = "+1")]
    PlusOne,
    #[serde(rename = "-1")]
    MinusOne,
    #[serde(rename = "laugh")]
    Laugh,
    #[serde(rename = "confused")]
    Confused,
    #[serde(rename = "heart")]
    Heart,
    #[serde(rename = "hooray")]
    Hooray,
    #[serde(rename = "rocket")]
    Rocket,
    #[serde(rename = "eyes")]
    Eyes,
  }

  impl ToString for RequestContent {
    fn to_string(&self) -> String {
      match self {
        RequestContent::PlusOne => "+1".to_string(),
        RequestContent::MinusOne => "-1".to_string(),
        RequestContent::Laugh => "laugh".to_string(),
        RequestContent::Confused => "confused".to_string(),
        RequestContent::Heart => "heart".to_string(),
        RequestContent::Hooray => "hooray".to_string(),
        RequestContent::Rocket => "rocket".to_string(),
        RequestContent::Eyes => "eyes".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The [reaction type](https://docs.github.com/rest/reactions/reactions#about-reactions) to add to the pull request review comment.
    pub content: RequestContent,
  }
}

pub mod delete_for_pull_request_comment {
  #[allow(unused_imports)]
  use super::*;
}

pub mod list_for_release {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Reaction>;

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QueryContent {
    #[serde(rename = "+1")]
    PlusOne,
    #[serde(rename = "laugh")]
    Laugh,
    #[serde(rename = "heart")]
    Heart,
    #[serde(rename = "hooray")]
    Hooray,
    #[serde(rename = "rocket")]
    Rocket,
    #[serde(rename = "eyes")]
    Eyes,
  }

  impl ToString for QueryContent {
    fn to_string(&self) -> String {
      match self {
        QueryContent::PlusOne => "+1".to_string(),
        QueryContent::Laugh => "laugh".to_string(),
        QueryContent::Heart => "heart".to_string(),
        QueryContent::Hooray => "hooray".to_string(),
        QueryContent::Rocket => "rocket".to_string(),
        QueryContent::Eyes => "eyes".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// Returns a single [reaction type](https://docs.github.com/rest/reactions/reactions#about-reactions). Omit this parameter to list all reactions to a release.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub content: Option<QueryContent>,
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
  }
}

pub mod create_for_release {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Reaction;

  /// The [reaction type](https://docs.github.com/rest/reactions/reactions#about-reactions) to add to the release.
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum RequestContent {
    #[serde(rename = "+1")]
    PlusOne,
    #[serde(rename = "laugh")]
    Laugh,
    #[serde(rename = "heart")]
    Heart,
    #[serde(rename = "hooray")]
    Hooray,
    #[serde(rename = "rocket")]
    Rocket,
    #[serde(rename = "eyes")]
    Eyes,
  }

  impl ToString for RequestContent {
    fn to_string(&self) -> String {
      match self {
        RequestContent::PlusOne => "+1".to_string(),
        RequestContent::Laugh => "laugh".to_string(),
        RequestContent::Heart => "heart".to_string(),
        RequestContent::Hooray => "hooray".to_string(),
        RequestContent::Rocket => "rocket".to_string(),
        RequestContent::Eyes => "eyes".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The [reaction type](https://docs.github.com/rest/reactions/reactions#about-reactions) to add to the release.
    pub content: RequestContent,
  }
}

pub mod delete_for_release {
  #[allow(unused_imports)]
  use super::*;
}

pub mod list_for_team_discussion_comment_legacy {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Reaction>;

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QueryContent {
    #[serde(rename = "+1")]
    PlusOne,
    #[serde(rename = "-1")]
    MinusOne,
    #[serde(rename = "laugh")]
    Laugh,
    #[serde(rename = "confused")]
    Confused,
    #[serde(rename = "heart")]
    Heart,
    #[serde(rename = "hooray")]
    Hooray,
    #[serde(rename = "rocket")]
    Rocket,
    #[serde(rename = "eyes")]
    Eyes,
  }

  impl ToString for QueryContent {
    fn to_string(&self) -> String {
      match self {
        QueryContent::PlusOne => "+1".to_string(),
        QueryContent::MinusOne => "-1".to_string(),
        QueryContent::Laugh => "laugh".to_string(),
        QueryContent::Confused => "confused".to_string(),
        QueryContent::Heart => "heart".to_string(),
        QueryContent::Hooray => "hooray".to_string(),
        QueryContent::Rocket => "rocket".to_string(),
        QueryContent::Eyes => "eyes".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// Returns a single [reaction type](https://docs.github.com/rest/reactions/reactions#about-reactions). Omit this parameter to list all reactions to a team discussion comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub content: Option<QueryContent>,
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
  }
}

pub mod create_for_team_discussion_comment_legacy {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Reaction;

  /// The [reaction type](https://docs.github.com/rest/reactions/reactions#about-reactions) to add to the team discussion comment.
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum RequestContent {
    #[serde(rename = "+1")]
    PlusOne,
    #[serde(rename = "-1")]
    MinusOne,
    #[serde(rename = "laugh")]
    Laugh,
    #[serde(rename = "confused")]
    Confused,
    #[serde(rename = "heart")]
    Heart,
    #[serde(rename = "hooray")]
    Hooray,
    #[serde(rename = "rocket")]
    Rocket,
    #[serde(rename = "eyes")]
    Eyes,
  }

  impl ToString for RequestContent {
    fn to_string(&self) -> String {
      match self {
        RequestContent::PlusOne => "+1".to_string(),
        RequestContent::MinusOne => "-1".to_string(),
        RequestContent::Laugh => "laugh".to_string(),
        RequestContent::Confused => "confused".to_string(),
        RequestContent::Heart => "heart".to_string(),
        RequestContent::Hooray => "hooray".to_string(),
        RequestContent::Rocket => "rocket".to_string(),
        RequestContent::Eyes => "eyes".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The [reaction type](https://docs.github.com/rest/reactions/reactions#about-reactions) to add to the team discussion comment.
    pub content: RequestContent,
  }
}

pub mod list_for_team_discussion_legacy {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<Reaction>;

  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum QueryContent {
    #[serde(rename = "+1")]
    PlusOne,
    #[serde(rename = "-1")]
    MinusOne,
    #[serde(rename = "laugh")]
    Laugh,
    #[serde(rename = "confused")]
    Confused,
    #[serde(rename = "heart")]
    Heart,
    #[serde(rename = "hooray")]
    Hooray,
    #[serde(rename = "rocket")]
    Rocket,
    #[serde(rename = "eyes")]
    Eyes,
  }

  impl ToString for QueryContent {
    fn to_string(&self) -> String {
      match self {
        QueryContent::PlusOne => "+1".to_string(),
        QueryContent::MinusOne => "-1".to_string(),
        QueryContent::Laugh => "laugh".to_string(),
        QueryContent::Confused => "confused".to_string(),
        QueryContent::Heart => "heart".to_string(),
        QueryContent::Hooray => "hooray".to_string(),
        QueryContent::Rocket => "rocket".to_string(),
        QueryContent::Eyes => "eyes".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Query {
    /// Returns a single [reaction type](https://docs.github.com/rest/reactions/reactions#about-reactions). Omit this parameter to list all reactions to a team discussion.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub content: Option<QueryContent>,
    /// The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub per_page: Option<i64>,
    /// The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api)."
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub page: Option<i64>,
  }
}

pub mod create_for_team_discussion_legacy {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Reaction;

  /// The [reaction type](https://docs.github.com/rest/reactions/reactions#about-reactions) to add to the team discussion.
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
  pub enum RequestContent {
    #[serde(rename = "+1")]
    PlusOne,
    #[serde(rename = "-1")]
    MinusOne,
    #[serde(rename = "laugh")]
    Laugh,
    #[serde(rename = "confused")]
    Confused,
    #[serde(rename = "heart")]
    Heart,
    #[serde(rename = "hooray")]
    Hooray,
    #[serde(rename = "rocket")]
    Rocket,
    #[serde(rename = "eyes")]
    Eyes,
  }

  impl ToString for RequestContent {
    fn to_string(&self) -> String {
      match self {
        RequestContent::PlusOne => "+1".to_string(),
        RequestContent::MinusOne => "-1".to_string(),
        RequestContent::Laugh => "laugh".to_string(),
        RequestContent::Confused => "confused".to_string(),
        RequestContent::Heart => "heart".to_string(),
        RequestContent::Hooray => "hooray".to_string(),
        RequestContent::Rocket => "rocket".to_string(),
        RequestContent::Eyes => "eyes".to_string(),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
  #[builder(field_defaults(setter(into)))]
  pub struct Request {
    /// The [reaction type](https://docs.github.com/rest/reactions/reactions#about-reactions) to add to the team discussion.
    pub content: RequestContent,
  }
}

/// Interact with reactions to various GitHub entities.
pub struct GitHubReactionsAPI {
  config: SharedAPIConfig,
}

impl GitHubReactionsAPI {
  pub fn new(config: &SharedAPIConfig) -> Self {
    Self {
      config: config.clone(),
    }
  }

  /// **List reactions for a team discussion comment**
  ///
  /// List the reactions to a [team discussion comment](https://docs.github.com/rest/teams/discussion-comments#get-a-discussion-comment).
  ///
  /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/:org_id/team/:team_id/discussions/:discussion_number/comments/:comment_number/reactions`.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `read:discussion` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/reactions/reactions#list-reactions-for-a-team-discussion-comment](https://docs.github.com/rest/reactions/reactions#list-reactions-for-a-team-discussion-comment)
  pub fn list_for_team_discussion_comment_in_org(
    &self,
    org: impl Into<String>,
    team_slug: impl Into<String>,
    discussion_number: impl Into<i64>,
    comment_number: impl Into<i64>,
  ) -> Request<
    (),
    list_for_team_discussion_comment_in_org::Query,
    list_for_team_discussion_comment_in_org::Response,
  > {
    let org = org.into();
    let team_slug = team_slug.into();
    let discussion_number = discussion_number.into();
    let comment_number = comment_number.into();
    let url = format!("/orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/comments/{comment_number}/reactions");

    Request::<
      (),
      list_for_team_discussion_comment_in_org::Query,
      list_for_team_discussion_comment_in_org::Response,
    >::builder(&self.config)
    .get(url)
    .build()
  }

  /// **Create reaction for a team discussion comment**
  ///
  /// Create a reaction to a [team discussion comment](https://docs.github.com/rest/teams/discussion-comments#get-a-discussion-comment).
  ///
  /// A response with an HTTP `200` status means that you already added the reaction type to this team discussion comment.
  ///
  /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `POST /organizations/:org_id/team/:team_id/discussions/:discussion_number/comments/:comment_number/reactions`.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `write:discussion` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/reactions/reactions#create-reaction-for-a-team-discussion-comment](https://docs.github.com/rest/reactions/reactions#create-reaction-for-a-team-discussion-comment)
  pub fn create_for_team_discussion_comment_in_org(
    &self,
    org: impl Into<String>,
    team_slug: impl Into<String>,
    discussion_number: impl Into<i64>,
    comment_number: impl Into<i64>,
  ) -> Request<
    create_for_team_discussion_comment_in_org::Request,
    (),
    create_for_team_discussion_comment_in_org::Response,
  > {
    let org = org.into();
    let team_slug = team_slug.into();
    let discussion_number = discussion_number.into();
    let comment_number = comment_number.into();
    let url = format!("/orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/comments/{comment_number}/reactions");

    Request::<
      create_for_team_discussion_comment_in_org::Request,
      (),
      create_for_team_discussion_comment_in_org::Response,
    >::builder(&self.config)
    .post(url)
    .build()
  }

  /// **Delete team discussion comment reaction**
  ///
  /// **Note:** You can also specify a team or organization with `team_id` and `org_id` using the route `DELETE /organizations/:org_id/team/:team_id/discussions/:discussion_number/comments/:comment_number/reactions/:reaction_id`.
  ///
  /// Delete a reaction to a [team discussion comment](https://docs.github.com/rest/teams/discussion-comments#get-a-discussion-comment).
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `write:discussion` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/reactions/reactions#delete-team-discussion-comment-reaction](https://docs.github.com/rest/reactions/reactions#delete-team-discussion-comment-reaction)
  pub fn delete_for_team_discussion_comment(
    &self,
    org: impl Into<String>,
    team_slug: impl Into<String>,
    discussion_number: impl Into<i64>,
    comment_number: impl Into<i64>,
    reaction_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let team_slug = team_slug.into();
    let discussion_number = discussion_number.into();
    let comment_number = comment_number.into();
    let reaction_id = reaction_id.into();
    let url = format!("/orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/comments/{comment_number}/reactions/{reaction_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List reactions for a team discussion**
  ///
  /// List the reactions to a [team discussion](https://docs.github.com/rest/teams/discussions#get-a-discussion).
  ///
  /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/:org_id/team/:team_id/discussions/:discussion_number/reactions`.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `read:discussion` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/reactions/reactions#list-reactions-for-a-team-discussion](https://docs.github.com/rest/reactions/reactions#list-reactions-for-a-team-discussion)
  pub fn list_for_team_discussion_in_org(
    &self,
    org: impl Into<String>,
    team_slug: impl Into<String>,
    discussion_number: impl Into<i64>,
  ) -> Request<(), list_for_team_discussion_in_org::Query, list_for_team_discussion_in_org::Response>
  {
    let org = org.into();
    let team_slug = team_slug.into();
    let discussion_number = discussion_number.into();
    let url = format!("/orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/reactions");

    Request::<(), list_for_team_discussion_in_org::Query, list_for_team_discussion_in_org::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create reaction for a team discussion**
  ///
  /// Create a reaction to a [team discussion](https://docs.github.com/rest/teams/discussions#get-a-discussion).
  ///
  /// A response with an HTTP `200` status means that you already added the reaction type to this team discussion.
  ///
  /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `POST /organizations/:org_id/team/:team_id/discussions/:discussion_number/reactions`.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `write:discussion` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/reactions/reactions#create-reaction-for-a-team-discussion](https://docs.github.com/rest/reactions/reactions#create-reaction-for-a-team-discussion)
  pub fn create_for_team_discussion_in_org(
    &self,
    org: impl Into<String>,
    team_slug: impl Into<String>,
    discussion_number: impl Into<i64>,
  ) -> Request<
    create_for_team_discussion_in_org::Request,
    (),
    create_for_team_discussion_in_org::Response,
  > {
    let org = org.into();
    let team_slug = team_slug.into();
    let discussion_number = discussion_number.into();
    let url = format!("/orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/reactions");

    Request::<
      create_for_team_discussion_in_org::Request,
      (),
      create_for_team_discussion_in_org::Response,
    >::builder(&self.config)
    .post(url)
    .build()
  }

  /// **Delete team discussion reaction**
  ///
  /// **Note:** You can also specify a team or organization with `team_id` and `org_id` using the route `DELETE /organizations/:org_id/team/:team_id/discussions/:discussion_number/reactions/:reaction_id`.
  ///
  /// Delete a reaction to a [team discussion](https://docs.github.com/rest/teams/discussions#get-a-discussion).
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `write:discussion` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/reactions/reactions#delete-team-discussion-reaction](https://docs.github.com/rest/reactions/reactions#delete-team-discussion-reaction)
  pub fn delete_for_team_discussion(
    &self,
    org: impl Into<String>,
    team_slug: impl Into<String>,
    discussion_number: impl Into<i64>,
    reaction_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let org = org.into();
    let team_slug = team_slug.into();
    let discussion_number = discussion_number.into();
    let reaction_id = reaction_id.into();
    let url = format!(
      "/orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/reactions/{reaction_id}"
    );

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List reactions for a commit comment**
  ///
  /// List the reactions to a [commit comment](https://docs.github.com/rest/commits/comments#get-a-commit-comment).
  ///
  /// *Documentation*: [https://docs.github.com/rest/reactions/reactions#list-reactions-for-a-commit-comment](https://docs.github.com/rest/reactions/reactions#list-reactions-for-a-commit-comment)
  pub fn list_for_commit_comment(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    comment_id: impl Into<i64>,
  ) -> Request<(), list_for_commit_comment::Query, list_for_commit_comment::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let comment_id = comment_id.into();
    let url = format!("/repos/{owner}/{repo}/comments/{comment_id}/reactions");

    Request::<(), list_for_commit_comment::Query, list_for_commit_comment::Response>::builder(
      &self.config,
    )
    .get(url)
    .build()
  }

  /// **Create reaction for a commit comment**
  ///
  /// Create a reaction to a [commit comment](https://docs.github.com/rest/commits/comments#get-a-commit-comment). A response with an HTTP `200` status means that you already added the reaction type to this commit comment.
  ///
  /// *Documentation*: [https://docs.github.com/rest/reactions/reactions#create-reaction-for-a-commit-comment](https://docs.github.com/rest/reactions/reactions#create-reaction-for-a-commit-comment)
  pub fn create_for_commit_comment(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    comment_id: impl Into<i64>,
  ) -> Request<create_for_commit_comment::Request, (), create_for_commit_comment::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let comment_id = comment_id.into();
    let url = format!("/repos/{owner}/{repo}/comments/{comment_id}/reactions");

    Request::<create_for_commit_comment::Request, (), create_for_commit_comment::Response>::builder(
      &self.config,
    )
    .post(url)
    .build()
  }

  /// **Delete a commit comment reaction**
  ///
  /// **Note:** You can also specify a repository by `repository_id` using the route `DELETE /repositories/:repository_id/comments/:comment_id/reactions/:reaction_id`.
  ///
  /// Delete a reaction to a [commit comment](https://docs.github.com/rest/commits/comments#get-a-commit-comment).
  ///
  /// *Documentation*: [https://docs.github.com/rest/reactions/reactions#delete-a-commit-comment-reaction](https://docs.github.com/rest/reactions/reactions#delete-a-commit-comment-reaction)
  pub fn delete_for_commit_comment(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    comment_id: impl Into<i64>,
    reaction_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let comment_id = comment_id.into();
    let reaction_id = reaction_id.into();
    let url = format!("/repos/{owner}/{repo}/comments/{comment_id}/reactions/{reaction_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List reactions for an issue comment**
  ///
  /// List the reactions to an [issue comment](https://docs.github.com/rest/issues/comments#get-an-issue-comment).
  ///
  /// *Documentation*: [https://docs.github.com/rest/reactions/reactions#list-reactions-for-an-issue-comment](https://docs.github.com/rest/reactions/reactions#list-reactions-for-an-issue-comment)
  pub fn list_for_issue_comment(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    comment_id: impl Into<i64>,
  ) -> Request<(), list_for_issue_comment::Query, list_for_issue_comment::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let comment_id = comment_id.into();
    let url = format!("/repos/{owner}/{repo}/issues/comments/{comment_id}/reactions");

    Request::<(), list_for_issue_comment::Query, list_for_issue_comment::Response>::builder(
      &self.config,
    )
    .get(url)
    .build()
  }

  /// **Create reaction for an issue comment**
  ///
  /// Create a reaction to an [issue comment](https://docs.github.com/rest/issues/comments#get-an-issue-comment). A response with an HTTP `200` status means that you already added the reaction type to this issue comment.
  ///
  /// *Documentation*: [https://docs.github.com/rest/reactions/reactions#create-reaction-for-an-issue-comment](https://docs.github.com/rest/reactions/reactions#create-reaction-for-an-issue-comment)
  pub fn create_for_issue_comment(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    comment_id: impl Into<i64>,
  ) -> Request<create_for_issue_comment::Request, (), create_for_issue_comment::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let comment_id = comment_id.into();
    let url = format!("/repos/{owner}/{repo}/issues/comments/{comment_id}/reactions");

    Request::<create_for_issue_comment::Request, (), create_for_issue_comment::Response>::builder(
      &self.config,
    )
    .post(url)
    .build()
  }

  /// **Delete an issue comment reaction**
  ///
  /// **Note:** You can also specify a repository by `repository_id` using the route `DELETE delete /repositories/:repository_id/issues/comments/:comment_id/reactions/:reaction_id`.
  ///
  /// Delete a reaction to an [issue comment](https://docs.github.com/rest/issues/comments#get-an-issue-comment).
  ///
  /// *Documentation*: [https://docs.github.com/rest/reactions/reactions#delete-an-issue-comment-reaction](https://docs.github.com/rest/reactions/reactions#delete-an-issue-comment-reaction)
  pub fn delete_for_issue_comment(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    comment_id: impl Into<i64>,
    reaction_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let comment_id = comment_id.into();
    let reaction_id = reaction_id.into();
    let url = format!("/repos/{owner}/{repo}/issues/comments/{comment_id}/reactions/{reaction_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List reactions for an issue**
  ///
  /// List the reactions to an [issue](https://docs.github.com/rest/issues/issues#get-an-issue).
  ///
  /// *Documentation*: [https://docs.github.com/rest/reactions/reactions#list-reactions-for-an-issue](https://docs.github.com/rest/reactions/reactions#list-reactions-for-an-issue)
  pub fn list_for_issue(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    issue_number: impl Into<i64>,
  ) -> Request<(), list_for_issue::Query, list_for_issue::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let issue_number = issue_number.into();
    let url = format!("/repos/{owner}/{repo}/issues/{issue_number}/reactions");

    Request::<(), list_for_issue::Query, list_for_issue::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create reaction for an issue**
  ///
  /// Create a reaction to an [issue](https://docs.github.com/rest/issues/issues#get-an-issue). A response with an HTTP `200` status means that you already added the reaction type to this issue.
  ///
  /// *Documentation*: [https://docs.github.com/rest/reactions/reactions#create-reaction-for-an-issue](https://docs.github.com/rest/reactions/reactions#create-reaction-for-an-issue)
  pub fn create_for_issue(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    issue_number: impl Into<i64>,
  ) -> Request<create_for_issue::Request, (), create_for_issue::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let issue_number = issue_number.into();
    let url = format!("/repos/{owner}/{repo}/issues/{issue_number}/reactions");

    Request::<create_for_issue::Request, (), create_for_issue::Response>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Delete an issue reaction**
  ///
  /// **Note:** You can also specify a repository by `repository_id` using the route `DELETE /repositories/:repository_id/issues/:issue_number/reactions/:reaction_id`.
  ///
  /// Delete a reaction to an [issue](https://docs.github.com/rest/issues/issues#get-an-issue).
  ///
  /// *Documentation*: [https://docs.github.com/rest/reactions/reactions#delete-an-issue-reaction](https://docs.github.com/rest/reactions/reactions#delete-an-issue-reaction)
  pub fn delete_for_issue(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    issue_number: impl Into<i64>,
    reaction_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let issue_number = issue_number.into();
    let reaction_id = reaction_id.into();
    let url = format!("/repos/{owner}/{repo}/issues/{issue_number}/reactions/{reaction_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List reactions for a pull request review comment**
  ///
  /// List the reactions to a [pull request review comment](https://docs.github.com/rest/pulls/comments#get-a-review-comment-for-a-pull-request).
  ///
  /// *Documentation*: [https://docs.github.com/rest/reactions/reactions#list-reactions-for-a-pull-request-review-comment](https://docs.github.com/rest/reactions/reactions#list-reactions-for-a-pull-request-review-comment)
  pub fn list_for_pull_request_review_comment(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    comment_id: impl Into<i64>,
  ) -> Request<
    (),
    list_for_pull_request_review_comment::Query,
    list_for_pull_request_review_comment::Response,
  > {
    let owner = owner.into();
    let repo = repo.into();
    let comment_id = comment_id.into();
    let url = format!("/repos/{owner}/{repo}/pulls/comments/{comment_id}/reactions");

    Request::<
      (),
      list_for_pull_request_review_comment::Query,
      list_for_pull_request_review_comment::Response,
    >::builder(&self.config)
    .get(url)
    .build()
  }

  /// **Create reaction for a pull request review comment**
  ///
  /// Create a reaction to a [pull request review comment](https://docs.github.com/rest/pulls/comments#get-a-review-comment-for-a-pull-request). A response with an HTTP `200` status means that you already added the reaction type to this pull request review comment.
  ///
  /// *Documentation*: [https://docs.github.com/rest/reactions/reactions#create-reaction-for-a-pull-request-review-comment](https://docs.github.com/rest/reactions/reactions#create-reaction-for-a-pull-request-review-comment)
  pub fn create_for_pull_request_review_comment(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    comment_id: impl Into<i64>,
  ) -> Request<
    create_for_pull_request_review_comment::Request,
    (),
    create_for_pull_request_review_comment::Response,
  > {
    let owner = owner.into();
    let repo = repo.into();
    let comment_id = comment_id.into();
    let url = format!("/repos/{owner}/{repo}/pulls/comments/{comment_id}/reactions");

    Request::<
      create_for_pull_request_review_comment::Request,
      (),
      create_for_pull_request_review_comment::Response,
    >::builder(&self.config)
    .post(url)
    .build()
  }

  /// **Delete a pull request comment reaction**
  ///
  /// **Note:** You can also specify a repository by `repository_id` using the route `DELETE /repositories/:repository_id/pulls/comments/:comment_id/reactions/:reaction_id.`
  ///
  /// Delete a reaction to a [pull request review comment](https://docs.github.com/rest/pulls/comments#get-a-review-comment-for-a-pull-request).
  ///
  /// *Documentation*: [https://docs.github.com/rest/reactions/reactions#delete-a-pull-request-comment-reaction](https://docs.github.com/rest/reactions/reactions#delete-a-pull-request-comment-reaction)
  pub fn delete_for_pull_request_comment(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    comment_id: impl Into<i64>,
    reaction_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let comment_id = comment_id.into();
    let reaction_id = reaction_id.into();
    let url = format!("/repos/{owner}/{repo}/pulls/comments/{comment_id}/reactions/{reaction_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List reactions for a release**
  ///
  /// List the reactions to a [release](https://docs.github.com/rest/releases/releases#get-a-release).
  ///
  /// *Documentation*: [https://docs.github.com/rest/reactions/reactions#list-reactions-for-a-release](https://docs.github.com/rest/reactions/reactions#list-reactions-for-a-release)
  pub fn list_for_release(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    release_id: impl Into<i64>,
  ) -> Request<(), list_for_release::Query, list_for_release::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let release_id = release_id.into();
    let url = format!("/repos/{owner}/{repo}/releases/{release_id}/reactions");

    Request::<(), list_for_release::Query, list_for_release::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create reaction for a release**
  ///
  /// Create a reaction to a [release](https://docs.github.com/rest/releases/releases#get-a-release). A response with a `Status: 200 OK` means that you already added the reaction type to this release.
  ///
  /// *Documentation*: [https://docs.github.com/rest/reactions/reactions#create-reaction-for-a-release](https://docs.github.com/rest/reactions/reactions#create-reaction-for-a-release)
  pub fn create_for_release(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    release_id: impl Into<i64>,
  ) -> Request<create_for_release::Request, (), create_for_release::Response> {
    let owner = owner.into();
    let repo = repo.into();
    let release_id = release_id.into();
    let url = format!("/repos/{owner}/{repo}/releases/{release_id}/reactions");

    Request::<create_for_release::Request, (), create_for_release::Response>::builder(&self.config)
      .post(url)
      .build()
  }

  /// **Delete a release reaction**
  ///
  /// **Note:** You can also specify a repository by `repository_id` using the route `DELETE delete /repositories/:repository_id/releases/:release_id/reactions/:reaction_id`.
  ///
  /// Delete a reaction to a [release](https://docs.github.com/rest/releases/releases#get-a-release).
  ///
  /// *Documentation*: [https://docs.github.com/rest/reactions/reactions#delete-a-release-reaction](https://docs.github.com/rest/reactions/reactions#delete-a-release-reaction)
  pub fn delete_for_release(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
    release_id: impl Into<i64>,
    reaction_id: impl Into<i64>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let release_id = release_id.into();
    let reaction_id = reaction_id.into();
    let url = format!("/repos/{owner}/{repo}/releases/{release_id}/reactions/{reaction_id}");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **List reactions for a team discussion comment (Legacy)**
  ///
  /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [`List reactions for a team discussion comment`](https://docs.github.com/rest/reactions/reactions#list-reactions-for-a-team-discussion-comment) endpoint.
  ///
  /// List the reactions to a [team discussion comment](https://docs.github.com/rest/teams/discussion-comments#get-a-discussion-comment).
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `read:discussion` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/reactions/reactions#list-reactions-for-a-team-discussion-comment-legacy](https://docs.github.com/rest/reactions/reactions#list-reactions-for-a-team-discussion-comment-legacy)
  pub fn list_for_team_discussion_comment_legacy(
    &self,
    team_id: impl Into<i64>,
    discussion_number: impl Into<i64>,
    comment_number: impl Into<i64>,
  ) -> Request<
    (),
    list_for_team_discussion_comment_legacy::Query,
    list_for_team_discussion_comment_legacy::Response,
  > {
    let team_id = team_id.into();
    let discussion_number = discussion_number.into();
    let comment_number = comment_number.into();
    let url = format!(
      "/teams/{team_id}/discussions/{discussion_number}/comments/{comment_number}/reactions"
    );

    Request::<
      (),
      list_for_team_discussion_comment_legacy::Query,
      list_for_team_discussion_comment_legacy::Response,
    >::builder(&self.config)
    .get(url)
    .build()
  }

  /// **Create reaction for a team discussion comment (Legacy)**
  ///
  /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new "[Create reaction for a team discussion comment](https://docs.github.com/rest/reactions/reactions#create-reaction-for-a-team-discussion-comment)" endpoint.
  ///
  /// Create a reaction to a [team discussion comment](https://docs.github.com/rest/teams/discussion-comments#get-a-discussion-comment).
  ///
  /// A response with an HTTP `200` status means that you already added the reaction type to this team discussion comment.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `write:discussion` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/reactions/reactions#create-reaction-for-a-team-discussion-comment-legacy](https://docs.github.com/rest/reactions/reactions#create-reaction-for-a-team-discussion-comment-legacy)
  pub fn create_for_team_discussion_comment_legacy(
    &self,
    team_id: impl Into<i64>,
    discussion_number: impl Into<i64>,
    comment_number: impl Into<i64>,
  ) -> Request<
    create_for_team_discussion_comment_legacy::Request,
    (),
    create_for_team_discussion_comment_legacy::Response,
  > {
    let team_id = team_id.into();
    let discussion_number = discussion_number.into();
    let comment_number = comment_number.into();
    let url = format!(
      "/teams/{team_id}/discussions/{discussion_number}/comments/{comment_number}/reactions"
    );

    Request::<
      create_for_team_discussion_comment_legacy::Request,
      (),
      create_for_team_discussion_comment_legacy::Response,
    >::builder(&self.config)
    .post(url)
    .build()
  }

  /// **List reactions for a team discussion (Legacy)**
  ///
  /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [`List reactions for a team discussion`](https://docs.github.com/rest/reactions/reactions#list-reactions-for-a-team-discussion) endpoint.
  ///
  /// List the reactions to a [team discussion](https://docs.github.com/rest/teams/discussions#get-a-discussion).
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `read:discussion` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/reactions/reactions#list-reactions-for-a-team-discussion-legacy](https://docs.github.com/rest/reactions/reactions#list-reactions-for-a-team-discussion-legacy)
  pub fn list_for_team_discussion_legacy(
    &self,
    team_id: impl Into<i64>,
    discussion_number: impl Into<i64>,
  ) -> Request<(), list_for_team_discussion_legacy::Query, list_for_team_discussion_legacy::Response>
  {
    let team_id = team_id.into();
    let discussion_number = discussion_number.into();
    let url = format!("/teams/{team_id}/discussions/{discussion_number}/reactions");

    Request::<(), list_for_team_discussion_legacy::Query, list_for_team_discussion_legacy::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Create reaction for a team discussion (Legacy)**
  ///
  /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [`Create reaction for a team discussion`](https://docs.github.com/rest/reactions/reactions#create-reaction-for-a-team-discussion) endpoint.
  ///
  /// Create a reaction to a [team discussion](https://docs.github.com/rest/teams/discussions#get-a-discussion).
  ///
  /// A response with an HTTP `200` status means that you already added the reaction type to this team discussion.
  ///
  /// OAuth app tokens and personal access tokens (classic) need the `write:discussion` scope to use this endpoint.
  ///
  /// *Documentation*: [https://docs.github.com/rest/reactions/reactions#create-reaction-for-a-team-discussion-legacy](https://docs.github.com/rest/reactions/reactions#create-reaction-for-a-team-discussion-legacy)
  pub fn create_for_team_discussion_legacy(
    &self,
    team_id: impl Into<i64>,
    discussion_number: impl Into<i64>,
  ) -> Request<
    create_for_team_discussion_legacy::Request,
    (),
    create_for_team_discussion_legacy::Response,
  > {
    let team_id = team_id.into();
    let discussion_number = discussion_number.into();
    let url = format!("/teams/{team_id}/discussions/{discussion_number}/reactions");

    Request::<
      create_for_team_discussion_legacy::Request,
      (),
      create_for_team_discussion_legacy::Response,
    >::builder(&self.config)
    .post(url)
    .build()
  }
}
