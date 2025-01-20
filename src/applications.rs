use std::sync::Arc;

use reqwest::{Client, Response};
use thiserror::Error;

use crate::webdynpro::client::{SapSsrClient, SapSsrClientError};

pub mod course_grades;
pub mod course_schedules;

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("SAP SSR 클라이언트 생성 오류: {0}")]
    SapSsrClientError(#[from] SapSsrClientError),
    #[error("HTTP 요청 오류: {0}")]
    RequestError(#[from] reqwest::Error),
}

pub struct Application {
    client: Arc<Client>,
    sap_ssr_client: SapSsrClient,
}

impl Application {
    // SAP SSR Client 정보 획득
    pub async fn new(client: Arc<Client>, app_name: &str) -> Result<Self, ApplicationError> {
        let sap_ssr_client = SapSsrClient::new(client.clone(), app_name).await?;
        Ok(Application {
            client,
            sap_ssr_client,
        })
    }

    // SAP 이벤트 큐 전송
    pub async fn send_request(
        &self,
        sap_event_queue: Option<&str>,
    ) -> Result<Response, ApplicationError> {
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
}
