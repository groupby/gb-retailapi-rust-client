# \AutocompleteApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**autocompletesearch**](AutocompleteApi.md#autocompletesearch) | **GET** /api/request | 



## autocompletesearch

> crate::models::SearchResults autocompletesearch(x_groupby_customer_id, identity, merchandiser, request)


A simple request used to get completes the specified prefix with keyword suggestions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_groupby_customer_id** | **String** | Header on incoming HTTP requests that is populated by the API gateway and indicates the customer ID. | [required] |
**identity** | [**Identity**](.md) |  | [required] |
**merchandiser** | [**Merchandiser**](.md) |  | [required] |
**request** | Option<[**Request**](.md)> | Object which is represent autocomplete request and encapsulate all passed parameters.  |  |

### Return type

[**crate::models::SearchResults**](SearchResults.md)

### Authorization

[GroupByIncEmployee](../README.md#GroupByIncEmployee), [ClientKey](../README.md#ClientKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

