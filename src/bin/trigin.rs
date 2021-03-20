use trigout::{get_args, get_socket_name, get_socket_path};

use std::io::prelude::Write;
use std::io::stdin;
use std::os::unix::net::UnixStream;

fn main() {
    println!("TRIGIN - WIP");
    let args = get_args();
    let socket_name = get_socket_name(args);
    let mut socket = UnixStream::connect(get_socket_path(socket_name)).unwrap();
    let mut input = String::new();

    stdin().read_line(&mut input).unwrap();

    socket.write_all(input.as_bytes()).unwrap();
    use std::net::Shutdown;
    socket.shutdown(Shutdown::Read).unwrap();
}
