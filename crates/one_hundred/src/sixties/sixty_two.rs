use std::collections::HashMap;

use common::number_series::Cubes;

pub fn run() {
    let mut solver = Solver::new(5);
    println!("{}", solver.solve());
}

struct Solver {
    perms: usize,
}

impl Solver {
    fn new(perms: usize) -> Self {
        Self { perms }
    }
    pub fn solve(&self) -> u64 {
        let mut map: HashMap<u64, (u64, usize)> = HashMap::new();
        for c in Cubes::<u64>::default() {
            let e = map
                .entry(Self::sort_digits(c))
                .and_modify(|a| a.1 += 1)
                .or_insert((c, 1));
            if e.1 == self.perms {
                return e.0;
            }
        }
        unreachable!()
    }
    fn sort_digits(mut n: u64) -> u64 {
        let mut arr = [0; 10];
        while n > 0 {
            arr[(n % 10) as usize] += 1;
            n /= 10;
        }
        for (i, &v) in arr.iter().enumerate().rev() {
            for _ in 0..v {
                n *= 10;
                n += i as u64
            }
        }
        n
    }
}

#[cfg(test)]
mod tests {
    use super::Solver;

    #[test]
    fn test_solve() {
        let mut solver = Solver::new(3);
        assert_eq!(solver.solve(), 41063625);
    }
}
