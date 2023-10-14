use std::collections::HashSet;

pub fn ten() {
    let a = sieve_of_eratosthenes(2_000_000);
    let sum: u64 = a.iter().sum();
    println!("{sum}");
}
pub fn sieve_of_eratosthenes(upper: u64) -> Vec<u64> {
    let range = 0u64..upper;
    let mut vec: Vec<bool> = range.map(|_| true).collect();
    for i in 2.. {
        if i * i > upper {
            break;
        }
        if !vec[i as usize] {
            continue;
        }
        let vec_length = vec.len() as u64;
        for j in ((2 * i)..vec_length).step_by(i as usize) {
            vec[j as usize] = false;
        }
    }
    let mut primes: Vec<u64> = vec![];
    (2..vec.len()).for_each(|i| {
        if vec[i] {
            primes.push(i as u64);
        }
    });
    primes
}

pub fn sieve_of_eratosthenes_set(upper: u64) -> HashSet<u64> {
    let primes = sieve_of_eratosthenes(upper);
    primes.iter().copied().collect()
}
