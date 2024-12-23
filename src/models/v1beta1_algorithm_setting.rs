/*
 * Katib
 *
 * Swagger description for Katib
 *
 * The version of the OpenAPI document: v1beta1-0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1beta1AlgorithmSetting : AlgorithmSetting represents key-value pair for HP or NAS algorithm settings.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1beta1AlgorithmSetting {
    /// Name is setting name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Value is the setting value.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl V1beta1AlgorithmSetting {
    /// AlgorithmSetting represents key-value pair for HP or NAS algorithm settings.
    pub fn new() -> V1beta1AlgorithmSetting {
        V1beta1AlgorithmSetting {
            name: None,
            value: None,
        }
    }
}


