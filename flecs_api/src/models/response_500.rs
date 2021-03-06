/*
 * FLECS Daemon API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0-beta.4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Response500 : Internal error



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Response500 {
    #[serde(rename = "additionalInfo", skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<String>,
}

impl Response500 {
    /// Internal error
    pub fn new() -> Response500 {
        Response500 {
            additional_info: None,
        }
    }
}


