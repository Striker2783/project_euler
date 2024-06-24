#![feature(test)]

extern crate test;

pub(crate) mod macros;
pub mod problems;

use problems::one_hundred::{self};

pub fn run(n: usize) {
    if n <= 100 {
        one_hundred::run(n)
    }
}
