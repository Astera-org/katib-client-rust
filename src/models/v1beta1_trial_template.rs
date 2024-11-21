/*
 * Katib
 *
 * Swagger description for Katib
 *
 * The version of the OpenAPI document: v1beta1-0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1beta1TrialTemplate : TrialTemplate describes structure of trial template



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1beta1TrialTemplate {
    #[serde(rename = "configMap", skip_serializing_if = "Option::is_none")]
    pub config_map: Option<Box<crate::models::V1beta1ConfigMapSource>>,
    /// Condition when trial custom resource is failed. Condition must be in GJSON format, ref https://github.com/tidwall/gjson. For example for BatchJob: status.conditions.#(type==\"Failed\")#|#(status==\"True\")#
    #[serde(rename = "failureCondition", skip_serializing_if = "Option::is_none")]
    pub failure_condition: Option<String>,
    /// Name of training container where actual model training is running
    #[serde(rename = "primaryContainerName", skip_serializing_if = "Option::is_none")]
    pub primary_container_name: Option<String>,
    /// Labels that determines if pod needs to be injected by Katib sidecar container. If PrimaryPodLabels is omitted, metrics collector wraps all Trial's pods.
    #[serde(rename = "primaryPodLabels", skip_serializing_if = "Option::is_none")]
    pub primary_pod_labels: Option<::std::collections::HashMap<String, String>>,
    /// Retain indicates that trial resources must be not cleanup
    #[serde(rename = "retain", skip_serializing_if = "Option::is_none")]
    pub retain: Option<bool>,
    /// Condition when trial custom resource is succeeded. Condition must be in GJSON format, ref https://github.com/tidwall/gjson. For example for BatchJob: status.conditions.#(type==\"Complete\")#|#(status==\"True\")#
    #[serde(rename = "successCondition", skip_serializing_if = "Option::is_none")]
    pub success_condition: Option<String>,
    /// List of parameters that are used in trial template
    #[serde(rename = "trialParameters", skip_serializing_if = "Option::is_none")]
    pub trial_parameters: Option<Vec<crate::models::V1beta1TrialParameterSpec>>,
    #[serde(rename = "trialSpec", skip_serializing_if = "Option::is_none")]
    pub trial_spec: Option<crate::models::V1UnstructuredUnstructured>,
}

impl V1beta1TrialTemplate {
    /// TrialTemplate describes structure of trial template
    pub fn new() -> V1beta1TrialTemplate {
        V1beta1TrialTemplate {
            config_map: None,
            failure_condition: None,
            primary_container_name: None,
            primary_pod_labels: None,
            retain: None,
            success_condition: None,
            trial_parameters: None,
            trial_spec: None,
        }
    }
}

