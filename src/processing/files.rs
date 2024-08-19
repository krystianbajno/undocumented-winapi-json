use std::fs::File;
use std::io::Write;
use crate::models::output_data::OutputData;

pub fn save_to_json(file_name: &str, data: &OutputData) {
    let res = serde_json::to_string(data).unwrap();
    let mut handler = File::create(file_name).unwrap();
    handler.write_all(res.as_bytes()).unwrap();
}
