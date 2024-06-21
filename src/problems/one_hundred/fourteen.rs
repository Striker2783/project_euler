use std::collections::HashMap;

struct Collatz {
    n: u64,
}
impl Collatz {
    #[inline]
    pub fn new(n: u64) -> Self {
        Self { n }
    }
    #[inline]
    pub fn is_loop(&self) -> bool {
        self.n == 1
    }
    #[inline]
    pub fn get(&self) -> u64 {
        self.n
    }
}
impl Iterator for Collatz {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.n % 2 == 0 {
            self.n /= 2;
        } else {
            self.n = 3 * self.n + 1;
        }
        Some(self.n)
    }
}

struct Solver {
    fast: Vec<u32>,
}

impl<'a> Solver {
    fn new(size: usize) -> Self {
        Self {
            fast: vec![0; size],
        }
    }
    fn solve(&mut self, max: u64) -> u64 {
        let (mut num, mut max_chain) = (0, 0u32);
        for n in 2..max {
            let length = self.get_chain_length(n);
            if length > max_chain {
                max_chain = length;
                num = n;
            }
        }
        num
    }
    fn get_chain_length(&mut self, n: u64) -> u32 {
        let mut stack = vec![];
        let mut collatz = Collatz::new(n);
        let mut chain = 1;
        while (!collatz.is_loop()) {
            let curr = collatz.next().unwrap();
            if (curr as usize) < self.fast.len() {
                if self.fast[curr as usize] != 0 {
                    chain = self.fast[curr as usize];
                    break;
                }
            }
            stack.push(curr);
        }
        for v in stack {
            chain += 1;
            if (n as usize) < self.fast.len() {
                self.fast[n as usize] = chain;
            }
        }
        chain
    }
}

pub fn run() {
    let x = Solver::new(1_000_000).solve(1_000_000);
    println!("{x}");
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::fourteen::Solver;

    use super::Collatz;

    #[test]
    fn test_collatz() {
        let mut collatz = Collatz::new(13);
        assert_eq!(collatz.next(), Some(40));
        assert_eq!(collatz.next(), Some(20));
        assert_eq!(collatz.next(), Some(10));
        assert_eq!(collatz.next(), Some(5));
        assert_eq!(collatz.next(), Some(16));
        assert_eq!(collatz.next(), Some(8));
        assert_eq!(collatz.next(), Some(4));
        assert_eq!(collatz.next(), Some(2));
        assert_eq!(collatz.next(), Some(1));
    }
    #[test]
    fn test_get_chain_length() {
        let mut solver = Solver::new(10);
        assert_eq!(solver.get_chain_length(13), 10);
        assert_eq!(solver.get_chain_length(40), 9);
    }
    #[test]
    fn test_solve() {
        assert_eq!(Solver::new(1_000_000).solve(1_000_000), 837799);
    }
}
