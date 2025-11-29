use std::fs;

use crate::ones::three::is_special_sum_set;

const FILE: &str = "Files/105.txt";

pub fn run() {
    let v = parse(FILE);
    let res = solve(&v);
    println!("{res}");
}

fn parse(path: &str) -> Vec<Vec<u32>> {
    let f = fs::read_to_string(path).unwrap();
    f.lines()
        .map(|line| line.split(',').map(|n| n.parse::<u32>().unwrap()).collect())
        .collect()
}

fn solve(v: &[Vec<u32>]) -> u32 {
    v.iter()
        .filter(|v| is_special_sum_set(v))
        .map(|v| v.iter().sum::<u32>())
        .sum()
}
