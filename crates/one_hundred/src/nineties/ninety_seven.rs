pub fn run() {
    println!("{}", solve());
}
fn solve() -> u128 {
    let mut res = 28433;
    let mut pow = 7_830_457;
    let mut base = 2;
    while pow > 0 {
        if pow & 1 == 1 {
            res = (res * base) % 10_000_000_000;
        }
        pow >>= 1;
        base = (base * base) % 10_000_000_000;
    }
    (res + 1) % 10_000_000_000
}
