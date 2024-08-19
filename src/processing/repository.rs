use std::{io::Cursor, path::Path};

use reqwest::blocking;
use zip::ZipArchive;

pub fn download_and_extract_zip(url: &str, output_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let response = blocking::get(url)?;
    let mut zip = ZipArchive::new(Cursor::new(response.bytes()?))?;
    zip.extract(output_dir)?;
    Ok(())
}