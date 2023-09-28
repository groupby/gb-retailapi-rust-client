/*
 * GroupBy Retail
 *
 * GroupBy Retail API
 *
 * The version of the OpenAPI document: 0.0.0
 * Contact: ops@groupbyinc.com
 * Generated by: https://openapi-generator.tech
 */

/// NavigationDto : Navigation available for the shopper to refine the results on.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NavigationDto {
    /// Name of the field the navigation in on.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Name of the navigation for display purposes.
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "type")]
    pub r#type: crate::models::NavigationTypeDto,
    #[serde(rename = "refinements")]
    pub refinements: Vec<crate::models::RefinementDto>,
    /// Flag if this navigation supports or queries.
    #[serde(rename = "or", skip_serializing_if = "Option::is_none")]
    pub or: Option<bool>,
    #[serde(rename = "source")]
    pub source: String,
    #[serde(rename = "metadata")]
    pub metadata: Vec<crate::models::Metadata>,
    /// Place id for inventory navigations.
    #[serde(rename = "placeId")]
    pub place_id: String,
}

impl NavigationDto {
    /// Navigation available for the shopper to refine the results on.
    pub fn new(r#type: crate::models::NavigationTypeDto, refinements: Vec<crate::models::RefinementDto>, source: String, metadata: Vec<crate::models::Metadata>, place_id: String) -> NavigationDto {
        NavigationDto {
            name: None,
            display_name: None,
            r#type,
            refinements,
            or: None,
            source,
            metadata,
            place_id,
        }
    }
}


