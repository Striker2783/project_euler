pub fn run() {
    println!("{}", solve_2(10_000_000));
}
// Brute Force
fn solve(n: u32) -> u32 {
    (2..n)
        .filter(|&n| {
            let mut curr = n;
            while curr != 1 && curr != 89 {
                curr = next(curr);
            }
            curr == 89
        })
        .count() as u32
}
// Some DP but worse memory usage
fn solve_2(n: u32) -> u32 {
    let mut v = vec![0; n as usize];
    v[1] = 1;
    if let Some(n) = v.get_mut(89) {
        *n = 89;
    }
    let mut count = 0;
    let mut stack = vec![];
    for n in 1..n {
        let mut curr = n as usize;
        while v[curr] == 0 {
            stack.push(curr);
            curr = next(curr as u32) as usize;
        }
        if v[curr] == 89 {
            count += 1;
        }
        while let Some(idx) = stack.pop() {
            v[idx] = v[curr];
        }
    }
    count
}

fn next(mut n: u32) -> u32 {
    let mut res = 0;
    while n > 0 {
        res += (n % 10) * (n % 10);
        n /= 10;
    }
    res
}
#[cfg(test)]
mod tests {
    use crate::nineties::ninety_two::next;

    #[test]
    fn test_next() {
        assert_eq!(next(44), 32);
    }
}
