use num::BigUint;

pub fn run() {
    println!("{}", solve(100));
}

fn solve(max: u32) -> u32 {
    let mut sum = 0;
    'outer: for i in 2..=max {
        for n in 2.. {
            if n * n == i {
                continue 'outer;
            } else if n * n > i {
                break;
            }
        }
        sum += sqrt(i);
    }
    sum
}

fn sqrt(n: u32) -> u32 {
    let mut sqrt = (n * BigUint::from(10u32).pow(198)).sqrt();
    let mut sum = 0;
    for _ in 0..100 {
        sum += match (sqrt.clone() % 10u32).to_u32_digits().first() {
            Some(n) => *n,
            None => 0,
        };
        sqrt /= 10u32;
    }
    sum
}

#[cfg(test)]
mod tests {
    use crate::eighties::eighty::sqrt;

    #[test]
    fn test_sqrt() {
        assert_eq!(sqrt(2), 475);
    }
}