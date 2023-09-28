/*
 * GroupBy Retail
 *
 * GroupBy Retail API
 *
 * The version of the OpenAPI document: 0.0.0
 * Contact: ops@groupbyinc.com
 * Generated by: https://openapi-generator.tech
 */

/// Promotion : The promotions applied to the product. A maximum of 10 values are allowed per product.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Promotion {
    /// ID of the promotion. For example, 'free gift'. Length limit of 128 characters.
    #[serde(rename = "promotionId", skip_serializing_if = "Option::is_none")]
    pub promotion_id: Option<String>,
}

impl Promotion {
    /// The promotions applied to the product. A maximum of 10 values are allowed per product.
    pub fn new() -> Promotion {
        Promotion {
            promotion_id: None,
        }
    }
}


