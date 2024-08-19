use serde_derive::Serialize;

use super::module::Module;

#[derive(Serialize)]
pub struct OutputData {
    pub modules: Vec<Module>,
    // pub syscalls: HashMap<String, Vec<Syscall>>,
}