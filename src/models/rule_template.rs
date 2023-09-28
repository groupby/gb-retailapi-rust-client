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
pub struct RuleTemplate {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "enableExactMatching")]
    pub enable_exact_matching: bool,
    #[serde(rename = "sections")]
    pub sections: Vec<crate::models::RuleTemplateSection>,
}

impl RuleTemplate {
    pub fn new(name: String, enable_exact_matching: bool, sections: Vec<crate::models::RuleTemplateSection>) -> RuleTemplate {
        RuleTemplate {
            name,
            enable_exact_matching,
            sections,
        }
    }
}


