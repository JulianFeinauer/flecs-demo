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
pub struct InstalledAppAllOf {
    #[serde(rename = "versions", skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<crate::models::AppVersions>>,
    #[serde(rename = "instances", skip_serializing_if = "Option::is_none")]
    pub instances: Option<Vec<crate::models::AppInstances>>,
}

impl InstalledAppAllOf {
    pub fn new() -> InstalledAppAllOf {
        InstalledAppAllOf {
            versions: None,
            instances: None,
        }
    }
}


