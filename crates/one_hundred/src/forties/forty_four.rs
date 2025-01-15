use std::{collections::HashSet, ops::Add};

use num::integer::Roots;

pub fn run() {
    let min = solve();
    println!("{min}");
}

fn is_pentagonal(n: u32) -> bool {
    24u32
        .checked_mul(n)
        .and_then(|m| {
            let x = (m + 1).sqrt() + 1;
            let x = (x / 6);
            if x * (3 * x - 1) / 2 == n {
                Some(())
            } else {
                None
            }
        })
        .is_some()
}

fn get_min_pent(p: u32, min: u32) -> u32 {
    (1 + (p
        .saturating_mul(8)
        .saturating_add(min.saturating_mul(8))
        .saturating_add(1))
    .sqrt())
        / 6
}

fn solve() -> u32 {
    let mut min = u32::MAX;
    for (i, a) in PentagonalNumbers::default().enumerate() {
        for b in PentagonalNumbers::default()
            .skip(i + 1)
            .take((get_min_pent(a, min) as usize).saturating_sub(i + 1))
        {
            if is_pentagonal(a + b) && is_pentagonal(b - a) {
                min = (b - a).min(min);
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

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.n += n as u32;
        self.next()
    }
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test_solve() {
        assert_eq!(solve(), 5482660)
    }
}
