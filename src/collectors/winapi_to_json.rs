use std::collections::HashMap;
use reqwest::blocking::get;
use crate::models::module::Module;
use crate::models::function::Function;
use crate::models::winapi_to_json::winapi_to_json_module::WinapiToJsonModule;

pub fn collect() -> Result<HashMap<String, Module>, Box<dyn std::error::Error>> {
    let url = "https://github.com/Artideusz/winapi_to_json/releases/latest/download/output.json";
    
    let response = get(url)?.text()?;
    
    let json_modules: Vec<WinapiToJsonModule> = serde_json::from_str(&response)?;
    
    let mut modules_map = HashMap::new();
    for json_module in json_modules {
        let functions = json_module.functions.into_iter().map(|json_function| {
            Function {
                function_name: json_function.function_name,
                function_link: String::from("/api/ms"),
                ret_type: json_function.ret_type,
                params: json_function.params,
                syscalls: Vec::new()
            }
        }).collect();

        let module = Module {
            module_name: json_module.module_name.clone(),
            functions,
        };

        modules_map.insert(json_module.module_name, module);
    }

    println!("Fetched {} modules from winapi_to_json", modules_map.len());

    Ok(modules_map)
}
