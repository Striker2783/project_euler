use std::collections::HashSet;

pub fn run() {
    println!("{}", solve(4_000_000));
}

const PRIMES: &[u64] = &[
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
    101, 103, 107, 109, 113, 127, 131,
];

fn calculate_distinct(v: &[u64]) -> u64 {
    v.iter().map(|&n| n + 1).product::<u64>().div_ceil(2)
}

fn solve(min: u64) -> u64 {
    let mut set = HashSet::new();
    fn helper(
        v: &mut Vec<u64>,
        num: u64,
        min: u64,
        set: &mut HashSet<Vec<u64>>,
        mut min_n: u64,
    ) -> u64 {
        if num >= min_n {
            return min_n;
        } else if calculate_distinct(v) > min {
            return num;
        } else if set.contains(v) {
            return min_n;
        }
        set.insert(v.clone());
        v.push(2);
        min_n = min_n.min(helper(v, num * PRIMES[v.len() - 1], min, set, min_n));
        v.pop();
        for i in 0..v.len() {
            v[i] += 2;
            min_n = min_n.min(helper(v, num * PRIMES[i], min, set, min_n));
            v[i] -= 2;
        }
        min_n
    }
    let mut v = Vec::new();
    helper(&mut v, 1, min, &mut set, u64::MAX)
}

#[cfg(test)]
mod tests {
    use crate::tens::ten::{calculate_distinct, solve};

    #[test]
    fn test_solve() {
        assert_eq!(solve(100), 1260);
    }
    #[test]
    fn test_helpers() {
        assert_eq!(calculate_distinct(&[2, 2]), 5);
    }
}
