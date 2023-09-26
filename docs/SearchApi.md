# \SearchApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**facet_search**](SearchApi.md#facet_search) | **POST** /api/search/facet | Provided search functionality
[**search**](SearchApi.md#search) | **POST** /api/search | Provided search functionality



## facet_search

> crate::models::FacetSearchResponseDto facet_search(x_groupby_customer_id, facet_search_request_dto)
Provided search functionality

Perform a facet search against the GroupBy Retail Search API.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_groupby_customer_id** | **String** | Custom HTTP header which may contain tenant name. Used to define a client by its name. | [required] |
**facet_search_request_dto** | [**FacetSearchRequestDto**](FacetSearchRequestDto.md) | Request that should be populated to configure a search API call, made by the client on behalf of a shopper. Contains original request and information about facet for which extra keys requested. | [required] |

### Return type

[**crate::models::FacetSearchResponseDto**](FacetSearchResponseDto.md)

### Authorization

[GroupByIncEmployee](../README.md#GroupByIncEmployee), [ClientKey](../README.md#ClientKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search

> crate::models::SearchResponseDto search(x_groupby_customer_id, search_request_dto)
Provided search functionality

Perform a search against the GroupBy Retail Search API.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_groupby_customer_id** | **String** | Custom HTTP header which may contain tenant name. Used to define a client by its name. | [required] |
**search_request_dto** | [**SearchRequestDto**](SearchRequestDto.md) | Request that should be populated to configure a search API call, made by the client on behalf of a shopper. | [required] |

### Return type

[**crate::models::SearchResponseDto**](SearchResponseDto.md)

### Authorization

[GroupByIncEmployee](../README.md#GroupByIncEmployee), [ClientKey](../README.md#ClientKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

