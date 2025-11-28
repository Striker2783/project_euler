use common::combinations;

pub fn run() {
    println!("{}", Solver::new(1_000_000, 100).solve());
}
struct Solver {
    max: u32,
    n: u32,
}

impl Solver {
    fn new(max: u32, n: u32) -> Self {
        Self { max, n }
    }

    fn solve(self) -> u32 {
        (1..=self.n)
            .map(|v| {
                (0..=v)
                    .filter(|&y| combinations(v, y) > self.max as u64)
                    .count() as u32
            })
            .sum()
    }
}
