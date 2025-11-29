pub fn run() {
    println!("{}", solve(15));
}

fn total_winning(n: u64, curr: u64, curr_blue: u64) -> u64 {
    if curr >= n {
        if n / 2 < curr_blue {
            return 1;
        }
        return 0;
    }
    let denominator = curr + 2;
    total_winning(n, curr + 1, curr_blue + 1)
        + (denominator - 1) * total_winning(n, curr + 1, curr_blue)
}

fn solve(n: u64) -> u64 {
    (2..=(n + 1)).product::<u64>() / total_winning(n, 0, 0)
}

#[cfg(test)]
mod tests {
    use crate::twenties::twenty_one::solve;

    #[test]
    fn test_solve() {
        assert_eq!(solve(1), 2);
        assert_eq!(solve(2), 6);
        assert_eq!(solve(4), 10);
    }
}
