#![allow(dead_code, unused)]

use std::{char::MAX, ops::Range};
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
pub fn six() {
    const MAX: u64 = 100;
    let sum_of_squares: u64 = (1..MAX + 1).map(|x| x * x).sum();
    let square_of_sum = (1..MAX + 1).sum::<u64>().pow(2);
    println!("{sum_of_squares}, {square_of_sum}");
    let diff = square_of_sum - sum_of_squares;
    println!("{diff}");
}

pub fn seven() {
    const NTH: u64 = 10001;
    let a = utilities::get_nth_prime(&NTH);
    println!("{a}");
}

pub fn eight() {
    const MAX_ADJACENT: usize = 13;
    let test = file_parsers::read_txt_into_nums("Files\\eight.txt");
    let closure = |x: Vec<u32>| -> u64 {
        let mut largest: u64 = 0;
        for (i, v) in x.iter().enumerate() {
            if *v == 0 {
                continue;
            }
            let mut product = *v as u64;
            let range = (i + 1)..(MAX_ADJACENT + i);
            for j in range {
                let Some(n) = x.get(j) else {
                    break;
                };
                if *n == 0 {
                    break;
                }
                product *= *n as u64;
            }
            if product > largest {
                largest = product;
            }
        }
        return largest;
    };
    let t: u64 = closure(test.collect::<Vec<u32>>());
    println!("{t}");
}

pub fn nine() {
    let mut triplets = utilities::pythagoreon_triples(&500);
    let max = triplets.filter(|x| x.0 + x.1 + x.2 == 1000).last();
    if let Some(a) = max {
        println!("{}", a.0 * a.1 * a.2);
    }
}
