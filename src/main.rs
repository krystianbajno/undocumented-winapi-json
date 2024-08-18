use serde_derive::Serialize;
use std::fs::File;
use std::io::Write;
use reqwest;
use regex::Regex;
use std::collections::HashMap;

#[derive(Serialize)]
struct Module {
    module_name: String,
    functions: Vec<Function>,
}

#[derive(Serialize)]
struct Function {
    function_name: String,
    function_link: String,
    ret_type: String,
    params: Vec<String>,
}

fn download_file(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::blocking::get(url)?;
    Ok(response.text()?)
}

fn parse_function_definitions(content: &str, file_link: &str) -> Vec<Function> {
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
            function_link
        });
    }
    
    functions
}

fn main() {
    let files = vec![
        "ntbcd.h", "ntdbg.h", "ntexapi.h", "ntgdi.h", "ntimage.h", "ntioapi.h", "ntkeapi.h", 
        "ntldr.h", "ntlpcapi.h", "ntmisc.h", "ntmmapi.h", "ntnls.h", "ntobapi.h", "ntpebteb.h",
        "ntpfapi.h", "ntpnpapi.h", "ntpoapi.h", "ntpsapi.h", "ntregapi.h", "ntrtl.h", "ntsam.h",
        "ntseapi.h", "ntsmss.h", "ntsxs.h", "nttmapi.h", "nttp.h", "ntwmi.h", "ntwow64.h", 
        "ntxcapi.h", "ntzwapi.h", "phnt.h", "phnt_ntdef.h", "phnt_windows.h", "subprocesstag.h",
        "usermgr.h", "winsta.h"
    ];

    let base_url = "https://raw.githubusercontent.com/winsiderss/phnt/master/";
    let phnt_base_link = "https://github.com/winsiderss/phnt/blob/master/";

    let mut modules = HashMap::new();

    for file_name in files {
        let file_url = format!("{}{}", base_url, file_name);
        let file_link = format!("{}{}", phnt_base_link, file_name);
        
        match download_file(&file_url) {
            Ok(content) => {
                let module_name = "ntdll.dll".to_string();
                let functions = parse_function_definitions(&content, &file_link);

                if !functions.is_empty() {
                    let module = modules.entry(module_name.clone()).or_insert(Module {
                        module_name,
                        functions: Vec::new(),
                    });

                    module.functions.extend(functions);
                }
            },
            Err(err) => {
                eprintln!("Failed to download or parse {}: {}", file_name, err);
            }
        }
    }

    let res = serde_json::to_string_pretty(&modules.values().collect::<Vec<&Module>>()).unwrap();
    let mut handler = File::create("./output.json").unwrap();
    handler.write_all(res.as_bytes()).unwrap();
}
