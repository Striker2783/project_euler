use std::collections::VecDeque;

use common::number_series::Primes;

pub fn run() {
    let mut solver = Solver::new(8);
    println!("{}", solver.solve());
}
#[derive(Default)]
struct Solver {
    generator: Generator,
    max: usize,
    primes: Primes<u32>,
}
impl Solver {
    fn new(max: usize) -> Self {
        Self {
            max,
            ..Default::default()
        }
    }
    fn solve(mut self) -> u32 {
        while let Some(mut arr) = self.generator.next() {
            for a in [1, 3, 7, 9] {
                arr.push(a);
                let family = self.prime_family(&arr);
                if family.len() == self.max {
                    return family[0];
                }
                arr.pop();
            }
        }
        unreachable!()
    }
    fn prime_family(&mut self, p: &[u8]) -> Vec<u32> {
        if p.iter().find(|x| **x == 10).is_none() {
            return vec![];
        }
        let mut family = vec![];
        let start = if p[0] == 10 { 1 } else { 0 };
        for i in start..10 {
            let mut n = 0;
            for &v in p.iter() {
                n *= 10;
                if v == 10 {
                    n += i;
                } else {
                    n += v as u32;
                }
            }
            if self.primes.is_prime(n) {
                family.push(n);
            }
        }
        family
    }
}
struct Generator {
    curr: Vec<u8>,
}

impl Default for Generator {
    fn default() -> Self {
        Self {
            curr: Default::default(),
        }
    }
}
impl Iterator for Generator {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut carry = true;
        let mut i = 0;
        while carry {
            if i >= self.curr.len() {
                self.curr.push(0);
            }
            self.curr[i] += 1;
            if self.curr[i] <= 10 {
                break;
            }
            self.curr[i] = 0;
            i += 1;
            carry = true;
        }
        Some(self.curr.iter().rev().cloned().collect())
    }
}
#[cfg(test)]
mod tests {
    use super::Solver;

    #[test]
    fn test_prime_family() {
        let mut solver = Solver::default();
        assert_eq!(
            solver.prime_family(&[5, 6, 10, 10, 3]),
            vec![56003, 56113, 56333, 56443, 56663, 56773, 56993]
        );
        assert_eq!(solver.prime_family(&[10, 3]), vec![13, 23, 43, 53, 73, 83]);
    }
    #[test]
    fn test_solve() {
        let mut solver = Solver::new(7);
        assert_eq!(solver.solve(), 56003);
        solver = Solver::new(8);
        assert_eq!(solver.solve(), 121313);
    }
}
