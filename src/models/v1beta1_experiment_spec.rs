/*
 * Katib
 *
 * Swagger description for Katib
 *
 * The version of the OpenAPI document: v1beta1-0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1beta1ExperimentSpec : ExperimentSpec is the specification of an Experiment.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1beta1ExperimentSpec {
    #[serde(rename = "algorithm", skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<Box<crate::models::V1beta1AlgorithmSpec>>,
    #[serde(rename = "earlyStopping", skip_serializing_if = "Option::is_none")]
    pub early_stopping: Option<Box<crate::models::V1beta1EarlyStoppingSpec>>,
    /// Max failed trials to mark experiment as failed.
    #[serde(rename = "maxFailedTrialCount", skip_serializing_if = "Option::is_none")]
    pub max_failed_trial_count: Option<i32>,
    /// Max completed trials to mark experiment as succeeded
    #[serde(rename = "maxTrialCount", skip_serializing_if = "Option::is_none")]
    pub max_trial_count: Option<i32>,
    #[serde(rename = "metricsCollectorSpec", skip_serializing_if = "Option::is_none")]
    pub metrics_collector_spec: Option<Box<crate::models::V1beta1MetricsCollectorSpec>>,
    #[serde(rename = "nasConfig", skip_serializing_if = "Option::is_none")]
    pub nas_config: Option<Box<crate::models::V1beta1NasConfig>>,
    #[serde(rename = "objective", skip_serializing_if = "Option::is_none")]
    pub objective: Option<Box<crate::models::V1beta1ObjectiveSpec>>,
    /// How many trials can be processed in parallel. Defaults to 3
    #[serde(rename = "parallelTrialCount", skip_serializing_if = "Option::is_none")]
    pub parallel_trial_count: Option<i32>,
    /// List of hyperparameter configurations.
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<crate::models::V1beta1ParameterSpec>>,
    /// Describes resuming policy which usually take effect after experiment terminated. Default value is Never.
    #[serde(rename = "resumePolicy", skip_serializing_if = "Option::is_none")]
    pub resume_policy: Option<String>,
    #[serde(rename = "trialTemplate", skip_serializing_if = "Option::is_none")]
    pub trial_template: Option<Box<crate::models::V1beta1TrialTemplate>>,
}

impl V1beta1ExperimentSpec {
    /// ExperimentSpec is the specification of an Experiment.
    pub fn new() -> V1beta1ExperimentSpec {
        V1beta1ExperimentSpec {
            algorithm: None,
            early_stopping: None,
            max_failed_trial_count: None,
            max_trial_count: None,
            metrics_collector_spec: None,
            nas_config: None,
            objective: None,
            parallel_trial_count: None,
            parameters: None,
            resume_policy: None,
            trial_template: None,
        }
    }
}


