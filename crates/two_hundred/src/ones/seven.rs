use std::fs;

const FILE: &str = "Files/107.txt";

pub fn run() {
    let mut edges = parse_str(&fs::read_to_string(FILE).unwrap());
    let res = solve(&mut edges);
    let sum: u32 = edges.iter().map(|e| e.weight).sum();
    println!("{}", sum / 2 - res);
}

fn solve(edges: &mut [Edge]) -> u32 {
    let vertices = edges.iter().map(|e| e.a.max(e.b)).max().unwrap() + 1;
    edges.sort_by(|e1, e2| e1.weight.cmp(&e2.weight));
    let mut union = UnionFind::new(vertices);
    let mut sum = 0;
    for edge in edges.iter() {
        if union.same_set(edge.a, edge.b) {
            continue;
        }
        union.connect(edge.a, edge.b);
        sum += edge.weight;
    }
    for i in 0..vertices {
        union.update_nodes(i);
    }
    sum
}

fn parse_str(str: &str) -> Vec<Edge> {
    str.lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.split(',')
                .enumerate()
                .filter(|(_, c)| *c != "-")
                .map(move |(j, n)| Edge::new(i, j, n.parse::<u32>().unwrap()))
        })
        .collect()
}

#[derive(Debug)]
struct Edge {
    a: usize,
    b: usize,
    weight: u32,
}

impl Edge {
    fn new(a: usize, b: usize, weight: u32) -> Self {
        Self { a, b, weight }
    }
}

#[derive(Debug)]
struct UnionFind {
    p: Vec<usize>,
}
impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            p: (0..n).collect(),
        }
    }
    fn update_nodes(&mut self, mut i: usize) {
        let mut stack = Vec::new();
        while self.p[i] != i {
            stack.push(i);
            i = self.p[i];
        }
        while let Some(j) = stack.pop() {
            self.p[j] = i;
        }
    }
    pub fn connect(&mut self, x: usize, y: usize) {
        let root = self.p[x];
        self.p[root] = y;
    }
    pub fn same_set(&mut self, x: usize, y: usize) -> bool {
        self.update_nodes(x);
        self.update_nodes(y);
        self.p[x] == self.p[y]
    }
}

#[cfg(test)]
mod tests {
    use crate::ones::seven::{UnionFind, parse_str, solve};

    #[test]
    fn test_union_find() {
        let mut find = UnionFind::new(5);
        find.connect(1, 2);
        assert!(find.same_set(1, 2));
        assert!(find.same_set(2, 1));
        assert!(!find.same_set(1, 3));
        find.connect(2, 3);
        assert!(find.same_set(1, 3));
    }
    const TEST: &str = "-,16,12,21,-,-,-
16,-,-,17,20,-,-
12,-,-,28,-,31,-
21,17,28,-,18,19,23
-,20,-,18,-,-,11
-,-,31,19,-,-,27
-,-,-,23,11,27,-";
    #[test]
    fn test_solve() {
        let mut v = parse_str(TEST);
        assert_eq!(solve(&mut v), 93);
    }
}
