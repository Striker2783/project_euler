use crate::common::is_prime;

pub fn run() {
    println!("{}", solve());
}

fn solve() -> u32 {
    (10..)
        .filter(|&n| is_truncated_prime(n))
        .take(11)
        .inspect(|x| {
            println!("{x}");
        })
        .sum()
}

fn is_truncated_prime(mut n: u32) -> bool {
    let mut i = 0;
    let mut m = 0;
    while n > 0 {
        if !is_prime(n) {
            return false;
        }
        m += (n % 10) * 10u32.pow(i);
        if !is_prime(m) {
            return false;
        }
        i += 1;
        n /= 10;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::is_truncated_prime;

    #[test]
    fn test_is_truncated_prime() {
        assert!(is_truncated_prime(3797));
        assert!(!is_truncated_prime(13));
    }
}
