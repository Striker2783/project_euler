mod ninety;
mod ninety_one;
mod ninety_two;
mod ninety_three;
mod ninety_four;

use crate::macros::tens;

tens!(&[ninety::run, ninety_one::run, ninety_two::run, ninety_three::run, ninety_four::run]);
