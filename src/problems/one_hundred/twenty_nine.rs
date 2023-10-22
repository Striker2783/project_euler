use std::{collections::HashSet, ops::RangeInclusive};

use num::BigUint;

pub fn run() {
    let nums = naive(2..=100, 2..=100);
    println!("{nums}");
}

fn naive(a: RangeInclusive<u32>, b: RangeInclusive<u32>) -> u32 {
    let mut set = HashSet::new();
    for i in a.clone() {
        for j in b.clone() {
            set.insert(BigUint::from(i).pow(j));
        }
    }
    set.len() as u32
}

fn solve(a: RangeInclusive<u32>, b: RangeInclusive<u32>) -> u32 {
    let mut total = (a.end() - a.start() + 1) * (b.end() - b.start() + 1);

    for i in (*a.start()..).take_while(|&x| x * x <= *a.end()) {
        for j in b.clone().skip(1) {
            if j % 2 == 0 {
                total -= 1;
            }
        }
    }

    total
}

fn square_map_while(x: u32, a: RangeInclusive<u32>) -> Option<u32> {
    if x * x <= *a.end() {
        Some(x * x)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::problems::one_hundred::twenty_nine::{naive, solve};

    #[test]
    fn test_solve() {
        assert_eq!(naive(2..=5, 2..=5), 15);
        assert_eq!(naive(2..=100, 2..=100), 2);
    }
}
