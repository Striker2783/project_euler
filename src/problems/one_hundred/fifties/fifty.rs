use common::{is_prime, number_series::Primes};

pub fn run() {
    println!("{}", solve(1_000_000));
}
fn solve(max: u32) -> u32 {
    let mut max_prime = 0;
    let mut max_len = 0;
    let mut prime_iter = Primes::default();
    let primes: Vec<_> = Primes::default().take_while(|&p| p < max).collect();
    let mut sum = 2;
    let (mut left, mut right) = (0, 1);
    while left <= right && right < primes.len() && left < primes.len() {
        if sum + primes[right] < max {
            sum += primes[right];
            right += 1;
        } else {
            sum -= primes[left];
            left += 1;
        }
        if (right - left) > max_len && prime_iter.is_prime(sum) {
            max_len = right - left;
            max_prime = sum;
        }
    }
    max_prime
}
#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test_solve() {
        assert_eq!(solve(100), 41);
        assert_eq!(solve(1000), 953);
        assert_eq!(solve(1000000), 997651);
    }
}
