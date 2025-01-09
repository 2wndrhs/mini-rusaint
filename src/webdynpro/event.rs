use std::{
    collections::{HashMap, VecDeque},
    fmt::{Display, Formatter, Result as FmtResult},
};

use derive_builder::Builder;
use regex::Regex;

const SAP_ENCODED_NEWLINE: &str = "~E001";
const SAP_ENCODED_OPEN_BRACE: &str = "~E002";
const SAP_ENCODED_CLOSE_BRACE: &str = "~E003";
const SAP_ENCODED_COLON: &str = "~E004";
const SAP_ENCODED_COMMA: &str = "~E005";

#[derive(Debug, Builder, Clone)]
#[builder(setter(into))]
pub struct SapEvent {
    pub event: String,
    pub control: String,

    #[builder(setter(each = "add_parameter"))]
    #[builder(default)]
    pub parameters: HashMap<String, String>,

    #[builder(setter(each = "add_ucf_parameter"))]
    #[builder(default)]
    pub ucf_parameters: HashMap<String, String>,

    #[builder(setter(each = "add_custom_parameter"))]
    #[builder(default)]
    pub custom_parameters: HashMap<String, String>,
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

#[derive(Debug, Builder)]
pub struct SapEventQueue {
    #[builder(setter(each = "add_event"))]
    #[builder(default)]
    queue: VecDeque<SapEvent>,
}

impl Display for SapEventQueue {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        for (i, event) in self.queue.iter().enumerate() {
            write!(f, "{}", event)?;
            if i < self.queue.len() - 1 {
                write!(f, "{}", SAP_ENCODED_NEWLINE)?;
            }
        }

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
