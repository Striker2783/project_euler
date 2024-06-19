use std::{collections::HashSet, ops::Add};

pub fn run() {
    let min = solve();
    println!("{min}");
}

fn solve() -> u32 {
    let set: HashSet<_> = PentagonalNumbers::default().collect();
    let mut min = u32::MAX;
    for a in PentagonalNumbers::default() {
        for b in PentagonalNumbers::default() {
            if b.abs_diff(a) > min {
                continue;
            }
            if set.contains(&a.add(b)) && set.contains(&b.abs_diff(a)) {
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
