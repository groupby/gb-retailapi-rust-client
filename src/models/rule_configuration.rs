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
pub struct RuleConfiguration {
    #[serde(rename = "ruleInExperiment", skip_serializing_if = "Option::is_none")]
    pub rule_in_experiment: Option<bool>,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "areaId")]
    pub area_id: i32,
    #[serde(rename = "priority")]
    pub priority: i32,
    #[serde(rename = "enabled")]
    pub enabled: bool,
    #[serde(rename = "activeHoursEnabled")]
    pub active_hours_enabled: bool,
    #[serde(rename = "activeFrom")]
    pub active_from: i64,
    #[serde(rename = "activeTo")]
    pub active_to: i64,
    #[serde(rename = "triggerSets")]
    pub trigger_sets: Vec<crate::models::TriggerSet>,
    #[serde(rename = "biasingProfileName")]
    pub biasing_profile_name: String,
    #[serde(rename = "sort")]
    pub sort: Box<crate::models::Sort>,
    #[serde(rename = "includedNavigations")]
    pub included_navigations: Vec<String>,
    #[serde(rename = "valueFilters")]
    pub value_filters: Vec<crate::models::ValueFilter>,
    #[serde(rename = "searchFilters")]
    pub search_filters: Vec<crate::models::SearchFilter>,
    #[serde(rename = "rangeFilters")]
    pub range_filters: Vec<crate::models::RangeFilter>,
    #[serde(rename = "template")]
    pub template: Box<crate::models::RuleTemplate>,
    #[serde(rename = "boostedProductBuckets", deserialize_with = "Option::deserialize")]
    pub boosted_product_buckets: Option<Vec<crate::models::BoostedProductBucket>>,
    #[serde(rename = "pinnedRefinements")]
    pub pinned_refinements: Vec<crate::models::PinnedRefinement>,
    #[serde(rename = "messageType")]
    pub message_type: crate::models::MessageType,
    #[serde(rename = "type")]
    pub r#type: crate::models::RuleType,
    #[serde(rename = "variants")]
    pub variants: Vec<crate::models::ExperimentVariant>,
}

impl RuleConfiguration {
    pub fn new(id: i32, name: String, area_id: i32, priority: i32, enabled: bool, active_hours_enabled: bool, active_from: i64, active_to: i64, trigger_sets: Vec<crate::models::TriggerSet>, biasing_profile_name: String, sort: crate::models::Sort, included_navigations: Vec<String>, value_filters: Vec<crate::models::ValueFilter>, search_filters: Vec<crate::models::SearchFilter>, range_filters: Vec<crate::models::RangeFilter>, template: crate::models::RuleTemplate, boosted_product_buckets: Option<Vec<crate::models::BoostedProductBucket>>, pinned_refinements: Vec<crate::models::PinnedRefinement>, message_type: crate::models::MessageType, r#type: crate::models::RuleType, variants: Vec<crate::models::ExperimentVariant>) -> RuleConfiguration {
        RuleConfiguration {
            rule_in_experiment: None,
            id,
            name,
            area_id,
            priority,
            enabled,
            active_hours_enabled,
            active_from,
            active_to,
            trigger_sets,
            biasing_profile_name,
            sort: Box::new(sort),
            included_navigations,
            value_filters,
            search_filters,
            range_filters,
            template: Box::new(template),
            boosted_product_buckets,
            pinned_refinements,
            message_type,
            r#type,
            variants,
        }
    }
}


