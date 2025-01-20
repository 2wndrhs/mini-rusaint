pub mod model;

use std::{collections::HashMap, sync::Arc};

use reqwest::{Client, Response};
use scraper::{ElementRef, Html, Selector};
use thiserror::Error;

use crate::webdynpro::{
    client::SapSsrClientError,
    event::{
        SapEventBuilder, SapEventBuilderError, SapEventQueueBuilder, SapEventQueueBuilderError,
    },
};

use model::{CourseGrade, SemesterGrade, SemesterType};

use super::{Application, ApplicationError};

#[derive(Debug, Error)]
pub enum CourseGradesApplicationError {
    #[error("Application 오류: {0}")]
    ApplicationError(#[from] ApplicationError),
    #[error("SAP SSR 클라이언트 생성 오류: {0}")]
    SapSsrClientError(#[from] SapSsrClientError),
    #[error("HTTP 요청 오류: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("SAP 이벤트 빌더 오류: {0}")]
    SapEventBuilderError(#[from] SapEventBuilderError),
    #[error("SAP 이벤트 큐 빌더 오류: {0}")]
    SapEventQueueBuilderError(#[from] SapEventQueueBuilderError),
    #[error("HTML 파싱 오류")]
    HtmlParseError,
}

pub struct CourseGradesApplication {
    application: Application,
}

impl CourseGradesApplication {
    pub const APP_NAME: &'static str = "ZCMB3W0017";

    const SEMESTER_GRADES_SUMMARY_TABLE_ID: &'static str =
        "ZCMB3W0017.ID_0001:VIW_MAIN.TABLE-contentTBody";
    const SEMESTER_GRADES_DETAIL_TABLE_ID: &'static str =
        "ZCMB3W0017.ID_0001:VIW_MAIN.TABLE_1-contentTBody";
    const COURSE_GRADES_DETAIL_TABLE_ID: &'static str =
        "ZCMB3W0017.ID_0001:V_DETAIL.TABLE-contentTBody";
    const COURSE_GRADES_DETAIL_POPUP_CLOSE_BUTTON_ID: &'static str =
        "ZCMB3W0017.ID_0001:W_POPUP.WDBUTTON_5";
    const YEAR_COMBO_BOX_ID: &'static str =
        "ZCMW_PERIOD_RE.ID_0DC742680F42DA9747594D1AE51A0C69:VIW_MAIN.PERYR";
    const SEMESTER_COMBO_BOX_ID: &'static str =
        "ZCMW_PERIOD_RE.ID_0DC742680F42DA9747594D1AE51A0C69:VIW_MAIN.PERID";

    pub async fn new(
        client: Arc<Client>,
    ) -> Result<CourseGradesApplication, CourseGradesApplicationError> {
        let application = Application::new(client, Self::APP_NAME).await?;
        Ok(CourseGradesApplication { application })
    }

    /// 모든 학기별 성적을 가져옵니다.
    pub async fn get_all_semester_grades(
        &self,
    ) -> Result<Vec<SemesterGrade>, CourseGradesApplicationError> {
        let response = self.application.send_request(None).await?;
        let body = response.text().await?;

        // HTML 문자열 파싱
        let document = Html::parse_document(&body);
        // 학기별 성적 테이블 선택자
        let tbody_selector = Selector::parse(
            format!(r#"[id="{}"]"#, Self::SEMESTER_GRADES_SUMMARY_TABLE_ID).as_str(),
        )
        .unwrap();

        let mut semester_grades = Vec::new();

        for tbody_element in document.select(&tbody_selector) {
            // tbody 요소의 한 단계 아래에 있는 tr 요소들을 순회
            // 첫 번째 tr 요소는 테이블 헤더이므로 스킵
            for child in tbody_element.children().skip(1) {
                if let Some(element) = ElementRef::wrap(child) {
                    // tr 요소이고 rr 속성(row index)이 0이 아닌 경우에만 성적 정보를 가져옴
                    if element.value().name() == "tr" && element.attr("rr") != Some("0") {
                        let semester_grade = SemesterGrade::from_html_element(element);
                        semester_grades.push(semester_grade);
                    }
                }
            }
        }

        Ok(semester_grades)
    }

    /// 주어진 (년도, 학기)의 세부 성적을 가져옵니다.
    /// `fetch_details` 값이 `true`이면 상세 성적(출석, 중간고사, 기말고사..)을 함께 가져옵니다.
    pub async fn get_semester_grades_details(
        &self,
        year: u32,
        semester: SemesterType,
        fetch_details: bool,
    ) -> Result<Vec<CourseGrade>, CourseGradesApplicationError> {
        self.select_year(year).await?;

        let response = self.select_semester(semester).await?;
        let body = response.text().await?;

        // HTML 문자열 파싱
        let document = Html::parse_document(&body);
        // 학기별 세부 성적 테이블 선택자
        let tbody_selector = Selector::parse(
            format!(r#"[id="{}"]"#, Self::SEMESTER_GRADES_DETAIL_TABLE_ID).as_str(),
        )
        .unwrap();

        let mut course_grades = Vec::new();

        for tbody_element in document.select(&tbody_selector) {
            // tbody 요소의 한 단계 아래에 있는 tr 요소들을 순회
            // 첫 번째 tr 요소는 테이블 헤더이므로 스킵
            for child in tbody_element.children().skip(1) {
                if let Some(element) = ElementRef::wrap(child) {
                    // tr 요소이고 rr 속성(row index)이 0이 아닌 경우에만 성적 정보를 가져옴
                    if element.value().name() == "tr" && element.attr("rr") != Some("0") {
                        let mut course_grade = CourseGrade::from_html_element(element);

                        // `fetch_details` 값이 `true`이면 상세 성적을 함께 가져옴
                        if fetch_details {
                            let detailed_grades = self.get_course_grades_details(element).await?;
                            course_grade.detailed_grade = detailed_grades;
                        }

                        course_grades.push(course_grade);
                    }
                }
            }
        }

        Ok(course_grades)
    }

    /// 주어진 과목의 상세 성적 정보(출석, 중간고사, 기말고사..)를 가져옵니다.
    async fn get_course_grades_details<'tr>(
        &self,
        tr_element: ElementRef<'tr>,
    ) -> Result<HashMap<String, f32>, CourseGradesApplicationError> {
        // 5번째 자식 요소인 상세성적 조회 버튼의 id 값 가져오기
        if let Some(td_element) = tr_element.children().nth(4) {
            if let Some(td_element_ref) = ElementRef::wrap(td_element) {
                let button_selector = Selector::parse(r#"[ct="B"]"#).unwrap();

                match td_element_ref.select(&button_selector).next() {
                    Some(button_element) => {
                        let id_value = button_element.value().attr("id").unwrap();

                        let sap_event_queue = SapEventQueueBuilder::default()
                            .add_event(
                                SapEventBuilder::default()
                                    .event("ClientInspector")
                                    .control("Notify")
                                    .add_parameter(("Id".to_string(), "WD01".to_string()))
                                    .add_parameter(("Data".to_string(), "".to_string()))
                                    .build()?,
                            )
                            .add_event(
                                SapEventBuilder::default()
                                    .event("Button")
                                    .control("Press")
                                    .add_parameter(("Id".to_string(), id_value.to_string()))
                                    .build()?,
                            )
                            .build()?
                            .to_string();

                        let response = self
                            .application
                            .send_request(Some(&sap_event_queue))
                            .await?;
                        let body = response.text().await?;

                        // HTML 문자열 파싱
                        let document = Html::parse_document(&body);
                        // 과목 상세 성적 테이블 선택자
                        let tbody_selector = Selector::parse(
                            format!(r#"[id="{}"]"#, Self::COURSE_GRADES_DETAIL_TABLE_ID).as_str(),
                        )
                        .unwrap();

                        let tbody_element = document.select(&tbody_selector).next().unwrap();
                        let detailed_grades = CourseGrade::create_detailed_grades(tbody_element);

                        // 상세 성적 조회 팝업 창 닫기
                        self.close_popup_window().await?;

                        return Ok(detailed_grades);
                    }
                    None => return Ok(HashMap::new()),
                }
            }
        }

        Err(CourseGradesApplicationError::HtmlParseError)
    }

    /// 상세 성적 조회 팝업 창을 닫습니다.
    async fn close_popup_window(&self) -> Result<Response, CourseGradesApplicationError> {
        let sap_event_queue = SapEventBuilder::default()
            .event("Button")
            .control("Press")
            .add_parameter((
                "Id".to_string(),
                Self::COURSE_GRADES_DETAIL_POPUP_CLOSE_BUTTON_ID.to_string(),
            ))
            .build()?
            .to_string();

        let response = self
            .application
            .send_request(Some(&sap_event_queue))
            .await?;

        Ok(response)
    }

    /// 주어진 년도를 선택하는 SAP 이벤트를 발행하고 응답을 반환합니다.
    async fn select_year(&self, year: u32) -> Result<Response, CourseGradesApplicationError> {
        let sap_event_queue = SapEventBuilder::default()
            .event("ComboBox")
            .control("Select")
            .add_parameter(("Id".to_string(), Self::YEAR_COMBO_BOX_ID.to_string()))
            .add_parameter(("Key".to_string(), year.to_string()))
            .build()?
            .to_string();

        let response = self
            .application
            .send_request(Some(&sap_event_queue))
            .await?;

        Ok(response)
    }

    /// 주어진 학기를 선택하는 SAP 이벤트를 발행하고 응답을 반환합니다.
    async fn select_semester(
        &self,
        semester: SemesterType,
    ) -> Result<Response, CourseGradesApplicationError> {
        let sap_event_queue = SapEventBuilder::default()
            .event("ComboBox")
            .control("Select")
            .add_parameter(("Id".to_string(), Self::SEMESTER_COMBO_BOX_ID.to_string()))
            .add_parameter(("Key".to_string(), semester.key().to_string()))
            .build()?
            .to_string();

        let response = self
            .application
            .send_request(Some(&sap_event_queue))
            .await?;

        Ok(response)
    }
}
