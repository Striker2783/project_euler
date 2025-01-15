use num::{BigUint, One, Zero};

pub fn run() {
    let n = (0..=10_000).filter(|&n| !is_lychrel(n)).count();
    println!("{n}");
}

fn is_lychrel(n: u32) -> bool {
    let mut n = BigUint::from(n);
    n += reverse(&n);
    for _ in 0..49 {
        let reverse = reverse(&n);
        if reverse == n {
            return true;
        }
        n += reverse;
    }
    false
}

fn reverse(n: &BigUint) -> BigUint {
    let mut m = BigUint::ZERO;
    let mut pow = BigUint::one();
    for i in 0.. {
        let big_uint = n / pow.clone();
        if big_uint.is_zero() {
            break;
        }
        pow *= BigUint::from(10u32);
        m *= BigUint::from(10u32);
        m += big_uint % BigUint::from(10u32);
    }
    m
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use num::BigUint;

    use crate::fifties::fifty_five::{is_lychrel, reverse};

    #[test]
    fn test_reverse() {
        assert_eq!(
            reverse(&BigUint::from_str("4668731596684224866951378664").unwrap()),
            BigUint::from_str("4668731596684224866951378664").unwrap()
        );
    }
    #[test]
    fn test_is_lychrel() {
        assert!(is_lychrel(47));
        assert!(is_lychrel(349));
        assert!(!is_lychrel(10677));
        assert!(!is_lychrel(4994));
    }
}
