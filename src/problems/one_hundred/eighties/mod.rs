mod eighty;
mod eighty_one;

use crate::macros::tens;

tens!(&[eighty::run, eighty_one::run]);