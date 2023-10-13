use std::{mem::swap, ops::AddAssign};

use num::BigUint;

struct BigFib {
    prev: BigUint,
    curr: BigUint,
}
impl BigFib {
    pub fn new() -> Self {
        Self {
            prev: BigUint::from(0u32),
            curr: BigUint::from(1u32),
        }
    }
    pub fn get(&self) -> BigUint {
        self.curr.clone()
    }
    pub fn next(&mut self) {
        swap(&mut self.curr, &mut self.prev);
        self.curr.add_assign(&self.prev);
    }
    pub fn digits(n: u32) -> u32 {
        let mut f = Self::new();
        let mut i = 1;
        while (f.get().to_str_radix(10).len() < n as usize) {
            f.next();
            i += 1;
        }
        i
    }
}

pub fn run() {
    println!("{}", BigFib::digits(1000));
}

#[cfg(test)]
mod tests {
    use num::BigUint;

    use super::BigFib;

    #[test]
    fn fib() {
        let mut f = BigFib::new();
        f.next();
        assert_eq!(f.get(), BigUint::from(1u32));
        f.next();
        f.next();
        f.next();
        f.next();
        assert_eq!(f.get(), BigUint::from(8u32));
    }
    #[test]
    fn digits() {
        assert_eq!(BigFib::digits(2), 7);
        assert_eq!(BigFib::digits(1000), 4782);
    }
}
