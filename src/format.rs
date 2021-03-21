//asdf
use super::write_to_file;

use serde::{Deserialize, Serialize};

use std::fs::OpenOptions;
use std::path::PathBuf;

use serde_json;

#[derive(Serialize, Deserialize)]
pub struct FormatType {
    pub sock_name: String,
    pub format_str: String,
    pub data_file: Option<String>,
}

impl FormatType {
    pub fn new() -> Self {
        FormatType {
            sock_name: "0".to_owned(),
            data_file: None,
            format_str: "Hi! Date is {date}".to_owned(),
        }
    }
    pub fn format(&self) -> &String {
        &self.format_str
    }
}

pub fn get_format_type(sock_name: String) -> FormatType {
    let path = PathBuf::new().join("/home/devhypercoder/.config/trigout.json");

    if !path.exists() {
        write_default_config_file(path).unwrap();
        return FormatType::new();
    }

    let formats = read_config_file(path).unwrap();

    for i in formats {
        if i.sock_name != sock_name {
            continue;
        }

        return i;
    }
    panic!(format!(
        "Can not find a configuration for socket: {}",
        sock_name
    ));
}

/// Read from config file and give back a struct
fn read_config_file(path: PathBuf) -> Result<Vec<FormatType>, std::io::Error> {
    let file = OpenOptions::new().read(true).open(&path);
    match file {
        Ok(_) => {
            let content = std::fs::read_to_string(&path).unwrap();
            let formats: Vec<FormatType> = serde_json::from_str(&content).unwrap();
            Ok(formats)
        }
        Err(e) => Err(e),
    }
}

fn write_default_config_file(path: PathBuf) -> Result<(), std::io::Error> {
    write_to_file(path, &generate_default_config())
}

fn generate_default_config() -> String {
    let format_vec = vec![FormatType::new()];
    serde_json::to_string_pretty(&format_vec).unwrap()
}
