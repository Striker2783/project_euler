use std::collections::HashMap;

pub fn run() {
    println!("{}", solve(10_000));
}

// Extremely Unoptimal
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

fn solve(max: u32) -> usize {
    (2..=max)
        .filter(|&n| match get_cycle(n) {
            None => false,
            Some(n) => n & 1 == 1,
        })
        .count()
}

fn get_cycle(n: u32) -> Option<usize> {
    let min_square = min_square_without_square(n)?;
    let mut map: HashMap<(u32, u32), usize> = HashMap::new();
    let mut num = min_square;
    let mut d = 1;
    for i in 0.. {
        if let Some(j) = map.get(&(num, d)) {
            return Some(i - j);
        }
        map.insert((num, d), i);
        let diff = n - num.pow(2);
        d = diff / d;
        let a = (num + min_square) / d;
        num = a * d - num;
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use crate::sixties::sixty_four::{get_cycle, min_square_without_square, solve};

    #[test]
    fn test_min_square() {
        assert_eq!(min_square_without_square(100), None);
        assert_eq!(min_square_without_square(99), Some(9));
    }
    #[test]
    fn test_cycle() {
        assert_eq!(get_cycle(2), Some(1));
        assert_eq!(get_cycle(13), Some(5));
        assert!(get_cycle(4).is_none());
    }
    #[test]
    fn test_solve() {
        assert_eq!(solve(13), 4);
        assert_eq!(solve(10_000), 1322);
    }
}
