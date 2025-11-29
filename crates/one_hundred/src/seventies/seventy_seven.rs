use common::number_series::Primes;

pub fn run() {
    println!("{:?}", solve(5_000));
}

fn solve(ways: u64) -> u64 {
    let dp = combinations(1_000);
    for (i, &n) in dp.iter().enumerate() {
        if n > ways {
            return i as u64;
        }
    }
    unreachable!()
}

fn combinations(max: usize) -> Vec<u64> {
    let mut dp = vec![0; max];
    dp[0] = 1;
    for p in Primes::<u64>::default() {
        if p as usize > dp.len() {
            break;
        }
        for i in 0..(dp.len() as u64 - p) {
            dp[(i + p) as usize] += dp[i as usize];
        }
    }
    dp
}

#[cfg(test)]
mod tests {
    use crate::seventies::seventy_seven::{combinations, solve};

    #[test]
    fn test_combinations() {
        let combs = combinations(100);
        assert_eq!(combs[2], 1);
        assert_eq!(combs[3], 1);
        assert_eq!(combs[5], 2);
    }
}
