use std::{
    fs::{self},
    num::ParseIntError,
    path::Path,
};

pub fn run() {
    match solve(FILE) {
        Ok(a) => println!("{a}"),
        Err(e) => eprintln!("{e:?}"),
    }
}
const FILE: &str = "Files/fifty_nine.txt";
#[derive(Debug)]
enum SolveError {
    FileIO(std::io::Error),
    Parse(std::num::ParseIntError),
    NotFound,
}
fn to_bytes(v: String) -> Result<Vec<u8>, ParseIntError> {
    v.split(',').map(|s| s.parse::<u8>()).collect()
}
fn solve<T: AsRef<Path>>(f: T) -> Result<u32, SolveError> {
    let input = fs::read_to_string(f).map_err(SolveError::FileIO)?;
    let input = to_bytes(input).map_err(SolveError::Parse)?;
    'outer: for k in KeyGenerate::new(3) {
        let mut cycle = k.iter().cycle().cloned();
        let mut output = String::new();
        for b in input.iter().cloned() {
            output.push((b ^ cycle.next().unwrap()) as char);
        }
        // "the" is a commonly used word
        if !output.contains(" the ") {
            continue;
        }
        return Ok(output
            .as_bytes()
            .iter()
            .cloned()
            .fold(0u32, |acc, i| acc + i as u32));
    }
    Err(SolveError::NotFound)
}

struct KeyGenerate {
    size: usize,
    curr: u32,
}

impl KeyGenerate {
    fn new(n: usize) -> Self {
        Self { size: n, curr: 0 }
    }
    fn get_byte(n: u32) -> Option<u8> {
        if n >= 26 {
            return None;
        }
        Some((n as u8) + b'a')
    }
    fn get_bytes(&self) -> Option<Vec<u8>> {
        let mut curr = self.curr;
        let mut vec = Vec::new();
        for i in (0..self.size).rev() {
            vec.push(Self::get_byte(curr / 26u32.pow(i as u32))?);
            curr %= 26u32.pow(i as u32);
        }
        Some(vec)
    }
}

impl Iterator for KeyGenerate {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        let curr = self.get_bytes();
        self.curr += 1;
        curr
    }
}

#[cfg(test)]
mod tests {
    use super::KeyGenerate;

    #[test]
    fn test_key_generate() {
        let mut key = KeyGenerate::new(3);
        assert_eq!(key.get_bytes(), Some(vec![b'a', b'a', b'a']));
        key.curr = u32::MAX;
        assert_eq!(key.get_bytes(), None);
    }
}
