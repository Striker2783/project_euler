use crate::tens::fourteen::counting_blocks;

pub fn run() {
    println!("{}", solve(1_000_000, 50));
}

fn solve(min: u64, m: usize) -> u32 {
    for n in 1.. {
        if counting_blocks(n, m) > min {
            return n as u32;
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use crate::tens::fifteen::solve;

    #[test]
    fn test_solve() {
        assert_eq!(solve(1_000_000, 10), 57);
    }
}
