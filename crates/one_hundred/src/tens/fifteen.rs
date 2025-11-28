
pub fn run() {
    let x = lattice_path(20, 20);
    println!("{x}");
}

fn lattice_path(n: u64, m: u64) -> u64 {
    let mut sum = n + m;
    let mut max = n.max(m);
    let mut other = sum - max;
    let mut n = 1;
    let mut r = 1;
    while sum > 1 && max > 1 {
        if n > r {
            r *= other * max;
            other -= 1;
            max -= 1;
        } else {
            n *= sum;
            sum -= 1;
        }
        let mut i = 2;
        while i * i <= n.max(r) {
            if n % i == 0 && r % i == 0 {
                n /= i;
                r /= i;
                i = 2;
            } else {
                i += 1;
            }
        }
        if n == r {
            n = 1;
            r = 1;
        }
    }
    n *= factorial(sum);
    n /= factorial(other);
    n /= factorial(max);
    n / r
}
fn factorial(n: u64) -> u64 {
    (1..=n).product()
}

#[cfg(test)]
mod test {
    use crate::tens::fifteen::{factorial, lattice_path};
    #[test]
    fn test_lattice_path() {
        assert_eq!(lattice_path(2, 2), 6);
        assert_eq!(lattice_path(2, 3), 10);
        assert_eq!(lattice_path(20, 20), 137846528820);
    }
    #[test]
    fn test_factorial() {
        assert_eq!(factorial(2), 2);
        assert_eq!(factorial(3), 6);
        assert_eq!(factorial(4), 24);
    }
}
