use crate::common::is_prime;

const DIGITS: [u32; 7] = [1, 2, 3, 4, 5, 6, 7];
struct Solver {
    available: [bool; 7],
    curr: [u32; 7],
}
impl Solver {
    fn helper(&mut self, i: usize) -> u32 {
        if i == 7 {
            let (n, is_prime) = self.is_prime();
            if is_prime {
                return n;
            }
        }
        let mut max = 0;
        for j in 0..7 {
            if !self.available[j] {
                continue;
            }
            self.available[j] = false;
            self.curr[i] = (j + 1) as u32;
            max = self.helper(i + 1).max(max);
            self.available[j] = true;
        }
        max
    }
    fn is_prime(&self) -> (u32, bool) {
        let mut n = 0;
        for x in self.curr {
            n *= 10;
            n += x;
        }
        if is_prime(n) {
            return (n, true);
        }
        (n, false)
    }
    fn solve(&mut self) -> u32 {
        self.helper(0)
    }
}
impl Default for Solver {
    fn default() -> Self {
        Self {
            available: [true; 7],
            curr: [0; 7],
        }
    }
}

pub fn run() {
    let mut solver = Solver::default();
    println!("{}", solver.solve());
}
