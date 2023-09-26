# ProductCustomAttribute

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**text** | Option<**Vec<String>**> | The textual values of this custom attribute. At most 400 values are allowed. Empty values are not allowed. Length limit of 256 characters. Exactly one of text or numbers should be set. | [optional]
**numbers** | Option<**Vec<f64>**> | The numerical values of this custom attribute. At most 400 values are allowed. Exactly one of text or numbers should be set. | [optional]
**searchable** | Option<**bool**> | If true, custom attribute values are searchable by text queries in. search. Only set if type text set. | [optional]
**indexable** | Option<**bool**> | If true, custom attribute values are indexed, so that it can be filtered, faceted or boosted in search. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


