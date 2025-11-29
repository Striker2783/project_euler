pub fn run() {
    solve(1);
}

fn solve(pow: u32) -> u64 {
    let mut dp_dec = vec![vec![0; 10]; pow as usize + 1];
    dp_dec[0] = vec![1; 10];
    dp_dec[1] = vec![1; 10];
    for i in 2..=(pow as usize) {
        dp_dec[i][0] = 1;
        for j in 1..=9 {
            dp_dec[i][j] = 0;
            for k in 0..=j {
                dp_dec[i][j] += dp_dec[i - 1][k];
            }
        }
    }
    let dec = dp_dec
        .iter()
        .skip(1)
        .map(|v| v.iter().sum::<u64>())
        .sum::<u64>();
    let mut dp_inc = vec![vec![0; 10]; pow as usize + 1];
    
    dec
}

#[cfg(test)]
mod tests {
    use crate::tens::thirteen::solve;

    #[test]
    fn test_solve() {
        assert_eq!(solve(1), 9);
        assert_eq!(solve(3), 99);
        assert_eq!(solve(6), 12951);
    }
}
