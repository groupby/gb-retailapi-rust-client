# SearchResponseDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Unique identifier for the search. | [optional]
**area** | Option<**String**> | Area Id the search was performed in. | [optional]
**query** | Option<**String**> | Original search query. | [optional]
**corrected_query** | Option<**String**> | Search query after any changes/corrections are done by the engine. | [optional]
**biasing_profile** | Option<**String**> | Name of the biasing profile which was used to bias products in the search results. | [optional]
**biasing_profile_applied_id** | Option<**i32**> | Id of the biasing profile which was used to bias products in the search results. | [optional]
**filter** | **String** |  | 
**original_request** | [**crate::models::SearchRequestDto**](SearchRequestDto.md) |  | 
**records** | Option<[**Vec<crate::models::RecordDto>**](RecordDto.md)> | The list of records that match the search. | [optional]
**total_record_count** | Option<**i64**> | The total number of products that match the search. If all products were filtered out on S4R site equals to 0. | [optional]
**metadata** | [**crate::models::SearchMetadataDto**](SearchMetadataDto.md) |  | 
**page_info** | [**crate::models::PageInfoDto**](PageInfoDto.md) |  | 
**available_navigation** | [**Vec<crate::models::NavigationDto>**](NavigationDto.md) |  | 
**selected_navigation** | [**Vec<crate::models::NavigationDto>**](NavigationDto.md) |  | 
**redirect** | Option<**String**> | URL to which the merchandiser should redirect the shopper to. | [optional]
**experiments** | [**Vec<crate::models::Experiment>**](Experiment.md) |  | 
**template** | [**crate::models::TemplateDto**](TemplateDto.md) |  | 
**empty** | Option<**bool**> | True if the search yielded no results, otherwise false. | [optional]
**site_params** | [**Vec<crate::models::Metadata>**](Metadata.md) |  | 
**debug** | [**crate::models::DebugDto**](DebugDto.md) |  | 
**warnings** | Option<**Vec<String>**> | Warning messages containing information about invalid products, etc. | [optional]
**include_expanded_results** | Option<**bool**> | When a shopper uses an ambiguous or a multi-word search phrase, they can get an empty response. After turning on include expanded results, Retail Search analyzes the request and returns the expanded list of products based on the parsed search query. For example, if you search \"Google Pixel 5\" without query expansion, you might only get \"google_pixel_5\" in the result. With query expansion, you might get \"google_pixel_4a_with_5g\", \"google_pixel_4a\" and \"google_pixel_5_case\" as well.The default value is configured in the tenant settings or true if there is no such setting | [optional]
**facet_limit** | Option<**i32**> | Maximum of facet values that should be returned for this facet. If not specified, defaults to 20. The maximum allowed value is 300. Values above 300 will be coerced to 300.  If this field is negative, an INVALID_ARGUMENT is returned.  This limit (300) is configured on Google side, but Google have an ability to change it for specific project.  | [optional]
**redirect_metadata** | [**Vec<crate::models::Metadata>**](Metadata.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


