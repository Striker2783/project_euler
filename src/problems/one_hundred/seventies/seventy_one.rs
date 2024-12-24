use std::{collections::HashMap, mem::swap};

pub fn run() {
    println!("{}", solve(Fraction(3, 7), 1_000_000).0);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Fraction(u64, u64);
impl Fraction {
    fn get_min(&self, d2: u64) -> Fraction {
        Fraction((self.0 * d2) / self.1, d2)
    }
}
impl PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some((self.0 * other.1).cmp(&(other.0 * self.1)))
    }
}
impl Ord for Fraction {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.0 * other.1).cmp(&(other.0 * self.1))
    }
}
fn solve(f: Fraction, max: u64) -> Fraction {
    let mut max_f = Fraction(0, 1);
    for d in 1..=max {
        if d == f.1 {
            continue;
        }
        let f2 = f.get_min(d);
        if gcd(f2.0, f2.1) != 1 {
            continue;
        }
        max_f = max_f.max(f2);
    }
    max_f
}
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b > 0 {
        swap(&mut a, &mut b);
        b %= a;
    }
    a
}

#[cfg(test)]
mod tests {
    use crate::problems::one_hundred::seventies::seventy_one::{solve, Fraction};
    #[test]
    fn test_fractions() {
        assert_eq!(Fraction(3, 7).get_min(3), Fraction(1, 3));
        assert_eq!(Fraction(3, 7).get_min(8), Fraction(3, 8));
        assert!(Fraction(2, 7) < Fraction(1, 3));
        assert!(Fraction(1, 3) < Fraction(3, 8));
    }
    #[test]
    fn test_solve() {
        assert_eq!(solve(Fraction(3, 7), 8), Fraction(2,5));
    }
}
