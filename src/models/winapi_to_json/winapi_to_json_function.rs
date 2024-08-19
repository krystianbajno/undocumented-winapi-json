use serde_derive::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct WinapiToJsonFunction {
    pub function_name: String,
    pub ret_type: String,
    pub params: Vec<String>,
}