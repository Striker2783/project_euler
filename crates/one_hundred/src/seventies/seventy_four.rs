use std::collections::HashSet;

pub fn run() {
    println!("{}", solve(1_000_000, 60));
}

fn solve(max: u32, len: u32) -> u32 {
    (1..=max).filter(|n| get_chain_length(*n) == len).count() as u32
}

fn get_chain_length(mut n: u32) -> u32 {
    let mut set = HashSet::new();
    while !set.contains(&n) {
        set.insert(n);
        n = sum_factorial_digits(n);
    }
    set.iter().count() as u32
}

fn sum_factorial_digits(mut n: u32) -> u32 {
    let mut sum = 0;
    while n > 0 {
        sum += (2..=(n % 10)).product::<u32>();
        n /= 10;
    }
    sum
}

#[cfg(test)]
mod tests {
    use crate::seventies::seventy_four::{
        get_chain_length, sum_factorial_digits,
    };

    #[test]
    fn test_sum_factorial_digits() {
        assert_eq!(sum_factorial_digits(145), 145);
        assert_eq!(sum_factorial_digits(169), 363601);
    }
    #[test]
    fn test_get_chain_length() {
        assert_eq!(get_chain_length(145), 1);
        assert_eq!(get_chain_length(169), 3);
    }
    
}
