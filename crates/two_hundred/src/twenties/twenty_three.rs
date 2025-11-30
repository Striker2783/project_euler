use common::number_series::Primes;

pub fn run() {
    println!("{}", solve(10u64.pow(10)));
}

fn solve(num: u64) -> u64 {
    for (i, p) in Primes::<u64>::default().enumerate() {
        let n = i as u64 + 1;
        if n % 2 == 0 {
            continue;
        }
        let r = (2 * n * p) % (p * p);
        if r > num {
            return n;
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tets {
    use crate::twenties::twenty_three::solve;

    #[test]
    fn test_solve() {
        assert_eq!(solve(10u64.pow(9)), 7037);
    }
}
