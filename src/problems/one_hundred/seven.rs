use super::five::is_prime;

pub fn seven() {
    const NTH: u64 = 10001;
    let a = get_nth_prime(&NTH);
    println!("{a}");
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
