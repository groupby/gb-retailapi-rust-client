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
pub struct RangeFilter {
    /// Field the range applies to.
    #[serde(rename = "field")]
    pub field: String,
    /// Range of values the field value can be.
    #[serde(rename = "range", deserialize_with = "Option::deserialize")]
    pub range: Option<serde_json::Value>,
}

impl RangeFilter {
    pub fn new(field: String, range: Option<serde_json::Value>) -> RangeFilter {
        RangeFilter {
            field,
            range,
        }
    }
}


