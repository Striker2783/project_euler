use std::collections::HashSet;

use common::{
    number_series::Primes,
    sieve::{self, Sieve},
};

pub fn run() {
    println!("{}", solve(50_000_000));
}
/// Brute Force
fn solve(n: u32) -> u32 {
    let mut primes: Vec<_> = Primes::<u32>::default()
        .take_while(|x| *x * *x <= n)
        .collect();
    let mut count = 0;
    let mut set = HashSet::new();
    for &p1 in &primes {
        if p1.pow(4) >= n {
            break;
        }
        for &p2 in &primes {
            if p1.pow(4) + p2.pow(3) >= n {
                break;
            }
            for &p3 in &primes {
                let sum = p1.pow(4) + p2.pow(3) + p3.pow(2);
                if sum > n {
                    break;
                }
                set.insert(sum);
            }
        }
    }
    set.len() as u32
}

#[cfg(test)]
mod tests {
    use crate::problems::one_hundred::eighties::eighty_seven::solve;

    #[test]
    fn test_solve() {
        assert_eq!(solve(50), 4);
    }
}
