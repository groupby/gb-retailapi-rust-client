# PriceInfoPriceRangeOriginalPrice

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**minimum** | Option<**f64**> | Inclusive lower bound. The lower bound of the interval. If neither of the min fields (minimum or exclusiveMinimum) are set, then the lower bound is negative infinity. This field must be not larger than maximum or exclusiveMaximum. | [optional]
**exclusive_minimum** | Option<**f64**> | Exclusive lower bound. The lower bound of the interval. If neither of the min fields (minimum or exclusiveMinimum) are set, then the lower bound is negative infinity. This field must be not larger than maximum or exclusiveMaximum. | [optional]
**maximum** | Option<**f64**> | Inclusive upper bound. The upper bound of the interval. If neither of the max fields are set, then the upper bound is positive infinity. This field must be not smaller than minimum or exclusiveMinimum. | [optional]
**exclusive_maximum** | Option<**f64**> | Exclusive upper bound. The upper bound of the interval. If neither of the max fields are set, then the upper bound is positive infinity. This field must be not smaller than minimum or exclusiveMinimum. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


