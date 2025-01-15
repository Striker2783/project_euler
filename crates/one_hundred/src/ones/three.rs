pub fn run() {
    const NUM: u64 = 600851475143;
    let mut largest = 0;
    let mut n = NUM;
    let mut i = 2;
    while n > 1 {
        if n % i == 0 {
            n /= i;
            if i > largest {
                largest = i;
            }
            i = 2;
            continue;
        }
        i += 1;
    }
    println!("{largest}");
}
