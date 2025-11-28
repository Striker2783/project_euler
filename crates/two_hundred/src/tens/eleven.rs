use common::number_series::Primes;

pub fn run() {
    println!("{}", solve2(10));
}

fn solve2(n: u32) -> u64 {
    let mut primes = Primes::<u64>::default();
    let mut total = 0;
    for digit in 0..10 {
        let mut sum = 0;
        for i in (2..=n).rev() {
            if sum != 0 {
                break;
            }
            generate_repeated(digit, n, i, |n| {
                if primes.is_prime(n) {
                    sum += n;
                }
            });
        }
        total += sum;
    }
    total
}

fn generate_repeated(digit: u32, digits: u32, reps: u32, mut f: impl FnMut(u64)) {
    fn helper(digit: u32, digits: u32, reps: u32, curr: u64, f: &mut impl FnMut(u64)) {
        if digits == 0 {
            if reps == 0 {
                f(curr);
            }
            return;
        } else if reps > digits {
            return;
        }
        if reps > 0 && (digit != 0 || curr != 0) {
            helper(digit, digits - 1, reps - 1, curr * 10 + digit as u64, f);
        }
        for n in 0..10 {
            if digit == n || curr == 0 && n == 0 {
                continue;
            }
            helper(digit, digits - 1, reps, curr * 10 + n as u64, f);
        }
    }
    helper(digit, digits, reps, 0, &mut f);
}

fn get_digits(mut n: u64) -> [u32; 10] {
    let mut v = [0; 10];
    while n > 0 {
        v[n as usize % 10] += 1;
        n /= 10;
    }
    v
}

fn solve(n: u32) -> u64 {
    let mut digits = vec![(2, Vec::new()); 10];
    for p in Primes::<u64>::default()
        .skip_while(|&x| x < 10u64.pow(n - 1))
        .take_while(|&x| x < 10u64.pow(n))
    {
        let digs = get_digits(p);
        for (i, &d) in digs.iter().enumerate() {
            if d < digits[i].0 {
            } else if d > digits[i].0 {
                digits[i].0 = d;
                digits[i].1 = vec![p];
            } else {
                digits[i].1.push(p);
            }
        }
    }
    digits.iter().map(|(_, v)| v.iter().sum::<u64>()).sum()
}

#[cfg(test)]
mod tests {
    use crate::tens::eleven::{solve, solve2};

    #[test]
    fn test_solve() {
        assert_eq!(solve2(1), 0);
        assert_eq!(solve2(2), 11);
        assert_eq!(solve2(3), 24400);
        assert_eq!(solve2(4), 273700);

        assert_eq!(solve(1), 0);
        assert_eq!(solve(2), 11);
        assert_eq!(solve(3), 24400);
        assert_eq!(solve(4), 273700);
    }
}
