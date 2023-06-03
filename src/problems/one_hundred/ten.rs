use crate::problems::{file_parsers, utilities};

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
pub fn ten() {
    let a = utilities::sieve_of_eratosthenes(&2_000_000);
    let sum: u64 = a.iter().sum();
    println!("{sum}");
}
