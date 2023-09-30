use std::collections::HashMap;

struct Collatz {
    n: u64,
}
impl Collatz {
    #[inline]
    pub fn new(n: u64) -> Self {
        Self { n }
    }
    #[inline]
    pub fn is_loop(&self) -> bool {
        self.n == 1
    }
    #[inline]
    pub fn get(&self) -> u64 {
        self.n
    }
}
impl Iterator for Collatz {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.n % 2 == 0 {
            self.n /= 2;
        } else {
            self.n = 3 * self.n + 1;
        }
        Some(self.n)
    }
}

fn longest_chain(max: u64) -> u64 {
    let (mut num, mut max_chain) = (0, 0u64);
    let mut map = HashMap::new();
    for n in 2..max {
        let length = get_chain_length(n, &mut map);
        if length > max_chain {
            max_chain = length;
            num = n;
        }
    }
    num
}

fn get_chain_length(n: u64, map: &mut HashMap<u64, u64>) -> u64 {
    let mut stack = vec![];
    let mut collatz = Collatz::new(n);
    let mut chain = 1;
    while (!collatz.is_loop()) {
        let curr = collatz.next().unwrap();
        if let Some(&ch) = map.get(&curr) {
            chain = ch;
            break;
        }
        stack.push(curr);
    }
    while let Some(n) = stack.pop() {
        chain += 1;
        map.insert(n, chain);
    }
    chain
}

pub fn run() {
    let x = longest_chain(1_000_000);
    println!("{x}");
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::problems::one_hundred::fourteen::get_chain_length;

    use super::Collatz;

    #[test]
    fn test_collatz() {
        let mut collatz = Collatz::new(13);
        assert_eq!(collatz.next(), Some(40));
        assert_eq!(collatz.next(), Some(20));
        assert_eq!(collatz.next(), Some(10));
        assert_eq!(collatz.next(), Some(5));
        assert_eq!(collatz.next(), Some(16));
        assert_eq!(collatz.next(), Some(8));
        assert_eq!(collatz.next(), Some(4));
        assert_eq!(collatz.next(), Some(2));
        assert_eq!(collatz.next(), Some(1));
    }
    #[test]
    fn test_get_chain_length() {
        let mut map = HashMap::new();
        assert_eq!(get_chain_length(13, &mut map), 10);
        assert_eq!(get_chain_length(40, &mut map), 9);
    }
}
