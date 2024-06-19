use std::{collections::HashSet, ops::Add};

use num::integer::Roots;

pub fn run() {
    let min = solve();
    println!("{min}");
}

fn is_pentagonal(n: u32) -> bool {
    let x = (1 + (24 * n + 1).sqrt()) / 6;
    x * (3 * x - 1) / 2 == n
}

fn solve() -> u32 {
    let mut min = u32::MAX;
    for a in PentagonalNumbers::default() {
        for b in PentagonalNumbers::default() {
            if b.abs_diff(a) > min {
                continue;
            }
            if is_pentagonal(a.add(b)) && is_pentagonal(b.abs_diff(a)) {
                min = b.abs_diff(a).min(min);
            }
        }
    }
    min
}

#[derive(Debug, Default)]
struct PentagonalNumbers {
    n: u32,
}
impl Iterator for PentagonalNumbers {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.n += 1;
        Some(self.n.checked_mul(3 * self.n - 1)? / 2)
    }
}
