/*
 * Katib
 *
 * Swagger description for Katib
 *
 * The version of the OpenAPI document: v1beta1-0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1beta1TrialSpec : TrialSpec is the specification of a Trial.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1beta1TrialSpec {
    /// Rules for early stopping techniques. Each rule should be met to early stop Trial.
    #[serde(rename = "earlyStoppingRules", skip_serializing_if = "Option::is_none")]
    pub early_stopping_rules: Option<Vec<crate::models::V1beta1EarlyStoppingRule>>,
    /// Condition when trial custom resource is failed. Condition must be in GJSON format, ref https://github.com/tidwall/gjson. For example for BatchJob: status.conditions.#(type==\"Failed\")#|#(status==\"True\")#
    #[serde(rename = "failureCondition", skip_serializing_if = "Option::is_none")]
    pub failure_condition: Option<String>,
    /// Labels that provide additional metadata for services (e.g. Suggestions tracking)
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "metricsCollector", skip_serializing_if = "Option::is_none")]
    pub metrics_collector: Option<Box<crate::models::V1beta1MetricsCollectorSpec>>,
    #[serde(rename = "objective", skip_serializing_if = "Option::is_none")]
    pub objective: Option<Box<crate::models::V1beta1ObjectiveSpec>>,
    /// Key-value pairs for hyperparameters and assignment values.
    #[serde(rename = "parameterAssignments", skip_serializing_if = "Option::is_none")]
    pub parameter_assignments: Option<Vec<crate::models::V1beta1ParameterAssignment>>,
    /// Name of training container where actual model training is running
    #[serde(rename = "primaryContainerName", skip_serializing_if = "Option::is_none")]
    pub primary_container_name: Option<String>,
    /// Label that determines if pod needs to be injected by Katib sidecar container
    #[serde(rename = "primaryPodLabels", skip_serializing_if = "Option::is_none")]
    pub primary_pod_labels: Option<::std::collections::HashMap<String, String>>,
    /// Whether to retain the trial run object after completed.
    #[serde(rename = "retainRun", skip_serializing_if = "Option::is_none")]
    pub retain_run: Option<bool>,
    #[serde(rename = "runSpec", skip_serializing_if = "Option::is_none")]
    pub run_spec: Option<crate::models::V1UnstructuredUnstructured>,
    /// Condition when trial custom resource is succeeded. Condition must be in GJSON format, ref https://github.com/tidwall/gjson. For example for BatchJob: status.conditions.#(type==\"Complete\")#|#(status==\"True\")#
    #[serde(rename = "successCondition", skip_serializing_if = "Option::is_none")]
    pub success_condition: Option<String>,
}

impl V1beta1TrialSpec {
    /// TrialSpec is the specification of a Trial.
    pub fn new() -> V1beta1TrialSpec {
        V1beta1TrialSpec {
            early_stopping_rules: None,
            failure_condition: None,
            labels: None,
            metrics_collector: None,
            objective: None,
            parameter_assignments: None,
            primary_container_name: None,
            primary_pod_labels: None,
            retain_run: None,
            run_spec: None,
            success_condition: None,
        }
    }
}


