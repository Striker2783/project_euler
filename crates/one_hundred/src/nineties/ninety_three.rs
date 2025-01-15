use std::collections::HashSet;

pub fn run() {
    let s = solve();
    println!("{s}");
}

fn solve() -> String {
    let mut max = 0;
    let mut v = vec![];
    for a in 1..=6 {
        for b in (a + 1)..=7 {
            for c in (b + 1)..=8 {
                for d in (c + 1)..=9 {
                    let cons = get_consecutive(vec![a, b, c, d]);
                    if cons > max {
                        max = cons;
                        v = vec![a, b, c, d];
                    }
                }
            }
        }
    }
    format!("{}{}{}{}", v[0], v[1], v[2], v[3])
}

fn get_consecutive(n: Vec<u32>) -> u32 {
    let set = get_set2(n);
    for i in 1.. {
        if !set.contains(&i) {
            return i - 1;
        }
    }
    unreachable!()
}

fn get_set(n: Vec<u32>) -> HashSet<u32> {
    let mut set = HashSet::new();
    helper(&n, &mut set);
    set
}
fn get_set2(n: Vec<u32>) -> HashSet<u32> {
    let mut set = HashSet::new();
    let n = n.into_iter().map(|n| n as f64).collect();
    helper2(&n, &mut set);
    set
}
fn helper2(v: &Vec<f64>, set: &mut HashSet<u32>) {
    if v.len() == 1 {
        if v[0] > i32::MAX as f64 {
            return;
        }
        if v[0].rem_euclid(1f64) > 0.001 {return;}
        let n = (v[0] + 0.00001) as i32;
        if n <= 0 {
            return;
        }
        set.insert(n as u32);
        return;
    }
    for i in 0..v.len() {
        for j in (i + 1)..v.len() {
            let mut v = v.clone();
            let m = v.remove(j);
            let n = v.remove(i);
            v.push(m + n);
            helper2(&v, set);
            v.pop();
            v.push(m - n);
            helper2(&v, set);
            v.pop();
            v.push(n - m);
            helper2(&v, set);
            v.pop();
            v.push(m * n);
            helper2(&v, set);
            v.pop();
            if m != 0f64 {
                v.push(n / m);
                helper2(&v, set);
                v.pop();
            }
            if n != 0f64 {
                v.push(m / n);
                helper2(&v, set);
                v.pop();
            }
        }
    }
}
/// Wrong somehow :shrug:
fn helper(v: &Vec<u32>, set: &mut HashSet<u32>) {
    if v.len() == 1 {
        if v[0] == 0 {
            return;
        }
        set.insert(v[0]);
        return;
    }
    for i in 0..v.len() {
        for j in (i + 1)..v.len() {
            let mut v = v.clone();
            let m = v.remove(j);
            let n = v.remove(i);
            v.push(m + n);
            helper(&v, set);
            v.pop();
            v.push(m * n);
            helper(&v, set);
            v.pop();
            if m >= n {
                v.push(m - n);
            } else {
                v.push(n - m);
            }
            helper(&v, set);
            v.pop();
            if m != 0 && n % m == 0 {
                v.push(n / m);
                helper(&v, set);
            } else if n != 0 && m % n == 0 {
                v.push(m / n);
                helper(&v, set);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::nineties::ninety_three::{get_set, get_set2};

    #[test]
    fn test_get_set() {
        let expected: HashSet<_> = [
            1, 2, 3, 4, 5, 6, 8, 10, 11, 12, 13, 14, 15, 16, 18, 19, 20, 21, 23, 24, 25, 27, 28,
            29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 40, 43, 44, 45, 47, 48, 49, 51, 56, 58, 59, 61,
            64, 65, 67, 69, 72, 74, 75, 77, 79, 81, 83, 86, 91, 92, 93, 95, 96, 97, 99, 100, 101,
            107, 111, 115, 123, 124, 126, 131, 132, 139, 147, 152, 160, 161, 171, 180, 187, 188,
            192, 195, 196, 215, 216, 220, 225, 232, 233, 244, 252, 260, 280, 281, 284, 287, 295,
            296, 308, 316, 324, 344, 351, 468, 472, 476, 500, 508, 512, 532, 536, 540, 728, 756,
            792, 2016,
        ]
        .into();
        let actual = get_set2(vec![4, 7, 8, 9]);
        assert_eq!(actual.len(), expected.len());
        assert_eq!(actual, expected);
    }
}
