use std::{cmp::Ordering, fs, str::FromStr};

pub fn run() {
    let str = match fs::read_to_string("Files/fifty_four.txt") {
        Ok(a) => a,
        Err(e) => {
            eprintln!("{e}");
            return;
        }
    };
    let mut count = 0;
    for line in str.lines() {
        let hand1 = Hand::from_str(&line[0..14]).unwrap();
        let hand2 = Hand::from_str(&line[15..]).unwrap();
        if hand1 > hand2 {
            count += 1;
        }
    }
    println!("{count}");
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Rank {
    High(u8),
    Pair(u8),
    TwoPair(u8, u8),
    Three(u8),
    Straight(u8),
    Flush,
    Full(u8, u8),
    Four(u8),
    StraightFlush(u8),
}
#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let r = self.get_rank();
        let r_o = other.get_rank();
        if r != r_o {
            return r.cmp(&r_o);
        }
        for i in (0..5).rev() {
            let v = self.cards[i].value;
            let v_o = other.cards[i].value;
            if v != v_o {
                return v.cmp(&v_o);
            }
        }
        Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Hand {
    fn get_value_count(&self) -> Vec<(u8, u8)> {
        let mut count = [0; 13];
        for c in self.cards {
            count[(c.value - 2) as usize] += 1;
        }
        let mut count: Vec<_> = count
            .iter()
            .enumerate()
            .filter(|(_, b)| **b > 1)
            .map(|(a, b)| ((a + 2) as u8, *b))
            .collect();
        count.sort_by(|a, b| b.1.cmp(&a.1));
        count
    }
    fn is_straight(&self) -> bool {
        let past = self.cards[0].value;
        for i in 1..5 {
            if self.cards[i].value != past + i as u8 {
                return false;
            }
        }
        true
    }
    fn is_flush(&self) -> bool {
        let suit = self.cards[0].suit;
        for i in 1..5 {
            if self.cards[i].suit != suit {
                return false;
            }
        }
        true
    }
    pub fn get_rank(&self) -> Rank {
        let is_flush = self.is_flush();
        let is_straight = self.is_straight();
        if is_flush && is_straight {
            return Rank::StraightFlush(self.cards[0].value);
        }
        let counts = self.get_value_count();
        if !counts.is_empty() && counts[0].1 == 4 {
            return Rank::Four(counts[0].0);
        } else if counts.len() >= 2 && counts[0].1 == 3 && counts[1].1 == 2 {
            return Rank::Full(counts[0].0, counts[1].0);
        } else if is_flush {
            return Rank::Flush;
        } else if is_straight {
            return Rank::Straight(self.cards[0].value);
        } else if !counts.is_empty() && counts[0].1 == 3 {
            return Rank::Three(counts[0].0);
        } else if !counts.is_empty() && counts.len() >= 2 {
            return Rank::TwoPair(counts[0].0.max(counts[1].0), counts[0].0.min(counts[1].0));
        } else if !counts.is_empty() {
            return Rank::Pair(counts[0].0);
        } else {
            return Rank::High(self.cards[4].value);
        }
    }
}
#[derive(Debug, PartialEq, Eq)]
pub enum HandParseError {
    BadCard(usize, CardParseError),
    InvalidLength,
}
impl FromStr for Hand {
    type Err = HandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cards_str = s.split(' ');
        let mut cards = [Card::default(); 5];
        for i in 0..5 {
            let card_str = cards_str.next().ok_or(HandParseError::InvalidLength)?;
            let card = Card::from_str(card_str).map_err(|e| HandParseError::BadCard(i, e))?;
            cards[i] = card;
        }
        if cards_str.next().is_some() {
            return Err(HandParseError::InvalidLength);
        }
        cards.sort();
        Ok(Hand { cards })
    }
}
#[derive(PartialEq, Eq, Debug, PartialOrd, Ord, Default, Clone, Copy)]
struct Card {
    value: u8,
    suit: u8,
}

impl Card {
    fn new(value: u8, suit: u8) -> Self {
        Self { value, suit }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CardParseError {
    NoValue,
    NoSuit,
    InvalidValue,
    InvalidSuit,
}
impl FromStr for Card {
    type Err = CardParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let v = chars.next().ok_or(CardParseError::NoValue)?;
        let value = match v.to_digit(10) {
            Some(v) => v,
            None => match v {
                'T' => 10,
                'J' => 11,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => return Err(CardParseError::InvalidValue),
            },
        };
        let suit = match chars.next().ok_or(CardParseError::NoSuit)? {
            'C' => 0,
            'D' => 1,
            'S' => 2,
            'H' => 3,
            _ => return Err(CardParseError::InvalidSuit),
        };
        Ok(Self {
            value: value as u8,
            suit,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::{cmp::Ordering, str::FromStr};

    use crate::fifties::fifty_four::{
        Card, CardParseError, Hand, HandParseError,
    };

    use super::Rank;

    #[test]
    fn test_card() {
        assert_eq!(Card::from_str("5H"), Ok(Card::new(5, 3)));
        assert_eq!(Card::from_str(""), Err(CardParseError::NoValue));
        assert_eq!(Card::from_str("5"), Err(CardParseError::NoSuit));
        assert_eq!(Card::from_str("PS"), Err(CardParseError::InvalidValue));
        assert_eq!(Card::from_str("5T"), Err(CardParseError::InvalidSuit));
    }
    #[test]
    fn test_hand() {
        assert!(Hand::from_str("5H 5C 6S 7S KD").is_ok());
        assert_eq!(
            Hand::from_str("5H 5C 6S 7S KT"),
            Err(HandParseError::BadCard(4, CardParseError::InvalidSuit))
        );
        assert_eq!(
            Hand::from_str("5H 5C 6S 7S"),
            Err(HandParseError::InvalidLength)
        );
        assert_eq!(
            Hand::from_str("5H 5C 6S 7S KD KD"),
            Err(HandParseError::InvalidLength)
        );
    }
    #[test]
    fn test_ordering() {
        fn helper(a: &str, b: &str, o: Ordering) {
            assert_eq!(
                Hand::from_str(a).unwrap().cmp(&Hand::from_str(b).unwrap()),
                o
            )
        }
        helper("5H 5C 6S 7S KD", "2C 3S 8S 8D TD", Ordering::Less);
        helper("5D 8C 9S JS AC", "2C 5C 7D 8S QH", Ordering::Greater);
        helper("2D 9C AS AH AC", "3D 6D 7D TD QD", Ordering::Less);
        helper("4D 6S 9H QH QC", "3D 6D 7H QD QS", Ordering::Greater);
        helper("2H 2D 4C 4D 4S", "3C 3D 3S 9S 9D", Ordering::Greater);
    }
    #[test]
    fn test_rank() {
        assert!(Hand::from_str("2H 3H 4H 5H 6H").unwrap().is_straight());
        assert!(!Hand::from_str("3H 3H 4H 5H 6H").unwrap().is_straight());
        assert!(Hand::from_str("2H 3H 4H 5H 6H").unwrap().is_flush());
        assert!(!Hand::from_str("3H 3D 4H 5H 6H").unwrap().is_flush());
        assert_eq!(
            Hand::from_str("2H 3H 4H 5H 6H").unwrap().get_rank(),
            Rank::StraightFlush(2)
        );
        assert_eq!(
            Hand::from_str("2H 2D 2S 2C 5D").unwrap().get_rank(),
            Rank::Four(2)
        );
        assert_eq!(
            Hand::from_str("2H 2D 2S 3C 3D").unwrap().get_rank(),
            Rank::Full(2, 3)
        );
        assert_eq!(
            Hand::from_str("6H 3H JH QH AH").unwrap().get_rank(),
            Rank::Flush
        );
        assert_eq!(
            Hand::from_str("2H 3D 4C 5S 6C").unwrap().get_rank(),
            Rank::Straight(2)
        );
        assert_eq!(
            Hand::from_str("2H 2D 2S KH JH").unwrap().get_rank(),
            Rank::Three(2)
        );
        assert_eq!(
            Hand::from_str("2H 2D 3H 3S JH").unwrap().get_rank(),
            Rank::TwoPair(3, 2)
        );
        assert_eq!(
            Hand::from_str("2H 2D 4S JH QD").unwrap().get_rank(),
            Rank::Pair(2)
        );
        assert_eq!(
            Hand::from_str("5D 8C 9S JS AC").unwrap().get_rank(),
            Rank::High(14)
        )
    }
}
