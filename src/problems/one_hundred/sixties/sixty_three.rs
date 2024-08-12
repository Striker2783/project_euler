use num::pow::Pow;

pub fn run() {
    println!("{}", solve(21));
}

fn get_digits(mut n: f64) -> u32 {
    let mut digs = 0;
    while n >= 1f64 {
        n /= 10f64;
        digs += 1;
    }
    digs
}

fn solve(max: u32) -> u32 {
    let mut count = 0;
    for a in 1..=max {
        for b in 1..10u64 {
            let pow = (b as f64).pow(a as f64);
            if get_digits(pow) == a {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use crate::problems::one_hundred::sixties::sixty_three::solve;

    #[test]
    fn test_solve() {
        assert_eq!(solve(21), 49);
    }
}
