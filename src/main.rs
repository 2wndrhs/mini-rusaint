use mini_rusaint::{session::USaintSession, webdynpro::event};

const SSU_WEBDYNPRO_BASE_URL: &str =
    "https://ecc.ssu.ac.kr/sap/bc/webdynpro/SAP/ZCMB3W0017?sap-wd-stableids=x";

#[tokio::main]
async fn main() {
    let session = USaintSession::new()
        .await
        .expect("세션 생성에 실패했습니다.");

    let response = session
        .client
        .get(SSU_WEBDYNPRO_BASE_URL)
        .send()
        .await
        .expect("요청에 실패했습니다.");

    let body = response
        .text()
        .await
        .expect("응답 본문을 읽는데 실패했습니다.");

    println!("{}", body);

    // let encoded = "Form_Submit~E002Id~E004SL__FORM~E003~E002ClientAction~E004submit~E005ActionUrl~E004~E005ResponseData~E004full~E005PrepareScript~E004~E003~E002~E003";
    // let decoded = event::decode_sap_event_encoding(&encoded);

    // println!("{}", decoded);
}
