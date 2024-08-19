use serde_derive::Deserialize;

use super::winapi_to_json_function::WinapiToJsonFunction;

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct WinapiToJsonModule {
    pub module_name: String,
    pub functions: Vec<WinapiToJsonFunction>,
}
