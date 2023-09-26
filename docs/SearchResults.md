# SearchResults

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**stats** | Option<[**crate::models::SearchResultsStats**](SearchResults_stats.md)> |  | [optional]
**search_terms** | [**Vec<crate::models::SearchTerms>**](SearchTerms.md) |  | 
**extended_attributes** | Option<[**::std::collections::HashMap<String, Vec<String>>**](array.md)> | Map with extended attributes which are returned in autocomplete response.  | [optional]
**attribute_results** | Option<[**::std::collections::HashMap<String, crate::models::AttributeSuggestion>**](AttributeSuggestion.md)> | SAYT response attributes. Contains list of direct matching attributes. | [optional]
**site_filter** | Option<**String**> | SiteFilter object used with request. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


