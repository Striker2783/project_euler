use common::shape_numbers::Triangular;

fn get_something(max_divisors: u32) -> u32 {
    let mut tri = Triangular::default();
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
    use super::get_something;
    #[test]
    fn run() {
        assert_eq!(get_something(5), 28)
    }
}
