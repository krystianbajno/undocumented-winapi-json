use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Syscall {
    pub architecture: String,
    pub module_type: String,
    pub os_version: String,
    pub service_pack: String,  
    pub syscall_number: u32,
}