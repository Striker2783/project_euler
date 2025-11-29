pub fn run() {
    println!("{}", solve(99));
}

fn solve(percent: u64) -> u64 {
    let mut bouncies = 0;
    for n in 1.. {
        if is_bouncy(n) {
            bouncies += 1;
            if bouncies * 100 == n * percent {
                return n;
            }
        }
    }
    unreachable!()
}

fn is_bouncy(mut n: u64) -> bool {
    let (mut inc, mut dec) = (true, true);
    let mut prev = n % 10;
    n /= 10;
    while n > 0 {
        if prev > n % 10 {
            dec = false;
        } else if prev < n % 10 {
            inc = false;
        }
        prev = n % 10;
        n /= 10;
    }
    !inc && !dec
}

#[cfg(test)]
mod tests {
    use crate::tens::twelve::{is_bouncy, solve};

    #[test]
    fn test_solve() {
        assert_eq!(solve(50), 538);
        assert_eq!(solve(90), 21780);
    }

    #[test]
    fn test_is_bouncy() {
        assert!(is_bouncy(155349));
        assert!(!is_bouncy(134468));
        assert!(!is_bouncy(66420));
    }
}
