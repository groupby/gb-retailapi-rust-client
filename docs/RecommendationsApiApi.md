# \RecommendationsApiApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**predict**](RecommendationsApiApi.md#predict) | **POST** /api/predict | Provide Recommendations AI functionality.
[**predict_v2**](RecommendationsApiApi.md#predict_v2) | **POST** /api/recommendation | Provide Recommendations AI functionality.



## predict

> crate::models::PredictResults predict(x_groupby_customer_id, recommendations_request)
Provide Recommendations AI functionality.

Perform a recommendation request against the GroupBy Retail Recommendations API.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_groupby_customer_id** | **String** | Custom HTTP header which may contain tenant name. Used to define a client by its name. | [required] |
**recommendations_request** | [**RecommendationsRequest**](RecommendationsRequest.md) | Request that should be populated to configure a recommendations API call made by the client. | [required] |

### Return type

[**crate::models::PredictResults**](PredictResults.md)

### Authorization

[GroupByIncEmployee](../README.md#GroupByIncEmployee), [ClientKey](../README.md#ClientKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## predict_v2

> crate::models::PredictResults predict_v2(x_groupby_customer_id, recommendations_request)
Provide Recommendations AI functionality.

Perform a recommendation request against the GroupBy Retail Recommendations API.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_groupby_customer_id** | **String** | Custom HTTP header which may contain tenant name. Used to define a client by its name. | [required] |
**recommendations_request** | [**RecommendationsRequest**](RecommendationsRequest.md) | Request that should be populated to configure a recommendations API call made by the client. | [required] |

### Return type

[**crate::models::PredictResults**](PredictResults.md)

### Authorization

[GroupByIncEmployee](../README.md#GroupByIncEmployee), [ClientKey](../README.md#ClientKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

