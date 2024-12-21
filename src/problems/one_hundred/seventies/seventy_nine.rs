use std::{
    collections::{HashMap, HashSet},
    fs,
};

const FILE: &str = "Files/79.txt";
pub fn run() {
    let v: Vec<Vec<_>> = fs::read_to_string(FILE)
        .unwrap()
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.bytes().collect())
        .collect();

    println!(
        "{}",
        solve(&v).into_iter().map(|b| b as char).collect::<String>()
    )
}

fn solve(v: &Vec<Vec<u8>>) -> Vec<u8> {
    let mut map = HashMap::new();
    for attempt in v {
        map.entry(attempt[0]).or_insert(HashSet::new());
        for i in 1..attempt.len() {
            map.entry(attempt[i])
                .and_modify(|set: &mut HashSet<u8>| {
                    set.insert(attempt[i - 1]);
                })
                .or_insert_with(|| {
                    let mut set = HashSet::new();
                    set.insert(attempt[i - 1]);
                    set
                });
        }
    }
    let mut v = vec![];
    while !map.is_empty() {
        let (&k, _) = map.iter().find(|(_, s)| s.is_empty()).unwrap();
        map.remove(&k).unwrap();
        v.push(k);
        for (_, set) in map.iter_mut() {
            set.remove(&k);
        }
    }
    v
}
