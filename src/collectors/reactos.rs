use std::collections::HashMap;
use std::env;
use std::path::Path;

use crate::models::module::Module;
use crate::processing::extract::process_files_in_directory;
use crate::processing::repository::download_and_extract_zip;

pub fn collect() -> Result<HashMap<String, Module>, Box<dyn std::error::Error>> {
    let zip_url = "https://github.com/reactos/reactos/archive/refs/heads/master.zip";
    let output_dir = Path::new("reactos-master");

    let base_dir = env::current_dir()?;
    println!("The current directory is: {}", base_dir.display());

    let mut modules = HashMap::new();

    if let Err(err) = download_and_extract_zip(zip_url, &output_dir) {
        eprintln!("Failed to download or extract ZIP file: {}", err);
        return Ok(modules);
    }

    let reactos_base_link = "https://github.com/reactos/reactos/blob/master/";

    process_files_in_directory(&output_dir, base_dir.as_path(), reactos_base_link, &mut modules);

    Ok(modules)
}
