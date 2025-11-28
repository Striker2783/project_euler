use num::BigUint;

pub fn run() {
    println!("{}", solve());
}

fn solve() -> u32 {
    for (i, n) in BigFibonacci::new().enumerate() {
        if !is_pandigital((n.clone() % 10u64.pow(10)).to_u64_digits()[0]) {
            continue;
        }
        let str = n.to_string();
        if str.len() < 10 {
            continue;
        }
        if !is_pandigital(str[0..9].parse().unwrap()) {
            continue;
        }
        return i as u32 + 1;
    }
    unreachable!()
}

fn is_pandigital(mut n: u64) -> bool {
    let mut set = [false; 9];
    while n > 0 {
        let digit = n % 10;
        if digit != 0 {
            set[digit as usize - 1] = true;
        }
        n /= 10;
    }
    set.iter().all(|b| *b)
}

struct BigFibonacci {
    prev: BigUint,
    curr: BigUint,
}

impl BigFibonacci {
    fn new() -> Self {
        Self {
            prev: BigUint::ZERO,
            curr: BigUint::from(1u32),
        }
    }
}
impl Iterator for BigFibonacci {
    type Item = BigUint;

    fn next(&mut self) -> Option<Self::Item> {
        let added = self.prev.clone() + self.curr.clone();
        let curr = self.curr.clone();
        self.prev = self.curr.clone();
        self.curr = added;
        Some(curr)
    }
}

#[cfg(test)]
mod tests {
    use num::{BigUint, FromPrimitive};

    use crate::tens::four::{BigFibonacci, is_pandigital};

    #[test]
    fn test_fibonacci() {
        let mut fib = BigFibonacci::new();
        assert_eq!(fib.next(), BigUint::from_u32(1));
        assert_eq!(fib.next(), BigUint::from_u32(1));
        assert_eq!(fib.next(), BigUint::from_u32(2));
        assert_eq!(fib.next(), BigUint::from_u32(3));
    }
    #[test]
    fn test_is_pandigital() {
        assert!(is_pandigital(123456789));
        assert!(!is_pandigital(12345679));
    }
}
