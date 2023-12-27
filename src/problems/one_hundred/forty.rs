use crate::common::num_len;

pub fn run() {
    //
}

fn get_tens(n: u32) -> u32 {
    0
}

fn get_digit(n: u32) -> u32 {
    let mut len = 1;
    let mut curr_digit = 0;
    loop {
        let num = 9 * 10u32.pow(len - 1) * len + curr_digit;
        if num <= n {
            curr_digit = num;
            len += 1;
        } else {
            break;
        }
    }
    let num = 10u32.pow(len - 1);
    println!("{:?}", (len, curr_digit, num));
    0
}

#[cfg(test)]
mod tests {
    use crate::problems::one_hundred::forty::get_digit;

    #[test]
    fn test_get_digit() {
        assert_eq!(get_digit(10000), 1);
        assert_eq!(get_digit(1), 1);
        assert_eq!(get_digit(11), 0);
        assert_eq!(get_digit(12), 1);
    }
}
