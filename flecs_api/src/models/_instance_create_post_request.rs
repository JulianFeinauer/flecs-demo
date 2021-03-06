/*
 * FLECS Daemon API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0-beta.4
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InstanceCreatePostRequest {
    #[serde(rename = "app", skip_serializing_if = "Option::is_none")]
    pub app: Option<String>,
    #[serde(rename = "instanceName", skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl InstanceCreatePostRequest {
    pub fn new() -> InstanceCreatePostRequest {
        InstanceCreatePostRequest {
            app: None,
            instance_name: None,
            version: None,
        }
    }
}


