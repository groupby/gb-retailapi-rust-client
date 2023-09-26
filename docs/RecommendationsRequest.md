# RecommendationsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**collection** | **String** |  | 
**visitor_id** | Option<**String**> |  | [optional]
**limit** | Option<**String**> |  | [optional]
**page_size** | Option<**String**> |  | [optional]
**event_type** | Option<**String**> |  | [optional]
**login_id** | Option<**String**> |  | [optional]
**product_id** | Option<**Vec<String>**> |  | [optional]
**fields** | Option<**Vec<String>**> |  | [optional]
**filters** | Option<[**Vec<crate::models::Filter>**](Filter.md)> |  | [optional]
**raw_filter** | Option<**String**> |  | [optional]
**placement** | Option<**String**> |  | [optional]
**name** | Option<**String**> |  | [optional]
**strict_filtering** | Option<**bool**> | The default is true. If strictFiltering true only products that are within the scope of the filter specified. If false, relax the filtering so that the response may contain other products that are outside the scope of the filtering. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


