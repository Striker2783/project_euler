use std::{
    collections::{HashMap, HashSet},
    u32,
};

pub fn run() {
    println!("{}", solve(12_000));
}

fn prime_factorization(mut n: u32) -> HashMap<u32, u32> {
    let mut map = HashMap::new();
    for i in 2.. {
        while n % i == 0 {
            n /= i;
            map.entry(i).and_modify(|n| *n += 1).or_insert(1);
        }
        if i * i > n {
            break;
        }
    }
    if n > 1 {
        map.insert(n, 1);
    }
    map
}

fn solve(max: u32) -> u64 {
    let mut set: HashSet<u32> = (2..=max).collect();
    let mut set2 = HashSet::new();
    let mut i = 2;
    while !set.is_empty() {
        let ks = get_ks(i);
        for k in ks {
            if set.remove(&k) {
                set2.insert(i);
            }
        }
        i += 1;
    }
    set2.into_iter().map(|n| n as u64).sum()
}

fn get_ks(ori: u32) -> HashSet<u32> {
    fn helper(n: u32, sum: u32, k: u32, ori: u32, set: &mut HashSet<u32>) {
        if sum > ori {
            return;
        } else if n == 1 {
            if k == 1 {
                return;
            }
            set.insert(k + (ori - sum));
            return;
        }
        let mut i = 2;
        while i * i <= n {
            if n % i == 0 {
                helper(n / i, sum + i, k + 1, ori, set);
                helper(i, sum + n / i, k + 1, ori, set);
            }
            i += 1;
        }
        if n > 1 {
            helper(1, sum + n, k + 1, ori, set);
        }
    }
    let mut set = HashSet::new();
    helper(ori, 0, 0, ori, &mut set);
    set
}

fn min_k(ori: u32) -> u32 {
    fn helper(n: u32, sum: u32, k: u32, ori: u32) -> u32 {
        if sum > ori {
            return u32::MAX;
        } else if n == 1 {
            if k == 1 {
                return u32::MAX;
            }
            return k + (ori - sum);
        }
        let mut i = 2;
        let mut min = u32::MAX;
        while i * i <= n {
            if n % i == 0 {
                min = helper(n / i, sum + i, k + 1, ori).min(min);
                min = helper(i, sum + n / i, k + 1, ori).min(min);
            }
            i += 1;
        }
        if n > 1 {
            min = helper(n / n, sum + n, k + 1, ori).min(min);
        }
        min
    }
    helper(ori, 0, 0, ori)
}

#[cfg(test)]
mod tests {

    use crate::eighties::eighty_eight::{min_k, solve};

    use super::{get_ks, prime_factorization};

    #[test]
    fn test_prime_factorization() {
        let test1 = prime_factorization(12);
        assert_eq!(test1.len(), 2);
        assert_eq!(*test1.get(&2).unwrap(), 2);
        assert_eq!(*test1.get(&3).unwrap(), 1);
        let test2 = prime_factorization(7);
        assert_eq!(test2.len(), 1);
        assert_eq!(test2.get(&7).unwrap().clone(), 1);
    }

    #[test]
    fn test_min_k() {
        assert_eq!(min_k(4), 2);
        assert_eq!(min_k(12), 6);
    }
    #[test]
    fn test_solve() {
        assert_eq!(solve(6), 30);
        assert_eq!(solve(12), 61);
    }
}
