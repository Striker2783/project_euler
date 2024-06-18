pub fn one() {
    const MAX: i32 = 1000;
    let sum: i32 = (0..)
        .take_while(|x| x < &MAX)
        .filter(|x| x % 5 == 0 || x % 3 == 0)
        .sum();
    println!("{sum}")
}

pub fn three() {
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

pub fn six() {
    const MAX: u64 = 100;
    let sum_of_squares: u64 = (1..MAX + 1).map(|x| x * x).sum();
    let square_of_sum = (1..MAX + 1).sum::<u64>().pow(2);
    let diff = square_of_sum - sum_of_squares;
    println!("{diff}");
}
