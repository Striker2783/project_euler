use std::{collections::HashSet, u32};
// TODO Optimize
pub fn run() {
    println!("{}", solve(1_000_000));
}

fn solve(max: u32) -> u32 {
    let mut max_l = 0;
    let mut min_l = 0;
    for n in 1..=max {
        let (a, b) = match get_amicable(n, max) {
            Some(a) => a,
            None => continue,
        };
        if a > max_l {
            max_l = a;
            min_l = b;
        }
    }
    min_l
}

fn get_amicable(n: u32, max: u32) -> Option<(usize, u32)> {
    let mut set = HashSet::new();
    let mut min = u32::MAX;
    let mut curr = n;
    while !set.contains(&curr) {
        set.insert(curr);
        min = curr.min(min);
        curr = proper(curr, max)?;
    }
    if curr == n {
        Some((set.len(), min))
    } else {
        None
    }
}

fn proper(n: u32, max: u32) -> Option<u32> {
    let mut sum = 1;
    for i in 2.. {
        if i * i > n {
            break;
        } else if i * i == n {
            sum += i;
        } else if n % i == 0 {
            sum += i;
            sum += n / i;
        }
        if sum > max {
            return None;
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use crate::nineties::ninety_five::{get_amicable, proper};

    #[test]
    fn test_proper() {
        assert_eq!(proper(28, 1000), Some(28));
        assert_eq!(proper(220, 1000), Some(284));
        assert_eq!(proper(284, 1000), Some(220));
    }
    #[test]
    fn test_amicable() {
        assert_eq!(get_amicable(220, 1000), Some((2, 220)));
        assert_eq!(get_amicable(220, 10), None);
    }
}
