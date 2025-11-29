pub fn run() {
    println!("{}", solve(50));
}

fn solve(n: usize) -> u64 {
    (2..=4).map(|m| calculate_counting_blocks(n, m)).sum()
}

fn calculate_counting_blocks(n: usize, m: usize) -> u64 {
    if n < m {
        return 0;
    }
    let mut dp = vec![0; n + 1];
    for squares in m..=n {
        for rem_before in 0..=(squares - m) {
            dp[squares] += 1;
            if rem_before >= m {
                dp[squares] += dp[rem_before];
            }
        }
    }
    *dp.last().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::tens::sixteen::{calculate_counting_blocks, solve};

    #[test]
    fn test_solve() {
        assert_eq!(calculate_counting_blocks(5, 2), 7);
        assert_eq!(calculate_counting_blocks(5, 3), 3);
        assert_eq!(calculate_counting_blocks(5, 4), 2);

        assert_eq!(solve(5), 12);
    }
}
