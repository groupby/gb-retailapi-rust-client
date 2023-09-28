/*
 * GroupBy Retail
 *
 * GroupBy Retail API
 *
 * The version of the OpenAPI document: 0.0.0
 * Contact: ops@groupbyinc.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PriceInfoPriceRange {
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<Box<crate::models::PriceInfoPriceRangePrice>>,
    #[serde(rename = "originalPrice", skip_serializing_if = "Option::is_none")]
    pub original_price: Option<Box<crate::models::PriceInfoPriceRangeOriginalPrice>>,
}

impl PriceInfoPriceRange {
    pub fn new() -> PriceInfoPriceRange {
        PriceInfoPriceRange {
            price: None,
            original_price: None,
        }
    }
}


