pub fn run() {
    println!("{}", pentagonal(1000000));
}

fn dp(d: u64) -> u32 {
    let mut dp = vec![];
    dp.push(vec![1u64]);
    for n in 1.. {
        let mut vec = vec![];
        vec.push(1);
        for m in 2..=n {
            let mut val = *vec.last().unwrap();
            let o = n - m;
            let row = &dp[o as usize];
            val += match row.get((m - 1) as usize) {
                Some(a) => *a,
                None => *row.last().unwrap(),
            };
            vec.push(val % d);
        }
        if *vec.last().unwrap() % d == 0 {
            return n;
        }
        dp.push(vec);
    }
    unreachable!()
}
/// Solution based on https://en.wikipedia.org/wiki/Pentagonal_number_theorem#Relation_with_partitions
fn pentagonal(d: u32) -> u32 {
    let mut cache = vec![1u32];
    for i in 1.. {
        let mut v = 0i32;
        for k in 1.. {
            let p = (3 * k * k - k) / 2;
            if p > i {
                break;
            }
            if k % 2 == 1 {
                v += cache[i - p] as i32;
            } else {
                v -= cache[i - p] as i32;
            }
            let p = (3 * k * k + k) / 2;
            if p > i {
                break;
            }
            if k % 2 == 1 {
                v += cache[i - p] as i32;
            } else {
                v -= cache[i - p] as i32;
            }
        }
        if v.rem_euclid(d as i32) == 0 {
            return i as u32;
        }
        cache.push(v.rem_euclid(d as i32) as u32);
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use crate::seventies::seventy_eight::{dp, pentagonal};

    #[test]
    fn test_pentagonal() {
        assert_eq!(dp(100), pentagonal(100));
    }
}
