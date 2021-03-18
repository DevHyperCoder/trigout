use trigout::get_args;

use std::fs::create_dir;
use std::path::PathBuf;

use std::io::{BufRead, BufReader};
use std::net::Shutdown;
use std::os::unix::net::{UnixListener, UnixStream};

const QUIT_CHAR: &str = "q";

/// Read stream and print to stdout
/// Shutdown the stream if, QUIT_CHAR is read from the stream
fn handle_client(stream: UnixStream) {
    let reader = BufReader::new(&stream);
    for line in reader.lines() {
        let data = line.unwrap();
        println!("{}", data);
        if data == QUIT_CHAR {
            break;
        }
    }
    stream.shutdown(Shutdown::Both).expect("Unable to shutdown");
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
    let listener = UnixListener::bind(format!("/tmp/trigout/{}", socket_name));
    match listener {
        Ok(a) => {
            for stream in a.incoming() {
                match stream {
                    Ok(stream) => {
                        handle_client(stream);
                        break;
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

/// Return the first argument to the program
/// If no argument is given, "0" is retuned
fn get_socket_name(args: Vec<String>) -> String {
    if args.len() < 2 {
        return "0".to_owned();
    }
    return args[1].clone();
}
