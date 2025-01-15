pub fn run() {
    println!("{}", solve());
}

fn solve() -> u32 {
    let fracs: Vec<_> = (1..100)
        .flat_map(|a| {
            ((a + 1)..100)
                .filter_map(|b| {
                    if is_cancelable(a, b) {
                        Some((a, b))
                    } else {
                        None
                    }
                })
                .collect::<Vec<(u32, u32)>>()
        })
        .collect();
    let numerator: u32 = fracs.iter().map(|x| x.0).product();
    let den: u32 = fracs.iter().map(|x| x.1).product();
    reduce(numerator, den).1
}

fn reduce(mut num: u32, mut den: u32) -> (u32, u32) {
    let mut i = 2;
    while num > 1 && i * i <= den {
        if num % i == 0 && den % i == 0 {
            num /= i;
            den /= i;
            i = 2;
        }
        i += 1;
    }
    if den % num == 0 {
        den /= num;
        num = 1;
    }
    (num, den)
}

fn is_cancelable(a_n: u32, b_n: u32) -> bool {
    let a = (a_n / 10, a_n % 10);
    let b = (b_n / 10, b_n % 10);
    if a.0 != 0 {
        if a.0 == b.0 && (a.1 as f32 / b.1 as f32) == (a_n as f32 / b_n as f32) {
            return true;
        }
        if a.0 == b.1 && (a.1 as f32 / b.0 as f32) == (a_n as f32 / b_n as f32) {
            return true;
        }
    }
    if a.1 != 0 {
        if a.1 == b.0 && (a.0 as f32 / b.1 as f32) == (a_n as f32 / b_n as f32) {
            return true;
        }
        if a.1 == b.1 && (a.0 as f32 / b.0 as f32) == (a_n as f32 / b_n as f32) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::{is_cancelable, reduce, solve};

    #[test]
    fn test_reduce() {
        assert_eq!(reduce(49, 98), (1, 2));
        assert_eq!(reduce(30, 50), (3, 5));
    }

    #[test]
    fn test_is_cancelable() {
        assert!(is_cancelable(49, 98));
    }
}
