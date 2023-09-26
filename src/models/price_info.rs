/*
 * GroupBy Retail
 *
 * GroupBy Retail API
 *
 * The version of the OpenAPI document: 0.0
 * Contact: ops@groupbyinc.com
 * Generated by: https://openapi-generator.tech
 */

/// PriceInfo : Price info representation.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PriceInfo {
    /// Currency code.
    #[serde(rename = "currencyCode", skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    /// Price value.
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<f32>,
    /// Original price value.
    #[serde(rename = "originalPrice", skip_serializing_if = "Option::is_none")]
    pub original_price: Option<f32>,
    /// Cost
    #[serde(rename = "cost", skip_serializing_if = "Option::is_none")]
    pub cost: Option<f32>,
    #[serde(rename = "priceEffectiveTime", skip_serializing_if = "Option::is_none")]
    pub price_effective_time: Option<Box<crate::models::PriceInfoPriceEffectiveTime>>,
    #[serde(rename = "priceExpireTime", skip_serializing_if = "Option::is_none")]
    pub price_expire_time: Option<Box<crate::models::PriceInfoPriceExpireTime>>,
    #[serde(rename = "priceRange", skip_serializing_if = "Option::is_none")]
    pub price_range: Option<Box<crate::models::PriceInfoPriceRange>>,
}

impl PriceInfo {
    /// Price info representation.
    pub fn new() -> PriceInfo {
        PriceInfo {
            currency_code: None,
            price: None,
            original_price: None,
            cost: None,
            price_effective_time: None,
            price_expire_time: None,
            price_range: None,
        }
    }
}


