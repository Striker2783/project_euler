pub fn run() {
    const MAX: u64 = 4_000_000;
    let test = Fibonacci::default();
    let sum: u64 = test
        .into_iter()
        .take_while(|x| x < &MAX)
        .filter(|x| x % 2 == 0)
        .sum();
    println!("{sum}")
}
pub struct Fibonacci {
    next: u64,
    curr: u64,
}

impl Default for Fibonacci {
    fn default() -> Self {
        Fibonacci { next: 1, curr: 0 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr.checked_add(self.next);

        if let Some(n) = new_next {
            self.curr = self.next;
            self.next = n;

            return Some(self.curr);
        }
        None
    }
}
