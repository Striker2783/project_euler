use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

pub fn eight() {
    const MAX_ADJACENT: usize = 13;
    let test = read_txt_into_nums("Files\\eight.txt");
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
        largest
    };
    let t: u64 = closure(test.collect::<Vec<u32>>());
    println!("{t}");
}
pub fn read_txt_into_nums<P: AsRef<Path>>(filename: P) -> impl Iterator<Item = u32> {
    let read_lines = read_lines(filename);

    read_lines.flat_map(|x| {
        let Ok(a) = x else {
            panic!();
        };
        a.chars()
            .map(|b| {
                let Some(x) = b.to_digit(10) else { panic!() };
                x
            })
            .collect::<Vec<u32>>()
    })
}
pub fn read_lines<P: AsRef<Path>>(filename: P) -> io::Lines<io::BufReader<File>> {
    let value = File::open(filename).expect("No File");
    io::BufReader::new(value).lines()
}
