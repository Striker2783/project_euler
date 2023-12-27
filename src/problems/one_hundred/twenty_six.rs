use std::collections::HashMap;
pub fn run() {
    let solution = solve(1000);
    println!("{solution}");
}
fn solve(n: u32) -> u32 {
    let (mut num, mut dig) = (0, 0);
    for x in 1..n {
        let length = repeating_length(x);
        if length > dig {
            dig = length;
            num = x;
        }
    }
    num
}
fn repeating_length(n: u32) -> u32 {
    let mut map = HashMap::new();
    let mut x = 1;
    let mut count = 0;
    while !map.contains_key(&(x % n)) {
        x %= n;
        if x == 0 {
            return 0;
        }
        count += 1;
        map.insert(x, count);
        x *= 10;
    }
    count - *map.get(&(x % n)).unwrap() + 1
}

#[cfg(test)]
mod tests {
    use crate::problems::one_hundred::twenty_six::{repeating_length, solve};

    #[test]
    fn test_repeating_length() {
        assert_eq!(repeating_length(1), 0);
        assert_eq!(repeating_length(2), 0);
        assert_eq!(repeating_length(3), 1);
        assert_eq!(repeating_length(6), 1);
        assert_eq!(repeating_length(7), 6);
        assert_eq!(repeating_length(9), 1);
    }
    #[test]
    fn test_solve() {
        assert_eq!(solve(10), 7);
        assert_eq!(solve(1000), 983);
    }
}
