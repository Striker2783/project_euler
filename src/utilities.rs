pub struct Fibonacci {
    next: u64,
    curr: u64,
}

impl Fibonacci {
    pub fn new() -> Self {
        return Fibonacci { next: 1, curr: 0 };
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
        return None;
    }
}

pub fn is_palindromic(x: &u64) -> bool {
    let reverse: Vec<char> = x.to_string().chars().rev().collect();
    let curr: Vec<char> = x.to_string().chars().collect();
    return reverse == curr;
}

fn is_prime(n: &u64) -> bool {
    let mut n = *n;
    if n <= 1 {
        return false;
    }
    for a in 2..(((n as f64).sqrt() as u64) + 1) {
        if n % a == 0 {
            return false;
        }
    }
    true
}
pub fn get_primes_bad_method<'a>(upper: &'a u64) -> impl Iterator<Item = u64> + 'a {
    return (2..).take_while(|x| x < upper).filter(|x| is_prime(x));
}
pub fn get_nth_prime(nth: &u64) -> u64 {
    if *nth == 1 {
        return 2;
    }
    let mut n = 0;
    let nth_prime = (3..)
        .step_by(2)
        .filter(|x| is_prime(x))
        .map(|x| {
            n += 1;
            (x, n)
        })
        .take_while(|a| a.1 < *nth)
        .last();
    if let Some(a) = nth_prime {
        return a.0;
    } else {
        unreachable!();
    }
}
