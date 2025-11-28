mod zero;
mod one;
mod two;
pub mod three;

use common::tens;

tens!(&[zero::run, one::run, two::run, three::run]);