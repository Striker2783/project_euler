use std::{collections::HashMap, result};

use num::pow::Pow;

use common::number_series::Primes;

pub fn run() {
    let mut solver = Solver::default();
    println!("{:?}", solver.solve()[1])
}
#[derive(Default)]
struct Solver {
    map: HashMap<u32, Vec<u32>>,
}
impl Solver {
    fn solve(&mut self) -> Vec<u64> {
        self.fill_map();
        self.map
            .values()
            .filter_map(|v| Self::is_correlated(v))
            .map(|arr| Self::concat(&arr))
            .collect()
    }
    fn concat(v: &[u32]) -> u64 {
        let mut n = 0;
        for &x in v {
            n *= 10u64.pow(4);
            n += x as u64;
        }
        n
    }
    fn is_correlated(vec: &[u32]) -> Option<Vec<u32>> {
        if vec.len() < 3 {
            return None;
        }
        let mut vec = Vec::from(vec);
        vec.sort_unstable();
        for i in 0..(vec.len() - 2) {
            let n1 = vec[i];
            for j in (i + 1)..vec.len() {
                let n2 = vec[j];
                let diff = n2 - n1;
                let mut correlated = vec![n1, n2];
                for k in (j + 1)..vec.len() {
                    let n3 = vec[k];
                    if (n3 - n1) == diff * correlated.len() as u32 {
                        correlated.push(n3);
                    }
                }
                if correlated.len() >= 3 {
                    return Some(correlated);
                }
            }
        }
        None
    }
    fn fill_map(&mut self) {
        for p in Primes::default() {
            if p < 10u32.pow(3) {
                continue;
            } else if p >= 10u32.pow(4) {
                break;
            }
            let sort_digits = Self::sort_digits(p);
            self.map.entry(sort_digits).or_insert_with(Vec::new).push(p);
        }
    }
    fn sort_digits(mut n: u32) -> u32 {
        let mut arr = [0; 10];
        while n > 0 {
            arr[(n % 10) as usize] += 1;
            n /= 10;
        }
        for (i, &v) in arr.iter().enumerate() {
            for _ in 0..v {
                n *= 10;
                n += i as u32
            }
        }
        n
    }
}

#[cfg(test)]
mod tests {
    use super::Solver;

    #[test]
    fn test_sort_digits() {
        assert_eq!(Solver::sort_digits(1487), 1478);
        assert_eq!(
            Solver::is_correlated(&[1487, 4817, 8147]),
            Some(vec![1487, 4817, 8147])
        );
        assert_eq!(
            Solver::is_correlated(&[1487, 1847, 4817, 4871, 7481, 7841, 8147, 8741]),
            Some(vec![1487, 4817, 8147])
        )
    }
}
