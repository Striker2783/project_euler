use std::collections::HashMap;

pub fn run() {
    let solution = solve(200);
    println!("{solution}");
}

const VALUES: [u32; 8] = [1, 2, 5, 10, 20, 50, 100, 200];
fn solve(target: u32) -> u32 {
    let mut vec = vec![vec![1; 8]; (target + 1) as usize];
    for i in 0..=(target as usize) {
        for j in 1..8 {
            vec[i][j] = vec[i][j - 1];
            if VALUES[j] <= i as u32 {
                vec[i][j] += vec[i - VALUES[j] as usize][j];
            }
        }
    }
    *vec.last().unwrap().last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test() {
        assert_eq!(solve(1), 1);
        assert_eq!(solve(2), 2);
        assert_eq!(solve(200), 73682);
    }
}
