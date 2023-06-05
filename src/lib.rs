pub mod problems;
use std::process;

use problems::one_hundred::*;

pub fn run() {
    if let Err(e) = eleven::run() {
        eprintln!("{e}");
        process::exit(1);
    }
}
