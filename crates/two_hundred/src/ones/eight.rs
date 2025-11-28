use std::collections::HashMap;

use common::number_series::Primes;

pub fn run() {
    let ans = solve(1000);
    println!("{ans}");
}

pub fn solve(min: u64) -> u64 {
    for n in 2u64.. {
        if distinct(n) > min {
            return n;
        }
    }
    unreachable!()
}

pub fn distinct(mut n: u64) -> u64 {
    let mut primes = Primes::<u64>::default();
    let mut map = HashMap::new();
    while let Some(prime) = primes.next() {
        if prime * prime > n {
            break;
        }
        if n % prime == 0 {
            n /= prime;
            map.entry(prime).and_modify(|n| *n += 2).or_insert(2);
            primes.reset();
        }
    }
    if n != 1 {
        map.entry(n).and_modify(|n| *n += 2).or_insert(2);
    }
    map.into_values()
        .fold(1u64, |acc, n| acc * (n + 1))
        .div_ceil(2)
}

#[cfg(test)]
mod tests {
    use crate::ones::eight::distinct;

    #[test]
    fn test_distinct() {
        assert_eq!(distinct(4), 3);
        assert_eq!(distinct(1260), 113);
    }
}
