pub fn run() {
    println!("{}", solve2(1_000_000_000));
}
fn solve(max: u32) -> u64 {
    let mut sum = 0;
    for n in 5.. {
        if 2 * n + (n - 1) >= max {
            break;
        }
        if is_almost(n, n - 1) {
            println!("{n} {}", n - 1);
            sum += (2 * n + (n - 1)) as u64;
        }
        if 2 * n + (n + 1) >= max {
            break;
        }
        if is_almost(n, n + 1) {
            println!("{n} {}", n + 1);
            sum += (2 * n + (n + 1)) as u64;
        }
    }
    sum
}
/// Alternating from https://oeis.org/A120893
fn solve2(max: u32) -> u64 {
    let mut b = true;
    let mut prev = [1, 1, 5];
    let mut sum = 16;
    while prev[2] <= max / 3 {
        let next = 3 * prev[2] + 3 * prev[1] - prev[0];
        prev[0] = prev[1];
        prev[1] = prev[2];
        prev[2] = next;
        if next * 3 > max {break;}
        if b {
            sum += next as u64 * 3 - 1;
        } else {
            sum += next as u64 * 3 + 1;
        }
        b = !b;
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

fn is_almost(a: u32, b: u32) -> bool {
    assert!(a.abs_diff(b) <= 1);
    let (a, b) = (a as u64, b as u64);
    if b % 2 == 1 {
        return false;
    }
    let c_squared = 4 * a * a - b * b;
    let c = match is_square(c_squared) {
        Some(a) => a,
        None => return false,
    };
    true
}

#[cfg(test)]
mod tests {
    use crate::nineties::ninety_four::{is_almost, is_square};

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
