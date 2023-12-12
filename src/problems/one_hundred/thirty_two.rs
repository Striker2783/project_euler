use std::collections::HashSet;

const MAX_A: u64 = 10u64.pow(4);
const MIN_B: u64 = 1;
const MAX_B: u64 = 100;

pub fn run() {
    let solution = solve();
    println!("{solution}");
}

fn solve() -> u64 {
    let mut map = HashSet::new();
    (0..MAX_A)
        // .inspect(|x| {
        //     println!("{x}");
        // })
        .map(|a| {
            (MIN_B..MAX_B)
                .filter_map(|b| {
                    let prod = a * b;
                    if map.contains(&prod) {
                        return None;
                    }
                    if is_pandigital(a, b, prod) {
                        map.insert(prod);
                        Some(prod)
                    } else {
                        None
                    }
                })
                .sum::<u64>()
        })
        .sum()
}
fn is_pandigital(n: u64, m: u64, p: u64) -> bool {
    let mut vec = [false; 9];
    let mut closure = |mut x: u64| {
        while x > 0 {
            let remainder = x % 10;
            if remainder == 0 || vec[(remainder - 1) as usize] {
                return false;
            }
            vec[(remainder - 1) as usize] = true;
            x /= 10;
        }
        true
    };
    closure(n) && closure(m) && closure(p) && vec.iter().copied().all(|x| x)
}

#[cfg(test)]
mod tests {
    use crate::problems::one_hundred::thirty_two::{is_pandigital, solve};

    #[test]
    fn test_is_pandigital() {
        assert!(is_pandigital(39, 186, 7254));
        solve();
    }
}
