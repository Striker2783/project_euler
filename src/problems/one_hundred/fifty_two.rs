// TODO OPTIMIZE
use std::{collections::HashSet, ops::Range};

pub fn run() {
    println!("{}", Solver::new(6).solve())
}

struct Permutator<T: FnMut(u64)> {
    len: u32,
    f: T,
    n: u64,
}

impl<T: FnMut(u64)> Permutator<T> {
    fn new(len: u32, f: T, n: u64) -> Self {
        Self { len, f, n }
    }
    fn helper(&mut self, n: u64, rem: u64, len: u32) {
        if rem == 0 || len >= self.len {
            let n = n * 10u64.pow(self.len - len);
            (self.f)(n);
            return;
        }
        let mut set = [false; 10];
        let mut i = 0;
        while rem / 10u64.pow(i) > 0 {
            let digit = (rem % 10u64.pow(i + 1)) / 10u64.pow(i);
            let rem2 = rem % 10u64.pow(i) + rem / 10u64.pow(i + 1) * 10u64.pow(i);
            if set[digit as usize] || (n == 0 && digit == 0) {
                i += 1;
                continue;
            }
            self.helper(n * 10 + digit, rem2, len + 1);
            set[digit as usize] = true;
            i += 1;
        }
    }
    pub fn permutate(mut self) {
        self.helper(0, self.n, 0);
    }
}

struct Solver {
    set: HashSet<u64>,
    max: u32,
}
impl Solver {
    fn new(max: u32) -> Self {
        Self {
            max,
            set: HashSet::new(),
        }
    }
    #[inline]
    fn range(&self, n: u32) -> Range<u64> {
        10u64.pow(n)..(10u64.pow(n + 1) / self.max as u64)
    }
    fn solve(mut self) -> u64 {
        for i in 0..9 {
            for n in self.range(i) {
                if self.set.contains(&n) {
                    continue;
                }
                let num = self.get_max(n, i + 1);
                if num == self.max {
                    return n;
                }
            }
        }
        unreachable!()
    }
    fn get_max(&mut self, n: u64, len: u32) -> u32 {
        let mut set = HashSet::new();
        Permutator::new(
            len,
            |a| {
                set.insert(a);
                if self.range(len).contains(&a) {
                    self.set.insert(a);
                }
            },
            n,
        )
        .permutate();
        for i in 2..=9 {
            if !set.contains(&(n * i)) {
                return (i - 1) as u32;
            }
        }
        9
    }
}

#[cfg(test)]
mod tests {
    use super::{Permutator, Solver};

    #[test]
    fn test_permutate() {
        let mut len = 0;
        Permutator::new(5, |_| len += 1, 12345).permutate();
        assert_eq!(len, 5 * 4 * 3 * 2);
        len = 0;
        Permutator::new(5, |_| len += 1, 12340).permutate();
        // Number cannot start with 0
        assert_eq!(len, 4 * 4 * 3 * 2);
    }
    #[test]
    fn test_get_max() {
        assert_eq!(Solver::new(2).get_max(125874, 6), 2);
        assert_eq!(Solver::new(2).get_max(1386, 4), 1);
    }
}
