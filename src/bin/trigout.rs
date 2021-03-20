use trigout::{get_args, get_socket_name, get_socket_path};

use std::fs::create_dir;
use std::path::PathBuf;

use std::io::{BufRead, BufReader};
use std::os::unix::net::{UnixListener, UnixStream};

const QUIT_CHAR: &str = "q";

/// Read stream and print to stdout
/// Return true if, QUIT_CHAR is read from the stream
/// Return false when the stream is closed by client
fn handle_client(stream: UnixStream) -> bool {
    let reader = BufReader::new(&stream);
    for line in reader.lines() {
        let data = line.unwrap();
        println!("{}", data);
        if data == QUIT_CHAR {
            return true;
        }
    }
    false
}

fn main() {
    let args = get_args();

    let socket_name = get_socket_name(args);

    match create_socket_dir() {
        true => {}
        false => panic!("Unable to create socket dir"),
    };

    listen_to_socket(socket_name);
}

/// Listen to "/tmp/trigout/<socket_name>"
/// Only ONE client supported
/// Calls `handle_client()` internally.
fn listen_to_socket(socket_name: String) {
    let listener = UnixListener::bind(get_socket_path(socket_name));
    match listener {
        Ok(a) => {
            for stream in a.incoming() {
                match stream {
                    Ok(stream) => {
                        // If client sends a QUIT_CHAR, break out of the loop
                        if handle_client(stream) {
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
