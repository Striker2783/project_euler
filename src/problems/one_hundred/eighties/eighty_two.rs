use std::{cmp::Reverse, collections::BinaryHeap, fs};

const FILE: &str = "Files/81.txt";
pub fn run() {
    let v: Vec<Vec<u32>> = fs::read_to_string(FILE)
        .unwrap()
        .lines()
        .map(|l| l.split(',').map(|s| s.parse().unwrap()).collect())
        .collect();
    println!("{}", solve(&v));
}

fn solve(v: &Vec<Vec<u32>>) -> u32 {
    (0..v.len()).map(|row| dijkstra(v, row)).min().unwrap()
}

fn dijkstra(v: &Vec<Vec<u32>>, row: usize) -> u32 {
    let mut pq = BinaryHeap::new();
    let mut distances = vec![vec![u32::MAX; v[0].len()]; v.len()];
    pq.push(Reverse((v[row][0], row, 0)));
    while let Some(Reverse((d, i, j))) = pq.pop() {
        if j == v[i].len() - 1 {
            return d;
        }
        if d >= distances[i][j] {
            continue;
        }
        distances[i][j] = d;
        if i + 1 < v.len() {
            pq.push(Reverse((d + v[i + 1][j], i + 1, j)));
        }
        if i > 0 {
            pq.push(Reverse((d + v[i - 1][j], i - 1, j)));
        }
        if j + 1 < v[i].len() {
            pq.push(Reverse((d + v[i][j + 1], i, j + 1)));
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use crate::problems::one_hundred::eighties::eighty_two::{dijkstra, solve};

    #[test]
    fn test_dijkstra() {
        assert_eq!(
            dijkstra(
                &vec![
                    vec![131, 673, 234, 103, 18],
                    vec![201, 96, 342, 965, 150],
                    vec![630, 803, 746, 422, 111],
                    vec![537, 699, 497, 121, 956],
                    vec![805, 732, 524, 37, 331]
                ],
                1
            ),
            994
        );
    }
    #[test]
    fn test_solve() {
        assert_eq!(
            solve(&vec![
                vec![131, 673, 234, 103, 18],
                vec![201, 96, 342, 965, 150],
                vec![630, 803, 746, 422, 111],
                vec![537, 699, 497, 121, 956],
                vec![805, 732, 524, 37, 331]
            ],),
            994
        );
    }
}
