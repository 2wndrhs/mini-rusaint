use mini_rusaint::{
    applications::course_grades::{model::SemesterType, CourseGradesApplication},
    session::USaintSession,
};

#[tokio::test]
async fn test_get_all_semester_grades() {
    let session = USaintSession::with_env()
        .await
        .expect("세션 생성에 실패했습니다.");

    let course_grades_app = CourseGradesApplication::new(session)
        .await
        .expect("CourseGradesApplication 생성에 실패했습니다.");

    let semester_grades = course_grades_app
        .get_all_semester_grades()
        .await
        .expect("모든 학기별 성적 정보를 가져오는데 실패했습니다.");

    assert!(
        !semester_grades.is_empty(),
        "학기별 성적 정보가 비어 있습니다."
    );
}

#[tokio::test]
async fn test_get_semester_grades_details() {
    let session = USaintSession::with_env()
        .await
        .expect("세션 생성에 실패했습니다.");

    let course_grades_app = CourseGradesApplication::new(session)
        .await
        .expect("CourseGradesApplication 생성에 실패했습니다.");

    let course_grades = course_grades_app
        .get_semester_grades_details(2024, SemesterType::SecondSemester, true)
        .await
        .expect("학기별 세부 성적 정보를 가져오는데 실패했습니다.");

    assert!(
        !course_grades.is_empty(),
        "학기별 세부 성적 정보가 비어 있습니다."
    );
}
