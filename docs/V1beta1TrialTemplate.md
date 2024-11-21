# V1beta1TrialTemplate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**config_map** | Option<[**crate::models::V1beta1ConfigMapSource**](v1beta1.ConfigMapSource.md)> |  | [optional]
**failure_condition** | Option<**String**> | Condition when trial custom resource is failed. Condition must be in GJSON format, ref https://github.com/tidwall/gjson. For example for BatchJob: status.conditions.#(type==\"Failed\")#|#(status==\"True\")# | [optional]
**primary_container_name** | Option<**String**> | Name of training container where actual model training is running | [optional]
**primary_pod_labels** | Option<**::std::collections::HashMap<String, String>**> | Labels that determines if pod needs to be injected by Katib sidecar container. If PrimaryPodLabels is omitted, metrics collector wraps all Trial's pods. | [optional]
**retain** | Option<**bool**> | Retain indicates that trial resources must be not cleanup | [optional]
**success_condition** | Option<**String**> | Condition when trial custom resource is succeeded. Condition must be in GJSON format, ref https://github.com/tidwall/gjson. For example for BatchJob: status.conditions.#(type==\"Complete\")#|#(status==\"True\")# | [optional]
**trial_parameters** | Option<[**Vec<crate::models::V1beta1TrialParameterSpec>**](v1beta1.TrialParameterSpec.md)> | List of parameters that are used in trial template | [optional]
**trial_spec** | Option<[**crate::models::V1UnstructuredUnstructured**](v1.unstructured.Unstructured.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


