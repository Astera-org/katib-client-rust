# V1beta1TrialSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**early_stopping_rules** | Option<[**Vec<crate::models::V1beta1EarlyStoppingRule>**](v1beta1.EarlyStoppingRule.md)> | Rules for early stopping techniques. Each rule should be met to early stop Trial. | [optional]
**failure_condition** | Option<**String**> | Condition when trial custom resource is failed. Condition must be in GJSON format, ref https://github.com/tidwall/gjson. For example for BatchJob: status.conditions.#(type==\"Failed\")#|#(status==\"True\")# | [optional]
**labels** | Option<**::std::collections::HashMap<String, String>**> | Labels that provide additional metadata for services (e.g. Suggestions tracking) | [optional]
**metrics_collector** | Option<[**crate::models::V1beta1MetricsCollectorSpec**](v1beta1.MetricsCollectorSpec.md)> |  | [optional]
**objective** | Option<[**crate::models::V1beta1ObjectiveSpec**](v1beta1.ObjectiveSpec.md)> |  | [optional]
**parameter_assignments** | Option<[**Vec<crate::models::V1beta1ParameterAssignment>**](v1beta1.ParameterAssignment.md)> | Key-value pairs for hyperparameters and assignment values. | [optional]
**primary_container_name** | Option<**String**> | Name of training container where actual model training is running | [optional]
**primary_pod_labels** | Option<**::std::collections::HashMap<String, String>**> | Label that determines if pod needs to be injected by Katib sidecar container | [optional]
**retain_run** | Option<**bool**> | Whether to retain the trial run object after completed. | [optional]
**run_spec** | Option<[**crate::models::V1UnstructuredUnstructured**](v1.unstructured.Unstructured.md)> |  | [optional]
**success_condition** | Option<**String**> | Condition when trial custom resource is succeeded. Condition must be in GJSON format, ref https://github.com/tidwall/gjson. For example for BatchJob: status.conditions.#(type==\"Complete\")#|#(status==\"True\")# | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


