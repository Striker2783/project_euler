#[derive(Default)]
struct Triangle {
    sum: u64,
    i: u64,
}

impl Iterator for Triangle {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.i += 1;
        self.sum += self.i;
        Some(self.sum)
    }
}
fn get_something(max_divisors: u32) -> u64 {
    let mut tri = Triangle::default();
    loop {
        let n = tri.next().unwrap();
        let mut divisors = 0;
        for i in (1..n).take_while(|&x| x * x <= n) {
            if n % i == 0 {
                if n / i == i {
                    divisors += 1
                } else {
                    divisors += 2;
                }
            }
        }
        if divisors > max_divisors {
            return n;
        }
    }
}

pub fn run() {
    let a = get_something(500);
    println!("{a}");
}

#[cfg(test)]
mod test {
    use crate::problems::one_hundred::twelve::get_something;

    use super::Triangle;

    #[test]
    fn triangle() {
        let mut tri = Triangle::default();
        assert_eq!(tri.next(), Some(1));
        assert_eq!(tri.next(), Some(3));
        assert_eq!(tri.next(), Some(6));
        assert_eq!(tri.next(), Some(10));
        assert_eq!(tri.next(), Some(15));
    }
    #[test]
    fn run() {
        assert_eq!(get_something(5), 28)
    }
}
