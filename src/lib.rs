#![feature(test)]

extern crate test;

pub fn run(n: usize) {
    if n <= 100 {
        one_hundred::run(n)
    }
}

pub fn run_all() {
    one_hundred::run_all();
}