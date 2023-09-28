/*
 * GroupBy Retail
 *
 * GroupBy Retail API
 *
 * The version of the OpenAPI document: 0.0.0
 * Contact: ops@groupbyinc.com
 * Generated by: https://openapi-generator.tech
 */

/// BiasDto : Biases the search results to either increase or decrease product relevancy for products that match the given field and content.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BiasDto {
    /// The field the bias refers to.
    #[serde(rename = "field")]
    pub field: String,
    /// The content the field must match for the bias to be applied.
    #[serde(rename = "content", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub content: Option<Option<String>>,
    #[serde(rename = "strength")]
    pub strength: crate::models::BiasDtoPeriodStrengthDto,
}

impl BiasDto {
    /// Biases the search results to either increase or decrease product relevancy for products that match the given field and content.
    pub fn new(field: String, strength: crate::models::BiasDtoPeriodStrengthDto) -> BiasDto {
        BiasDto {
            field,
            content: None,
            strength,
        }
    }
}


