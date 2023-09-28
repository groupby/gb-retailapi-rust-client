/*
 * GroupBy Retail
 *
 * GroupBy Retail API
 *
 * The version of the OpenAPI document: 0.0.0
 * Contact: ops@groupbyinc.com
 * Generated by: https://openapi-generator.tech
 */

/// ZoneDtoType : Define type of content which is can be stored in zone.

/// Define type of content which is can be stored in zone.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ZoneDtoType {
    #[serde(rename = "Content")]
    Content,
    #[serde(rename = "Rich_Content")]
    RichContent,
    #[serde(rename = "Products")]
    Products,
    #[serde(rename = "Generated_Content")]
    GeneratedContent,

}

impl ToString for ZoneDtoType {
    fn to_string(&self) -> String {
        match self {
            Self::Content => String::from("Content"),
            Self::RichContent => String::from("Rich_Content"),
            Self::Products => String::from("Products"),
            Self::GeneratedContent => String::from("Generated_Content"),
        }
    }
}

impl Default for ZoneDtoType {
    fn default() -> ZoneDtoType {
        Self::Content
    }
}




