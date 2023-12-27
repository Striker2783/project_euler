use std::{error::Error, fs};

pub fn run() {
    let contents = fs::read_to_string("Files/thirteen.txt").unwrap();
    let vec = parse(&contents, 11);
    let sum: u64 = vec.iter().sum();
    println!("{}", &sum.to_string()[..10]);
}

fn parse(contents: &str, precision: u32) -> Vec<u64> {
    let mut vec = vec![];

    for line in contents.lines() {
        let mut n = 0;
        let mut chars = line.chars();
        for _ in 0..precision {
            n *= 10;
            n += chars.next().unwrap().to_digit(10).unwrap() as u64;
        }
        vec.push(n);
    }

    vec
}

#[cfg(test)]
mod test {
    use crate::problems::one_hundred::thirteen::parse;

    #[test]
    fn test_parse() {
        let str = "561256\n61234523";
        assert_eq!(parse(str, 4), vec![5612, 6123])
    }
}
