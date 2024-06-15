use std::env;

// get arg nth by idx
pub fn get_arg(idx: usize) -> String {
    env::args().nth(idx).unwrap_or("".to_string())
}
