use common::number_series::{Primes, Spiral};

pub fn run() {
    let result = solve();
    println!("{result}");
}

fn solve() -> u64 {
    let mut prime: Primes<u64> = Primes::default();
    let mut primes = 3;
    let mut composite = 2;
    let mut spiral: Spiral<u64> = Spiral::default();
    spiral.next(); // Remove 1
    // Outer loop
    spiral.next();
    spiral.next();
    spiral.next();
    spiral.next();
    while primes * 10 > composite + primes {
        for _ in 0..4 {
            let n = spiral.next().unwrap();
            if prime.is_prime(n) {
                primes += 1;
            } else {
                composite += 1;
            }
        }
    }
    spiral.side() + 1
}
