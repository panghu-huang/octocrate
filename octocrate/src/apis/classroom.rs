use octocrate_core::*;
#[allow(unused_imports)]
use octocrate_types::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use typed_builder::TypedBuilder;

pub mod get_an_assignment {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = ClassroomAssignment;
}

pub mod list_accepted_assigments_for_an_assignment {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<ClassroomAcceptedAssignment>;

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
  }
}

pub mod get_assignment_grades {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<ClassroomAssignmentGrade>;
}

pub mod list_classrooms {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<SimpleClassroom>;

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
  }
}

pub mod get_a_classroom {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Classroom;
}

pub mod list_assignments_for_a_classroom {
  #[allow(unused_imports)]
  use super::*;

  pub type Response = Vec<SimpleClassroomAssignment>;

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
  }
}

/// Interact with GitHub Classroom.
pub struct GitHubClassroomAPI {
  config: SharedAPIConfig,
}

impl GitHubClassroomAPI {
  pub fn new(config: &SharedAPIConfig) -> Self {
    Self {
      config: config.clone(),
    }
  }

  /// **Get an assignment**
  ///
  /// Gets a GitHub Classroom assignment. Assignment will only be returned if the current user is an administrator of the GitHub Classroom for the assignment.
  ///
  /// *Documentation*: [https://docs.github.com/rest/classroom/classroom#get-an-assignment](https://docs.github.com/rest/classroom/classroom#get-an-assignment)
  pub fn get_an_assignment(
    &self,
    assignment_id: impl Into<i64>,
  ) -> Request<(), (), get_an_assignment::Response> {
    let assignment_id = assignment_id.into();
    let url = format!("/assignments/{assignment_id}");

    Request::<(), (), get_an_assignment::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List accepted assignments for an assignment**
  ///
  /// Lists any assignment repositories that have been created by students accepting a GitHub Classroom assignment. Accepted assignments will only be returned if the current user is an administrator of the GitHub Classroom for the assignment.
  ///
  /// *Documentation*: [https://docs.github.com/rest/classroom/classroom#list-accepted-assignments-for-an-assignment](https://docs.github.com/rest/classroom/classroom#list-accepted-assignments-for-an-assignment)
  pub fn list_accepted_assigments_for_an_assignment(
    &self,
    assignment_id: impl Into<i64>,
  ) -> Request<
    (),
    list_accepted_assigments_for_an_assignment::Query,
    list_accepted_assigments_for_an_assignment::Response,
  > {
    let assignment_id = assignment_id.into();
    let url = format!("/assignments/{assignment_id}/accepted_assignments");

    Request::<
      (),
      list_accepted_assigments_for_an_assignment::Query,
      list_accepted_assigments_for_an_assignment::Response,
    >::builder(&self.config)
    .get(url)
    .build()
  }

  /// **Get assignment grades**
  ///
  /// Gets grades for a GitHub Classroom assignment. Grades will only be returned if the current user is an administrator of the GitHub Classroom for the assignment.
  ///
  /// *Documentation*: [https://docs.github.com/rest/classroom/classroom#get-assignment-grades](https://docs.github.com/rest/classroom/classroom#get-assignment-grades)
  pub fn get_assignment_grades(
    &self,
    assignment_id: impl Into<i64>,
  ) -> Request<(), (), get_assignment_grades::Response> {
    let assignment_id = assignment_id.into();
    let url = format!("/assignments/{assignment_id}/grades");

    Request::<(), (), get_assignment_grades::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List classrooms**
  ///
  /// Lists GitHub Classroom classrooms for the current user. Classrooms will only be returned if the current user is an administrator of one or more GitHub Classrooms.
  ///
  /// *Documentation*: [https://docs.github.com/rest/classroom/classroom#list-classrooms](https://docs.github.com/rest/classroom/classroom#list-classrooms)
  pub fn list_classrooms(&self) -> Request<(), list_classrooms::Query, list_classrooms::Response> {
    let url = format!("/classrooms");

    Request::<(), list_classrooms::Query, list_classrooms::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get a classroom**
  ///
  /// Gets a GitHub Classroom classroom for the current user. Classroom will only be returned if the current user is an administrator of the GitHub Classroom.
  ///
  /// *Documentation*: [https://docs.github.com/rest/classroom/classroom#get-a-classroom](https://docs.github.com/rest/classroom/classroom#get-a-classroom)
  pub fn get_a_classroom(
    &self,
    classroom_id: impl Into<i64>,
  ) -> Request<(), (), get_a_classroom::Response> {
    let classroom_id = classroom_id.into();
    let url = format!("/classrooms/{classroom_id}");

    Request::<(), (), get_a_classroom::Response>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List assignments for a classroom**
  ///
  /// Lists GitHub Classroom assignments for a classroom. Assignments will only be returned if the current user is an administrator of the GitHub Classroom.
  ///
  /// *Documentation*: [https://docs.github.com/rest/classroom/classroom#list-assignments-for-a-classroom](https://docs.github.com/rest/classroom/classroom#list-assignments-for-a-classroom)
  pub fn list_assignments_for_a_classroom(
    &self,
    classroom_id: impl Into<i64>,
  ) -> Request<
    (),
    list_assignments_for_a_classroom::Query,
    list_assignments_for_a_classroom::Response,
  > {
    let classroom_id = classroom_id.into();
    let url = format!("/classrooms/{classroom_id}/assignments");

    Request::<(), list_assignments_for_a_classroom::Query, list_assignments_for_a_classroom::Response>::builder(&self.config)
      .get(url)
      .build()
  }
}
