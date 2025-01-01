use std::collections::HashMap;

use regex::Regex;

pub fn decode_sap_event_encoding(encoded: &str) -> String {
    // sap event encoding에서 쓰이는 특수 문자 처리
    let mut replacements = HashMap::new();
    replacements.insert("~E001", "\n");
    replacements.insert("~E002", "{");
    replacements.insert("~E003", "}");
    replacements.insert("~E004", ":");
    replacements.insert("~E005", ",");

    // 특수 문자 변환
    let mut result = encoded.to_string();
    for (key, value) in replacements {
        result = result.replace(key, value);
    }

    // ~XXXX 형식의 문자열에 매칭되는 정규식 생성
    let hex_pattern = Regex::new(r"~([0-9A-Fa-f]{4})").unwrap();

    // ~XXXX 형식의 문자열 변환
    result = hex_pattern
        .replace_all(&result, |caps: &regex::Captures| {
            // XXXX 형식의 문자열을 16진수로 변환 후 문자로 변환
            let hex_str = &caps[1];
            if let Ok(value) = u32::from_str_radix(hex_str, 16) {
                if let Some(c) = char::from_u32(value) {
                    return c.to_string();
                }
            }
            // 디코딩이 실패하면 원래 문자열 반환
            caps[0].to_string()
        })
        .to_string();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_inspector_decode() {
        let input =
            "ClientInspector_Notify~E002Id~E004WD01~E005Data~E004ClientWidth~003A995px~E003~E002ResponseData~E004delta~E005EnqueueCardinality~E004single~E003~E002~E003";
        let expected = "ClientInspector_Notify{Id:WD01,Data:ClientWidth:995px}{ResponseData:delta,EnqueueCardinality:single}{}";
        assert_eq!(decode_sap_event_encoding(input), expected);
    }

    #[test]
    fn test_button_press_decode() {
        let input = "Button_Press~E002Id~E004ZCMW_PERIOD_RE.ID_0DC742680F42DA9747594D1AE51A0C69~003AVIW_MAIN.BUTTON_PREV~E003~E002ResponseData~E004delta~E005ClientAction~E004submit~E003~E002~E003";
        let expected = "Button_Press{Id:ZCMW_PERIOD_RE.ID_0DC742680F42DA9747594D1AE51A0C69:VIW_MAIN.BUTTON_PREV}{ResponseData:delta,ClientAction:submit}{}";
        assert_eq!(decode_sap_event_encoding(input), expected);
    }

    #[test]
    fn test_form_request_decode() {
        let input = "Form_Request~E002Id~E004sap.client.SsrClient.form~E005Async~E004false~E005FocusInfo~E004~0040~007B~0022sFocussedId~0022~003A~0022BUTTON_ID~0022~007D~E005Hash~E004~E005DomChanged~E004false~E005IsDirty~E004false~E003~E002ResponseData~E004delta~E003~E002~E003";
        let expected = "Form_Request{Id:sap.client.SsrClient.form,Async:false,FocusInfo:@{\"sFocussedId\":\"BUTTON_ID\"},Hash:,DomChanged:false,IsDirty:false}{ResponseData:delta}{}";
        assert_eq!(decode_sap_event_encoding(input), expected);
    }
}
