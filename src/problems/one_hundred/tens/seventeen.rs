pub fn run() {
    let total: u32 = (1..=1000).map(get_num_letters).sum();
    println!("{total}");
}

const ONES: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
const TEENS: [&str; 10] = [
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];
const TENS: [&str; 8] = [
    "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];
fn get_number_word(mut n: u32) -> String {
    let mut str = String::new();
    if (n / 1000 > 0) {
        str += &get_number_word(n / 1000);
        str += " thousand";
        n %= 1000;
        if (n > 0) {
            str += " ";
        }
    }
    if (n / 100 > 0) {
        str += &get_number_word(n / 100);
        str += " hundred";
        n %= 100;
        if (n > 0) {
            str += " and ";
        }
    }
    match (n / 10).cmp(&1) {
        std::cmp::Ordering::Less => (),
        std::cmp::Ordering::Equal => {
            str += TEENS[(n % 10) as usize];
            n = 0;
        }
        std::cmp::Ordering::Greater => {
            str += TENS[(n / 10 - 2) as usize];
            n %= 10;
            if n > 0 {
                str += "-";
            }
        }
    }
    if str.is_empty() || n > 0 {
        str += ONES[(n % 10) as usize];
    }
    str
}

fn get_num_letters(n: u32) -> u32 {
    let str = get_number_word(n);
    str.chars().filter(|&c| c.is_alphabetic()).count() as u32
}

#[cfg(test)]
mod test {
    use crate::problems::one_hundred::tens::seventeen::{get_num_letters, get_number_word};

    #[test]
    fn test_get_number_word() {
        assert_eq!(get_number_word(1), "one");
        assert_eq!(get_number_word(0), "zero");
        assert_eq!(get_number_word(342), "three hundred and forty-two");
        assert_eq!(get_number_word(115), "one hundred and fifteen");
        assert_eq!(get_number_word(1000), "one thousand");
    }
    #[test]
    fn test_get_num_letters() {
        assert_eq!(get_num_letters(1), 3);
        assert_eq!(get_num_letters(342), 23);
        assert_eq!(get_num_letters(115), 20);
        assert_eq!((1..=1000).map(get_num_letters).sum::<u32>(), 21124);
    }
}
