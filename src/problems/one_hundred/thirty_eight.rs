use num::traits::Pow;

use crate::common::num_len;

const MAX: u32 = 10u32.pow(4);
pub fn run() {
    println!("{}", solve(MAX));
}

fn solve(max: u32) -> u32 {
    (2..max)
        .filter_map(|n| {
            let mut v = vec![n];
            for i in 0..4 {
                v.push((i + 2) * n);
                if is_pandigital(&v) {
                    return Some(concat(&v));
                }
            }
            None
        })
        .max()
        .unwrap()
}

fn concat(nums: &[u32]) -> u32 {
    let mut n = 0;
    for &num in nums {
        n *= 10u32.pow(num_len(num));
        n += num;
    }
    n
}

fn is_pandigital(nums: &[u32]) -> bool {
    let mut vec = [false; 9];
    for n in nums {
        let mut n = *n;
        while n > 0 {
            let r = (n % 10) as usize;
            if r == 0 || vec[r - 1] {
                return false;
            } else {
                vec[r - 1] = true;
            }
            n /= 10;
        }
    }
    vec.iter().all(|b| *b)
}

#[cfg(test)]
mod tests {
    use crate::problems::one_hundred::thirty_eight::{is_pandigital, solve, MAX};

    #[test]
    fn t_is_pandigital() {
        assert!(is_pandigital(&[192, 384, 576]));
        assert!(is_pandigital(&[9, 18, 27, 36, 45]));
    }
    #[test]
    fn t_solve() {
        assert_eq!(solve(10), 918273645);
        assert_eq!(solve(MAX), 932718654);
    }
}
