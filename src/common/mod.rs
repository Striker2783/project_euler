pub mod number_series;

pub const SMALL_PRIMES: &[u32] = &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29];

pub fn is_prime(n: u32) -> bool {
    if n == 0 || n == 1 {
        return false;
    } else if n == 2 || n == 3 {
        return true;
    } else if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut k: u32 = 0;
    loop {
        let n1 = 6 * k + 7;
        let n2 = 6 * k + 5;
        if n2 * n2 > n {
            return true;
        }
        if n % n1 == 0 {
            return false;
        }
        if n % n2 == 0 {
            return false;
        }
        k += 1;
    }
}
pub fn num_len(mut n: u32) -> u32 {
    let mut len = 0;
    while n > 0 {
        n /= 10;
        len += 1;
    }
    len
}

#[cfg(test)]
mod tests {
    use crate::common::is_prime;
    #[test]
    fn test_is_prime() {
        let primes = [2, 3, 5, 53, 97];
        let non_primes = [0, 1, 6, 10, 91, 5777];
        for p in primes {
            assert!(is_prime(p))
        }
        for np in non_primes {
            assert!(!is_prime(np))
        }
    }
}
