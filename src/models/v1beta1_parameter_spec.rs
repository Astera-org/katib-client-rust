/*
 * Katib
 *
 * Swagger description for Katib
 *
 * The version of the OpenAPI document: v1beta1-0.1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1beta1ParameterSpec {
    #[serde(rename = "feasibleSpace", skip_serializing_if = "Option::is_none")]
    pub feasible_space: Option<Box<crate::models::V1beta1FeasibleSpace>>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "parameterType", skip_serializing_if = "Option::is_none")]
    pub parameter_type: Option<String>,
}

impl V1beta1ParameterSpec {
    pub fn new() -> V1beta1ParameterSpec {
        V1beta1ParameterSpec {
            feasible_space: None,
            name: None,
            parameter_type: None,
        }
    }
}


