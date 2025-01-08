use mini_rusaint::{
    applications::{course_grades::CourseGradesApplication, model::SemesterType},
    session::USaintSession,
    webdynpro::event,
};

#[tokio::main]
async fn main() {
    let session = USaintSession::with_env()
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
        .get_semester_grades_details(2024, SemesterType::SecondSemester, true)
        .await
        .expect("학기별 세부 성적 정보를 가져오는데 실패했습니다.");

    println!("{:#?}", course_grades);

    // let encoded = "ClientInspector_Notify~E002Id~E004WD01~E005Data~E004CssMatchesHtmlVersion~003ATRUE~E003~E002ResponseData~E004delta~E005EnqueueCardinality~E004single~E003~E002~E003~E001Button_Press~E002Id~E004ZCMB3W0017.ID_0001~003AW_POPUP.WDBUTTON_5~E003~E002ResponseData~E004delta~E005ClientAction~E004submit~E003~E002~E003~E001Form_Request~E002Id~E004sap.client.SsrClient.form~E005Async~E004false~E005FocusInfo~E004~0040~007B~0022sFocussedId~0022~003A~0022ZCMB3W0017.ID_0001~003AW_POPUP.WDBUTTON_5~0022~007D~E005Hash~E004~E005DomChanged~E004false~E005IsDirty~E004false~E003~E002ResponseData~E004delta~E003~E002~E003";
    // println!("{}", event::decode_sap_event(encoded));
}
