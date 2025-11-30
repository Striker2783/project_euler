pub fn run() {
    println!("{:?}", solve(1_000));
}

fn pow_mod(mut b: u64, mut e: u64, d: u64) -> u64 {
    let mut n = 1;
    b %= d;

    while e > 0 {
        if e & 1 == 1 {
            n = (n * b) % d;
        }
        b = (b * b) % d;
        e >>= 1;
    }
    n
}

fn r_max(a: u64) -> u64 {
    let mut max = 0;
    for n in 0..(2 * a) {
        let num1 = pow_mod(a - 1, n, a * a);
        let num2 = pow_mod(a + 1, n, a * a);
        max = max.max((num1 + num2) % (a * a));
    }
    max
}
// Brute Force (Pretty slow)
fn solve(m: u64) -> u64 {
    (3..=m).map(r_max).sum()
}

#[cfg(test)]
mod tests {
    use crate::twenties::twenty::r_max;

    #[test]
    fn test_r_max() {
        assert_eq!(r_max(7), 42);
    }
}
