use std::fs;

pub fn run() {
    let s = fs::read_to_string(FILE).unwrap();
    println!("{}", solve(&s));
}
fn is_square(n: u64) -> bool {
    let mut left = 0;
    let mut right = n;
    while left <= right {
        let mid = left + (right - left) / 2;
        let square = match mid.checked_mul(mid) {
            Some(n) => n,
            None => {
                right = mid - 1;
                continue;
            }
        };
        if square == n {
            return true;
        }
        if (square < n) {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    return false;
}
const FILE: &str = "Files/98.txt";
fn solve(s: &str) -> u64 {
    let words = Words::from(s);
    words.solve2()
}

fn to_num2(s: &[u8]) -> u64 {
    let mut n = 0;
    for &b in s {
        n *= 10;
        n += b as u64;
    }
    n
}

fn to_num(s: &[u8], map: &[u32]) -> u64 {
    let mut n = 0;
    for &b in s {
        n *= 10;
        n += map[(b - b'A') as usize] as u64;
    }
    n
}
/// Does not Work
struct Solver<'a> {
    a: &'a [u8],
    b: &'a [u8],
    v: &'a [u32],
    set: [bool; 10],
    map: [u32; 26],
    max: &'a mut u64,
}
impl<'a> Solver<'a> {
    fn solve(&mut self, mut i: usize) {
        while i < self.a.len() && self.v[i] == 0 {
            i += 1;
        }
        if i >= self.a.len() {
            if self.map[(self.a[0] - b'A') as usize] == 0
                || self.map[(self.b[0] - b'A') as usize] == 0
            {
                return;
            }
            let a = to_num(self.a, &self.map);
            let b = to_num(self.b, &self.map);
            if is_square(a) && is_square(b) {
                *self.max = a.max(b).max(*self.max);
            }
            return;
        }
        for j in 0..self.set.len() {
            if self.set[j] {
                continue;
            }
            self.set[j] = true;
            self.map[i] = j as u32;
            self.solve(i + 1);
            self.set[j] = false;
        }
    }
}

struct Words {
    v: Vec<Vec<String>>,
}

impl Words {
    fn solve2(&self) -> u64 {
        let mut max = 0;
        for v in &self.v {
            for i in 0..v.len() {
                let mut outer = [0; 26];
                for b in v[i].bytes() {
                    outer[(b - b'A') as usize] += 1;
                }
                for j in (i + 1)..v.len() {
                    let mut inner = [0; 26];
                    for b in v[j].bytes() {
                        inner[(b - b'A') as usize] += 1;
                    }
                    if outer != inner {
                        continue;
                    }
                    'outer: for n in 4.. {
                        let mut square = n * n;
                        if square >= 10u64.pow(v[i].len() as u32) {
                            break;
                        } else if square < 10u64.pow(v[i].len() as u32 - 1) {
                            continue;
                        }
                        let mut a: Vec<_> = v[i].bytes().collect();
                        let mut b: Vec<_> = v[j].bytes().collect();
                        let mut set = [false; 10];
                        for i in (0..a.len()).rev() {
                            let curr = (square % 10) as u8;
                            if a[i] == curr {
                                continue;
                            } else if a[i] < 10 && a[i] != curr {
                                continue 'outer;
                            } else if set[curr as usize] && a[i] >= 10 {
                                continue 'outer;
                            }
                            set[curr as usize] = true;
                            let char = a[i];
                            for j in 0..a.len() {
                                if a[j] == char {
                                    a[j] = curr;
                                }
                                if b[j] == char {
                                    b[j] = curr;
                                }
                            }
                            square /= 10;
                        }
                        if b[0] == 0 {
                            continue 'outer;
                        }
                        let b = to_num2(&b);
                        if is_square(b) {
                            let a = to_num2(&a);
                            max = a.max(b).max(max);
                        }
                    }
                }
            }
        }
        max
    }
    /// Does not work
    fn solve(&self) -> u64 {
        let mut max = 0;
        let mut outer = vec![0; 26];
        let mut inner = vec![0; 26];
        for v in &self.v {
            for i in 0..v.len() {
                for i in 0..outer.len() {
                    outer[i] = 0
                }
                for b in v[i].bytes() {
                    outer[(b - b'A') as usize] += 1;
                }
                for j in (i + 1)..v.len() {
                    for i in 0..inner.len() {
                        inner[i] = 0;
                    }
                    for b in v[j].bytes() {
                        inner[(b - b'A') as usize] += 1;
                    }
                    if outer != inner {
                        continue;
                    }
                    Solver {
                        a: v[i].as_bytes(),
                        b: v[j].as_bytes(),
                        v: &outer,
                        set: [false; 10],
                        map: [0; 26],
                        max: &mut max,
                    }
                    .solve(0);
                }
            }
        }
        max
    }
}

fn get_map(s: &String) -> Vec<bool> {
    let mut i_map = vec![false; 26];
    for b in s.bytes() {
        i_map[(b - b'A') as usize] = true;
    }
    i_map
}

impl From<&str> for Words {
    fn from(value: &str) -> Self {
        let mut lens = Vec::new();
        for word in value.split(',') {
            let word = word[1..(word.len() - 1)].to_string();
            while lens.len() < word.len() {
                lens.push(vec![]);
            }
            lens[word.len() - 1].push(word);
        }
        Self { v: lens }
    }
}

#[cfg(test)]
mod tests {

    use super::solve;
    #[test]
    fn test_solve() {
        assert_eq!(solve("\"RACE\",\"CARE\""), 9216);
    }
}
