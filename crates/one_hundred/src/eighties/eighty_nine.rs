use std::{fs, u32};

const FILE: &str = "Files/89.txt";
pub fn run() {
    let s = fs::read_to_string(FILE).unwrap();
    println!("{}", solve(&s.lines().map(|s| s.to_string()).collect()));
}

fn solve(v: &Vec<String>) -> u32 {
    v.iter().map(|s| saved(s)).sum()
}

fn saved(s: &str) -> u32 {
    let len = s.len() as u32;
    let num = to_num(s);
    s.len() as u32 - count_chars(to_num(s))
}

fn count_chars(mut n: u32) -> u32 {
    let mut count = n / 1000;
    let mut str = String::new();
    for i in 0..(n/1000) {
        str.push('M');
    }
    n = n % 1000;
    if n / 900 == 1 {
        count += 2;
        str.push_str("CM");
        n %= 900;
    } else if n >= 500 {
        count += 1;
        str.push_str("D");
        n %= 500;
    } else if n >= 400 {
        count += 2;
        str.push_str("CD");
        n %= 400;
    }
    for i in 0..(n / 100) {
        str.push('C');
    }
    count += n / 100;
    n %= 100;
    if n / 90 == 1 {
        count += 2;
        str.push_str("XC");
        n %= 90;
    } else if n >= 50 {
        count += 1;
        str.push('L');
        n %= 50;
    } else if n >= 40 {
        str.push_str("XL");
        count += 2;
        n %= 20;
    }
    for _ in 0..(n/10) {
        str.push('X');
    }
    count += n / 10;
    n %= 10;
    if n / 9 == 1 {
        count += 2;
        str.push_str("IX");
        n %= 9;
    } else if n >= 5 {
        str.push('V');
        count += 1;
        n %= 5;
    } else if n >= 4 {
        str.push_str("IV");
        count += 2;
        n %= 4;
    }
    for i in 0..n {
        str.push('I');
    }
    count += n;
    count
}

fn to_num(s: &str) -> u32 {
    let mut bytes = s.bytes();
    let mut n = get_val(bytes.next().unwrap());
    let mut prev = n;
    for b in bytes {
        let get_val = get_val(b);
        if get_val > prev {
            n += get_val - 2 * prev;
        } else {
            n += get_val;
        }
        prev = get_val;
    }
    n
}

fn get_val(c: u8) -> u32 {
    match c {
        b'I' => 1,
        b'V' => 5,
        b'X' => 10,
        b'L' => 50,
        b'C' => 100,
        b'D' => 500,
        b'M' => 1000,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use crate::eighties::eighty_nine::{count_chars, saved, to_num};

    #[test]
    fn test_to_num() {
        assert_eq!(to_num("XIV"), 14);
        assert_eq!(to_num("XIIIIII"), 16);
        assert_eq!(to_num("XVI"), 16);
        assert_eq!(to_num("MMMMCCCLXXXXVII"), 4397);
        assert_eq!(to_num("MDCCCXII"), 1812);
    }
    #[test]
    fn test_saved() {
        assert_eq!(saved("XIIIIII"), 4);
    }
    #[test]
    fn test_count_chars() {
        assert_eq!(count_chars(16), 3);
        assert_eq!(count_chars(19), 3);
        assert_eq!(count_chars(4397), 12);
        assert_eq!(count_chars(1812), 8);
    }
}
