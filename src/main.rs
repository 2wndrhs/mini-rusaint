use mini_rusaint::{
    applications::course_grades::CourseGradesApplication,
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

    // let encoded = "ClientInspector_Notify~E002Id~E004WD01~E005Data~E004ClientWidth~003A1213px~003BClientHeight~003A884px~003BScreenWidth~003A1512px~003BScreenHeight~003A982px~003BScreenOrientation~003Alandscape~003BThemedTableRowHeight~003A33px~003BThemedFormLayoutRowHeight~003A32px~003BThemedSvgLibUrls~003A~007B~0022SAPGUI-icons~0022~003A~0022https~003A~002F~002Fecc.ssu.ac.kr~002Fsap~002Fpublic~002Fbc~002Fur~002Fnw5~002Fthemes~002F~007Ecache-20220217154731~002FBase~002FbaseLib~002Fsap_fiori_3~002Fsvg~002Flibs~002FSAPGUI-icons.svg~0022~002C~0022SAPWeb-icons~0022~003A~0022https~003A~002F~002Fecc.ssu.ac.kr~002Fsap~002Fpublic~002Fbc~002Fur~002Fnw5~002Fthemes~002F~007Ecache-20220217154731~002FBase~002FbaseLib~002Fsap_fiori_3~002Fsvg~002Flibs~002FSAPWeb-icons.svg~0022~007D~003BThemeTags~003AFiori_3~002CTouch~003BThemeID~003Asap_fiori_3~003BSapThemeID~003Asap_fiori_3~003BDeviceType~003ADESKTOP~003BDocumentDomain~003Aecc.ssu.ac.kr~003BClientURL~003Ahttps~003A~002F~002Fecc.ssu.ac.kr~002Fsap~002Fbc~002Fwebdynpro~002FSAP~002FZCMB3W0017~003Fsap-wd-stableids~003Dx~0023~003BIsTopWindow~003ATRUE~003BParentAccessible~003ATRUE~E003~E002ResponseData~E004delta~E005EnqueueCardinality~E004single~E003~E002~E003~E001ClientInspector_Notify~E002Id~E004WD02~E005Data~E004ThemedTableRowHeight~003A25px~E003~E002ResponseData~E004delta~E005EnqueueCardinality~E004single~E003~E002~E003~E001LoadingPlaceHolder_Load~E002Id~E004_loadingPlaceholder_~E003~E002ResponseData~E004delta~E005ClientAction~E004submit~E003~E002~E003~E001Form_Request~E002Id~E004sap.client.SsrClient.form~E005Async~E004false~E005FocusInfo~E004~E005Hash~E004~E005DomChanged~E004false~E005IsDirty~E004false~E003~E002ResponseData~E004delta~E003~E002~E003";
    // let decoded = event::decode_sap_event_encoding(&encoded);

    // println!("{:#?}", decoded);
}
