use core::f32;
use std::{collections::HashMap, mem::swap};

pub fn run() {
    println!("{}", solve(10_000_000));
}
/// Brute Force Solution
fn solve(max: u32) -> u32 {
    let mut max_f = f32::MAX;
    let mut max_n = 0;
    for n in 2..max {
        let totient = totient_2(n);
        if is_permutation(totient, n) && max_f > n as f32 / totient as f32 {
            max_f = n as f32 / totient as f32;
            max_n = n;
        }
    }
    max_n
}
/// Brute Force Totient Function
fn totient(n: u32) -> u32 {
    (1..n).filter(|&m| gcd(n, m) == 1).count() as u32
}

fn totient_2(mut n: u32) -> u32 {
    let mut result = n;
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            while n % i == 0 {
                n /= i;
            }
            result -= result / i;
        }
        i += 1;
    }
    if n > 1 {
        result -= result / n;
    }
    result
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

fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b > 0 {
        swap(&mut a, &mut b);
        b = b % a;
    }
    a
}

#[cfg(test)]
mod tests {
    use crate::problems::one_hundred::seventies::seventy::{
        gcd, is_permutation, totient, totient_2,
    };

    #[test]
    fn test_is_permutation() {
        assert!(is_permutation(87109, 79180));
    }
    #[test]
    fn test_gcd() {
        assert_eq!(gcd(10, 2), 2);
        assert_eq!(gcd(87109, 79180), 1);
    }
    #[test]
    fn test_totient() {
        assert_eq!(totient(9), 6);
        assert_eq!(totient(87109), 79180);
        assert_eq!(totient_2(9), totient(9));
        assert_eq!(totient_2(87109), totient(87109));
    }
}
