pub fn run() {
    println!("{}", solve(1_000_000_000_000));
}
/// From https://oeis.org/A011900 for blues
/// From https://oeis.org/A046090 for total
fn solve(min: u64) -> u64 {
    let mut prev = [1, 3];
    let mut prev2 = [1, 4];
    while prev2[1] <= min {
        let next = 6 * prev[1] - prev[0] - 2;
        let next2 = 6 * prev2[1] - prev2[0] - 2;
        prev[0] = prev[1];
        prev[1] = next;
        prev2[0] = prev2[1];
        prev2[1] = next2;
    }
    prev[1]
}
/// Brute force
fn print_upto(max: u64) {
    for n in 5..=max {
        for blue in (n / 2)..n {
            let num = blue * (blue - 1);
            let den = n * (n - 1);
            if den % num == 0 && den / num == 2 {
                println!("{blue}/{n}");
            }
        }
    }
}
