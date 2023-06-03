pub fn five() {
    const MAX: u64 = 20;
    let primes = get_primes_bad_method(&(MAX + 1));
    let product: u64 = primes
        .map(|x| (1..).map(|a| (x).pow(a)).take_while(|a| *a <= MAX).last())
        .map(|x| match x {
            None => 1,
            Some(a) => a,
        })
        .product();
    println!("{product}")
}
pub fn get_primes_bad_method<'a>(upper: &'a u64) -> impl Iterator<Item = u64> + 'a {
    return (2..).take_while(|x| x < upper).filter(|x| is_prime(x));
}
pub fn is_prime(n: &u64) -> bool {
    let n = *n;
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
