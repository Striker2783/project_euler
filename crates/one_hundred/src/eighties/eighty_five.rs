use std::u64;

pub fn run() {
    println!("{}", solve(2_000_000));
}
fn solve(n: u64) -> u64 {
    let mut best = 0u64;
    let mut best_area = 0;
    let mut y = (n as f64).sqrt() as u64 + 1;
    let mut x = y;
    while y > 0 {
        loop {
            if x == 0 {
                break;
            }
            let curr = rectangles_3(x, y);
            if curr.abs_diff(n) < best.abs_diff(n) {
                best_area = x * y;
                best = curr;
            }
            x -= 1;
        }
        y -= 1;
        x = y;
    }
    best_area
}
/// Not super optimized
fn rectangles_1(n: u64, m: u64) -> u64 {
    let mut sum = 0;
    for i in 0..n {
        for j in 0..m {
            sum += (n - i) * (m - j);
        }
    }
    sum
}

fn rectangles_2(n: u64, m: u64) -> u64 {
    let mut sum = 0;
    for i in 0..n {
        sum += (n - i) * ((m * (m + 1)) / 2);
    }
    sum
}

fn rectangles_3(n: u64, m: u64) -> u64 {
    ((n * (n + 1)) / 2) * ((m * (m + 1)) / 2)
}

#[cfg(test)]
mod tests {
    use crate::eighties::eighty_five::{rectangles_1, rectangles_2, rectangles_3, solve};

    #[test]
    fn test_rectangles() {
        assert_eq!(rectangles_1(3, 2), 18);
        assert_eq!(rectangles_1(2, 3), 18);
        assert_eq!(rectangles_2(2, 3), rectangles_1(2, 3));
        assert_eq!(rectangles_2(10, 9), rectangles_1(9, 10));
        assert_eq!(rectangles_3(10, 9), rectangles_1(9, 10));
    }
    #[test]
    fn test_solve() {
        assert_eq!(solve(18), 6);
        assert_eq!(solve(1_000), 56);
        assert_eq!(solve(1_000_000), 1632);
    }
}
