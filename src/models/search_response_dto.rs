/*
 * GroupBy Retail
 *
 * GroupBy Retail API
 *
 * The version of the OpenAPI document: 0.0.0
 * Contact: ops@groupbyinc.com
 * Generated by: https://openapi-generator.tech
 */

/// SearchResponseDto : Response of calling the search API, including various elements of the original request context, matching records and general metadata relating to the results.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchResponseDto {
    /// Unique identifier for the search.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Area Id the search was performed in.
    #[serde(rename = "area", skip_serializing_if = "Option::is_none")]
    pub area: Option<String>,
    /// Original search query.
    #[serde(rename = "query", skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Search query after any changes/corrections are done by the engine.
    #[serde(rename = "correctedQuery", skip_serializing_if = "Option::is_none")]
    pub corrected_query: Option<String>,
    /// Name of the biasing profile which was used to bias products in the search results.
    #[serde(rename = "biasingProfile", skip_serializing_if = "Option::is_none")]
    pub biasing_profile: Option<String>,
    /// Id of the biasing profile which was used to bias products in the search results.
    #[serde(rename = "biasingProfileAppliedId", skip_serializing_if = "Option::is_none")]
    pub biasing_profile_applied_id: Option<i32>,
    #[serde(rename = "filter")]
    pub filter: String,
    #[serde(rename = "originalRequest")]
    pub original_request: Box<crate::models::SearchRequestDto>,
    /// The list of records that match the search.
    #[serde(rename = "records", skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<crate::models::RecordDto>>,
    /// The total number of products that match the search. If all products were filtered out on S4R site equals to 0.
    #[serde(rename = "totalRecordCount", skip_serializing_if = "Option::is_none")]
    pub total_record_count: Option<i64>,
    #[serde(rename = "metadata")]
    pub metadata: Box<crate::models::SearchMetadataDto>,
    #[serde(rename = "pageInfo")]
    pub page_info: Box<crate::models::PageInfoDto>,
    #[serde(rename = "availableNavigation")]
    pub available_navigation: Vec<crate::models::NavigationDto>,
    #[serde(rename = "selectedNavigation")]
    pub selected_navigation: Vec<crate::models::NavigationDto>,
    /// URL to which the merchandiser should redirect the shopper to.
    #[serde(rename = "redirect", skip_serializing_if = "Option::is_none")]
    pub redirect: Option<String>,
    #[serde(rename = "experiments")]
    pub experiments: Vec<crate::models::Experiment>,
    #[serde(rename = "template")]
    pub template: Box<crate::models::TemplateDto>,
    /// True if the search yielded no results, otherwise false.
    #[serde(rename = "empty", skip_serializing_if = "Option::is_none")]
    pub empty: Option<bool>,
    #[serde(rename = "siteParams")]
    pub site_params: Vec<crate::models::Metadata>,
    #[serde(rename = "debug")]
    pub debug: Box<crate::models::DebugDto>,
    /// Warning messages containing information about invalid products, etc.
    #[serde(rename = "warnings", skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
    /// When a shopper uses an ambiguous or a multi-word search phrase, they can get an empty response. After turning on include expanded results, Retail Search analyzes the request and returns the expanded list of products based on the parsed search query. For example, if you search \"Google Pixel 5\" without query expansion, you might only get \"google_pixel_5\" in the result. With query expansion, you might get \"google_pixel_4a_with_5g\", \"google_pixel_4a\" and \"google_pixel_5_case\" as well.The default value is configured in the tenant settings or true if there is no such setting
    #[serde(rename = "includeExpandedResults", skip_serializing_if = "Option::is_none")]
    pub include_expanded_results: Option<bool>,
    /// Maximum of facet values that should be returned for this facet. If not specified, defaults to 20. The maximum allowed value is 300. Values above 300 will be coerced to 300.  If this field is negative, an INVALID_ARGUMENT is returned.  This limit (300) is configured on Google side, but Google have an ability to change it for specific project. 
    #[serde(rename = "facetLimit", skip_serializing_if = "Option::is_none")]
    pub facet_limit: Option<i32>,
    #[serde(rename = "redirectMetadata")]
    pub redirect_metadata: Vec<crate::models::Metadata>,
}

impl SearchResponseDto {
    /// Response of calling the search API, including various elements of the original request context, matching records and general metadata relating to the results.
    pub fn new(filter: String, original_request: crate::models::SearchRequestDto, metadata: crate::models::SearchMetadataDto, page_info: crate::models::PageInfoDto, available_navigation: Vec<crate::models::NavigationDto>, selected_navigation: Vec<crate::models::NavigationDto>, experiments: Vec<crate::models::Experiment>, template: crate::models::TemplateDto, site_params: Vec<crate::models::Metadata>, debug: crate::models::DebugDto, redirect_metadata: Vec<crate::models::Metadata>) -> SearchResponseDto {
        SearchResponseDto {
            id: None,
            area: None,
            query: None,
            corrected_query: None,
            biasing_profile: None,
            biasing_profile_applied_id: None,
            filter,
            original_request: Box::new(original_request),
            records: None,
            total_record_count: None,
            metadata: Box::new(metadata),
            page_info: Box::new(page_info),
            available_navigation,
            selected_navigation,
            redirect: None,
            experiments,
            template: Box::new(template),
            empty: None,
            site_params,
            debug: Box::new(debug),
            warnings: None,
            include_expanded_results: None,
            facet_limit: None,
            redirect_metadata,
        }
    }
}


