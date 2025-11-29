const ALL: &[u32] = &[
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 25, 2, 4, 6, 8, 10,
    12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 32, 34, 36, 38, 40, 50, 3, 6, 9, 12, 15, 18, 21, 24,
    27, 30, 33, 36, 39, 42, 45, 48, 51, 54, 57, 60,
];
const DOUBLES: &[u32] = &[
    2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 32, 34, 36, 38, 40, 50,
];

pub fn run() {
    let ans: u32 = (1..100).map(solve).sum();
    println!("{ans}");
}

fn solve(score: u32) -> u32 {
    let mut count = 0;
    for &n in DOUBLES {
        if n > score {
            continue;
        }
        let rem = score - n;
        for (i, &n1) in ALL.iter().enumerate() {
            for &n2 in ALL.iter().skip(i) {
                if n1 + n2 == rem {
                    count += 1;
                }
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use crate::ones::nine::solve;

    #[test]
    fn test_solve() {
        assert_eq!(solve(1), 0);
        assert_eq!(solve(2), 1);
        assert_eq!(solve(3), 1);
        assert_eq!(solve(6), 11);
    }
}
