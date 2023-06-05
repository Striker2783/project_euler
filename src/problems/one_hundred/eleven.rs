use std::{error::Error, fs, ops::Deref, path::Path};

pub fn run() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("Files/eleven.txt")?;
    let main = Main::parse(&contents)?;
    let max = main.find_max_adjacent(&4);
    println!("{max}");
    Ok(())
}
#[derive(Default, Debug)]
struct Main {
    nums: Vec<Vec<u8>>,
}
impl Main {
    fn find_max_adjacent(&self, &max: &u32) -> u64 {
        *vec![
            self.max_horizontal_adjacent(&max),
            self.max_vertical_adjacent(&max),
        ]
        .iter()
        .max()
        .expect("")
    }
    fn max_vertical_adjacent(&self, &max: &u32) -> u64 {
        let mut rows = &self.nums;
        let columns = self.nums[0].len();
        for i in 0..columns {
            //
        }

        todo!()
    }
    fn max_horizontal_adjacent(&self, &max: &u32) -> u64 {
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
    fn parse(contents: &str) -> Result<Self, Box<dyn Error>> {
        let mut main = Main::default();

        main.nums = contents
            .lines()
            .map(|line| {
                line.split(' ')
                    .filter_map(|num| match num.parse::<u8>() {
                        Ok(n) => Some(n),
                        Err(_) => None,
                    })
                    .collect()
            })
            .collect();

        Ok(main)
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
        assert_eq!(main.find_max_adjacent(&4), 70600674)
    }
}
