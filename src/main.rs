use mini_rusaint::session::USaintSession;

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
}
