const MAX: u32 = 1000;
pub fn run() {
    println!("{}", solve(MAX));
}

fn solve(max: u32) -> u32 {
    (3..=max)
        .map(|x| (x, get_solutions(x)))
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap()
        .0
}

fn get_solutions(p: u32) -> u32 {
    (3..=(p / 3))
        .map(|n| {
            (n..=(n + p / 3))
                .filter(|&m| {
                    let c_squared = m * m + n * n;
                    let c = (2..)
                        .take_while(|&x| x * x <= c_squared)
                        .find(|x| x * x == c_squared);
                    c.is_some() && n + m + c.unwrap() == p
                })
                .count() as u32
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::problems::one_hundred::thirty_nine::get_solutions;

    #[test]
    fn t_get_solutions() {
        assert_eq!(get_solutions(120), 3);
    }
}
