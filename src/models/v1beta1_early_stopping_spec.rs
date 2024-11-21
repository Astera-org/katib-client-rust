/*
 * Katib
 *
 * Swagger description for Katib
 *
 * The version of the OpenAPI document: v1beta1-0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1beta1EarlyStoppingSpec : EarlyStoppingSpec is the specification for a early stopping algorithm.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1beta1EarlyStoppingSpec {
    /// Early stopping algorithm name.
    #[serde(rename = "algorithmName", skip_serializing_if = "Option::is_none")]
    pub algorithm_name: Option<String>,
    /// Key-value pairs representing settings for early stopping algorithm.
    #[serde(rename = "algorithmSettings", skip_serializing_if = "Option::is_none")]
    pub algorithm_settings: Option<Vec<crate::models::V1beta1EarlyStoppingSetting>>,
}

impl V1beta1EarlyStoppingSpec {
    /// EarlyStoppingSpec is the specification for a early stopping algorithm.
    pub fn new() -> V1beta1EarlyStoppingSpec {
        V1beta1EarlyStoppingSpec {
            algorithm_name: None,
            algorithm_settings: None,
        }
    }
}


