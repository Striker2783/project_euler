pub fn run() {
    println!("{}", solve(50, 3));
}

fn solve(n: usize, m: usize) -> u64 {
    let mut dp = vec![0u64; n + 1];
    for i in 0..m {
        dp[i] = 1;
    }
    for squares in m..=n {
        dp[squares] += 1;
        for l in m..=squares {
            for offset in 0..=(squares - l) {
                let rem_after = squares - l - offset;
                if rem_after > m {
                    dp[squares] += dp[rem_after - 1];
                } else {
                    dp[squares] += 1;
                }
            }
        }
    }
    *dp.last().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::tens::fourteen::solve;

    #[test]
    fn test_solve() {
        assert_eq!(solve(4, 3), 4);
        assert_eq!(solve(7, 3), 17);
        assert_eq!(solve(29, 3), 673135);
    }
}
