pub fn run() {
    let solution = solve(1001);
    println!("{solution}");
}

fn solve(n: u32) -> u64 {
    let mut sum = 1;

    for i in (2..=n as u64).step_by(2) {
        let big = (i + 1).pow(2);
        sum += big;
        sum += big - i;
        sum += big - 2 * i;
        sum += big - 3 * i;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test_solve() {
        assert_eq!(solve(5), 101);
        assert_eq!(solve(1001), 669171001);
    }
}
