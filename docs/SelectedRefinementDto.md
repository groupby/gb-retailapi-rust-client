# SelectedRefinementDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**navigation_name** | **String** | The name of the navigation the refinement is for. | 
**r#type** | [**crate::models::NavigationTypeDto**](NavigationTypeDto.md) |  | 
**value** | Option<**String**> | Value of selected refinement, if type is value. | [optional]
**low** | Option<**f64**> | The lowest end or value of the range, if applicable. | [optional]
**high** | Option<**f64**> | The highest end or value of the range, if applicable. | [optional]
**source** | Option<**String**> | Field which is indicated that it is dynamic navigation. | [optional]
**or** | Option<**bool**> | Navigation multiselect. Indicate that it is possibly to select more than one navigation value due to search request. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


