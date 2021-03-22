//asdf
use super::write_to_file;

use substring::Substring;

use serde::{Deserialize, Serialize};

use std::fs::OpenOptions;
use std::path::PathBuf;

use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct FormatType {
    pub sock_name: String,
    pub format_str: String,
    pub data_file: Option<String>,

    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    vars: HashMap<String, String>,

    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    vars_key_order_vec: Vec<String>,

    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    split_str: Vec<String>,
}

impl FormatType {
    pub fn new() -> Self {
        FormatType {
            sock_name: "0".to_owned(),
            data_file: None,
            format_str: "Hi! Date is {date}. Time is {h}:{m}:{s}".to_owned(),
            vars: HashMap::new(),
            vars_key_order_vec: vec![],
            split_str: vec![],
        }
    }
    pub fn update_var(&mut self, var: &str, value: &str) {
        if !&self.vars.contains_key(var) {
            return;
        }
        self.vars
            .entry(var.to_owned())
            .and_modify(|e| *e = value.to_string());
        println!("{:?}", self.vars);
    }
    pub fn format(&mut self) -> String {
        // split the str into vectors based on the tuples, then put the values in it and then return
        let mut index = 0;

        let mut final_str = String::new();
        let empty_char = String::new();
        for i in &self.split_str {
            final_str += i;

            if &self.vars_key_order_vec.len() == (&index as &usize) {
                break;
            }
            let key = &self.vars_key_order_vec[index];

            let value = &self.vars.get(key).unwrap_or(&empty_char);

            final_str += value;

            index += 1;
        }
        final_str
    }

    pub fn compile_regex(&mut self) {
        use regex::Regex;

        let re = Regex::new(r"\{[a-zA-z0-9]+\}*").unwrap();

        for i in re.find_iter(&self.format_str) {
            println!("{:?}", i);

            let st = &self.format_str.substring(i.start() + 1, i.end() - 1);
            println!("{}", st);

            self.vars.insert(st.to_owned().to_string(), "".to_owned());
            self.vars_key_order_vec.push(st.to_owned().to_string());
        }
        let str_vec = re.split(&self.format_str);

        for i in str_vec {
            &self.split_str.push(i.to_string());
        }

        println!("{:?}", &self.split_str);
    }
}

pub fn get_format_type(sock_name: String) -> FormatType {
    let path = PathBuf::new().join("/home/devhypercoder/.config/trigout.json");

    if !path.exists() {
        write_default_config_file(path).unwrap();
        return FormatType::new();
    }

    let formats = read_config_file(path).unwrap();

    for mut format_type in formats {
        if format_type.sock_name != sock_name {
            continue;
        }
        format_type.compile_regex();
        return format_type;
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
