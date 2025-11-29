pub fn run() {
    println!("{}", solve(1_000_000));
}

fn solve(max: u64) -> u64 {
    (2..=max).map(|n| totient(n as u32) as u64).sum()
}

fn totient(mut n: u32) -> u32 {
    let mut result = n;
    let mut p = 2;
    while p * p <= n {
        if n % p == 0 {
            while n % p == 0 {
                n /= p;
            }
            result -= result / p;
        }
        p += 1;
    }
    if n > 1 {
        result -= result / n;
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::seventies::seventy_two::{solve, totient};

    #[test]
    fn test_totient() {
        assert_eq!(totient(9), 6);
        assert_eq!(totient(87109), 79180);
    }
    #[test]
    fn test_solve() {
        assert_eq!(solve(8), 21);
    }
}
