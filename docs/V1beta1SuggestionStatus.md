# V1beta1SuggestionStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**algorithm_settings** | Option<[**Vec<crate::models::V1beta1AlgorithmSetting>**](v1beta1.AlgorithmSetting.md)> | AlgorithmSettings defines HP or NAS algorithm settings which suggestion gRPC service returns. These settings overwrites Experiment's settings before the gRPC request. It can be empty if settings haven't been changed. | [optional]
**completion_time** | Option<[**crate::models::V1Time**](v1.Time.md)> |  | [optional]
**conditions** | Option<[**Vec<crate::models::V1beta1SuggestionCondition>**](.v1beta1.SuggestionCondition.md)> | List of observed runtime conditions for this Suggestion. | [optional]
**last_reconcile_time** | Option<[**crate::models::V1Time**](v1.Time.md)> |  | [optional]
**start_time** | Option<[**crate::models::V1Time**](v1.Time.md)> |  | [optional]
**suggestion_count** | Option<**i32**> | Number of suggestion results | [optional]
**suggestions** | Option<[**Vec<crate::models::V1beta1TrialAssignment>**](.v1beta1.TrialAssignment.md)> | Suggestion results | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


