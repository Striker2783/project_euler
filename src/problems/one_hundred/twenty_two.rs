use std::{error::Error, fs};

pub fn run() {
    let contents = fs::read_to_string("Files/twenty_two.txt").unwrap();
    let stuff = TwentyTwo::parse(&contents);
    let x = stuff.get_sum_name_scores();
    println!("{x}");
}

struct TwentyTwo {
    names: Vec<String>,
}
impl TwentyTwo {
    pub fn parse(contents: &str) -> Self {
        let mut vec = vec![];
        for name in contents.split(',') {
            let name = &name[1..(name.len() - 1)];
            vec.push(name.to_string());
        }
        vec.sort();
        Self { names: vec }
    }
    fn get_name_score(name: &str) -> u32 {
        let mut score = 0;
        for c in name.chars() {
            score += (c as u32) - ('A' as u32 - 1);
        }
        score
    }
    pub fn get_sum_name_scores(&self) -> u32 {
        let mut sum = 0;
        for (i, v) in self.names.iter().enumerate() {
            let i = i as u32;
            sum += (i + 1) * Self::get_name_score(v);
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::TwentyTwo;

    #[test]
    fn test_parse() {
        let s = "\"Mark\",\"Maria\"";
        assert_eq!(TwentyTwo::parse(s).names, vec!["Maria", "Mark"]);
    }
    #[test]
    fn test_name_score() {
        assert_eq!(TwentyTwo::get_name_score("COLIN"), 53);
    }
    #[test]
    fn test_sum_name_score() {
        let contents = fs::read_to_string("Files/twenty_two.txt").unwrap();
        let stuff = TwentyTwo::parse(&contents);
        assert_eq!(stuff.get_sum_name_scores(), 871198282)
    }
}
