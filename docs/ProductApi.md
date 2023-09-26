# \ProductApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_by_product_ids**](ProductApi.md#get_by_product_ids) | **GET** /api/search/product | Provided product search functionality



## get_by_product_ids

> crate::models::ProductDto get_by_product_ids(collection, product_id, x_groupby_customer_id, variant_ids)
Provided product search functionality

Perform a product search against the GroupBy Retail Search API.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection** | **String** | Collection name | [required] |
**product_id** | **String** | Product ID which need to be search | [required] |
**x_groupby_customer_id** | **String** | Required. This parameter will extract from header X-Groupby-Customer-Id. May contain tenant name. Used to define a                           client by its name. | [required] |
**variant_ids** | Option<[**Vec<String>**](String.md)> | Not required. If the product has variant list and the request specifies the variantIds, requested variants will be the                           first in the response. |  |

### Return type

[**crate::models::ProductDto**](ProductDto.md)

### Authorization

[GroupByIncEmployee](../README.md#GroupByIncEmployee), [ClientKey](../README.md#ClientKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

