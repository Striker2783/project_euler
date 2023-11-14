pub fn four() {
    const MIN: u64 = 100;
    const MAX: u64 = 1000;
    let map1 = MIN..MAX;
    let largest = map1
        .flat_map(|i| (i..MAX).map(move |j| i * j))
        .filter(is_palindromic)
        .max();
    if let Some(a) = largest {
        println!("{}", a);
    }
}

pub fn is_palindromic(x: &u64) -> bool {
    let reverse: Vec<char> = x.to_string().chars().rev().collect();
    let curr: Vec<char> = x.to_string().chars().collect();
    reverse == curr
}
