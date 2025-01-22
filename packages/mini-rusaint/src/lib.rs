pub mod applications;
pub mod session;
pub mod webdynpro;

pub use applications::course_grades::model::SemesterGrade;
pub use applications::course_grades::{CourseGradesApplication, CourseGradesApplicationError};
pub use session::{USaintSession, USaintSessionError};

uniffi::setup_scaffolding!();
