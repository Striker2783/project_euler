pub fn run() {
    let solver = Solver::new(1000, 10);
    println!("{}", solver.solve());
}

struct Solver {
    max: u64,
    digits: u32,
}

impl Solver {
    fn new(max: u64, digits: u32) -> Self {
        Self { max, digits }
    }
    fn solve(&self) -> u64 {
        (1..=self.max)
            .map(|n| self.number(n))
            .fold(0, |acc, n| (acc + n) % 10u64.pow(self.digits))
    }
    fn number(&self, n: u64) -> u64 {
        let mut prod = 1;
        for _ in 0..n {
            prod *= n;
            prod %= 10u64.pow(self.digits);
        }
        prod
    }
}

#[cfg(test)]
mod tests {
    use super::Solver;

    #[test]
    fn test_solve() {
        let solver = Solver::new(10, 10);
        assert_eq!(solver.solve(), 405071317)
    }
}
