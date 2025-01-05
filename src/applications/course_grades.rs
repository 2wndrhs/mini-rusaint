use std::sync::Arc;

use reqwest::Client;
use scraper::{ElementRef, Html, Selector};
use thiserror::Error;

use crate::webdynpro::client::{SapSsrClient, SapSsrClientError};

use super::model::SemesterGrade;

#[derive(Debug, Error)]
pub enum CourseGradesApplicationError {
    #[error("SAP SSR 클라이언트 생성 오류: {0}")]
    SapSsrClientError(#[from] SapSsrClientError),
    #[error("HTTP 요청 오류: {0}")]
    RequestError(#[from] reqwest::Error),
}

pub struct CourseGradesApplication {
    client: Arc<Client>,
    sap_ssr_client: SapSsrClient,
}

impl CourseGradesApplication {
    pub const APP_NAME: &'static str = "ZCMB3W0017";
    const SEMESTER_GRADES_SUMMARY_TABLE_SELECTOR: &'static str =
        "#ZCMB3W0017\\.ID_0001\\:VIW_MAIN\\.TABLE-contentTBody";

    pub async fn new(
        client: Arc<Client>,
    ) -> Result<CourseGradesApplication, CourseGradesApplicationError> {
        // SAP SSR Client 정보 획득
        let sap_ssr_client = SapSsrClient::new(client.clone(), Self::APP_NAME).await?;
        Ok(CourseGradesApplication {
            client,
            sap_ssr_client,
        })
    }

    /// 모든 학기별 성적을 가져옵니다.
    pub async fn get_all_semester_grades(
        &self,
    ) -> Result<Vec<SemesterGrade>, CourseGradesApplicationError> {
        let url = format!(
            "{}/{}",
            SapSsrClient::SSU_WEBDYNPRO_BASE_URL,
            self.sap_ssr_client.action_url
        );

        let form_data = [
            ("charset", self.sap_ssr_client.charset.as_str()),
            (
                "sap-wd-secure-id",
                self.sap_ssr_client.wd_secure_id.as_str(),
            ),
            ("fesrAppName", self.sap_ssr_client.app_name.as_str()),
            ("fesrUseBeacon", &self.sap_ssr_client.use_beacon.to_string()),
            // ("SAPEVENTQUEUE", "ClientInspector_Notify~E002Id~E004WD01~E005Data~E004ClientWidth~003A1213px~003BClientHeight~003A884px~003BScreenWidth~003A1512px~003BScreenHeight~003A982px~003BScreenOrientation~003Alandscape~003BThemedTableRowHeight~003A33px~003BThemedFormLayoutRowHeight~003A32px~003BThemedSvgLibUrls~003A~007B~0022SAPGUI-icons~0022~003A~0022https~003A~002F~002Fecc.ssu.ac.kr~002Fsap~002Fpublic~002Fbc~002Fur~002Fnw5~002Fthemes~002F~007Ecache-20220217154731~002FBase~002FbaseLib~002Fsap_fiori_3~002Fsvg~002Flibs~002FSAPGUI-icons.svg~0022~002C~0022SAPWeb-icons~0022~003A~0022https~003A~002F~002Fecc.ssu.ac.kr~002Fsap~002Fpublic~002Fbc~002Fur~002Fnw5~002Fthemes~002F~007Ecache-20220217154731~002FBase~002FbaseLib~002Fsap_fiori_3~002Fsvg~002Flibs~002FSAPWeb-icons.svg~0022~007D~003BThemeTags~003AFiori_3~002CTouch~003BThemeID~003Asap_fiori_3~003BSapThemeID~003Asap_fiori_3~003BDeviceType~003ADESKTOP~003BDocumentDomain~003Aecc.ssu.ac.kr~003BClientURL~003Ahttps~003A~002F~002Fecc.ssu.ac.kr~002Fsap~002Fbc~002Fwebdynpro~002FSAP~002FZCMB3W0017~003Fsap-wd-stableids~003Dx~0023~003BIsTopWindow~003ATRUE~003BParentAccessible~003ATRUE~E003~E002ResponseData~E004delta~E005EnqueueCardinality~E004single~E003~E002~E003~E001ClientInspector_Notify~E002Id~E004WD02~E005Data~E004ThemedTableRowHeight~003A25px~E003~E002ResponseData~E004delta~E005EnqueueCardinality~E004single~E003~E002~E003~E001LoadingPlaceHolder_Load~E002Id~E004_loadingPlaceholder_~E003~E002ResponseData~E004delta~E005ClientAction~E004submit~E003~E002~E003~E001Form_Request~E002Id~E004sap.client.SsrClient.form~E005Async~E004false~E005FocusInfo~E004~E005Hash~E004~E005DomChanged~E004false~E005IsDirty~E004false~E003~E002ResponseData~E004delta~E003~E002~E003".to_string())
        ];

        let response = self.client.post(&url).form(&form_data).send().await?;
        let body = response.text().await?;

        // HTML 문자열 파싱
        let document = Html::parse_document(&body);

        // 학기별 성적 테이블 선택자
        let tbody_selector = Selector::parse(Self::SEMESTER_GRADES_SUMMARY_TABLE_SELECTOR).unwrap();

        let mut semester_grades = Vec::new();

        for tbody_element in document.select(&tbody_selector) {
            // tbody 요소의 한 단계 아래에 있는 tr 요소들을 순회
            for (index, child) in tbody_element.children().enumerate() {
                // 첫 번째 tr 요소는 테이블 헤더이므로 스킵
                if index == 0 {
                    continue;
                }

                if let Some(element) = ElementRef::wrap(child) {
                    if element.value().name() == "tr" {
                        let semester_grade = SemesterGrade::from_html_element(element);
                        semester_grades.push(semester_grade);
                    }
                }
            }
        }

        Ok(semester_grades)
    }
}
