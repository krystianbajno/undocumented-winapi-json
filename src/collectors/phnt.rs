use std::collections::HashMap;
use std::env;
use std::path::Path;

use crate::models::module::Module;
use crate::processing::extract::process_files_in_directory;
use crate::processing::repository::download_and_extract_zip;

pub fn collect() -> Result<HashMap<String, Module>, Box<dyn std::error::Error>> {
    let zip_url = "https://github.com/winsiderss/phnt/archive/refs/heads/master.zip";
    let base_dir = env::current_dir()?;
    let output_dir = Path::new("phnt-master");
    let mut modules = HashMap::new();

    if let Err(err) = download_and_extract_zip(zip_url, &output_dir) {
        eprintln!("Failed to download or extract ZIP file: {}", err);
        return Ok(modules)
    }

    let phnt_base_link = "https://github.com/winsiderss/phnt/blob/master/";


    process_files_in_directory(&output_dir, base_dir.as_path(), phnt_base_link, &mut modules);

    Ok(modules)
}
