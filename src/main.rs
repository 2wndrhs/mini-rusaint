use mini_rusaint::{
    applications::{course_grades::CourseGradesApplication, model::SemesterType},
    session::USaintSession,
};

#[tokio::main]
async fn main() {
    let session = USaintSession::new()
        .await
        .expect("세션 생성에 실패했습니다.");

    let course_grades_app = CourseGradesApplication::new(session.client.clone())
        .await
        .expect("CourseGradesApplication 생성에 실패했습니다.");

    let semester_grades = course_grades_app
        .get_all_semester_grades()
        .await
        .expect("모든 학기별 성적 정보를 가져오는데 실패했습니다.");

    println!("{:#?}", semester_grades);

    let course_grades = course_grades_app
        .get_semester_grades_details(2024, SemesterType::SecondSemester)
        .await
        .expect("학기별 세부 성적 정보를 가져오는데 실패했습니다.");

    println!("{:#?}", course_grades);
}
