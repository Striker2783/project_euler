pub fn run() {
    let res = solve(12);
    println!("{res}");
}

fn all_after(a: u32, b: u32) -> bool {
    let (mut a,mut b) = (a.min(b), b.max(a));
    let mut count = 0;
    while a > 0 && b > 0 {
        if b & 1 == 1 {
            count -= 1;
        }
        if a & 1 == 1 {
            count += 1;
        }
        if count < 0 {
            return false;
        }
        a >>= 1;
        b >>= 1;
    }
    true
}

fn solve(n: u32) -> u32 {
    let mut sum = 0;
    for i in 2..=n.div_ceil(2) {
        let mut stuff = Vec::new();
        combinations_bit(n as usize, i as usize, |n| {
            stuff.push(n);
        });
        for (j, &n1) in stuff.iter().enumerate() {
            for &n2 in stuff.iter().skip(j + 1) {
                if n1 & n2 != 0 {
                    continue;
                }
                if !all_after(n1, n2) {
                    sum += 1;
                }
            }
        }
    }
    sum
}

fn combinations_bit(n: usize, r: usize, mut f: impl FnMut(u32)) {
    fn combinations_bit_helper(n: usize, r: usize, value: u32, f: &mut impl FnMut(u32)) {
        if n == 0 {
            if r == 0 {
                f(value);
            }
            return;
        }
        if r > 0 {
            combinations_bit_helper(n - 1, r - 1, (value << 1) + 1, f);
        }
        combinations_bit_helper(n - 1, r, value << 1, f);
    }
    combinations_bit_helper(n, r, 0, &mut f);
}

#[cfg(test)]
mod tests {
    use crate::tens::six::solve;

    #[test]
    fn test_solve() {
        assert_eq!(solve(4), 1);
        assert_eq!(solve(7), 70);
    }
}
