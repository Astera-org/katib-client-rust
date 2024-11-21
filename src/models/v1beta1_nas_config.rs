/*
 * Katib
 *
 * Swagger description for Katib
 *
 * The version of the OpenAPI document: v1beta1-0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1beta1NasConfig : NasConfig contains config for NAS job



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1beta1NasConfig {
    #[serde(rename = "graphConfig", skip_serializing_if = "Option::is_none")]
    pub graph_config: Option<Box<crate::models::V1beta1GraphConfig>>,
    #[serde(rename = "operations", skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<crate::models::V1beta1Operation>>,
}

impl V1beta1NasConfig {
    /// NasConfig contains config for NAS job
    pub fn new() -> V1beta1NasConfig {
        V1beta1NasConfig {
            graph_config: None,
            operations: None,
        }
    }
}


