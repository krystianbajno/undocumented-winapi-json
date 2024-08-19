
use serde_derive::Serialize;

#[derive(Serialize, Clone)]
pub struct Function {
    pub function_name: String,
    pub function_link: String,
    pub ret_type: String,
    pub params: Vec<String>,
}
