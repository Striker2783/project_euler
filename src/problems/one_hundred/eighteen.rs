use std::{error::Error, fs};

pub fn run() {
    let mut pyramid = Pyramid::parse(&fs::read_to_string("Files/eighteen.txt").unwrap());
    println!("{}", pyramid.max_path());
}

struct Pyramid(Vec<Vec<u32>>);
impl Pyramid {
    pub fn new(vec: Vec<Vec<u32>>) -> Self {
        Self(vec)
    }
    pub fn parse(str: &str) -> Self {
        let mut vec = vec![];
        for line in str.lines() {
            let mut row = vec![];
            for num in line.split(' ') {
                let n = num.parse::<u32>().unwrap();
                row.push(n);
            }
            vec.push(row);
        }
        Self::new(vec)
    }
    fn max_path_iter(&mut self) -> Result<(), Box<dyn Error>> {
        let mut new_vec = vec![];
        for (i, &v) in self.0[self.0.len() - 2].iter().enumerate() {
            let last_row = self.0.last().unwrap();
            let max = last_row[i].max(last_row[i + 1]) + v;
            new_vec.push(max);
        }
        self.0.pop();
        self.0.pop();
        self.0.push(new_vec);
        Ok(())
    }
    pub fn get_top(&self) -> Result<u32, Box<dyn Error>> {
        Ok(*self.0.first().ok_or("")?.first().ok_or("")?)
    }
    pub fn max_path(&mut self) -> u32 {
        while self.0.len() > 1 {
            self.max_path_iter();
        }
        self.get_top().unwrap()
    }
}

#[cfg(test)]
mod test {
    use std::fs;

    use crate::problems::one_hundred::eighteen::Pyramid;
    #[test]
    fn test_parse() {
        let str = "75\n95 64";
        assert_eq!(Pyramid::parse(str).0, vec![vec![75], vec![95, 64]]);
    }
    #[test]
    fn test_max_path_iter() {
        let str = "75\n95 64";
        let mut pyramid = Pyramid::parse(str);
        pyramid.max_path_iter();
        assert_eq!(pyramid.0, vec![vec![75 + 95]]);
    }
    #[test]
    fn test_max_path() {
        let str = "3\n7 4\n2 4 6\n8 5 9 3";
        let mut pyramid = Pyramid::parse(str);
        assert_eq!(pyramid.max_path(), 23);
        pyramid = Pyramid::parse(&fs::read_to_string("Files/eighteen.txt").unwrap());
        assert_eq!(pyramid.max_path(), 1074);
    }
}
