pub fn run() {
    println!("{}", solve(1_000_000_000));
}

fn solve(max: u32) -> u64 {
    let mut sum = 0;
    for n in 1.. {
        if 2 * n + (n - 1) >= max {
            break;
        }
        if is_almost(n, n - 1) {
            sum += (2 * n + (n - 1)) as u64;
        }
        if 2 * n + (n + 1) >= max {
            break;
        }
        if is_almost(n, n + 1) {
            sum += (2 * n + (n + 1)) as u64;
        } 
        if n % 1_000_000 == 0 {
            println!("{n}");
        }
    }
    sum
}

#[inline(always)]
fn is_square(n: u64) -> Option<u64> {
    let mut left = 0;
    let mut right = n;
    while left <= right {
        let mid = left + (right - left) / 2;
        let square = match mid.checked_mul(mid) {
            Some(n) => n,
            None => {
                right = mid - 1;
                continue;
            }
        };
        if square == n {
            return Some(mid);
        }
        if (square < n) {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    return None;
}
#[inline(always)]
fn sqrt2(n: u64) {
    
}

fn is_almost(a: u32, b: u32) -> bool {
    assert!(a.abs_diff(b) <= 1);
    let (a, b) = (a as u64, b as u64);
    let c_squared = a * a - (b/2) * (b/2);
    let c = match is_square(c_squared) {
        Some(a) => a,
        None => return false,
    };
    c % 2 == 0 || b % 2 == 0
}

#[cfg(test)]
mod tests {
    use crate::problems::one_hundred::nineties::ninety_four::{is_almost, is_square};

    #[test]
    fn test_is_square() {
        for i in 2..=1000 {
            assert_eq!(is_square(i * i), Some(i));
            assert_eq!(is_square(i * i - 1), None);
        }
        assert_eq!(is_square(22_568 * 22_568), Some(22_568));
    }
    #[test]
    fn test_is_almost() {
        assert!(is_almost(5, 6));
    }
}
