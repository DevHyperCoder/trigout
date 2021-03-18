use std::env;

pub fn get_args() -> Vec<String> {
    env::args().collect()
}
