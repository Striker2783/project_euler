use std::collections::{HashMap, HashSet};

use crate::common::number_series::Squares;

const MAX: u32 = 1000;
pub fn run() {
    let mut solver = Solver::new(MAX);
    println!("{}", solver.solve());
}

struct Solver {
    squares: HashMap<u32, u32>,
    map: HashMap<u32, u32>,
    max: u32,
}
impl Solver {
    fn new(max: u32) -> Solver {
        Self {
            squares: Squares::default()
                .enumerate()
                .take_while(|(_, x)| *x < max * max)
                .map(|(a, b)| (b, a as u32 + 1))
                .collect(),
            map: HashMap::new(),
            max,
        }
    }
    fn solve(mut self) -> u32 {
        for a in 1..self.max {
            for b in a..self.max {
                self.insert_pair(a, b);
            }
        }
        *self.map.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap().0
    }
    fn insert_pair(&mut self, a: u32, b: u32) -> bool {
        let c_s = a * a + b * b;
        let c = match self.squares.get(&c_s) {
            Some(a) => *a,
            None => return false,
        };
        if c + a + b > self.max {
            return false;
        }
        self.map
            .entry(c + a + b)
            .and_modify(|v| *v += 1)
            .or_insert(1);
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::thirty_nine::MAX;

    use super::Solver;

    #[test]
    fn test_solver() {
        let mut solver = Solver::new(120);
        assert!(solver.insert_pair(20, 48));
        solver = Solver::new(120);
        assert_eq!(solver.solve(), 120);
        solver = Solver::new(MAX);
        assert_eq!(solver.solve(), 840);
    }
}
