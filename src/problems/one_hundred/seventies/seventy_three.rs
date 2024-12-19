use std::mem::swap;

pub fn run() {
    println!("{}", solve(Fraction(1, 3), Fraction(1,2), 12_000));
}
/// Can be Optimized
fn solve(min: Fraction, max: Fraction, n: u32) -> i32 {
    let mut count = 0;
    for d in 2..=n {
        for num in 1..n {
            let f = Fraction(num, d);
            if gcd(num, d) != 1 {
                continue;
            }
            if min < f && f < max {
                count += 1;
            }
        }
    }
    count
}
#[derive(Debug, PartialEq, Eq)]
struct Fraction(u32, u32);

impl PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Fraction {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.0 * other.1).cmp(&(other.0 * self.1))
    }
}

fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b > 0 {
        swap(&mut a, &mut b);
        b %= a;
    }
    a
}

#[cfg(test)]
mod tests {
    use crate::problems::one_hundred::seventies::seventy_three::{gcd, solve, Fraction};

    #[test]
    fn test_fractions() {
        assert!(Fraction(1, 6) > Fraction(1, 10));
    }
    #[test]
    fn test_gcd() {
        assert_eq!(gcd(132, 11), 11);
    }
    #[test]
    fn test_solve() {
        assert_eq!(solve(Fraction(1, 3), Fraction(1,2), 8), 3);
    }
}
