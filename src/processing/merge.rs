use std::collections::HashMap;
use crate::models::{function::Function, module::Module};

pub fn merge_modules(
    mut primary_modules: HashMap<String, Module>,
    secondary_modules: HashMap<String, Module>,
) -> HashMap<String, Module> {
    for (module_name, secondary_module) in secondary_modules {
        if let Some(primary_module) = primary_modules.get_mut(&module_name) {
            merge_functions(&mut primary_module.functions, secondary_module.functions);
        } else {
            primary_modules.insert(module_name, secondary_module);
        }
    }

    primary_modules
}

fn merge_functions(primary_functions: &mut Vec<Function>, secondary_functions: Vec<Function>) {
    let mut existing_function_names: HashMap<String, ()> = primary_functions
        .iter()
        .map(|f| (f.function_name.clone(), ()))
        .collect();

    for function in secondary_functions {
        if !existing_function_names.contains_key(&function.function_name) {
            primary_functions.push(function.clone());
            existing_function_names.insert(function.function_name.clone(), ());
        }
    }
}