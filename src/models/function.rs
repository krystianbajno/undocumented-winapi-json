
use serde_derive::Serialize;

use super::syscall::Syscall;

#[derive(Serialize, Clone)]
pub struct Function {
    pub function_name: String,
    pub function_link: String,
    pub ret_type: String,
    pub params: Vec<String>,
    pub syscalls: Vec<Syscall>,
}
