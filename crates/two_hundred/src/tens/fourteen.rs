pub fn run() {
    println!("{}", counting_blocks(50, 3));
}

pub fn counting_blocks(n: usize, m: usize) -> u64 {
    let mut dp = vec![0u64; n + 1];
    for i in 0..m.min(n) {
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
    use crate::tens::fourteen::counting_blocks;

    #[test]
    fn test_counting_blocks() {
        assert_eq!(counting_blocks(4, 3), 4);
        assert_eq!(counting_blocks(7, 3), 17);
        assert_eq!(counting_blocks(29, 3), 673135);
    }
}
