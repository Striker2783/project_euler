use std::collections::HashSet;

use common::number_series::Primes;

pub fn run() {
    println!("{}", solve(100_000, 10_000));
}

fn rad(mut n: u64) -> u64 {
    let mut set = HashSet::new();
    let mut primes = Primes::<u64>::default();
    while let Some(p) = primes.next() {
        if n % p == 0 {
            n /= p;
            set.insert(p);
            primes.reset();
        } else if p * p > n {
            break;
        }
    }
    if n > 1 {
        set.insert(n);
    }
    set.into_iter().product()
}

fn solve(n: u64, i: usize) -> u64 {
    let mut v = Vec::new();
    for n in 1..=n {
        v.push((rad(n), n));
    }
    v.sort();
    v[i - 1].1
}

#[cfg(test)]
mod tests {
    use crate::twenties::twenty_four::{rad, solve};

    #[test]
    fn test_solve() {
        assert_eq!(solve(10, 4), 8);
        assert_eq!(solve(10, 6), 9);
    }
    #[test]
    fn test_rad() {
        assert_eq!(rad(504), 2 * 3 * 7);
    }
}
