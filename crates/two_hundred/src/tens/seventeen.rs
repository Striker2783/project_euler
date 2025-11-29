pub fn run() {
    println!("{}", solve(50));
}

fn solve(n: usize) -> u64 {
    let mut dp = vec![0; n + 1];
    for i in 0..=n {
        dp[i] = 1;
        for l in 2..=4.min(i) {
            for rem_before in 0..=(i - l) {
                dp[i] += dp[rem_before];
            }
        }
    }
    dp[n]
}

#[cfg(test)]
mod tests {
    use crate::tens::seventeen::solve;

    #[test]
    fn test_solve() {
        assert_eq!(solve(5), 15);
    }
}
