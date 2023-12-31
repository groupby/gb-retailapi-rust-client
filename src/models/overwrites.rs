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
pub struct Overwrites {
    #[serde(rename = "rules")]
    pub rules: Vec<crate::models::RuleConfiguration>,
}

impl Overwrites {
    pub fn new(rules: Vec<crate::models::RuleConfiguration>) -> Overwrites {
        Overwrites {
            rules,
        }
    }
}


