use std::env;

use dotenv::dotenv;
use reqwest::{
    header::{self, HeaderMap, HeaderValue, USER_AGENT},
    Client, Result as ReqwestResult,
};

const SMARTID_LOGIN_FORM_REQUEST_URL: &str = "https://smartid.ssu.ac.kr/Symtra_sso/smln_pcs.asp";
const USAINT_SAP_SSO_URL: &str = "https://saint.ssu.ac.kr/webSSO/sso.jsp";
const DEFAULT_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36";

#[derive(Debug)]
struct Credentials {
    id: String,
    password: String,
}

impl Credentials {
    fn from_env() -> Credentials {
        dotenv().ok();
        Credentials {
            id: env::var("USAINT_ID").expect("USAINT_ID가 설정되어야 합니다."),
            password: env::var("USAINT_PASSWORD").expect("USAINT_PASSWORD가 설정되어야 합니다."),
        }
    }
}

pub struct USaintSession {
    pub client: Client,
}

impl USaintSession {
    pub async fn new() -> ReqwestResult<Self> {
        let credentials = Credentials::from_env();

        // 기본 헤더 설정
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static(DEFAULT_USER_AGENT));

        let client = Client::builder()
            .default_headers(headers)
            .cookie_store(true)
            .build()?;

        let s_token = Self::fetch_s_token(&client, &credentials).await?;
        println!("s_token: {}", s_token);

        let sso_token = Self::fetch_sso_token(&client, &s_token, &credentials.id).await?;
        println!("sso_token: {}", sso_token);

        Ok(USaintSession { client })
    }

    async fn fetch_s_token(client: &Client, credentials: &Credentials) -> ReqwestResult<String> {
        let form_data = [("userid", &credentials.id), ("pwd", &credentials.password)];

        let response = client
            .post(SMARTID_LOGIN_FORM_REQUEST_URL)
            .form(&form_data)
            .send()
            .await?;

        let s_token = response
            .cookies()
            .find(|cookie| cookie.name() == "sToken")
            .expect("sToken 쿠키가 없습니다.");

        Ok(s_token.value().to_string())
    }

    async fn fetch_sso_token(client: &Client, s_token: &str, id: &str) -> ReqwestResult<String> {
        let response = client
            .get(USAINT_SAP_SSO_URL)
            .query(&[("sToken", s_token), ("sIdno", id)])
            .send()
            .await?;

        let sso_token = response
            .cookies()
            .find(|cookie| cookie.name() == "MYSAPSSO2")
            .expect("MYSAPSSO2 쿠키가 없습니다.");

        Ok(sso_token.value().to_string())
    }
}
