use num::BigUint;

use crate::sixties::sixty_five::continued_fraction;

pub fn run() {
    println!("{}", solve(1_000));
}

fn solve(max: u32) -> u32 {
    (2..=max)
        .filter_map(|n| Some((get_min(n)?, n)))
        .max_by(|a, b| a.0.cmp(&b.0))
        .unwrap()
        .1
}

fn get_min(n: u32) -> Option<BigUint> {
    let frac = get_continued_fraction(n)?;
    let iter = frac.iter().map(|n| BigUint::from(*n));
    if frac.len() & 1 == 1 {
        let fraction = continued_fraction(iter.rev().skip(1))?;
        Some(fraction.numerator().clone())
    } else {
        let iter2 = iter.clone().skip(1);
        let full = iter.chain(iter2).rev().skip(1);
        let fraction = continued_fraction(full)?;
        Some(fraction.numerator().clone())
    }
}

fn min_square_without_square(n: u32) -> Option<u32> {
    for i in 1.. {
        if i * i > n {
            return Some(i - 1);
        }
        if i * i == n {
            return None;
        }
    }
    unreachable!()
}

fn get_continued_fraction(n: u32) -> Option<Vec<u32>> {
    let min = min_square_without_square(n)?;
    let mut num = min;
    let mut d = 1;
    let mut result = vec![num];

    while *result.last().unwrap() != 2 * min {
        d = (n - num.pow(2)) / d;
        let a = (num + min) / d;
        num = a * d - num;
        result.push(a);
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use num::BigUint;

    use crate::sixties::sixty_six::{
        get_continued_fraction, get_min, solve,
    };

    #[test]
    fn test_get_continued_fraction() {
        assert_eq!(get_continued_fraction(2), Some(vec![1, 2]));
        assert_eq!(get_continued_fraction(13), Some(vec![3, 1, 1, 1, 1, 6]));
    }
    #[test]
    fn test_get_min() {
        assert_eq!(get_min(2), Some(BigUint::from(3u32)));
        assert_eq!(get_min(3), Some(BigUint::from(2u32)));
        assert_eq!(get_min(4), None);
        assert_eq!(get_min(5), Some(BigUint::from(9u32)));
        assert_eq!(get_min(6), Some(BigUint::from(5u32)));
        assert_eq!(get_min(7), Some(BigUint::from(8u32)));
        assert_eq!(get_min(13), Some(BigUint::from(649u32)));
    }
    #[test]
    fn test_solve() {
        assert_eq!(solve(7), 5);
        assert_eq!(solve(100), 61);
    }
}
