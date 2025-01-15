use std::fs;

use crate::tens::eighteen::Pyramid;

pub fn run() {
    let mut pyramid = Pyramid::parse(&fs::read_to_string("Files/sixty_seven.txt").unwrap());
    println!("{}", pyramid.max_path());
}
