/*
 * GroupBy Retail
 *
 * GroupBy Retail API
 *
 * The version of the OpenAPI document: 0.0.0
 * Contact: ops@groupbyinc.com
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Order {
    #[serde(rename = "ASCENDING")]
    Ascending,
    #[serde(rename = "DESCENDING")]
    Descending,

}

impl ToString for Order {
    fn to_string(&self) -> String {
        match self {
            Self::Ascending => String::from("ASCENDING"),
            Self::Descending => String::from("DESCENDING"),
        }
    }
}

impl Default for Order {
    fn default() -> Order {
        Self::Ascending
    }
}




