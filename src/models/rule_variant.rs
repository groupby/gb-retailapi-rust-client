/*
 * GroupBy Retail
 *
 * GroupBy Retail API
 *
 * The version of the OpenAPI document: 0.0
 * Contact: ops@groupbyinc.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RuleVariant {
    #[serde(rename = "biasingProfileName")]
    pub biasing_profile_name: String,
    #[serde(rename = "includedNavigations")]
    pub included_navigations: Vec<String>,
    #[serde(rename = "template")]
    pub template: Box<crate::models::RuleTemplate>,
    #[serde(rename = "boostedProductBuckets")]
    pub boosted_product_buckets: Vec<crate::models::BoostedProductBucket>,
    #[serde(rename = "pinnedRefinements")]
    pub pinned_refinements: Vec<crate::models::PinnedRefinement>,
    #[serde(rename = "sort")]
    pub sort: Box<crate::models::Sort>,
    #[serde(rename = "valueFilters")]
    pub value_filters: Vec<crate::models::ValueFilter>,
    #[serde(rename = "searchFilters")]
    pub search_filters: Vec<crate::models::SearchFilter>,
    #[serde(rename = "rangeFilters")]
    pub range_filters: Vec<crate::models::RangeFilter>,
}

impl RuleVariant {
    pub fn new(biasing_profile_name: String, included_navigations: Vec<String>, template: crate::models::RuleTemplate, boosted_product_buckets: Vec<crate::models::BoostedProductBucket>, pinned_refinements: Vec<crate::models::PinnedRefinement>, sort: crate::models::Sort, value_filters: Vec<crate::models::ValueFilter>, search_filters: Vec<crate::models::SearchFilter>, range_filters: Vec<crate::models::RangeFilter>) -> RuleVariant {
        RuleVariant {
            biasing_profile_name,
            included_navigations,
            template: Box::new(template),
            boosted_product_buckets,
            pinned_refinements,
            sort: Box::new(sort),
            value_filters,
            search_filters,
            range_filters,
        }
    }
}

