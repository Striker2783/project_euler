use std::{
    error::Error,
    fs::File,
    io::{self, BufRead},
    path::{Iter, Path},
};

fn read_lines<P: AsRef<Path>>(filename: P) -> io::Lines<io::BufReader<File>> {
    let value = File::open(filename).expect("No File");
    io::BufReader::new(value).lines()
}

pub fn read_txt_into_separate_nums<P: AsRef<Path>>(filename: P) -> Vec<Vec<u64>> {
    let read_lines = read_lines(filename);

    read_lines
        .map(|x| {
            x.expect("Failed")
                .split(" ")
                .map(|x| x.parse::<u64>().expect("Failed"))
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>()
}

pub fn read_txt_into_nums<P: AsRef<Path>>(filename: P) -> impl Iterator<Item = u32> {
    let read_lines = read_lines(filename);

    read_lines.flat_map(|x| {
        let Ok(a) = x else {
            panic!();
        };
        a.chars()
            .into_iter()
            .map(|b| {
                let Some(x) = b.to_digit(10) else {
                panic!()
            };
                x
            })
            .collect::<Vec<u32>>()
    })
}
