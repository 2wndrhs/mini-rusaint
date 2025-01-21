use mini_rusaint::session::USaintSession;

#[derive(uniffi::Object)]
pub struct CourseGradesApplication(
    mini_rusaint::applications::course_grades::CourseGradesApplication,
);

impl CourseGradesApplication {
    #[uniffi::constructor]
    pub async fn new(session: USaintSession) -> CourseGradesApplication {
        let application =
            mini_rusaint::applications::course_grades::CourseGradesApplication::new(session)
                .await
                .expect("CourseGradesApplication 생성에 실패했습니다.");

        CourseGradesApplication(application)
    }
}
