use std::{env, sync::Arc};

use dotenv::dotenv;
use reqwest::{
    cookie::{CookieStore, Jar},
    header::{HeaderMap, HeaderValue, USER_AGENT},
    Client, Error as ReqwestError, Response, Result as ReqwestResult, Url,
};
use thiserror::Error;

const SAP_LOGIN_FORM_REQUEST_URL: &str =
    "https://hana-prd-ap-4.ssu.ac.kr:8443/sap/bc/webdynpro/sap";
const DEFAULT_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36";

#[derive(Debug, Error)]
pub enum USaintSessionError {
    #[error("횐경 변수 오류: {0}")]
    EnvVarError(#[from] env::VarError),
    #[error("HTTP 요청 오류: {0}")]
    RequestError(#[from] ReqwestError),
    #[error("MYSAPSSO2 쿠키가 존재하지 않습니다.")]
    MissingMYSAPSSO2Cookie,
}

#[derive(Debug)]
struct Credentials {
    id: String,
    password: String,
}

impl Credentials {
    fn from_env() -> Result<Credentials, USaintSessionError> {
        dotenv().ok();
        Ok(Credentials {
            id: env::var("USAINT_ID")?,
            password: env::var("USAINT_PASSWORD")?,
        })
    }
}

pub struct USaintSession {
    pub client: Arc<Client>,
}

impl USaintSession {
    pub async fn new() -> Result<Self, USaintSessionError> {
        // env 파일에서 유세인트 사용자 정보 획득
        let credentials = Credentials::from_env()?;

        println!("USAINT_ID: {}", credentials.id);
        println!("USAINT_PASSWORD: {}", credentials.password);

        // 기본 헤더 설정
        let mut headers = HeaderMap::new();

        headers.insert(USER_AGENT, HeaderValue::from_static(DEFAULT_USER_AGENT));

        let cookie_store = Arc::new(Jar::default());
        let client = Client::builder()
            .default_headers(headers)
            .cookie_provider(cookie_store.clone())
            .build()?;

        // SAP SSO 토큰 발급
        Self::fetch_sso_token(&client, &credentials).await?;

        // 쿠키 저장소에 "MYSAPSSO2" 쿠키가 있는지 확인
        let parsed_url = Url::parse(SAP_LOGIN_FORM_REQUEST_URL).unwrap();
        match cookie_store.cookies(&parsed_url) {
            Some(cookie) => {
                if cookie.to_str().unwrap().contains("MYSAPSSO2") {
                    return Ok(USaintSession {
                        client: Arc::new(client),
                    });
                }

                return Err(USaintSessionError::MissingMYSAPSSO2Cookie);
            }
            None => Err(USaintSessionError::MissingMYSAPSSO2Cookie),
        }
    }

    async fn fetch_sso_token(
        client: &Client,
        credentials: &Credentials,
    ) -> ReqwestResult<Response> {
        let form_data = [
            ("sap-user", credentials.id.as_str()),
            ("sap-password", credentials.password.as_str()),
            ("sap-system-login", "onLogin"),
        ];

        let response = client
            .post(SAP_LOGIN_FORM_REQUEST_URL)
            .form(&form_data)
            .send()
            .await?;

        Ok(response)
    }
}
