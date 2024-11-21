/*
 * Katib
 *
 * Swagger description for Katib
 *
 * The version of the OpenAPI document: v1beta1-0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1beta1TrialParameterSpec : TrialParameterSpec describes parameters that must be replaced in trial template



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1beta1TrialParameterSpec {
    /// Description of the parameter
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Name of the parameter that must be replaced in trial template
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Reference to the parameter in search space
    #[serde(rename = "reference", skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}

impl V1beta1TrialParameterSpec {
    /// TrialParameterSpec describes parameters that must be replaced in trial template
    pub fn new() -> V1beta1TrialParameterSpec {
        V1beta1TrialParameterSpec {
            description: None,
            name: None,
            reference: None,
        }
    }
}

