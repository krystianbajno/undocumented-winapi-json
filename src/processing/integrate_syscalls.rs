use std::collections::HashMap;

use crate::models::{module::Module, syscall::Syscall};

pub fn integrate_syscalls_into_modules(
    modules: &mut HashMap<String, Module>, 
    syscalls: HashMap<String, Vec<Syscall>>
) {
    for (function_name, syscall_list) in syscalls {
        for module in modules.values_mut() {
            for function in module.functions.iter_mut() {
                if function.function_name == function_name {
                    function.syscalls.extend(syscall_list.clone());
                }
            }
        }
    }
}
