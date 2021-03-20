use std::env;

/// Return all arguments provided to program
pub fn get_args() -> Vec<String> {
    env::args().collect()
}

/// Get socket path `/tmp/trigout/<socket_name>`
pub fn get_socket_path(socket_name: String) -> String {
    format!("/tmp/trigout/{}", socket_name)
}

/// Return the first argument to the program
/// If no argument is given, "0" is retuned
pub fn get_socket_name(args: Vec<String>) -> String {
    if args.len() < 2 {
        return "0".to_owned();
    }
    args[1].clone()
}
