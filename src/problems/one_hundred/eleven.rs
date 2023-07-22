use std::{error::Error, fs, ops::Deref, path::Path};

pub fn run() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("Files/eleven.txt")?;
    let main = Main::parse(&contents)?;
    let max = main.find_max_adjacent(4);
    println!("{max}");
    Ok(())
}
#[derive(Default, Debug)]
struct Main {
    nums: Vec<Vec<u8>>,
}
impl Main {
    fn find_max_adjacent(&self, max: u32) -> u64 {
        self.max_horizontal_adjacent(max)
            .max(self.max_vertical_adjacent(max))
            .max(self.max_diagonal_adjacent(max))
    }
    fn max_vertical_adjacent(&self, max: u32) -> u64 {
        let max = max as usize;
        let mut rows = &self.nums;
        let columns = self.nums[0].len();
        let mut max_n = 0;
        for i in 0..=(rows.len() - max) {
            for j in 0..columns {
                let column: Vec<_> = rows.iter().skip(i).take(max).map(|v| v[j]).collect();
                let product = column.iter().fold(1u64, |acc, n| acc * *n as u64);
                max_n = max_n.max(product)
            }
        }
        max_n
    }
    fn max_horizontal_adjacent(&self, max: u32) -> u64 {
        let mut rows = &self.nums;
        rows.iter()
            .map(|row| {
                let selected = 0..=(row.len() - max as usize);
                selected
                    .map(|i| {
                        let nums = &row[i..(i + max as usize)];
                        let product = nums.iter().fold(1u64, |acc, &x| acc * x as u64);
                        product
                    })
                    .max()
                    .expect("")
            })
            .max()
            .expect("")
    }
    fn max_diagonal_adjacent(&self, max: u32) -> u64 {
        let max = max as usize;
        let mut rows = &self.nums;
        let columns = rows[0].len();
        let max_n = 0;
        for i in 0..=(rows.len() - max) {
            for j in 0..=(columns - max) {
                let selected: Vec<_> = (i..(i + max)).map(|i| rows[i + j][i]).collect();
                println!("{selected:?}")
            }
        }
        todo!();
    }
    fn parse(contents: &str) -> Result<Self, Box<dyn Error>> {
        Ok(Main {
            nums: contents
                .lines()
                .map(|line| {
                    line.split(' ')
                        .filter_map(|num| match num.parse::<u8>() {
                            Ok(n) => Some(n),
                            Err(_) => None,
                        })
                        .collect()
                })
                .collect(),
        })
    }
}

#[cfg(test)]
mod test {
    use std::fs;

    use super::Main;

    #[test]
    fn find_max_adjacent() {
        let contents = fs::read_to_string("Files/eleven.txt").expect("msg");
        let main = Main::parse(&contents).expect("");
        assert_eq!(main.find_max_adjacent(4), 70600674)
    }
    #[test]
    fn find_max_horizontal() {
        let contents = "54 64 2\n0 99 99";
        let main = Main::parse(contents).expect("");
        assert_eq!(main.max_horizontal_adjacent(2), 99 * 99);
    }
    #[test]
    fn find_max_vertical() {
        let contents = "23 64\n99 20\n99 10";
        let main = Main::parse(contents).expect("");
        assert_eq!(main.max_vertical_adjacent(2), 99 * 99);
    }
}
