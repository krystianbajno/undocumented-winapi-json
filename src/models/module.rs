use serde_derive::Serialize;

use crate::models::function::Function;

#[derive(Serialize, Clone)]
pub struct Module {
    pub module_name: String,
    pub functions: Vec<Function>,
}
