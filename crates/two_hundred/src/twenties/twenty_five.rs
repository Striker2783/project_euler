use std::collections::HashSet;

pub fn run() {
    println!("{}", solve(100_000_000));
}

fn solve(n: u64) -> u64 {
    let mut set = HashSet::new();
    for first in (1u64..).take_while(|x| x * x <= n) {
        let mut sum = first.pow(2);
        for next in (first + 1).. {
            sum += next.pow(2);
            if sum > n {
                break;
            } else if is_palindromic(sum) {
                set.insert(sum);
            }
        }
    }
    set.into_iter().sum()
}

fn is_palindromic(n: u64) -> bool {
    let mut rev = 0;
    let mut n2 = n;
    while n2 > 0 {
        rev *= 10;
        rev += n2 % 10;
        n2 /= 10;
    }
    rev == n
}

#[cfg(test)]
mod tests {
    use crate::twenties::twenty_five::{is_palindromic, solve};

    #[test]
    fn test_solve() {
        assert_eq!(solve(100), 137);
        assert_eq!(solve(1_000), 4164);
    }
    #[test]
    fn test_others() {
        assert!(is_palindromic(595));
        assert!(is_palindromic(5995));
        assert!(!is_palindromic(594));
    }
}
