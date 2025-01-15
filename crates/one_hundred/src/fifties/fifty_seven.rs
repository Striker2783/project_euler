use num::{BigUint, One, Zero};

pub fn run() {
    let mut result = Solver::new(1_000).solve();
    println!("{result}");
}

#[derive(Debug)]
struct Solver {
    a: Vec<BigUint>,
    b: Vec<BigUint>,
    max: u32,
}
impl Solver {
    fn new(max: u32) -> Self {
        Self {
            max,
            a: vec![BigUint::one()],
            b: vec![BigUint::one()],
        }
    }

    unsafe fn vec_cache<T: Clone>(v: &mut [T], n: usize) -> Option<&T> {
        if n < v.len() {
            return Some(&v[n]);
        }
        None
    }

    fn b(&mut self, n: u32) -> BigUint {
        if let Some(b) = unsafe { Self::vec_cache(&mut self.b, n as usize) } {
            return b.clone();
        }
        let b = self.a(n - 1) + self.b(n - 1);
        self.b.push(b.clone());
        b
    }

    fn a(&mut self, n: u32) -> BigUint {
        if let Some(a) = unsafe { Self::vec_cache(&mut self.a, n as usize) } {
            return a.clone();
        }
        let a = self.a(n - 1) + 2u32 * self.b(n - 1);
        self.a.push(a.clone());
        a
    }

    fn is_bigger(&mut self, n: u32) -> bool {
        let numerator = self.a(n);
        let denominator = self.b(n);
        Self::big_log10(numerator) > Self::big_log10(denominator)
    }

    fn big_log10(mut n: BigUint) -> u32 {
        let mut len = 0;
        while !n.is_zero() {
            n /= 10u32;
            len += 1;
        }
        len
    }

    fn solve(mut self) -> u32 {
        (1..=self.max).filter(|&a| self.is_bigger(a)).count() as u32
    }
}

#[cfg(test)]
mod tests {
    use test::{black_box, Bencher};

    use crate::fifties::fifty_seven::Solver;

    #[test]
    fn test_ab() {
        let mut solver = Solver::new(10);
        assert_eq!(solver.a(4), 41u32.into());
        assert_eq!(solver.b(4), 29u32.into());
        assert_eq!(solver.a(8), 1393u32.into());
        assert_eq!(solver.b(8), 985u32.into());
        assert!(!solver.is_bigger(7));
        assert!(solver.is_bigger(8));
    }
    #[bench]
    fn bench_is_bigger(b: &mut Bencher) {
        b.iter(|| {
            let mut solver = Solver::new(10);
            black_box(solver.is_bigger(1000));
        })
    }
}
