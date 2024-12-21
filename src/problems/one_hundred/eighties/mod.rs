mod eighty;
mod eighty_one;
mod eighty_two;
mod eighty_three;

use crate::macros::tens;

tens!(&[eighty::run, eighty_one::run, eighty_two::run, eighty_three::run]);