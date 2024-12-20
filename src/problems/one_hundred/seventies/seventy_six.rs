pub fn run() {
    println!("{}", solve(100));
}

fn solve(curr: u32) -> u32 {
    // i is number, j is how many combinations up to (j+1)
    let mut dp = vec![vec![0; (curr + 1) as usize]; (curr + 1) as usize];
    for i in 0..=curr {
        let i = i as usize;
        dp[i][0] = 1;
        for n in 1..=(curr as usize) {
            dp[i][n] = dp[i][n - 1];
            if i >= n + 1 {
                dp[i][n] += dp[i - n - 1][n];
            }
        }
    }
    dp[(curr ) as usize][(curr ) as usize] - 1
}
#[cfg(test)]
mod tests {
    use crate::problems::one_hundred::seventies::seventy_six::solve;

    #[test]
    fn test_solve() {
        assert_eq!(solve(4), 4);
        assert_eq!(solve(5), 6);
        assert_eq!(solve(6), 10);
    }
}
