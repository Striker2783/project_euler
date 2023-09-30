use std::{collections::VecDeque, error::Error, fs, ops::Deref, path::Path, vec};

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
    fn find_max_in_vec(vec: &[u8], max: usize) -> u64 {
        if vec.len() < max {
            return 0;
        }
        let mut queue = VecDeque::new();
        let mut mult = 0;
        vec.iter().for_each(|&x| {
            if queue.len() >= max {
                queue.pop_front();
            }
            queue.push_back(x);
            if queue.len() == max {
                mult = queue.iter().map(|x| *x as u64).product::<u64>().max(mult);
            }
        });
        mult
    }
    fn find_max_adjacent(&self, max: usize) -> u64 {
        self.max_horizontal_adjacent(max)
            .max(self.max_vertical_adjacent(max))
            .max(self.max_diagonal_row_adjacent(max))
            .max(self.max_diagonal_column_adjacent(max))
    }
    fn max_vertical_adjacent(&self, max: usize) -> u64 {
        let columns = self.nums[0].len();
        let mut max_n = 0;
        for column in 0..columns {
            let mut vec = vec![];
            for j in 0..self.nums.len() {
                vec.push(self.nums[j][column]);
            }
            max_n = Main::find_max_in_vec(&vec, max).max(max_n);
        }
        max_n
    }
    fn max_horizontal_adjacent(&self, max: usize) -> u64 {
        let mut max_n = 0;
        for row in &self.nums {
            max_n = Main::find_max_in_vec(row, max).max(max_n)
        }
        max_n
    }
    fn max_diagonal_row_adjacent(&self, max: usize) -> u64 {
        let rows = self.nums.len();
        let columns = self.nums[0].len();
        let mut max_n = 0;

        for row_offset in 0..rows {
            let mut right = vec![];
            let mut left = vec![];
            for i in 0..(rows - row_offset) {
                if i >= columns {
                    continue;
                }
                let row = i + row_offset;
                right.push(self.nums[row][i]);
                left.push(self.nums[rows - row - 1][i]);
            }
            max_n = Main::find_max_in_vec(&right, max).max(max_n);
            max_n = Main::find_max_in_vec(&left, max).max(max_n);
        }

        max_n
    }
    fn max_diagonal_column_adjacent(&self, max: usize) -> u64 {
        let rows = self.nums.len();
        let columns = self.nums[0].len();
        let mut max_n = 0;

        for column_offset in 0..columns {
            let mut top = vec![];
            let mut bottom = vec![];
            for i in 0..(columns - column_offset) {
                if i > rows {
                    continue;
                }
                let column = i + column_offset;
                top.push(self.nums[i][column]);
                bottom.push(self.nums[i][columns - column - 1]);
            }
            max_n = Main::find_max_in_vec(&top, max).max(max_n);
            max_n = Main::find_max_in_vec(&bottom, max).max(max_n);
        }

        max_n
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
    fn find_max_in_vec() {
        let vec = vec![52, 51, 2, 0, 3, 10, 2, 1];
        assert_eq!(Main::find_max_in_vec(&vec, 4), 3 * 10 * 2);
    }
    #[test]
    fn max_diagonal_row_adjacent() {
        let contents = "12 53\n24 23";
        let main = Main::parse(contents).expect("");
        assert_eq!(main.max_diagonal_row_adjacent(2), 53 * 24);
    }

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
