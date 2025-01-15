use std::collections::{HashMap, HashSet};

use common::number_series::Primes;

pub fn run() {
    println!("{}", Solver::new(5, 10_000).solve())
}

#[derive(Default)]
struct Solver {
    primes: Primes<u64>,
    pairs: HashSet<Vec<u64>>,
    len: usize,
    max: u64,
}
impl Solver {
    fn new(len: usize, max: u64) -> Self {
        Self {
            len,
            max,
            ..Default::default()
        }
    }
    fn solve(mut self) -> u64 {
        let mut primes: Primes<u64> = Primes::default();
        for p in primes {
            if p > self.max {
                break;
            }
            self.pair_iter(p);
            self.prev_iter(p);
        }
        self.pairs
            .iter()
            .filter(|v| v.len() == self.len)
            .map(|v| v.iter().sum())
            .min()
            .unwrap()
    }
    fn prev_iter(&mut self, n: u64) {
        let mut primes: Primes<u64> = Primes::default();
        for x in primes {
            if x >= n {
                break;
            }
            if self.concat(x, n) {
                self.pairs.insert(vec![x, n]);
            }
        }
    }
    fn pair_iter(&mut self, n: u64) {
        'outer: for mut v in self.pairs.clone() {
            for &x in &v {
                if !self.concat(n, x) {
                    continue 'outer;
                }
            }
            v.push(n);
            self.pairs.insert(v);
        }
    }
    fn concat(&mut self, a: u64, b: u64) -> bool {
        self.primes.is_prime(concat(a, b)) && self.primes.is_prime(concat(b, a))
    }
}

fn concat(mut a: u64, mut b: u64) -> u64 {
    let mut len = 0;
    let mut b_c = b;
    while b_c > 0 {
        b_c /= 10;
        len += 1;
    }
    if len == 0 {
        a * 10
    } else {
        a * 10u64.pow(len) + b
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::sixties::sixty::{concat, Solver};

    #[test]
    fn test_solve() {
        let mut solver = Solver::new(4, 674);
        assert_eq!(solver.solve(), 792);
    }

    #[test]
    fn test_prev_iter() {
        let mut solver = Solver::default();
        solver.prev_iter(7);
        let expected = vec![vec![3, 7]].into_iter().collect();
        assert_eq!(solver.pairs, expected);
    }
    #[test]
    fn test_pair_iter() {
        let mut solver = Solver::default();
        solver.prev_iter(7);
        solver.pair_iter(109);
        let expected = vec![vec![3, 7], vec![3, 7, 109]].into_iter().collect();
        assert_eq!(solver.pairs, expected);
    }

    #[test]
    fn test_concat() {
        assert_eq!(concat(3, 0), 30);
        assert_eq!(concat(3, 7), 37);

        let mut solver = Solver::default();
        assert!(solver.concat(3, 7));
        assert!(solver.concat(7, 109));
        assert!(!solver.concat(3, 2));
    }
}
