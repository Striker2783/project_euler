mod eighty;
mod eighty_one;
mod eighty_two;

use crate::macros::tens;

tens!(&[eighty::run, eighty_one::run, eighty_two::run]);