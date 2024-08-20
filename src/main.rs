use collectors::{phnt, reactos, syscalls::nt_syscalls::fetch_syscalls, winapi_to_json};
use models::output_data::OutputData;
use processing::{files::save_to_json, integrate_syscalls::integrate_syscalls_into_modules, merge::merge_modules};

mod collectors;
mod models;
mod processing;

fn main() {
    let winapi_to_json = winapi_to_json::collect().unwrap();
    let phnt_modules = phnt::collect().unwrap();
    let reactos_modules = reactos::collect().unwrap();
    let syscalls = fetch_syscalls().unwrap();

    let mut merged_modules = merge_modules(reactos_modules, phnt_modules);
    merged_modules = merge_modules(merged_modules, winapi_to_json);
    
    integrate_syscalls_into_modules(&mut merged_modules, syscalls.clone());

    let output_data = OutputData {
        modules: merged_modules.values().cloned().collect(),
    };

    let output_file = "output.json";
    save_to_json(output_file, &output_data);
}
