pub fn run() {
    println!("{}", solve_2(1_000_000))
}
/// Brute Force
fn solve(n: u32) -> u32 {
    for i in 1.. {
        let s = size_2(i);
        if s > n {
            return i as u32;
        }
    }
    unreachable!()
}

fn solve_2(n: u32) -> u32 {
    let mut sum = 0;
    for a in 1.. {
        for bc in 1..=(2 * a) {
            let c = match sqrt(a * a + bc * bc) {
                Some(a) => a,
                None => continue,
            };
            if a > bc {
                sum += bc as u32 / 2;
            } else {
                sum += a as u32 - ((bc as u32 - 1) / 2);
            }
        }
        if sum > n {
            return a as u32;
        }
    }
    unreachable!()
}

fn size(m: u64) -> u32 {
    let mut count = 0;
    for i in 1..=m {
        for j in 1..=i {
            for k in 1..=j {
                let a = i * i + (j + k) * (j + k);
                let b = j * j + (i + k) * (i + k);
                let c = k * k + (i + j) * (i + j);
                let min = a.min(b).min(c);
                if sqrt(min).is_none() {
                    continue;
                }
                count += 1;
            }
        }
    }
    count
}

fn sqrt(n: u64) -> Option<u64> {
    let mut i = 0;
    while i * i < n {
        i += 1;
    }
    if i * i == n { Some(i) } else { None }
}

fn size_2(m: u64) -> u32 {
    let mut sum = 0;
    for a in 1..m {
        for bc in 1..=(2 * a) {
            let c = match sqrt(a * a + bc * bc) {
                Some(a) => a,
                None => continue,
            };
            if a > bc {
                sum += bc as u32 / 2;
            } else {
                sum += a as u32 - ((bc as u32 - 1) / 2);
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use crate::eighties::eighty_six::{size, size_2, solve, solve_2};

    #[test]
    fn test_solve() {
        assert_eq!(solve(2059), 100);
    }
    #[test]
    fn test_size() {
        assert_eq!(size(10), size_2(10));
        assert_eq!(size(99), 1975);
        assert_eq!(size(100), 2060);
        assert_eq!(size(101), size_2(101));
    }
}
