use reqwest::blocking;
use serde_json::Value;
use std::error::Error;
use std::collections::HashMap;

use crate::models::syscall::Syscall;

const SYSCALLS_URLS: &[(&str, &str, &str)] = &[
    ("x64", "ntoskrnl", "https://raw.githubusercontent.com/j00ru/windows-syscalls/master/x64/json/nt-per-syscall.json"),
    ("x64", "win32k", "https://raw.githubusercontent.com/j00ru/windows-syscalls/master/x64/json/win32k-per-syscall.json"),
    ("x86", "ntoskrnl", "https://raw.githubusercontent.com/j00ru/windows-syscalls/master/x86/json/nt-per-syscall.json"),
    ("x86", "win32k", "https://raw.githubusercontent.com/j00ru/windows-syscalls/master/x86/json/win32k-per-syscall.json"),
];

pub fn fetch_syscalls() -> Result<HashMap<String, Vec<Syscall>>, Box<dyn Error>> {
    let mut syscall_data = HashMap::new();

    for &(arch, module, url) in SYSCALLS_URLS.iter() {
        let response = blocking::get(url)?;
        let json: Value = response.json()?;

        if let Value::Object(obj) = json {
            for (function_name, syscalls) in obj {
                let mut syscalls_list = Vec::new();

                if let Value::Object(os_map) = syscalls {
                    for (os_version, syscall_num) in os_map {
                        if let Value::Object(service_pack_map) = syscall_num {
                            for (service_pack, syscall_number) in service_pack_map {
                                if let Some(num) = syscall_number.as_u64() {
                                    syscalls_list.push(Syscall {
                                        architecture: arch.to_string(),
                                        module_type: module.to_string(),
                                        os_version: os_version.clone(),
                                        service_pack: service_pack.clone(),
                                        syscall_number: num as u32,
                                    });
                                }
                            }
                        }
                    }
                }

                syscall_data
                    .entry(function_name.clone())
                    .or_insert_with(Vec::new)
                    .extend(syscalls_list);
            }
        }
    }

    println!("[+] {} syscalls fetched", syscall_data.len());

    Ok(syscall_data)
}
