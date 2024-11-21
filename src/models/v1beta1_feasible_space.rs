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
pub struct V1beta1FeasibleSpace {
    #[serde(rename = "list", skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<String>>,
    #[serde(rename = "max", skip_serializing_if = "Option::is_none")]
    pub max: Option<String>,
    #[serde(rename = "min", skip_serializing_if = "Option::is_none")]
    pub min: Option<String>,
    #[serde(rename = "step", skip_serializing_if = "Option::is_none")]
    pub step: Option<String>,
}

impl V1beta1FeasibleSpace {
    pub fn new() -> V1beta1FeasibleSpace {
        V1beta1FeasibleSpace {
            list: None,
            max: None,
            min: None,
            step: None,
        }
    }
}

