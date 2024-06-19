use super::PrimeChecked;

/**
 * A very basic implementation of primes
 */
#[derive(Default)]
pub struct Primes {
    possible: PrimeChecked,
    vec: Vec<u32>,
    i: usize,
}
impl Primes {
    pub fn reset(&mut self) {
        self.possible.reset();
    }
    pub fn is_prime(&mut self, n: u32) -> bool {
        if n == 1 || n == 0 {
            false;
        }
        while n >= self.vec.last().get_or_insert(&0).pow(2) {
            self.add_prime();
        }
        unsafe { self.is_prime_no_check(n) }
    }
    unsafe fn is_prime_no_check(&mut self, n: u32) -> bool {
        for &x in &self.vec {
            if x * x > n {
                break;
            }
            if n % x == 0 {
                return false;
            }
        }
        true
    }
    fn add_prime(&mut self) -> bool {
        while let Some(i) = self.possible.next() {
            if unsafe { self.is_prime_no_check(i) } {
                self.vec.push(i);
                return true;
            }
        }
        false
    }
}
impl Iterator for Primes {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        while self.i >= self.vec.len() && self.add_prime() {}
        if self.i >= self.vec.len() {
            None
        } else {
            self.i += 1;
            return Some(self.vec[self.i - 1]);
        }
    }
}

#[derive(Default)]
pub struct Squares(u32);
impl Iterator for Squares {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.0 += 1;
        self.0.checked_mul(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::{Primes, Squares};

    macro_rules! test {
        ($name:ident, $series:ident,$value:expr) => {
            #[test]
            fn $name() {
                let mut series = $series::default();
                for v in $value {
                    assert_eq!(series.next(), Some(v))
                }
            }
        };
    }
    test!(test_primes, Primes, [2, 3, 5, 7, 11, 13, 17, 19, 23]);
    test!(test_squares, Squares, [1, 4, 9, 16, 25, 36, 49, 64, 81]);
}
