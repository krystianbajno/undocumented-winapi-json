use std::collections::HashMap;

use serde_derive::Serialize;

use super::{module::Module, syscall::Syscall};

#[derive(Serialize)]
pub struct OutputData {
    pub modules: Vec<Module>,
    pub syscalls: HashMap<String, Vec<Syscall>>,
}