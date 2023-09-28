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
pub struct Refinement {
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "priority")]
    pub priority: i32,
}

impl Refinement {
    pub fn new(value: String, priority: i32) -> Refinement {
        Refinement {
            value,
            priority,
        }
    }
}


