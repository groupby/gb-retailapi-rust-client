# Facet

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**prefix** | Option<**String**> | Only get facet values that start with the given string prefix. For example, suppose \"categories\" has three values \"Women > Shoe\", \"Women > Dress\" and \"Men > Shoe\". If set \"prefixes\" to \"Women\", the \"categories\" facet will give only \"Women > Shoe\" and \"Women > Dress\". Only supported on textual fields. Maximum is 10. This field is case-sensitive | [optional]
**contains** | Option<**String**> | Only get facet values that contains the given strings. For example, suppose \"categories\" has three values \"Women > Shoe\", \"Women > Dress\" and \"Men > Shoe\". If set \"contains\" to \"Shoe\", the \"categories\" facet will give only \"Women > Shoe\" and \"Men > Shoe\". Only supported on textual fields. Maximum is 10. This field is case-sensitive | [optional]
**display_name** | Option<**String**> | Display name of facet | [optional]
**r#type** | Option<[**crate::models::NavigationType**](NavigationType.md)> |  | [optional]
**navigation_name** | Option<**String**> | Represents the name of navigation. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


