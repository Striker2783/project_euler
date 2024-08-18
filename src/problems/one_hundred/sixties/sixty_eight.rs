use num::pow::Pow;

pub fn run() {
    let mut solver = MagicGonRing::new(5, 16);
    println!("{}", solver.solve());
}

struct MagicGonRing {
    max: u64,
    max_digs: usize,
    ring: Vec<Vec<u32>>,
    available: Vec<bool>,
}
impl MagicGonRing {
    pub fn new(n: usize, max_digs: usize) -> Self {
        Self {
            max: 0,
            max_digs,
            ring: vec![vec![0; 3]; n],
            available: vec![true; n * 2],
        }
    }
    fn reset_ring(&mut self) {
        let saved1 = self.ring[1][1];
        let saved2 = self.ring.last().unwrap()[2];
        self.ring
            .iter_mut()
            .skip(1)
            .for_each(|v| v.iter_mut().for_each(|n| *n = 0));
        self.ring[1][1] = saved1;
        self.ring.last_mut().unwrap()[2] = saved2;
    }
    pub fn solve(mut self) -> u64 {
        self.solve_helper(0);
        self.max
    }
    fn max_num(&self) -> usize {
        self.ring.len() * 2
    }
    fn solve_helper(&mut self, i: usize) {
        if i == 3 {
            self.recurse(1, 0, self.ring[0].iter().sum());
            return;
        }
        let optimized_max = if i == 0 {
            self.max_num() - 2
        } else {
            self.max_num()
        };
        for j in 0..optimized_max {
            if !self.available[j] {
                continue;
            }
            self.available[j] = false;
            self.ring[0][i] = (j + 1) as u32;
            if i == 1 {
                self.ring.last_mut().unwrap()[2] = (j + 1) as u32;
            } else if i == 2 {
                self.ring[1][1] = (j + 1) as u32;
            }
            self.solve_helper(i + 1);
            self.reset_ring();
            self.available[j] = true;
        }
    }
    fn calculate(&mut self) {
        for i in 1..self.ring.len() {
            if self.ring[i][0] < self.ring[0][0] {
                return;
            }
        }
        let mut n = 0;
        let mut digits = 0;
        for &x in self.ring.iter().flatten() {
            for i in 0..9 {
                if x >= 10u32.pow(i) {
                    digits += 1;
                    n *= 10;
                }
            }
            n += x as u64;
        }
        if digits > self.max_digs {
            return;
        }
        self.max = n.max(self.max);
    }
    fn recurse(&mut self, i: usize, j: usize, target: u32) {
        if i >= self.ring.len() {
            self.calculate();
            return;
        } else if j >= 3 {
            let curr_sum: u32 = self.ring[i].iter().sum();
            if curr_sum != target {
                return;
            }
            self.recurse(i + 1, 0, target);
            return;
        } else if self.ring[i][j] != 0 {
            self.recurse(i, j + 1, target);
            return;
        }
        let curr_sum: u32 = self.ring[i].iter().sum();
        let optimized_min = if i == 0 { self.ring[0][0] + 1 } else { 0 };
        for k in (optimized_min as usize)..self.max_num() {
            if !self.available[k] || curr_sum + (k + 1) as u32 > target {
                continue;
            }
            self.available[k] = false;
            self.ring[i][j] = (k + 1) as u32;
            if j == 1 {
                self.ring[i - 1][2] = (k + 1) as u32;
            } else if j == 2 {
                self.ring[i + 1][1] = (k + 1) as u32;
            }

            self.recurse(i, j + 1, target);

            self.available[k] = true;
            self.ring[i][j] = 0;
            if j == 1 {
                self.ring[i - 1][2] = 0;
            } else if j == 2 {
                self.ring[i + 1][1] = 0;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::problems::one_hundred::sixties::sixty_eight::MagicGonRing;

    #[test]
    fn test_solve() {
        assert_eq!(MagicGonRing::new(3, 99).solve(), 432621513);
        assert_eq!(MagicGonRing::new(5, 16).solve(), 6531031914842725);
    }
}
