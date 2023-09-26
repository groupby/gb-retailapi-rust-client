/*
 * GroupBy Retail
 *
 * GroupBy Retail API
 *
 * The version of the OpenAPI document: 0.0
 * Contact: ops@groupbyinc.com
 * Generated by: https://openapi-generator.tech
 */

/// Image : Product image



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Image {
    /// Absolute path to product image.
    #[serde(rename = "uri", skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    /// Height in pixels
    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    /// Width in pixels
    #[serde(rename = "width", skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
}

impl Image {
    /// Product image
    pub fn new() -> Image {
        Image {
            uri: None,
            height: None,
            width: None,
        }
    }
}


