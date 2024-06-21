pub fn six() {
    const MAX: u64 = 100;
    let sum_of_squares: u64 = (1..MAX + 1).map(|x| x * x).sum();
    let square_of_sum = (1..MAX + 1).sum::<u64>().pow(2);
    let diff = square_of_sum - sum_of_squares;
    println!("{diff}");
}
