use common::number_series::Primes;

use super::sixty_five::gcf;

pub fn run() {
    println!("{}", solve(1_000_000));
}
// Solution: Number with most primes as factors due to every number being a prime or a product of primes
fn solve(max: u32) -> u32 {
    let primes = Primes::<u32>::default();
    let mut product = 1;
    for p in primes {
        if product * p > max {
            return product;
        }
        product *= p;
    }
    unreachable!()
}

fn brute_solve(max: u32) -> u32 {
    (2..=max)
        .map(|x| (x, (x as f64) / (phi_brute(x) as f64)))
        .max_by(|a, b| a.1.total_cmp(&b.1).then_with(|| b.0.cmp(&a.0)))
        .unwrap()
        .0
}

fn phi_brute(n: u32) -> u32 {
    (1..n).filter(|&x| gcf(n, x) == 1).count() as u32
}

#[cfg(test)]
mod tests {
    use crate::sixties::sixty_nine::{brute_solve, phi_brute, solve};

    #[test]
    fn test_solve() {
        assert_eq!(solve(10), 6);
        assert_eq!(solve(6), 6);
        assert_eq!(solve(1_000_000), 510510);
    }

    #[test]
    fn test_phi_brute() {
        assert_eq!(phi_brute(2), 1);
        assert_eq!(phi_brute(3), 2);
        assert_eq!(phi_brute(9), 6);
    }
    #[test]
    fn test_brute_solve() {
        assert_eq!(brute_solve(10), 6);
        assert_eq!(brute_solve(6), 6);
    }
}
