pub fn run() {
    println!(
        "{}",
        solve(
            |n| 1 - n + n.pow(2) - n.pow(3) + n.pow(4) - n.pow(5) + n.pow(6) - n.pow(7) + n.pow(8)
                - n.pow(9)
                + n.pow(10),
            10
        )
    );
}

fn solve<T: Fn(i64) -> i64>(generator: T, deg: u32) -> i64 {
    let mut v = vec![generator(1)];
    let mut sum = 0;
    for i in 2..=(deg + 1) {
        let next = generator(i as i64);
        let next_optimum = next_optimum(&v);
        if next_optimum != next {
            sum += next_optimum;
        };
        v.push(next);
    }
    sum
}

fn next_optimum(v: &[i64]) -> i64 {
    let mut diffs = v.to_vec();
    let mut last = vec![];
    while !diffs.is_empty() {
        last.push(*diffs.last().unwrap());
        for i in 0..(diffs.len() - 1) {
            diffs[i] = diffs[i + 1] - diffs[i];
        }
        diffs.pop();
    }
    for i in (0..(last.len() - 1)).rev() {
        last[i] += last[i + 1];
    }
    last[0]
}
#[cfg(test)]
mod tests {
    use crate::ones::one::{next_optimum, solve};

    #[test]
    fn test_next() {
        assert_eq!(next_optimum(&[1]), 1);
        assert_eq!(next_optimum(&[1, 8]), 15);
        assert_eq!(next_optimum(&[1, 8, 27]), 58);
        assert_eq!(next_optimum(&[1, 8, 27, 64]), 125);
    }
    #[test]
    fn test_solve() {
        assert_eq!(solve(|n| n * n * n, 3), 74);
    }
}
