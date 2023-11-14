fn run() {
    let solution = solve(9);
    println!("{solution}");
}

fn solve(max_digits: u32) -> u32 {
    let mut sum = 0;
    sum
}

fn is_pandigital(n: u64, digits: u32) -> bool {
    let mut vec = vec![false; digits as usize];
    for i in 0..digits {
        let t = n / 10u64.pow(i);
        if vec[t as usize] {
            return false;
        }
        vec[t as usize] = true
    }
    true
}

#[cfg(test)]
mod tests {
    use crate::problems::one_hundred::thirty_two::is_pandigital;

    #[test]
    fn test_is_pandigital() {}
}
