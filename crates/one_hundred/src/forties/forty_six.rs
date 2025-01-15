use std::collections::HashSet;

use common::number_series::{Primes, Squares};

pub fn run() {
    let mut solver = Solver::default();
    println!("{}", solver.next().unwrap()); // 5777
}
struct Solver {
    primes: Primes<u32>,
    last_prime: u32,
    p_set: HashSet<u32>,
    squares: Squares,
    last_square: u32,
    s_set: HashSet<u32>,
    last: u32,
}

impl Default for Solver {
    fn default() -> Self {
        Self {
            primes: Default::default(),
            last_prime: Default::default(),
            p_set: Default::default(),
            squares: Default::default(),
            last_square: Default::default(),
            s_set: Default::default(),
            last: 1,
        }
    }
}
impl Iterator for Solver {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.last += 2;
            while self.last >= self.last_prime {
                let p_next = self.primes.next()?;
                self.last_prime = p_next;
                self.p_set.insert(p_next);
            }
            while self.last / 2 >= self.last_square {
                let s_next = self.squares.next()?;
                self.last_square = s_next;
                self.s_set.insert(s_next);
            }
            if self.p_set.contains(&self.last) {
                continue;
            }
            let mut has = false;
            for &p in &self.p_set {
                if p >= self.last {
                    continue;
                }
                let m = (self.last - p) / 2;
                if self.s_set.contains(&m) {
                    has = true;
                    break;
                }
            }
            if !has {
                return Some(self.last);
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::Solver;

    #[test]
    fn test_solve() {
        let mut solver = Solver::default();
        assert_eq!(solver.next(), Some(5777))
    }
}
