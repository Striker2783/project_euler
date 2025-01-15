use std::ops::Range;

pub fn run() {
    let solution = solve(1..1_000_000);
    println!("{solution}");
}

fn solve(range: Range<u64>) -> u64 {
    (range)
        .filter(|&n| is_palindromic(n, 10))
        .filter(|&n| is_palindromic(n, 2))
        .sum()
}
fn is_palindromic(n: u64, base: u64) -> bool {
    let mut x = n;
    let mut reverse = 0;
    while (x > 0) {
        reverse = (reverse * base) + (x % base);
        x /= base;
    }
    reverse == n
}
#[cfg(test)]
mod tests {
    use super::{is_palindromic, solve};

    #[test]
    fn test_is_palindromic() {
        assert!(!is_palindromic(100, 10));
        assert!(is_palindromic(101, 10));
        assert!(is_palindromic(585, 10));
        assert!(is_palindromic(585, 2));
    }
    #[test]
    fn test_solve() {
        assert_eq!(solve(1..2), 1);
        assert_eq!(solve(1..3), 1);
        assert_eq!(solve(1..4), 4);
        assert_eq!(solve(1..1_000_000), 872187);
    }
}
