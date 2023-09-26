/*
 * GroupBy Retail
 *
 * GroupBy Retail API
 *
 * The version of the OpenAPI document: 0.0
 * Contact: ops@groupbyinc.com
 * Generated by: https://openapi-generator.tech
 */

/// ProductCustomAttribute : A custom attribute that is not explicitly modeled in product.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProductCustomAttribute {
    /// The textual values of this custom attribute. At most 400 values are allowed. Empty values are not allowed. Length limit of 256 characters. Exactly one of text or numbers should be set.
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: Option<Vec<String>>,
    /// The numerical values of this custom attribute. At most 400 values are allowed. Exactly one of text or numbers should be set.
    #[serde(rename = "numbers", skip_serializing_if = "Option::is_none")]
    pub numbers: Option<Vec<f64>>,
    /// If true, custom attribute values are searchable by text queries in. search. Only set if type text set.
    #[serde(rename = "searchable", skip_serializing_if = "Option::is_none")]
    pub searchable: Option<bool>,
    /// If true, custom attribute values are indexed, so that it can be filtered, faceted or boosted in search.
    #[serde(rename = "indexable", skip_serializing_if = "Option::is_none")]
    pub indexable: Option<bool>,
}

impl ProductCustomAttribute {
    /// A custom attribute that is not explicitly modeled in product.
    pub fn new() -> ProductCustomAttribute {
        ProductCustomAttribute {
            text: None,
            numbers: None,
            searchable: None,
            indexable: None,
        }
    }
}


