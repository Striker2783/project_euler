use std::ops::Range;

use common::get_digits;

pub fn run() {
    println!("{}", Solver::new(6).solve())
}
struct Solver {
    max: u32,
}
impl Solver {
    fn new(max: u32) -> Self {
        Self { max }
    }
    #[inline]
    fn range(&self, n: u32) -> Range<u64> {
        10u64.pow(n)..(10u64.pow(n + 1) / self.max as u64)
    }
    fn solve(mut self) -> u64 {
        for i in 0..9 {
            for n in self.range(i) {
                let num = self.get_max(n, i + 1);
                if num == self.max {
                    return n;
                }
            }
        }
        unreachable!()
    }
    fn get_max(&mut self, n: u64, len: u32) -> u32 {
        for i in 2..=9 {
            if get_digits(n) != get_digits(n * i) {
                return (i - 1) as u32;
            }
        }
        9
    }
}

#[cfg(test)]
mod tests {
    use super::Solver;

    #[test]
    fn test_get_max() {
        assert_eq!(Solver::new(2).get_max(125874, 6), 2);
        assert_eq!(Solver::new(2).get_max(1386, 4), 1);
    }
    #[test]
    fn test_solve() {
        assert_eq!(Solver::new(6).solve(), 142857);
    }
}
