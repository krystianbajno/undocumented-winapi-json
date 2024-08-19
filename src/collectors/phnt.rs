use std::collections::HashMap;
use std::path::Path;

use crate::models::module::Module;
use crate::processing::extract::process_files_in_directory;
use crate::processing::repository::download_and_extract_zip;

pub fn collect() -> Result<HashMap<String, Module>, Box<dyn std::error::Error>> {
    let zip_url = "https://github.com/winsiderss/phnt/archive/refs/heads/master.zip";
    let output_dir = Path::new("phnt-master");
    let mut modules = HashMap::new();

    if let Err(err) = download_and_extract_zip(zip_url, &output_dir) {
        eprintln!("Failed to download or extract ZIP file: {}", err);
        return Ok(modules)
    }

    let phnt_base_link = "https://github.com/winsiderss/phnt/blob/master/";

    let root_dir = output_dir.join("phnt-master");

    process_files_in_directory(&output_dir, phnt_base_link, &mut modules, &root_dir);

    Ok(modules)
}
