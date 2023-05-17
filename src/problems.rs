#![allow(dead_code, unused)]

use std::ops::Range;
#[path = "../src/file_parsers.rs"]
mod file_parsers;
#[path = "../src/utilities.rs"]
mod utilities;

pub fn one() {
    const MAX: i32 = 1000;
    let sum: i32 = (0..)
        .take_while(|x| x < &MAX)
        .filter(|x| x % 5 == 0 || x % 3 == 0)
        .sum();
    println!("{sum}")
}
pub fn two() {
    const MAX: u64 = 4_000_000;
    let test = utilities::Fibonacci::new();
    let sum: u64 = test
        .into_iter()
        .take_while(|x| x < &MAX)
        .filter(|x| x % 2 == 0)
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
pub fn four() {
    const MIN: u64 = 100;
    const MAX: u64 = 1000;
    let map1 = MIN..MAX;
    let largest = map1
        .flat_map(|i| (i..MAX).map(move |j| i * j))
        .filter(|x| utilities::is_palindromic(x))
        .max();
    if let Some(a) = largest {
        println!("{}", a);
    }
}
pub fn five() {
    const MAX: u64 = 20;
    let primes = utilities::get_primes_bad_method(&(MAX + 1));
    let product: u64 = primes
        .map(|x| (1..).map(|a| (x).pow(a)).take_while(|a| *a <= MAX).last())
        .map(|x| match x {
            None => 1,
            Some(a) => a,
        })
        .product();
    println!("{product}")
}
pub fn curr() {}
