use std::{collections::HashMap, mem::swap};

pub fn run() {
    println!("{}", solve_2(1_500_000));
}
/// Brute Force... DEAD
fn solve(max: u64) -> u32 {
    let map: HashMap<_, _> = (2..)
        .map_while(|n| if n > max { None } else { Some((n * n, n)) })
        .collect();
    let mut map2 = HashMap::new();
    for n in 1..=max {
        for m in n..=(max - n) {
            if let Some(&o) = map.get(&(n * n + m * m)) {
                if n + m + o > max {
                    continue;
                }
                map2.entry(n + m + o).and_modify(|n| *n += 1).or_insert(1);
            }
        }
    }
    map2.into_iter().filter(|(a, b)| *b == 1).count() as u32
}
fn solve_2(max: u64) -> u32 {
    let mut map2 = HashMap::new();
    for n in 1.. {
        if n * n > max {
            break;
        }
        for m in (n + 1).. {
            if m * m + n * n > max {
                break;
            } else if !((m % 2 == 0) ^ (n % 2 == 0)) {
                continue;
            } else if n != 1 && gcd(m, n) != 1 {
                continue;
            }
            let a = m * m - n * n;
            let b = 2 * m * n;
            let c = m * m + n * n;
            for k in 1.. {
                if k * (a + b + c) > max {
                    break;
                }
                map2.entry(k * (a + b + c))
                    .and_modify(|n| *n += 1)
                    .or_insert(1);
            }
        }
    }
    map2.into_iter().filter(|(a, b)| *b == 1).count() as u32
}
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b > 0 {
        swap(&mut a, &mut b);
        b %= a;
    }
    a
}
#[cfg(test)]
mod tests {
    use crate::problems::one_hundred::seventies::seventy_five::{solve, solve_2};

    #[test]
    fn test_solve() {
        assert_eq!(solve(48), 6);
        assert_eq!(solve(48), solve_2(48));
    }
}
