use std::u32;

pub fn run() {
    println!("{}", solve(50));
}

fn solve(n: i32) -> u32 {
    let mut count = 0;
    for x1 in 0..=n {
        for x2 in 0..=n {
            for y1 in 0..=n {
                for y2 in 0..=n {
                    if !works((x1,y1), (x2,y2)) {
                        continue;
                    }
                    count += 1;
                }
            }
        }
    }
    count / 2
}

#[derive(Clone, Copy)]
struct Fraction(i32, i32);

type Pos = (i32, i32);
fn works(a: Pos, b: Pos) -> bool {
    if a == b || a == (0, 0) || b == (0, 0) {
        return false;
    }
    let a0 = a.0 * a.0 + a.1 * a.1;
    let b0 = b.0 * b.0 + b.1 * b.1;
    let ab = (a.0 - b.0) * (a.0 - b.0) + (a.1 - b.1) * (a.1 - b.1);
    let hypotenuse = a0.max(b0).max(ab);
    if hypotenuse == a0 {
        b0 + ab == a0
    } else if hypotenuse == b0 {
        a0 + ab == b0
    } else {
        a0 + b0 == ab
    }
}
#[cfg(test)]
mod tests {
    use crate::nineties::ninety_one::{solve, works};

    #[test]
    fn test_works() {
        assert!(works((1, 0), (0, 1)));
    }
    #[test]
    fn test_solve() {
        assert_eq!(solve(2), 14);
    }
}
