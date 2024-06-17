use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader, Read},
};

struct WordIterator {
    value: String,
    i: usize,
}
impl WordIterator {
    pub fn new(file: &File) -> Self {
        let mut reader = BufReader::new(file);
        let mut buffer = String::new();
        reader.read_to_string(&mut buffer);
        Self {
            value: buffer,
            i: 0,
        }
    }
}
impl Iterator for WordIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.value.len() {
            return None;
        }
        let mut word = String::new();
        for i in (self.i + 1).. {
            if self.value.as_bytes()[i] == b'"' {
                break;
            }
            word.push(self.value.as_bytes()[i] as char);
        }
        self.i += 1 + word.len() + 2;
        Some(word)
    }
}

pub fn run() {
    let triangles = get_triangles(10000);
    let file = match File::open("Files/forty_two.txt") {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };
    let words = WordIterator::new(&file);
    let triangle_words_count = words
        .map(|word| {
            word.as_bytes()
                .iter()
                .map(|&b| b - b'A' + 1)
                .fold(0, |acc, n| acc + n as u32)
        })
        .filter(|n| triangles.contains(n))
        .count();
    println!("{triangle_words_count}");
}

fn get_triangles(max: u32) -> HashSet<u32> {
    (1..)
        .map_while(|n| {
            let number = n * (n + 1) / 2;
            if number <= max {
                Some(number)
            } else {
                None
            }
        })
        .collect()
}
