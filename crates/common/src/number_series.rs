use super::PrimeChecked;

/**
 * A very basic implementation of primes
 */
#[derive(Default)]
pub struct Primes<T> {
    possible: PrimeChecked<T>,
    vec: Vec<T>,
    i: usize,
}
impl<T: Default> Primes<T> {
    pub fn reset(&mut self) {
        self.possible.reset();
    }
}
macro_rules! PrimesPrimitive {
    ($type:ident) => {
        impl Primes<$type> {
            fn add_prime(&mut self) -> bool {
                while let Some(i) = self.possible.next() {
                    if unsafe { self.is_prime_no_check(i) } {
                        self.vec.push(i);
                        return true;
                    }
                }
                false
            }
            pub fn is_prime(&mut self, n: $type) -> bool {
                if n == 1 || n == 0 {
                    return false;
                }
                while n >= self.vec.last().get_or_insert(&0).pow(2) {
                    self.add_prime();
                }
                unsafe { self.is_prime_no_check(n) }
            }
            unsafe fn is_prime_no_check(&mut self, n: $type) -> bool {
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
        }
        impl Iterator for Primes<$type> {
            type Item = $type;

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
    };
}
PrimesPrimitive!(u32);
PrimesPrimitive!(u64);

#[derive(Default)]
pub struct Squares(u32);
impl Iterator for Squares {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.0 += 1;
        self.0.checked_mul(self.0)
    }
}
pub struct Spiral<T> {
    side: T,
    i: T,
    last: T,
}

impl<T: Copy> Spiral<T> {
    pub fn side(&self) -> T {
        self.side
    }
}

impl<T: From<u8>> Default for Spiral<T> {
    fn default() -> Self {
        Self {
            side: 0.into(),
            i: 3.into(),
            last: 1.into(),
        }
    }
}
macro_rules! SpiralIterator {
    ($type:ident) => {
        impl Iterator for Spiral<$type> {
            type Item = $type;

            fn next(&mut self) -> Option<Self::Item> {
                if self.i == 4 {
                    self.side += 2;
                    self.i = 0;
                }
                self.last = self.last.checked_add(self.side)?;
                self.i += 1;
                Some(self.last)
            }
        }
    };
}
SpiralIterator!(u32);
SpiralIterator!(u64);
#[cfg(test)]
mod tests {
    use super::{Primes, Spiral, Squares};

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
    test!(test_spiral, Spiral, [1, 3, 5, 7, 9, 13, 17, 21]);
}
