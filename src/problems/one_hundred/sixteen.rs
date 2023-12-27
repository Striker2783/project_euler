use num::BigInt;
pub fn run() {}

fn power_digit_sum(power: u64) -> u64 {
    let mut int = BigInt::from(1);
    int <<= power;
    let str = int.to_str_radix(10);
    let mut sum = 0;
    for c in str.chars() {
        sum += c.to_digit(10).unwrap() as u64;
    }
    sum
}

#[cfg(test)]
mod test {
    use crate::problems::one_hundred::sixteen::power_digit_sum;

    #[test]
    fn test_power_digit_sum() {
        assert_eq!(power_digit_sum(15), 26);
        assert_eq!(power_digit_sum(1000), 1366);
    }
}
