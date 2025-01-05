use std::sync::Arc;

use reqwest::{Client, Response};
use scraper::{ElementRef, Html, Selector};
use thiserror::Error;

use crate::webdynpro::{
    client::{SapSsrClient, SapSsrClientError},
    event::{SapEventBuilder, SapEventBuilderError},
};

use super::model::{CourseGrade, SemesterGrade, SemesterType};

#[derive(Debug, Error)]
pub enum CourseGradesApplicationError {
    #[error("SAP SSR 클라이언트 생성 오류: {0}")]
    SapSsrClientError(#[from] SapSsrClientError),
    #[error("HTTP 요청 오류: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("SAP 이벤트 빌더 오류: {0}")]
    SapEventBuilderError(#[from] SapEventBuilderError),
}

pub struct CourseGradesApplication {
    client: Arc<Client>,
    sap_ssr_client: SapSsrClient,
}

impl CourseGradesApplication {
    pub const APP_NAME: &'static str = "ZCMB3W0017";

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

    async fn send_request(
        &self,
        sap_event_queue: Option<&str>,
    ) -> Result<Response, CourseGradesApplicationError> {
        let url = format!(
            "{}/{}",
            SapSsrClient::SSU_WEBDYNPRO_BASE_URL,
            self.sap_ssr_client.action_url
        );

        let use_beacon_str = self.sap_ssr_client.use_beacon.to_string();
        let mut form_data = vec![
            ("charset", self.sap_ssr_client.charset.as_str()),
            (
                "sap-wd-secure-id",
                self.sap_ssr_client.wd_secure_id.as_str(),
            ),
            ("fesrAppName", self.sap_ssr_client.app_name.as_str()),
            ("fesrUseBeacon", use_beacon_str.as_str()),
        ];

        if let Some(event_queue) = sap_event_queue {
            form_data.push(("SAPEVENTQUEUE", event_queue));
        }

        let response = self.client.post(&url).form(&form_data).send().await?;

        Ok(response)
    }

    /// 모든 학기별 성적을 가져옵니다.
    pub async fn get_all_semester_grades(
        &self,
    ) -> Result<Vec<SemesterGrade>, CourseGradesApplicationError> {
        const SEMESTER_GRADES_SUMMARY_TABLE_ID: &'static str =
            "ZCMB3W0017.ID_0001:VIW_MAIN.TABLE-contentTBody";

        let response = self.send_request(None).await?;
        let body = response.text().await?;

        // HTML 문자열 파싱
        let document = Html::parse_document(&body);
        // 학기별 성적 테이블 선택자
        let tbody_selector =
            Selector::parse(format!(r#"[id="{}"]"#, SEMESTER_GRADES_SUMMARY_TABLE_ID).as_str())
                .unwrap();

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

    /// 주어진 (년도, 학기)의 세부 성적을 가져옵니다.
    pub async fn get_semester_grades_details(
        &self,
        year: u32,
        semester: SemesterType,
    ) -> Result<Vec<CourseGrade>, CourseGradesApplicationError> {
        let response = self.select_year(year).await?;
        let body = response.text().await?;

        println!("{}", body);

        todo!()
    }

    async fn select_year(&self, year: u32) -> Result<Response, CourseGradesApplicationError> {
        const YEAR_COMBO_BOX_ID: &'static str =
            "ZCMW_PERIOD_RE.ID_0DC742680F42DA9747594D1AE51A0C69:VIW_MAIN.PERYR";

        let sap_event_queue = SapEventBuilder::default()
            .event("ComboBox")
            .control("Select")
            .add_parameter(("Id".to_string(), YEAR_COMBO_BOX_ID.to_string()))
            .add_parameter(("Key".to_string(), year.to_string()))
            .add_ucf_parameter(("ResponseData".to_string(), "delta".to_string()))
            .add_ucf_parameter(("ClientAction".to_string(), "submit".to_string()))
            .build()?
            .to_string();

        let response = self.send_request(Some(&sap_event_queue)).await?;

        Ok(response)
    }
}
