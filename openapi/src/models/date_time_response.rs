/*
 * gopro API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DateTimeResponse {
    #[serde(rename = "date", skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    #[serde(rename = "tzone", skip_serializing_if = "Option::is_none")]
    pub tzone: Option<i32>,
    #[serde(rename = "dst", skip_serializing_if = "Option::is_none")]
    pub dst: Option<i32>,
}

impl DateTimeResponse {
    pub fn new() -> DateTimeResponse {
        DateTimeResponse {
            date: None,
            time: None,
            tzone: None,
            dst: None,
        }
    }
}


