use super::PrimeChecked;

/**
 * A very basic implementation of primes
 */
#[derive(Default)]
pub struct Primes(PrimeChecked);
impl Primes {
    pub fn reset(&mut self) {
        self.0.reset();
    }
}
impl Iterator for Primes {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(i) = self.0.next() {
            if super::is_prime(i) {
                return Some(i);
            }
        }
        None
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
