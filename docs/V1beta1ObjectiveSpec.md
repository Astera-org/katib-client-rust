# V1beta1ObjectiveSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**additional_metric_names** | Option<**Vec<String>**> | AdditionalMetricNames represents metrics that should be collected from Trials. This can be empty if we only care about the objective metric. Note: If we adopt a push instead of pull mechanism, this can be omitted completely. | [optional]
**goal** | Option<**f64**> | Goal is the Experiment's objective goal that should be reached. In case of empty goal, Experiment is running until MaxTrialCount = TrialsSucceeded. | [optional]
**metric_strategies** | Option<[**Vec<crate::models::V1beta1MetricStrategy>**](v1beta1.MetricStrategy.md)> | MetricStrategies defines various rules (min, max or latest) to extract metrics values. This field is allowed to missing, experiment defaulter (webhook) will fill it. | [optional]
**objective_metric_name** | Option<**String**> | ObjectiveMetricName represents primary Experiment's metric to optimize. | [optional]
**_type** | Option<**String**> | Type for Experiment optimization. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


