use mini_rusaint::{
    applications::{course_grades::CourseGradesApplication, model::SemesterType},
    session::USaintSession,
    webdynpro::{client::SapSsrClient, event},
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

    course_grades_app
        .get_semester_grades_details(2023, SemesterType::FirstSemester)
        .await
        .expect("학기별 세부 성적 정보를 가져오는데 실패했습니다.");

    // let encoded = "ComboBox_Select~E002Id~E004ZCMW_PERIOD_RE.ID_0DC742680F42DA9747594D1AE51A0C69~003AVIW_MAIN.PERYR~E005Key~E0042024~E005ByEnter~E004false~E003~E002ResponseData~E004delta~E005ClientAction~E004submit~E003~E002~E003~E001Form_Request~E002Id~E004sap.client.SsrClient.form~E005Async~E004false~E005FocusInfo~E004~0040~007B~0022iSelectionStart~0022~003A0~002C~0022iSelectionEnd~0022~003A0~002C~0022iCursorPos~0022~003A0~002C~0022sValue~0022~003A~00222024~D559~B144~B3C4~0022~002C~0022sFocussedId~0022~003A~0022ZCMW_PERIOD_RE.ID_0DC742680F42DA9747594D1AE51A0C69~003AVIW_MAIN.PERYR~0022~002C~0022sApplyControlId~0022~003A~0022ZCMW_PERIOD_RE.ID_0DC742680F42DA9747594D1AE51A0C69~003AVIW_MAIN.PERYR~0022~007D~E005Hash~E004~E005DomChanged~E004false~E005IsDirty~E004false~E003~E002ResponseData~E004delta~E003~E002~E003";
    // let decoded = event::decode_sap_event(&encoded);

    // println!("{:#?}", decoded);
}
