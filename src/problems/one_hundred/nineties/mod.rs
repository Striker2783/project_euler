mod ninety;
mod ninety_one;

use crate::macros::tens;

tens!(&[ninety::run, ninety_one::run]);
