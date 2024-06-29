use std::vec;

pub struct Sieve {
    stored: Vec<bool>,
    i: usize,
}
impl Sieve {
    pub fn run(size: usize) -> impl Iterator<Item = u64> {
        let mut sieve = vec![true; size];
        for i in 2..size {
            if !sieve[i] {
                continue;
            }
            for j in (i + 1)..size {
                if j % i == 0 {
                    sieve[j] = false;
                }
            }
        }
        Self {
            stored: sieve,
            i: 1,
        }
    }
}
impl Iterator for Sieve {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        for i in (self.i + 1)..self.stored.len() {
            if self.stored[i] {
                self.i = i;
                return Some(i as u64);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::Sieve;
    use test::{black_box, Bencher};

    #[bench]
    fn bench(b: &mut Bencher) {
        b.iter(|| {
            black_box(Sieve::run(1000));
        });
    }
    #[test]
    fn sieve() {
        let mut primes = Sieve::run(100);
        for v in [2, 3, 5, 7, 11, 13, 17, 19, 23] {
            assert_eq!(primes.next(), Some(v));
        }
    }
}
