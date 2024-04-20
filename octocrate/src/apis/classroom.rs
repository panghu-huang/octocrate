use octocrate_core::*;
#[allow(unused_imports)]
use octocrate_types::*;

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
  ) -> Request<(), (), ClassroomAssignment> {
    let assignment_id = assignment_id.into();
    let url = format!("/assignments/{assignment_id}");

    Request::<(), (), ClassroomAssignment>::builder(&self.config)
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
    ClassroomListAcceptedAssigmentsForAnAssignmentQuery,
    ClassroomAcceptedAssignmentArray,
  > {
    let assignment_id = assignment_id.into();
    let url = format!("/assignments/{assignment_id}/accepted_assignments");

    Request::<
      (),
      ClassroomListAcceptedAssigmentsForAnAssignmentQuery,
      ClassroomAcceptedAssignmentArray,
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
  ) -> Request<(), (), ClassroomAssignmentGradeArray> {
    let assignment_id = assignment_id.into();
    let url = format!("/assignments/{assignment_id}/grades");

    Request::<(), (), ClassroomAssignmentGradeArray>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **List classrooms**
  ///
  /// Lists GitHub Classroom classrooms for the current user. Classrooms will only be returned if the current user is an administrator of one or more GitHub Classrooms.
  ///
  /// *Documentation*: [https://docs.github.com/rest/classroom/classroom#list-classrooms](https://docs.github.com/rest/classroom/classroom#list-classrooms)
  pub fn list_classrooms(&self) -> Request<(), ClassroomListClassroomsQuery, SimpleClassroomArray> {
    let url = format!("/classrooms");

    Request::<(), ClassroomListClassroomsQuery, SimpleClassroomArray>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Get a classroom**
  ///
  /// Gets a GitHub Classroom classroom for the current user. Classroom will only be returned if the current user is an administrator of the GitHub Classroom.
  ///
  /// *Documentation*: [https://docs.github.com/rest/classroom/classroom#get-a-classroom](https://docs.github.com/rest/classroom/classroom#get-a-classroom)
  pub fn get_a_classroom(&self, classroom_id: impl Into<i64>) -> Request<(), (), Classroom> {
    let classroom_id = classroom_id.into();
    let url = format!("/classrooms/{classroom_id}");

    Request::<(), (), Classroom>::builder(&self.config)
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
  ) -> Request<(), ClassroomListAssignmentsForAClassroomQuery, SimpleClassroomAssignmentArray> {
    let classroom_id = classroom_id.into();
    let url = format!("/classrooms/{classroom_id}/assignments");

    Request::<(), ClassroomListAssignmentsForAClassroomQuery, SimpleClassroomAssignmentArray>::builder(&self.config)
      .get(url)
      .build()
  }
}
