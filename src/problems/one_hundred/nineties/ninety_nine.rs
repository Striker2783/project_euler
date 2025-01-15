use std::fs;

pub fn run() {
    let s = fs::read_to_string(FILE).unwrap();
    println!("{}", solve(&s));
}

const FILE: &str = "Files/99.txt";
fn solve(s: &str) -> usize {
    let mut max_log = 0f64;
    let mut best = 0;
    for (i, line) in s.lines().enumerate() {
        let mut split = line.split(',');
        let base: u32 = split.next().unwrap().parse().unwrap();
        let exp: u32 = split.next().unwrap().parse().unwrap();
        let log = (base as f64).log10() * exp as f64;
        if log > max_log {
            max_log = log;
            best = i + 1;
        }
    }
    best
}
