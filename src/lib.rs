use std::env;
pub mod format;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

/// Return all arguments provided to program
pub fn get_args() -> Vec<String> {
    env::args().collect()
}

/// Get socket path `/tmp/trigout/<socket_name>`
pub fn get_socket_path(socket_name: &String) -> String {
    format!("/tmp/trigout/{}", socket_name)
}

/// Return the first argument to the program
/// If no argument is given, "0" is retuned
pub fn get_socket_name(args: &Vec<String>) -> String {
    if args.len() < 2 {
        return "0".to_owned();
    }
    args[1].clone()
}
/// Write `c` to a file at path
/// Propagates the error message to caller
pub fn write_to_file(path: PathBuf, c: &str) -> Result<(), std::io::Error> {
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
