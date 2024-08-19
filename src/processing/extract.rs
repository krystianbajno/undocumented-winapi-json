use std::{collections::HashMap, fs, path::Path};
use regex::Regex;
use crate::models::{function::Function, module::Module};

pub fn parse_ntoskrnl_syscalls_and_ntapi(content: &str, file_link: &str) -> Vec<Function> {
    parse_definitions(content, file_link, r"(?:NTSYSCALLAPI|NTSYSAPI)\s+(?P<ret_type>\w[\w\s\*]+)\s+(?:NTAPI|WINAPI)\s+(?P<name>\w+)\s*\((?P<params>[^)]*)\)")
}

pub fn parse_win32k_syscalls_and_functions(content: &str, file_link: &str) -> Vec<Function> {
    parse_definitions(content, file_link, r"(?:W32KAPI)\s+(?P<ret_type>\w[\w\s\*]+)\s+(?:NTAPI|WINAPI|__kernel_entry|APIENTRY)\s+(?P<name>\w+)\s*\((?P<params>[^)]*)\)")
}

pub fn parse_hidden_ntapi_and_winapi(content: &str, file_link: &str) -> Vec<Function> {
    parse_definitions(content, file_link, r"(?:NTSYSAPI|NTAPI|WINAPI|__stdcall|__fastcall)\s+(?P<ret_type>\w[\w\s\*]+)\s+(?P<name>Nt\w+|Zw\w+)\s*\((?P<params>[^)]*)\)")
}


pub fn process_files_in_directory(
    dir: &Path, 
    base_url: &str, 
    modules: &mut HashMap<String, Module>,
    root_dir: &Path,
) {
    if dir.is_dir() {
        for entry in fs::read_dir(dir).expect("Directory not found") {
            let entry = entry.expect("Failed to read entry");
            let path = entry.path();
            if path.is_dir() {
                process_files_in_directory(&path, base_url, modules, root_dir);
            } else if path.extension().map(|ext| ext == "h" || ext == "c").unwrap_or(false) {
                println!("{:?}", path.as_os_str());
                let content = match fs::read(&path) {
                    Ok(bytes) => String::from_utf8_lossy(&bytes).into_owned(),
                    Err(err) => {
                        eprintln!("Failed to read file {}: {}", path.display(), err);
                        continue;
                    }
                };

                let relative_path = path.strip_prefix(root_dir).unwrap();
                let file_link = format!("{}/{}", base_url.trim_end_matches('/'), relative_path.display());

                let ntoskrnl_functions = parse_ntoskrnl_syscalls_and_ntapi(&content, &file_link);
                let win32k_functions = parse_win32k_syscalls_and_functions(&content, &file_link);
                let winapi_functions = parse_hidden_ntapi_and_winapi(&content, &file_link);

                if !ntoskrnl_functions.is_empty() || !winapi_functions.is_empty() {
                    let module_name = "ntdll.dll".to_string();

                    let module = modules.entry(module_name.clone()).or_insert(Module {
                        module_name,
                        functions: Vec::new(),
                    });

                    module.functions.extend(ntoskrnl_functions);
                    module.functions.extend(winapi_functions);
                }

                if !win32k_functions.is_empty() {
                    let module_name = "win32k.sys".to_string();

                    let module = modules.entry(module_name.clone()).or_insert(Module {
                        module_name,
                        functions: Vec::new(),
                    });

                    module.functions.extend(win32k_functions);
                }
            }
        }
    }
}

pub fn parse_definitions(content: &str, file_link: &str, pattern: &str) -> Vec<Function> {
    let mut functions = Vec::new();
    
    let re = Regex::new(pattern).unwrap();
    
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
            syscalls: Vec::new(),
        });
    }
    
    functions
}