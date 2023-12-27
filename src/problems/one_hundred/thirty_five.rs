pub fn run() {
    println!("{}", solve(1_000_000));
}

fn solve(max: u32) -> u32 {
    (2..max).filter(|&n| is_circular_prime(n)).count() as u32
}

fn is_prime(n: u32) -> bool {
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}

fn is_circular_prime(mut n: u32) -> bool {
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
        if !is_prime(n) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use crate::problems::one_hundred::thirty_five::{is_circular_prime, solve};

    #[test]
    fn test_is_circular_prime() {
        assert!(is_circular_prime(197));
        assert!(is_circular_prime(97));
    }
    #[test]
    fn test_solve() {
        assert_eq!(solve(100), 13);
        assert_eq!(solve(1_000_000), 55);
    }
}
