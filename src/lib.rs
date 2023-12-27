pub mod common;
pub mod problems;

use problems::one_hundred::{self, *};

pub fn run(n: u32) {
    if n <= 100 {
        one_hundred::run(n)
    }
}
