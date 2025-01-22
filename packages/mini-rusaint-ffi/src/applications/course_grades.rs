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

#[uniffi::export]
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
}
