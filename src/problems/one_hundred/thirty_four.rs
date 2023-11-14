const MAX: u64 = 1_000_000;
pub fn run() {
    let solution = solve(MAX);
    println!("{solution}");
}
fn solve(max: u64) -> u64 {
    (10..max).filter(|x| is_curious(*x)).sum()
}

fn is_curious(n: u64) -> bool {
    let mut x = n;
    let mut sum = 0;
    while (x > 0) {
        sum += factorial(x % 10);
        x /= 10;
    }
    sum == n
}

fn factorial(n: u64) -> u64 {
    (2..=n).product()
}

#[cfg(test)]
mod tests {
    use crate::problems::one_hundred::thirty_four::{is_curious, MAX};

    use super::solve;

    #[test]
    fn test_is_curious() {
        assert!(is_curious(145));
    }
    #[test]
    fn test_solve() {
        assert_eq!(solve(1000), 145);
        assert_eq!(solve(MAX), 40730);
    }
}
