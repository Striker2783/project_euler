use std::collections::HashSet;

use common::number_series::Primes;

pub fn run() {
    let mut solver = Solver::new(4);
    println!("{}", solver.solve());
}

#[derive(Default)]
struct Solver {
    distinct_primes: usize,
    primes: Primes<u32>,
    p_last: u32,
    p_set: HashSet<u32>,
}
impl Solver {
    fn new(distinct_primes: usize) -> Self {
        Self {
            distinct_primes,
            ..Default::default()
        }
    }

    fn solve(&mut self) -> u32 {
        let mut last = vec![];
        for i in 6.. {
            if self.get_distinct_primes(i) == self.distinct_primes {
                last.push(i);
                if last.len() >= self.distinct_primes {
                    break;
                }
            } else {
                last.clear();
            }
        }
        *last.first().unwrap()
    }

    fn get_distinct_primes(&mut self, mut n: u32) -> usize {
        while n >= self.p_last * self.p_last {
            self.p_last = self.primes.next().unwrap();
            self.p_set.insert(self.p_last);
        }
        let mut set = HashSet::new();
        let mut primes: Primes<u32> = Primes::default();
        while let Some(prime) = primes.next() {
            if prime * prime >= n {
                break;
            }
            if n % prime == 0 {
                set.insert(prime);
                n /= prime;
                primes.reset();
            }
        }
        if n > 1 {
            set.insert(n);
        }
        set.len()
    }
}

#[cfg(test)]
mod tests {
    use super::Solver;

    #[test]
    fn test_get_distinct_primes() {
        let mut solver = Solver::default();
        assert_eq!(solver.get_distinct_primes(14), 2);
        assert_eq!(solver.get_distinct_primes(644), 3);
    }
    #[test]
    fn test_solve() {
        let mut solver = Solver::new(2);
        assert_eq!(solver.solve(), 14);
        solver = Solver::new(3);
        assert_eq!(solver.solve(), 644);
    }
}
