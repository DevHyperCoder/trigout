use std::fs::create_dir;
use std::io::{BufRead, BufReader};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::PathBuf;

use trigout::format::{get_format_type, FormatType};
use trigout::{get_args, get_socket_name, get_socket_path, write_to_file};

const QUIT_CHAR: &str = "q";

/// Return a Option based on the argument vector
fn get_file_name(args: &[String]) -> Option<String> {
    if args.len() < 3 {
        return None;
    }

    Some(args[2].clone())
}

/// Config, parsed from argument list
struct Config {
    socket_name: String,
    file: Option<String>,
    format_type: FormatType,
}

impl Config {
    fn parse_config(args: &[String]) -> Config {
        Config {
            socket_name: get_socket_name(&args),
            file: get_file_name(&args),
            format_type: get_format_type(get_socket_name(&args))
                .expect("Can not find config for given socket name"),
        }
    }
}

fn main() {
    let args = get_args();

    let cfg = Config::parse_config(&args);

    match create_socket_dir() {
        true => {}
        false => panic!("Unable to create socket dir"),
    };

    listen_to_socket(cfg);
}

/// Create the dir, `/tmp/trigout/` if it does not exist
/// Return true if dir exists, or if created successfully
/// Return false if unable to create dir
fn create_socket_dir() -> bool {
    if PathBuf::new().join("/tmp/trigout/").exists() {
        return true;
    }

    match create_dir("/tmp/trigout/") {
        Ok(_a) => true,
        Err(_a) => false,
    }
}

/// Listen to "/tmp/trigout/<socket_name>"
/// Only ONE client supported
/// Calls `handle_client()` internally.
fn listen_to_socket(mut cfg: Config) {
    let listener = UnixListener::bind(get_socket_path(&cfg.socket_name));
    match listener {
        Ok(a) => {
            for stream in a.incoming() {
                match stream {
                    Ok(stream) => {
                        // If client sends a QUIT_CHAR, break out of the loop
                        if handle_client(stream, &mut cfg) {
                            break;
                        }
                    }
                    Err(err) => {
                        eprintln!("Error: {}", err);
                    }
                }
            }
        }
        Err(err) => eprintln!("Error: {}", err),
    }
}

/// Read stream and print to stdout
/// Return true if, QUIT_CHAR is read from the stream
/// Return false when the stream is closed by client
fn handle_client(stream: UnixStream, cfg: &mut Config) -> bool {
    let reader = BufReader::new(&stream);
    for line in reader.lines() {
        let data = line.unwrap();

        if data == QUIT_CHAR {
            return true;
        }

        let var_val = get_var_val(&data);

        cfg.format_type.update_var(var_val.0, var_val.1);
        let data = &cfg.format_type.format();
        println!("{}", data);

        if cfg.file.is_some() {
            let filename = cfg.file.as_ref().unwrap();
            write_to_file(PathBuf::new().join(filename), &data).unwrap();
        }
    }
    false
}

/// Return a tuple for a split of data by '='
fn get_var_val(data: &str) -> (&str, &str) {
    if !data.contains('=') {
        return ("", "");
    }

    let v: Vec<&str> = data.split('=').collect();

    (v[0], v[1])
}
