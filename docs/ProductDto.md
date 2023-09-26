# ProductDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Relative path to product in Google Retail system. | [optional]
**id** | Option<**String**> | Product id in Google Retail system. | [optional]
**r#type** | Option<**String**> | Product type. Possible values: PRIMARY, VARIANT. If the product has variant list and the request specifies the variantIds, requested variants will be the first in the response. | [optional]
**primary_product_id** | Option<**String**> | Product ID that is primary in relation to the current one | [optional]
**collection_member_ids** | Option<**Vec<String>**> | The of the collection members when product type is COLLECTION | [optional]
**gtin** | Option<**String**> | Global Trade Item Number can be used by a company to uniquely identify all of its trade items.GTIN defines trade items as products or services that are priced, ordered or invoiced at any point in the supply chain. | [optional]
**categories** | Option<**Vec<String>**> | Product categories (array). | [optional]
**title** | Option<**String**> | Product title. | [optional]
**brands** | Option<**Vec<String>**> | Product brands. | [optional]
**description** | Option<**String**> | Product description. | [optional]
**language_code** | Option<**String**> | Language of the title/description and other string attributes. Use language tags defined by [BCP 47][https://www.rfc-editor.org/rfc/bcp/bcp47.txt]. For product search this field is in use. It defaults to 'en-US' if unset. | [optional]
**attributes** | Option<[**::std::collections::HashMap<String, crate::models::ProductCustomAttribute>**](ProductCustomAttribute.md)> | Highly encouraged. Extra product attributes to be included. For example, for products, this could include the store name, vendor, style, color, etc. These are very strong signals for recommendation model, thus we highly recommend providing the attributes here. Features that can take on one of a limited number of possible values. Two types of features can be set are: Textual features. some examples would be the brand/maker of a product, or country of a customer. Numerical features. Some examples would be the height/weight of a product, or age of a customer.  Max entries count: 200. Length limit of 128 characters. | [optional]
**tags** | Option<**Vec<String>**> | Product tags (array). | [optional]
**price_info** | Option<[**crate::models::ProductDtoPriceInfo**](ProductDto_priceInfo.md)> |  | [optional]
**rating** | Option<[**crate::models::ProductDtoRating**](ProductDto_rating.md)> |  | [optional]
**available_time** | Option<[**crate::models::ProductDtoAvailableTime**](ProductDto_availableTime.md)> |  | [optional]
**availability** | Option<**String**> | The online availability of the product. Default to IN_STOCK | [optional]
**available_quantity** | Option<**i32**> | The available quantity of the item. | [optional]
**fulfillment_infos** | Option<[**Vec<crate::models::FulfillmentInfo>**](FulfillmentInfo.md)> | Fulfillment information, such as the store IDs for in-store pickup or region IDs for different shipping methods. | [optional]
**uri** | Option<**String**> | Link to the appropriate product. | [optional]
**images** | Option<[**Vec<crate::models::Image>**](Image.md)> | Product Image. | [optional]
**audience** | Option<[**crate::models::ProductDtoAudience**](ProductDto_audience.md)> |  | [optional]
**color_info** | Option<[**crate::models::ProductDtoColorInfo**](ProductDto_colorInfo.md)> |  | [optional]
**sizes** | Option<**Vec<String>**> | Product sizes (array). | [optional]
**materials** | Option<**Vec<String>**> | The material of the product. For example, 'leather', 'wooden'. A maximum of 20 values are allowed. Length limit of 128 characters | [optional]
**patterns** | Option<**Vec<String>**> | The pattern or graphic print of the product. For example, 'striped', 'polka dot', 'paisley'. A maximum of 20 values are allowed per product. Length limit of 128 characters. | [optional]
**conditions** | Option<**Vec<String>**> | The condition of the product. Strongly encouraged to use the standardvalues: 'new', 'refurbished', 'used'. A maximum of 5 values are allowed per product. Length limit of 128 characters. | [optional]
**publish_time** | Option<[**crate::models::ProductDtoPublishTime**](ProductDto_publishTime.md)> |  | [optional]
**retrievable_fields** | Option<[**crate::models::ProductDtoRetrievableFields**](ProductDto_retrievableFields.md)> |  | [optional]
**promotions** | Option<[**Vec<crate::models::Promotion>**](Promotion.md)> | The promotions applied to the product. A maximum of 10 values are allowed per product. | [optional]
**variants** | Option<[**Vec<crate::models::ProductDto>**](ProductDto.md)> | If the product has variant list and the request specifies the variantIds, requested variants will be the first in the response. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


