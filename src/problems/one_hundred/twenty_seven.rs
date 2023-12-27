use std::{collections::HashSet, ops::RangeBounds};

use core::ops::Range;

use crate::problems::one_hundred::ten::sieve_of_eratosthenes;

pub fn run() {}
fn solve(a_range: Range<i32>, b_range: Range<i32>) -> i32 {
    let primes = sieve_of_eratosthenes(1_000_000);
    let mut primes_s = HashSet::new();
    primes.iter().for_each(|&n| {
        primes_s.insert(n);
    });
    let (mut a_m, mut b_m, mut l_m) = (0, 0, 0);

    for a in a_range {
        for b in b_range.clone() {
            let l = consecutive_primes(a, b, &primes_s);
            if l > l_m {
                a_m = a;
                b_m = b;
                l_m = l;
            }
        }
    }

    a_m * b_m
}
fn consecutive_primes(a: i32, b: i32, primes: &HashSet<u64>) -> u32 {
    (0..)
        .map(|n| n * n + a * n + b)
        .take_while(|n| primes.contains(&(*n as u64)))
        .count() as u32
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::problems::one_hundred::{
        ten::{sieve_of_eratosthenes, sieve_of_eratosthenes_set},
        twenty_seven::{consecutive_primes, solve},
    };

    #[test]
    fn test_consecutive_primes() {
        let primes = sieve_of_eratosthenes(1_000_000);
        let mut primes_s = HashSet::new();
        primes.iter().for_each(|&n| {
            primes_s.insert(n);
        });
        assert_eq!(consecutive_primes(-79, 1601, &primes_s), 80);
        assert_eq!(consecutive_primes(1, 41, &primes_s), 40);
    }
    #[test]
    fn test_solve() {
        let primes = sieve_of_eratosthenes_set(1_000_000);
        assert_eq!(solve((-999..1000), (-999..1000)), -59231);
    }
}
