pub mod board;
pub mod ai;

use std::io::stdin;

pub fn input() -> String {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    buf
}
