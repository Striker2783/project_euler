use core::f32;

use common::number_series::Primes;

pub fn run() {
    println!("{}", solve(10_000_000));
}
/// Brute Force Solution
fn solve(max: u32) -> u32 {
    let mut max_f = f32::MAX;
    let mut max_n = 0;
    for p1 in Primes::<u32>::default().take_while(|&p1| p1 * p1 <= max * 10) {
        for p2 in Primes::<u32>::default().take_while(|&p2| p2 < p1) {
            let totient = (p1 - 1) * (p2 - 1);
            let n = p1 * p2;
            if n > 10_000_000 {
                continue;
            }
            if is_permutation(totient, n) && max_f > n as f32 / totient as f32 {
                max_f = n as f32 / totient as f32;
                max_n = n;
            }
        }
    }
    max_n
}

fn is_permutation(mut a: u32, mut b: u32) -> bool {
    let mut digits = [0; 10];
    while a > 0 {
        digits[(a % 10) as usize] += 1;
        a /= 10;
    }
    while b > 0 {
        digits[(b % 10) as usize] -= 1;
        b /= 10;
    }
    digits.iter().all(|n| *n == 0)
}

#[cfg(test)]
mod tests {
    use crate::seventies::seventy::is_permutation;

    #[test]
    fn test_is_permutation() {
        assert!(is_permutation(87109, 79180));
    }
}
