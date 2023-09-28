/*
 * GroupBy Retail
 *
 * GroupBy Retail API
 *
 * The version of the OpenAPI document: 0.0.0
 * Contact: ops@groupbyinc.com
 * Generated by: https://openapi-generator.tech
 */

/// ColorInfo : Product color info.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ColorInfo {
    /// Product color families (array).
    #[serde(rename = "colorFamilies", skip_serializing_if = "Option::is_none")]
    pub color_families: Option<Vec<String>>,
    /// Product color (array).
    #[serde(rename = "colors", skip_serializing_if = "Option::is_none")]
    pub colors: Option<Vec<String>>,
}

impl ColorInfo {
    /// Product color info.
    pub fn new() -> ColorInfo {
        ColorInfo {
            color_families: None,
            colors: None,
        }
    }
}


