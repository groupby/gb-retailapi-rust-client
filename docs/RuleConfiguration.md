# RuleConfiguration

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**rule_in_experiment** | Option<**bool**> |  | [optional]
**id** | **i32** |  | 
**name** | **String** |  | 
**area_id** | **i32** |  | 
**priority** | **i32** |  | 
**enabled** | **bool** |  | 
**active_hours_enabled** | **bool** |  | 
**active_from** | **i64** |  | 
**active_to** | **i64** |  | 
**trigger_sets** | [**Vec<crate::models::TriggerSet>**](TriggerSet.md) |  | 
**biasing_profile_name** | **String** |  | 
**sort** | [**crate::models::Sort**](Sort.md) |  | 
**included_navigations** | **Vec<String>** |  | 
**value_filters** | [**Vec<crate::models::ValueFilter>**](ValueFilter.md) |  | 
**search_filters** | [**Vec<crate::models::SearchFilter>**](SearchFilter.md) |  | 
**range_filters** | [**Vec<crate::models::RangeFilter>**](RangeFilter.md) |  | 
**template** | [**crate::models::RuleTemplate**](RuleTemplate.md) |  | 
**boosted_product_buckets** | Option<[**Vec<crate::models::BoostedProductBucket>**](BoostedProductBucket.md)> |  | 
**pinned_refinements** | [**Vec<crate::models::PinnedRefinement>**](PinnedRefinement.md) |  | 
**message_type** | [**crate::models::MessageType**](MessageType.md) |  | 
**r#type** | [**crate::models::RuleType**](RuleType.md) |  | 
**variants** | [**Vec<crate::models::ExperimentVariant>**](ExperimentVariant.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


