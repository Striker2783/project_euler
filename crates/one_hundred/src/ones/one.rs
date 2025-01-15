pub fn run() {
    const MAX: i32 = 1000;
    let sum: i32 = (0..)
        .take_while(|x| x < &MAX)
        .filter(|x| x % 5 == 0 || x % 3 == 0)
        .sum();
    println!("{sum}")
}
