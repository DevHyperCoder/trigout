use trigout::{get_args, get_socket_name, get_socket_path};

use std::fs::create_dir;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::PathBuf;

const QUIT_CHAR: &str = "q";

/// Write `c` to a file at path
/// Propagates the error message to caller
fn write_to_file(path: PathBuf, c: &str) -> Result<(), std::io::Error> {
    // Create file if not exist already
    // Remove previous content and write to it
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path);

    match file {
        Ok(mut f) => f.write_all(format!("{}\n", c).as_bytes()),
        Err(e) => Err(e),
    }
}

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
}

impl Config {
    fn parse_config(args: &Vec<String>) -> Config {
        Config {
            socket_name: get_socket_name(&args),
            file: get_file_name(&args),
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
fn listen_to_socket(cfg: Config) {
    let listener = UnixListener::bind(get_socket_path(&cfg.socket_name));
    match listener {
        Ok(a) => {
            for stream in a.incoming() {
                match stream {
                    Ok(stream) => {
                        // If client sends a QUIT_CHAR, break out of the loop
                        if handle_client(stream, &cfg) {
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
fn handle_client(stream: UnixStream, cfg: &Config) -> bool {
    let reader = BufReader::new(&stream);
    for line in reader.lines() {
        let data = line.unwrap();

        if data == QUIT_CHAR {
            return true;
        }

        println!("{}", data);

        if cfg.file.is_some() {
            let filename = cfg.file.as_ref().unwrap();
            write_to_file(PathBuf::new().join(filename), &data).unwrap();
        }
    }
    false
}
