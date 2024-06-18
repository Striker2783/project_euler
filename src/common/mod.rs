pub mod number_series;

pub const SMALL_PRIMES: &[u32] = &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
const PRIME_CHECKED: [u32; 4] = [2, 0, 3, 5];
#[derive(Default)]
struct PrimeChecked {
    last: u32,
    alt: bool,
}
impl Iterator for PrimeChecked {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if (self.last as usize) < PRIME_CHECKED.len() {
            self.last = PRIME_CHECKED[self.last as usize];
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

pub fn is_prime(n: u32) -> bool {
    if n == 0 || n == 1 {
        return false;
    }
    let mut nums = PrimeChecked::default();
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

#[cfg(test)]
mod tests {
    use crate::common::is_prime;
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
}
