/*
 * GroupBy Retail
 *
 * GroupBy Retail API
 *
 * The version of the OpenAPI document: 0.0
 * Contact: ops@groupbyinc.com
 * Generated by: https://openapi-generator.tech
 */

/// SearchRequestDto : Request that should be populated to configure a search API call, made by the client on behalf of a shopper.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchRequestDto {
    /// Base textual search query.
    #[serde(rename = "query", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub query: Option<Option<String>>,
    /// Area name the search is being performed in.
    #[serde(rename = "area", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub area: Option<Option<String>>,
    /// Name of collection in project configuration setting which is mapped to the google retail backend.
    #[serde(rename = "collection", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub collection: Option<Option<String>>,
    /// Unique identifier identifying the shopper. Will be autogenerated if not provided.
    #[serde(rename = "visitorId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub visitor_id: Option<Option<String>>,
    #[serde(rename = "refinements")]
    pub refinements: Vec<crate::models::SelectedRefinementDto>,
    /// The number of products to be returned on each page.
    #[serde(rename = "pageSize", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub page_size: Option<Option<i32>>,
    /// Where in the list of products to begin the page.
    #[serde(rename = "skip", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub skip: Option<Option<i64>>,
    /// Name of a biasing profile which should be applied to the search. Takes priority over area default.
    #[serde(rename = "biasingProfile", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub biasing_profile: Option<Option<String>>,
    #[serde(rename = "biasing", deserialize_with = "Option::deserialize")]
    pub biasing: Option<Box<crate::models::BiasingProfileDto>>,
    #[serde(rename = "customUrlParams")]
    pub custom_url_params: Vec<crate::models::CustomParameterDto>,
    #[serde(rename = "sorts")]
    pub sorts: Vec<crate::models::SortDto>,
    /// Set of navigation fields to include in the search result. Cannot be set if 'excludedNavigations' is set.
    #[serde(rename = "includedNavigations", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub included_navigations: Option<Option<Vec<String>>>,
    /// Set of navigation fields to exclude in the search result. Cannot be set if 'includedNavigations' is set.
    #[serde(rename = "excludedNavigations", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub excluded_navigations: Option<Option<Vec<String>>>,
    /// Set the specifications of dynamically generated facets.
    #[serde(rename = "dynamicFacet", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub dynamic_facet: Option<Option<bool>>,
    /// Set the variant rollup keys.
    #[serde(rename = "variantRollupKeys", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub variant_rollup_keys: Option<Option<Vec<String>>>,
    /// Set of the prefilter specifications value.
    #[serde(rename = "preFilter", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub pre_filter: Option<Option<String>>,
    /// Name of site filter. If not specified, the specified area's default site will be applied if configured in Command Center. To not use default specify empty value i.e.\"\".  If the site doesn't exist then the search will execute without the site filter and a warning will be provided.
    #[serde(rename = "site", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub site: Option<Option<String>>,
    /// List with fields which should be included in metadata object associated with each record in response.
    #[serde(rename = "responseMask", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub response_mask: Option<Option<Vec<String>>>,
    /// The categories associated with a category page. Required for category navigation queries to achieve good search quality. To represent full path of category, use '>' sign to separate different hierarchies. If '>' is part of the category name, please replace it with other character(s).Max item length = 1.
    #[serde(rename = "pageCategories", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub page_categories: Option<Option<Vec<String>>>,
    #[serde(rename = "spellCorrectionMode", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub spell_correction_mode: Option<Option<Box<crate::models::SpellCorrectionMode>>>,
    /// When a shopper uses an ambiguous or a multi-word search phrase, they can get an empty response. After turning on include expanded results, Retail Search analyzes the request and returns the expanded list of products based on the parsed search query. For example, if you search \"Google Pixel 5\" without query expansion, you might only get \"google_pixel_5\" in the result. With query expansion, you might get \"google_pixel_4a_with_5g\", \"google_pixel_4a\" and \"google_pixel_5_case\" as well.The default value is configured in the tenant settings or true if there is no such setting
    #[serde(rename = "includeExpandedResults", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub include_expanded_results: Option<Option<bool>>,
    /// This configuration depends on include expanded results settings. If this field is set to true,unexpanded products are always at the top of the search results, followed  by the expanded results. Default value: true
    #[serde(rename = "pinUnexpandedResults", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub pin_unexpanded_results: Option<Option<bool>>,
    /// Enable additional debug info in response.  Note: attaching debug info significantly affects performance. Is not supposed to be used for large requests.  
    #[serde(rename = "debug", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub debug: Option<Option<bool>>,
    /// Maximum of facet values that should be returned for this facet. If not specified, defaults to 20. The maximum allowed value is 300. Values above 300 will be coerced to 300.  If this field is negative, an INVALID_ARGUMENT is returned.  This limit (300) is configured on Google side, but Google have an ability to change it for specific project. 
    #[serde(rename = "facetLimit", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub facet_limit: Option<Option<i32>>,
    /// Highly recommended for logged-in users. Unique identifier for logged-in user, such as a user name. Don't set for anonymous users.  Don't set the field to the same fixed ID for different users. This mixes the event history of those users together, which results in degraded model quality.  The field must be a UTF-8 encoded string with a length limit of 128 characters. 
    #[serde(rename = "loginId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub login_id: Option<Option<String>>,
    #[serde(rename = "overwrites", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub overwrites: Option<Option<Box<crate::models::SearchRequestDtoOverwrites>>>,
}

impl SearchRequestDto {
    /// Request that should be populated to configure a search API call, made by the client on behalf of a shopper.
    pub fn new(refinements: Vec<crate::models::SelectedRefinementDto>, biasing: Option<crate::models::BiasingProfileDto>, custom_url_params: Vec<crate::models::CustomParameterDto>, sorts: Vec<crate::models::SortDto>) -> SearchRequestDto {
        SearchRequestDto {
            query: None,
            area: None,
            collection: None,
            visitor_id: None,
            refinements,
            page_size: None,
            skip: None,
            biasing_profile: None,
            biasing: if let Some(x) = biasing {Some(Box::new(x))} else {None},
            custom_url_params,
            sorts,
            included_navigations: None,
            excluded_navigations: None,
            dynamic_facet: None,
            variant_rollup_keys: None,
            pre_filter: None,
            site: None,
            response_mask: None,
            page_categories: None,
            spell_correction_mode: None,
            include_expanded_results: None,
            pin_unexpanded_results: None,
            debug: None,
            facet_limit: None,
            login_id: None,
            overwrites: None,
        }
    }
}

