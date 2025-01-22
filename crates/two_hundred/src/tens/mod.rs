mod zero;
mod one;
mod two;

use common::tens;

tens!(&[zero::run, one::run, two::run]);