mod ninety;
mod ninety_one;
mod ninety_two;

use crate::macros::tens;

tens!(&[ninety::run, ninety_one::run, ninety_two::run]);
