pub fn run() {
    let x = get_sum_of_amicable(10000);
    println!("{x}");
}

fn get_sum_divsiors(n: u32) -> u32 {
    let mut sum = 0;
    for i in (1..).take_while(|&x| x * x <= n) {
        if n % i == 0 {
            sum += i;
            sum += n / i;
        }
    }
    sum -= n;
    sum
}

fn get_sum_of_amicable(n: u32) -> u32 {
    let mut sum = 0;
    for i in 1..n {
        let divs = get_sum_divsiors(i);
        if divs == i {
            continue;
        }
        let divs2 = get_sum_divsiors(divs);
        if divs2 == i {
            sum += i;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::{get_sum_divsiors, get_sum_of_amicable};

    #[test]
    fn test_get_sum_divsiors() {
        assert_eq!(get_sum_divsiors(220), 284);
        assert_eq!(get_sum_divsiors(284), 220);
    }
    #[test]
    fn test_get_sum_of_amicable() {
        assert_eq!(get_sum_of_amicable(10000), 31626);
    }
}
