use std::sync::Arc;

use crate::session::USaintSession;

#[derive(uniffi::Object)]
pub struct CourseGradesApplication(mini_rusaint::CourseGradesApplication);

#[derive(Debug, thiserror::Error, uniffi::Error)]
#[uniffi(flat_error)]
pub enum CourseGradesApplicationError {
    #[error(transparent)]
    OriginalCourseGradesApplicationError(#[from] mini_rusaint::CourseGradesApplicationError),
}

#[uniffi::export(async_runtime = "tokio")]
impl CourseGradesApplication {
    #[uniffi::constructor]
    pub async fn new(
        session: Arc<USaintSession>,
    ) -> Result<CourseGradesApplication, CourseGradesApplicationError> {
        let application = mini_rusaint::applications::course_grades::CourseGradesApplication::new(
            session.original(),
        )
        .await?;

        Ok(CourseGradesApplication(application))
    }

    /// 모든 학기별 성적을 가져옵니다.
    pub async fn get_all_semester_grades(
        &self,
    ) -> Result<Vec<mini_rusaint::SemesterGrade>, CourseGradesApplicationError> {
        let all_semester_grades = self.0.get_all_semester_grades().await?;
        Ok(all_semester_grades)
    }
}
