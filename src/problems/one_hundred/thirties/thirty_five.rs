use common::number_series::Primes;

pub fn run() {
    println!("{}", solve(1_000_000));
}

fn solve(max: u32) -> u32 {
    let mut primes = Primes::default();
    (2..max)
        .filter(|&n| is_circular_prime(n, &mut primes))
        .count() as u32
}

fn is_circular_prime(mut n: u32, primes: &mut Primes) -> bool {
    let mut digits = vec![];
    while n > 0 {
        digits.push(n % 10);
        n /= 10;
    }
    for i in (0..digits.len()).rev() {
        let mut n = 0;
        for j in (0..digits.len()).rev() {
            let j = (i + j) % digits.len();
            n = n * 10 + digits[j];
        }
        if !primes.is_prime(n) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::{is_circular_prime, solve};
    use common::number_series::Primes;

    #[test]
    fn test_is_circular_prime() {
        let mut primes = Primes::default();
        assert!(is_circular_prime(197, &mut primes));
        assert!(is_circular_prime(97, &mut primes));
    }
    #[test]
    fn test_solve() {
        assert_eq!(solve(100), 13);
        assert_eq!(solve(1_000_000), 55);
    }
}
