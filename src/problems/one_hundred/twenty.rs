use num::BigInt;

pub fn run() {
    println!("{}", factorial_digit_sum(100))
}

fn factorial_digit_sum(n: u32) -> u32 {
    let num = (1..=n).fold(BigInt::from(1), |acc, n| acc * n);
    let mut sum = 0;
    for d in num.to_str_radix(10).chars() {
        sum += d.to_digit(10).unwrap();
    }
    sum
}

#[cfg(test)]
mod test {
    use crate::problems::one_hundred::twenty::factorial_digit_sum;

    #[test]
    fn test_factorial_digit_sum() {
        assert_eq!(factorial_digit_sum(10), 27);
        assert_eq!(factorial_digit_sum(100), 648);
    }
}
