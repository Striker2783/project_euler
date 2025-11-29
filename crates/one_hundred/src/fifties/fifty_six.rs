use num::ToPrimitive;
use num::{BigUint, Zero};

pub fn run() {
    let max = (1u32..100)
        .map(|a| {
            (1..100)
                .map(|b| sum_digits(BigUint::from(a).pow(b)))
                .max()
                .unwrap()
        })
        .max()
        .unwrap();
    println!("{max}");
}
fn sum_digits(mut n: BigUint) -> u32 {
    let mut sum = 0;
    while !n.is_zero() {
        sum += (&n % 10_u32).to_u32().unwrap();
        n /= 10u32;
    }
    sum
}
#[cfg(test)]
mod tests {
    use num::{BigUint, Num};
    use test::{Bencher, black_box};

    use crate::fifties::fifty_six::sum_digits;

    #[test]
    fn test_sum_digits() {
        assert_eq!(sum_digits(BigUint::from(123456u32)), 1 + 2 + 3 + 4 + 5 + 6);
        let num = BigUint::from(10u32).pow(100u32) - 1 as u32;
        let expected = 9 * 100;
        assert_eq!(sum_digits(num), expected);
    }
    #[bench]
    fn bench_sum_digits(b: &mut Bencher) {
        let num = BigUint::from_str_radix("4dd12d1236a1258c7e1ccecf699cbb17d64521d2e3eeda3b89d1947a971462700459250711f62e588a48aef75a94d4c8dbce63cf6a92d86b4a382e3a6c77b4a360910b1a6a89096b255bcfae75189aeecb20858e8e61155bae74ea19f7abdbece897c6ae", 16).unwrap();
        b.iter(|| black_box(sum_digits(num.clone())))
    }
}
