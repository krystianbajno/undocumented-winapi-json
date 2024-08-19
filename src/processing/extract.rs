use std::{collections::HashMap, fs, path::Path};
use regex::Regex;
use std::sync::Arc;
use std::str;
use crate::models::{function::Function, module::Module};

pub fn parse_function_definitions(content: &str, file_link: &str) -> Vec<Function> {
    let mut functions = Vec::new();
    
    let re = Regex::new(r"NTSYSAPI\s+(?P<ret_type>\w[\w\s\*]+)\s+NTAPI\s+(?P<name>\w+)\s*\((?P<params>[^)]*)\)").unwrap();
    
    for cap in re.captures_iter(content) {
        let ret_type = cap["ret_type"].trim().to_string();
        let function_link = file_link.to_string();
        let function_name = cap["name"].to_string();
        let params = cap["params"]
            .split(',')
            .map(|p| p.trim().to_string())
            .collect::<Vec<String>>();

        functions.push(Function {
            function_name,
            ret_type,
            params,
            function_link,
        });
    }
    
    functions
}

pub fn process_files_in_directory(
    dir: &Path, 
    base_url: &str, 
    modules: &mut HashMap<String, Module>, 
    get_module_name: Arc<Box<dyn Fn(&Path) -> String>>
) {
    if dir.is_dir() {
        for entry in fs::read_dir(dir).expect("Directory not found") {
            let entry = entry.expect("Failed to read entry");
            let path = entry.path();
            if path.is_dir() {
                process_files_in_directory(&path, base_url, modules, Arc::clone(&get_module_name));
            } else if path.extension().map(|ext| ext == "h" || ext == "c").unwrap_or(false) {
                println!("{:?}", path.as_os_str());
                let content = match fs::read(&path) {
                    Ok(bytes) => String::from_utf8_lossy(&bytes).into_owned(),
                    Err(err) => {
                        eprintln!("Failed to read file {}: {}", path.display(), err);
                        continue;
                    }
                };

                let relative_path = path.strip_prefix(dir).unwrap();
                let file_link = format!("{}/{}", base_url, relative_path.display());

                let functions = parse_function_definitions(&content, &file_link);

                if !functions.is_empty() {
                    let module_name = get_module_name(&path);

                    let module = modules.entry(module_name.clone()).or_insert(Module {
                        module_name,
                        functions: Vec::new(),
                    });

                    module.functions.extend(functions);
                }
            }
        }
    }
}
