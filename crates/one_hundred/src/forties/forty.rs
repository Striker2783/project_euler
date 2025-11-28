use num::pow::Pow;

pub fn run() {
    println!(
        "{}",
        (0..=6)
            .map(|i| get_digit(10u32.pow(i)))
            .product::<u32>()
    );
}
fn get_digit(n: u32) -> u32 {
    let mut curr = 0;
    let mut digits = 1;
    let mut last;
    let mut diff = 9;
    loop {
        if diff * digits + curr < n {
            curr += diff * digits;
        } else {
            last = 10u32.pow(digits - 1) + (n - curr - 1) / digits;
            curr += (n - curr - 1) / digits * digits;
            for i in 0..(digits - (n - curr)) {
                last /= 10;
            }
            return last % 10;
        }
        digits += 1;
        diff = 10u32.pow(digits) - 10u32.pow(digits - 1);
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::get_digit;

    #[test]
    fn test_get_digit() {
        assert_eq!(get_digit(9), 9);
        assert_eq!(get_digit(1), 1);
        assert_eq!(get_digit(10), 1);
        assert_eq!(get_digit(11), 0);
        assert_eq!(get_digit(12), 1);
        assert_eq!(get_digit(23), 6);
        assert_eq!(get_digit(100), 5);
        assert_eq!(
            (0..=6)
                .map(|i| get_digit(10u32.pow(i)))
                .fold(1, |acc, i| acc * i),
            210
        )
    }
}
