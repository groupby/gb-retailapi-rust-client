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
pub struct QueryPatternTrigger {
    #[serde(rename = "type")]
    pub r#type: crate::models::QueryPatternTriggerPeriodType,
    #[serde(rename = "values", skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
    #[serde(rename = "patterns", skip_serializing_if = "Option::is_none")]
    pub patterns: Option<Vec<serde_json::Value>>,
}

impl QueryPatternTrigger {
    pub fn new(r#type: crate::models::QueryPatternTriggerPeriodType) -> QueryPatternTrigger {
        QueryPatternTrigger {
            r#type,
            values: None,
            patterns: None,
        }
    }
}


