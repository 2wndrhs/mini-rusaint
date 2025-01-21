use scraper::{Html, Selector};
use thiserror::Error;

use crate::session::USaintSession;

#[derive(Debug, Error)]
pub enum SapSsrClientError {
    #[error("HTTP 요청 오류: {0}")]
    RequestError(#[from] reqwest::Error),
}

#[derive(Debug)]
pub struct SapSsrClient {
    pub action_url: String,
    pub charset: String,
    pub wd_secure_id: String,
    pub app_name: String,
    pub use_beacon: bool,
}

impl SapSsrClient {
    pub const SSU_WEBDYNPRO_BASE_URL: &'static str = "https://ecc.ssu.ac.kr";

    pub async fn new(
        session: USaintSession,
        app_name: &str,
    ) -> Result<SapSsrClient, SapSsrClientError> {
        let url = format!(
            "{}/sap/bc/webdynpro/SAP/{}?sap-wd-stableids=x",
            Self::SSU_WEBDYNPRO_BASE_URL,
            app_name
        );

        let response = session.get(url).send().await?;
        let body = response.text().await?;

        // HTML 문자열 파싱
        let document = Html::parse_document(&body);

        let form_selector = Selector::parse("#sap\\.client\\.SsrClient\\.form").unwrap();
        let input_selector = Selector::parse("input").unwrap();

        let mut action_url = String::new();
        let mut charset = String::new();
        let mut wd_secure_id = String::new();
        let mut app_name = String::new();
        let mut use_beacon = false;

        for form_element in document.select(&form_selector) {
            action_url = form_element
                .value()
                .attr("action")
                .unwrap_or("")
                .to_string();

            for input_element in form_element.select(&input_selector) {
                let name = input_element.value().attr("name").unwrap_or("");
                let value = input_element.value().attr("value").unwrap_or("");

                match name {
                    "sap-charset" => charset = value.to_string(),
                    "sap-wd-secure-id" => wd_secure_id = value.to_string(),
                    "fesrAppName" => app_name = value.to_string(),
                    "fesrUseBeacon" => use_beacon = value == "true",
                    _ => {}
                }
            }
        }

        Ok(SapSsrClient {
            action_url,
            charset,
            wd_secure_id,
            app_name,
            use_beacon,
        })
    }
}
