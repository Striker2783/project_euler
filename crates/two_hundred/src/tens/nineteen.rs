use std::collections::BinaryHeap;

pub fn run() {
    println!("{}", solve(30));
}

fn sum_digits(mut n: u64) -> u64 {
    let mut sum = 0;
    while n > 0 {
        sum += n % 10;
        n /= 10;
    }
    sum
}

fn solve(max: usize) -> u64 {
    let mut heap = BinaryHeap::new();
    for base in 2u64..1000 {
        for pow in 2.. {
            if let Some(n) = base.checked_pow(pow) {
                if sum_digits(n) != base {
                    continue;
                }
                if heap.len() < max {
                    heap.push(n);
                    continue;
                }
                if *heap.peek().unwrap() > n {
                    heap.pop();
                    heap.push(n);
                }
            } else {
                break;
            }
        }
    }
    *heap.peek().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::tens::nineteen::solve;

    #[test]
    fn test_solve() {
        assert_eq!(solve(2), 512);
        assert_eq!(solve(10), 614656);
    }
}
