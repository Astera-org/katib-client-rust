# V1beta1ExperimentSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**algorithm** | Option<[**crate::models::V1beta1AlgorithmSpec**](v1beta1.AlgorithmSpec.md)> |  | [optional]
**early_stopping** | Option<[**crate::models::V1beta1EarlyStoppingSpec**](v1beta1.EarlyStoppingSpec.md)> |  | [optional]
**max_failed_trial_count** | Option<**i32**> | Max failed trials to mark experiment as failed. | [optional]
**max_trial_count** | Option<**i32**> | Max completed trials to mark experiment as succeeded | [optional]
**metrics_collector_spec** | Option<[**crate::models::V1beta1MetricsCollectorSpec**](v1beta1.MetricsCollectorSpec.md)> |  | [optional]
**nas_config** | Option<[**crate::models::V1beta1NasConfig**](v1beta1.NasConfig.md)> |  | [optional]
**objective** | Option<[**crate::models::V1beta1ObjectiveSpec**](v1beta1.ObjectiveSpec.md)> |  | [optional]
**parallel_trial_count** | Option<**i32**> | How many trials can be processed in parallel. Defaults to 3 | [optional]
**parameters** | Option<[**Vec<crate::models::V1beta1ParameterSpec>**](v1beta1.ParameterSpec.md)> | List of hyperparameter configurations. | [optional]
**resume_policy** | Option<**String**> | Describes resuming policy which usually take effect after experiment terminated. Default value is Never. | [optional]
**trial_template** | Option<[**crate::models::V1beta1TrialTemplate**](v1beta1.TrialTemplate.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


