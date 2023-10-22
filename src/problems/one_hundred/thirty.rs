pub fn run() {
    let solution = solve(5);
    println!("{solution}");
}

fn solve(n: u32) -> u32 {
    let mut sum = 0;
    for i in 10..(10u32.pow(n + 1)) {
        if get_digit_sum_power(i, n) == i {
            sum += i;
        }
    }
    sum
}
fn get_digit_sum_power(mut n: u32, p: u32) -> u32 {
    let mut sum = 0;
    while n > 0 {
        sum += (n % 10).pow(p);
        n /= 10;
    }
    sum
}

#[cfg(test)]
mod tests {
    use crate::problems::one_hundred::thirty::{get_digit_sum_power, solve};
    #[test]
    fn test_solve() {
        assert_eq!(solve(4), 19316);
        assert_eq!(solve(5), 443839);
    }
    #[test]
    fn test_get_digit_sum_power() {
        assert_eq!(get_digit_sum_power(1634, 4), 1634);
        assert_eq!(get_digit_sum_power(8208, 4), 8208);
        assert_eq!(get_digit_sum_power(9474, 4), 9474);
    }
}
