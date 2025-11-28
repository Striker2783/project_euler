pub fn run() {
    println!("{}", solve());
}

fn solve() -> u32 {
    for (i, mut n) in AnotherFibonacci::new().enumerate() {
        if !is_pandigital(n.1 % 10u64.pow(9)) {
            continue;
        }
        while n.0 >= 10u64.pow(9) {
            n.0 /= 10;
        }
        if !is_pandigital(n.0) {
            continue;
        }
        return i as u32 + 1;
    }
    unreachable!()
}

fn is_pandigital(mut n: u64) -> bool {
    let mut set = [false; 9];
    while n > 0 {
        let digit = n % 10;
        if digit != 0 {
            set[digit as usize - 1] = true;
        }
        n /= 10;
    }
    set.iter().all(|b| *b)
}

struct AnotherFibonacci {
    prev_low: u64,
    prev_high: u64,
    curr_low: u64,
    curr_high: u64,
}

impl AnotherFibonacci {
    fn new() -> Self {
        Self {
            prev_low: 0,
            prev_high: 0,
            curr_low: 1,
            curr_high: 1,
        }
    }
}
impl Iterator for AnotherFibonacci {
    type Item = (u64, u64);

    fn next(&mut self) -> Option<Self::Item> {
        let (curr_low, curr_high) = (self.curr_low, self.curr_high);

        let new_low = self.prev_low + self.curr_low % 10u64.pow(12);
        self.prev_low = self.curr_low;
        self.curr_low = new_low;

        let new_high = self.prev_high.checked_add(self.curr_high);
        if let Some(new_high) = new_high {
            self.prev_high = self.curr_high;
            self.curr_high = new_high;
        } else {
            let new_high = self.prev_high / 10 + self.curr_high / 10;
            self.prev_high = self.curr_high / 10;
            self.curr_high = new_high;
        }
        Some((curr_high, curr_low))
    }
}

#[cfg(test)]
mod tests {
    use crate::tens::four::{AnotherFibonacci, is_pandigital};

    #[test]
    fn test_fibonacci() {
        assert_eq!(AnotherFibonacci::new().next().unwrap(), (1, 1));
        assert_eq!(AnotherFibonacci::new().nth(1).unwrap(), (1, 1));
        assert_eq!(AnotherFibonacci::new().nth(2).unwrap(), (2, 2));

        assert!(is_pandigital(
            AnotherFibonacci::new().nth(540).unwrap().1 % 10u64.pow(9)
        ));
        assert!(is_pandigital(AnotherFibonacci::new().nth(2748).unwrap().0));
    }
    #[test]
    fn test_is_pandigital() {
        assert!(is_pandigital(123456789));
        assert!(!is_pandigital(12345679));
    }
}
