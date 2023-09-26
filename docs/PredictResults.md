# PredictResults

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**warnings** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | Warnings collected with validation and Recommendations AI API issues. | [optional]
**products** | Option<[**Vec<::std::collections::HashMap<String, serde_json::Value>>**](map.md)> | Recommendations built by Recommendations AI model. | [optional]
**records** | Option<[**Vec<::std::collections::HashMap<String, serde_json::Value>>**](map.md)> | Recommendations built by Recommendations AI model. | [optional]
**model_id** | Option<**String**> | Model Id used for predictions | [optional]
**model_name** | Option<**String**> | Model Name used for predictions | [optional]
**model_type** | Option<**String**> |   Currently supported values:   `recommended-for-you`   `others-you-may-like`,   `frequently-bought-together`   `page-optimization`   `similar-items`,   `buy-it-again`   `on-sale-items`   `recently-viewed`    This field together with optimization_objective describe model metadata to use to control model training and   serving. See https://cloud.google.com/retail/docs/models for more details.  | [optional]
**optimization_objective** | Option<**String**> |   Currently supported values: `ctr`, `cvr`, `revenue-per-order`.     If not specified, we choose default based on model type. Default depends on type of recommendation:   `recommended-for-you` => `ctr`   `others-you-may-like` => `ctr`   `frequently-bought-together` => `revenue_per_order`    This field together with modelType describe model metadata to use to control model training and serving.   See https://cloud.google.com/retail/docs/models for more details on what the model metadata control and which   combination of parameters are valid.  | [optional]
**filter_set** | Option<**String**> | Filter set applied to the recommendation | [optional]
**raw_filter** | Option<**String**> | RawFilter applied to the recommendation | [optional]
**filters** | Option<[**Vec<crate::models::FilterParameter>**](FilterParameter.md)> | Filters applied to the recommendation | [optional]
**metadata** | Option<[**serde_json::Value**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


