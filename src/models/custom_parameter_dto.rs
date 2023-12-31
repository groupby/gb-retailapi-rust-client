/*
 * GroupBy Retail
 *
 * GroupBy Retail API
 *
 * The version of the OpenAPI document: 0.0.0
 * Contact: ops@groupbyinc.com
 * Generated by: https://openapi-generator.tech
 */

/// CustomParameterDto : A key=value combination to allow for further triggering of rules or redirects.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomParameterDto {
    /// Key of the custom parameter.
    #[serde(rename = "key")]
    pub key: String,
    /// Value of the custom parameter
    #[serde(rename = "value")]
    pub value: String,
}

impl CustomParameterDto {
    /// A key=value combination to allow for further triggering of rules or redirects.
    pub fn new(key: String, value: String) -> CustomParameterDto {
        CustomParameterDto {
            key,
            value,
        }
    }
}


