# V1beta1ExperimentStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**completion_time** | Option<[**crate::models::V1Time**](v1.Time.md)> |  | [optional]
**conditions** | Option<[**Vec<crate::models::V1beta1ExperimentCondition>**](v1beta1.ExperimentCondition.md)> | List of observed runtime conditions for this Experiment. | [optional]
**current_optimal_trial** | Option<[**crate::models::V1beta1OptimalTrial**](v1beta1.OptimalTrial.md)> |  | [optional]
**early_stopped_trial_list** | Option<**Vec<String>**> | List of trial names which have been early stopped. | [optional]
**failed_trial_list** | Option<**Vec<String>**> | List of trial names which have already failed. | [optional]
**killed_trial_list** | Option<**Vec<String>**> | List of trial names which have been killed. | [optional]
**last_reconcile_time** | Option<[**crate::models::V1Time**](v1.Time.md)> |  | [optional]
**metrics_unavailable_trial_list** | Option<**Vec<String>**> | List of trial names which have been metrics unavailable | [optional]
**pending_trial_list** | Option<**Vec<String>**> | List of trial names which are pending. | [optional]
**running_trial_list** | Option<**Vec<String>**> | List of trial names which are running. | [optional]
**start_time** | Option<[**crate::models::V1Time**](v1.Time.md)> |  | [optional]
**succeeded_trial_list** | Option<**Vec<String>**> | List of trial names which have already succeeded. | [optional]
**trial_metrics_unavailable** | Option<**i32**> | How many trials are currently metrics unavailable. | [optional]
**trials** | Option<**i32**> | Trials is the total number of trials owned by the experiment. | [optional]
**trials_early_stopped** | Option<**i32**> | How many trials are currently early stopped. | [optional]
**trials_failed** | Option<**i32**> | How many trials have failed. | [optional]
**trials_killed** | Option<**i32**> | How many trials have been killed. | [optional]
**trials_pending** | Option<**i32**> | How many trials are currently pending. | [optional]
**trials_running** | Option<**i32**> | How many trials are currently running. | [optional]
**trials_succeeded** | Option<**i32**> | How many trials have succeeded. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


