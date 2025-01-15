use std::fs;

pub fn run() {
    let s = fs::read_to_string(FILE).unwrap();
    println!("{}", solve(&s));
}
const FILE: &str = "Files/96.txt";
fn solve(s: &str) -> u32 {
    let mut sum = 0;
    let mut split = s.split('\n');
    while let Some(_) = split.next() {
        let mut s = String::new();
        for i in 0..9 {
            s.push_str(split.next().unwrap());
            s.push('\n');
        }
        let mut board = Board::from(s.as_str());
        board.solve();
        sum += 100 * board.board[0] as u32 + 10 * board.board[1] as u32 + board.board[2] as u32;
    }
    sum
}
#[derive(Debug)]
struct Board {
    board: Box<[u8; 81]>,
}

impl Board {
    fn solve(&mut self) {
        fn helper(board: &mut Board, mut i: usize) -> bool {
            while i < 81 && board.board[i] != 0 {
                i += 1;
            }
            if i >= 81 {
                return true;
            }
            let availability = board.availability(i);
            for (j, &b) in availability.iter().enumerate() {
                if !b {
                    continue;
                }
                board.board[i] = (j + 1) as u8;
                if helper(board, i + 1) {
                    return true;
                }
                board.board[i] = 0;
            }
            false
        }
        helper(self, 0);
    }
    fn availability(&self, i: usize) -> [bool; 9] {
        let mut arr = [true; 9];
        for j in 0..9 {
            let curr = self.board[i % 9 + 9 * j];
            if curr == 0 {
                continue;
            }
            arr[curr as usize - 1] = false;
        }
        for j in 0..9 {
            let curr = self.board[i / 9 * 9 + j];
            if curr == 0 {
                continue;
            }
            arr[curr as usize - 1] = false;
        }
        for j in 0..3 {
            let j = (i % 9) / 3 * 3 + j;
            for k in 0..3 {
                let k = (i / 9) / 3 * 3 + k;
                let curr = self.board[j + 9 * k];
                if curr == 0 {
                    continue;
                }
                arr[curr as usize - 1] = false;
            }
        }
        arr
    }
}

impl From<&str> for Board {
    fn from(value: &str) -> Self {
        let mut arr = Box::new([0; 81]);
        for (i, line) in value.lines().enumerate() {
            for (j, b) in line.bytes().enumerate() {
                arr[i * 9 + j] = b - b'0'
            }
        }
        Self { board: arr }
    }
}
#[cfg(test)]
mod tests {
    use super::Board;

    const TEST: &str = "003020600
900305001
001806400
008102900
700000008
006708200
002609500
800203009
005010300";
    #[test]
    fn test_availability() {
        let board = Board::from(TEST);
        assert_eq!(
            board.availability(0),
            [false, false, false, true, true, false, false, false, false]
        );
        assert_eq!(
            board.availability(40),
            [false, false, true, true, true, true, false, false, true]
        );
    }
}
