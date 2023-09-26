/*
 * GroupBy Retail
 *
 * GroupBy Retail API
 *
 * The version of the OpenAPI document: 0.0
 * Contact: ops@groupbyinc.com
 * Generated by: https://openapi-generator.tech
 */

/// FieldMask : Retrievable fields.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FieldMask {
    /// Paths for retrievable fields (array).
    #[serde(rename = "paths", skip_serializing_if = "Option::is_none")]
    pub paths: Option<Vec<String>>,
}

impl FieldMask {
    /// Retrievable fields.
    pub fn new() -> FieldMask {
        FieldMask {
            paths: None,
        }
    }
}

