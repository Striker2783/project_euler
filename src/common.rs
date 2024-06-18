pub const SMALL_PRIMES: &[u32] = &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29];

pub fn is_prime(n: u32) -> bool {
    if n == 0 || n == 1 {
        return false;
    }
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}
pub fn num_len(mut n: u32) -> u32 {
    let mut len = 0;
    while n > 0 {
        n /= 10;
        len += 1;
    }
    len
}
