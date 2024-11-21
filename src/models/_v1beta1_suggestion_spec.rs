/*
 * Katib
 *
 * Swagger description for Katib
 *
 * The version of the OpenAPI document: v1beta1-0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1beta1SuggestionSpec : SuggestionSpec is the specification of a Suggestion.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1beta1SuggestionSpec {
    #[serde(rename = "algorithm", skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<Box<crate::models::V1beta1AlgorithmSpec>>,
    #[serde(rename = "earlyStopping", skip_serializing_if = "Option::is_none")]
    pub early_stopping: Option<Box<crate::models::V1beta1EarlyStoppingSpec>>,
    /// Number of suggestions requested.
    #[serde(rename = "requests", skip_serializing_if = "Option::is_none")]
    pub requests: Option<i32>,
    /// ResumePolicy describes resuming policy which usually take effect after experiment terminated. Default value is Never.
    #[serde(rename = "resumePolicy", skip_serializing_if = "Option::is_none")]
    pub resume_policy: Option<String>,
}

impl V1beta1SuggestionSpec {
    /// SuggestionSpec is the specification of a Suggestion.
    pub fn new() -> V1beta1SuggestionSpec {
        V1beta1SuggestionSpec {
            algorithm: None,
            early_stopping: None,
            requests: None,
            resume_policy: None,
        }
    }
}


