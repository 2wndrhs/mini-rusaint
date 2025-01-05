use std::{
    collections::HashMap,
    fmt::{Display, Formatter, Result as FmtResult},
};

use derive_builder::Builder;
use regex::Regex;

const SAP_ENCODED_NEWLINE: &str = "~E001";
const SAP_ENCODED_OPEN_BRACE: &str = "~E002";
const SAP_ENCODED_CLOSE_BRACE: &str = "~E003";
const SAP_ENCODED_COLON: &str = "~E004";
const SAP_ENCODED_COMMA: &str = "~E005";

#[derive(Debug, Builder)]
#[builder(setter(into))]
pub struct SapEvent {
    event: String,
    control: String,

    #[builder(setter(each = "add_parameter"))]
    #[builder(default)]
    parameters: HashMap<String, String>,

    #[builder(setter(each = "add_ucf_parameter"))]
    #[builder(default)]
    ucf_parameters: HashMap<String, String>,

    #[builder(setter(each = "add_custom_parameter"))]
    #[builder(default)]
    custom_parameters: HashMap<String, String>,
}

impl Display for SapEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}_{}", self.event, self.control)?;

        write!(f, "{}", SAP_ENCODED_OPEN_BRACE)?;
        for (i, (key, value)) in self.parameters.iter().enumerate() {
            write!(f, "{}{}{}", key, SAP_ENCODED_COLON, encode_sap_event(value))?;
            if i < self.parameters.len() - 1 {
                write!(f, "{}", SAP_ENCODED_COMMA)?;
            }
        }
        write!(f, "{}", SAP_ENCODED_CLOSE_BRACE)?;

        write!(f, "{}", SAP_ENCODED_OPEN_BRACE)?;
        for (i, (key, value)) in self.ucf_parameters.iter().enumerate() {
            write!(f, "{}{}{}", key, SAP_ENCODED_COLON, encode_sap_event(value))?;
            if i < self.ucf_parameters.len() - 1 {
                write!(f, "{}", SAP_ENCODED_COMMA)?;
            }
        }
        write!(f, "{}", SAP_ENCODED_CLOSE_BRACE)?;

        write!(f, "{}", SAP_ENCODED_OPEN_BRACE)?;
        for (i, (key, value)) in self.custom_parameters.iter().enumerate() {
            write!(f, "{}{}{}", key, SAP_ENCODED_COLON, encode_sap_event(value))?;
            if i < self.custom_parameters.len() - 1 {
                write!(f, "{}", SAP_ENCODED_COMMA)?;
            }
        }
        write!(f, "{}", SAP_ENCODED_CLOSE_BRACE)?;

        Ok(())
    }
}

/// 문자열을 sap event가 URL에서 전달되는 방식으로 인코딩합니다.
pub fn encode_sap_event(input: &str) -> String {
    let mut encoded = String::new();

    for ch in input.chars() {
        match ch {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => encoded.push(ch),
            _ => encoded.push_str(format!("~{:04X}", ch as u32).as_str()),
        }
    }

    encoded
}

/// sap event encoding된 문자열을 디코딩합니다.
pub fn decode_sap_event(encoded: &str) -> String {
    // sap event encoding에서 쓰이는 특수 문자 처리
    let mut replacements = HashMap::new();
    replacements.insert(SAP_ENCODED_NEWLINE, "\n");
    replacements.insert(SAP_ENCODED_OPEN_BRACE, "{");
    replacements.insert(SAP_ENCODED_CLOSE_BRACE, "}");
    replacements.insert(SAP_ENCODED_COLON, ":");
    replacements.insert(SAP_ENCODED_COMMA, ",");

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
            // 변환이 실패하면 원래 문자열 반환
            caps[0].to_string()
        })
        .to_string();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sap_event_decode() {
        let input = "Button_Press~E002Id~E004ZCMW_PERIOD_RE.ID_0DC742680F42DA9747594D1AE51A0C69~003AVIW_MAIN.BUTTON_PREV~E003~E002ResponseData~E004delta~E005ClientAction~E004submit~E003~E002~E003";
        let expected = "Button_Press{Id:ZCMW_PERIOD_RE.ID_0DC742680F42DA9747594D1AE51A0C69:VIW_MAIN.BUTTON_PREV}{ResponseData:delta,ClientAction:submit}{}";
        assert_eq!(decode_sap_event(input), expected);
    }

    #[test]
    fn test_sap_event_builder() {
        let sap_event = SapEventBuilder::default()
            .event("ComboBox")
            .control("Select")
            .add_parameter((
                "Id".to_string(),
                "ZCMW_PERIOD_RE.ID_0DC742680F42DA9747594D1AE51A0C69:VIW_MAIN.PERYR".to_string(),
            ))
            .add_parameter(("Key".to_string(), "2024".to_string()))
            .add_ucf_parameter(("ResponseData".to_string(), "delta".to_string()))
            .add_ucf_parameter(("ClientAction".to_string(), "submit".to_string()))
            .build()
            .unwrap();

        assert_eq!(sap_event.event, "ComboBox");
        assert_eq!(sap_event.control, "Select");
        assert_eq!(
            sap_event.parameters.get("Id").unwrap(),
            "ZCMW_PERIOD_RE.ID_0DC742680F42DA9747594D1AE51A0C69:VIW_MAIN.PERYR"
        );
        assert_eq!(
            sap_event.ucf_parameters.get("ResponseData").unwrap(),
            "delta"
        );

        // Test the Display implementation
        let sap_event_str = sap_event.to_string();

        assert!(sap_event_str.contains("ComboBox_Select"));
        assert!(sap_event_str.contains(
            "Id~E004ZCMW_PERIOD_RE.ID_0DC742680F42DA9747594D1AE51A0C69~003AVIW_MAIN.PERYR"
        ));
        assert!(sap_event_str.contains("Key~E0042024"));
        assert!(sap_event_str.contains("ResponseData~E004delta"));
        assert!(sap_event_str.contains("ClientAction~E004submit"));
        assert!(sap_event_str.contains("~E002~E003"));
    }
}
