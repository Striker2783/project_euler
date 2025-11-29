use std::collections::HashSet;

const MAX: u32 = 28123;
pub fn run() {
    let x = get_sum(MAX);
    println!("{x}");
}
fn get_sum(n: u32) -> u32 {
    let abundants = get_abundants(n);
    (1..=n)
        .filter(|&n| !is_sum_of_abundant(n, &abundants))
        .sum()
}
fn is_abundant(n: u32) -> bool {
    let mut sum = 1;
    for i in (2..).take_while(|&x| x * x <= n) {
        if i * i == n {
            sum += i;
        } else if n % i == 0 {
            sum += i;
            sum += n / i;
        }
    }
    sum > n
}
fn get_abundants(n: u32) -> HashSet<u32> {
    (12..n).filter(|&n| is_abundant(n)).collect()
}
fn is_sum_of_abundant(n: u32, abundants: &HashSet<u32>) -> bool {
    for &values in abundants.iter() {
        if values >= n {
            continue;
        }
        if abundants.contains(&(n - values)) {
            return true;
        }
    }
    false
}
#[cfg(test)]
mod tests {
    use super::{MAX, get_abundants, is_abundant, is_sum_of_abundant};

    #[test]
    fn test_is_abundant() {
        assert!(is_abundant(12));
        assert!(!is_abundant(28));
    }
    #[test]
    fn test_is_sum_of_abundant() {
        let abundants = get_abundants(MAX);
        assert!(is_sum_of_abundant(24, &abundants));
        assert!(!is_sum_of_abundant(23, &abundants));
        assert!(!is_sum_of_abundant(29, &abundants));
    }
}
