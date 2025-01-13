mod ninety;
mod ninety_one;
mod ninety_two;
mod ninety_three;
mod ninety_four;
mod ninety_five;

use crate::macros::tens;

tens!(&[ninety::run, ninety_one::run, ninety_two::run, ninety_three::run, ninety_four::run, ninety_five::run]);
