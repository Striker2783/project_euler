#![feature(test)]

extern crate test;

use std::ops::Div;

pub mod number_series;
pub mod shape_numbers;
pub mod sieve;
pub mod macros;

pub const SMALL_PRIMES: &[u32] = &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
#[derive(Default)]
struct PrimeChecked<T> {
    last: T,
    alt: bool,
}
impl<T: Default> PrimeChecked<T> {
    pub fn reset(&mut self) {
        self.last = Default::default();
        self.alt = false;
    }
}
macro_rules! PrimeCheckedIterator {
    ($type:ident) => {
        impl Iterator for PrimeChecked<$type> {
            type Item = $type;

            fn next(&mut self) -> Option<Self::Item> {
                if (self.last as usize) < 4 {
                    self.last = [2, 0, 3, 5][self.last as usize];
                    return Some(self.last);
                }
                self.alt = !self.alt;
                if self.alt {
                    self.last += 2;
                } else {
                    self.last += 4;
                }
                Some(self.last)
            }
        }
    };
}
PrimeCheckedIterator!(u32);
PrimeCheckedIterator!(u64);

pub fn is_prime(n: u32) -> bool {
    if n == 0 || n == 1 {
        return false;
    }
    let mut nums: PrimeChecked<u32> = PrimeChecked::default();
    while let Some(n2) = nums.next() {
        if n2 * n2 > n {
            break;
        }
        if n % n2 == 0 {
            return false;
        }
    }
    true
}
pub fn num_len(mut n: u32) -> u32 {
    let mut len = 0;
    while n > 0 {
        n /= 10;
        len += 1;
    }
    len
}

pub fn get_digits(mut n: u64) -> [u8; 10] {
    let mut digits = [0; 10];
    while n > 0 {
        digits[(n % 10) as usize] += 1;
        n /= 10;
    }
    digits
}
/**
 * Returns a max of `u64::MAX`
 */
pub fn combinations(n: u32, r: u32) -> u64 {
    let min = r.min(n - r);
    let max = r.max(n - r);
    let mut n_mul = max + 1;
    let mut d_mul = 2;
    let mut product = 1;
    while n_mul <= n {
        if let Some(a) = (n_mul as u64).checked_mul(product) {
            product = a;
            n_mul += 1;
            continue;
        }
        if d_mul > min {
            for i in n_mul..=n {
                product = product.saturating_mul(i as u64);
            }
            break;
        }
        product /= d_mul as u64;
        d_mul += 1;
    }
    for i in d_mul..=min {
        product = product.div(i as u64);
    }
    product
}

#[cfg(test)]
mod tests {
    use super::is_prime;

    use super::combinations;
    #[test]
    fn test_is_prime() {
        let primes = [2, 3, 5, 53, 97];
        let non_primes = [0, 1, 6, 10, 91, 5777];
        for p in primes {
            assert!(is_prime(p))
        }
        for np in non_primes {
            assert!(!is_prime(np))
        }
    }
    #[test]
    fn test_combinations() {
        assert_eq!(combinations(5, 3), 10);
        assert_eq!(combinations(23, 10), 1144066);
    }
}
