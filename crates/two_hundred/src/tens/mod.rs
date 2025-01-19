mod zero;
mod one;

use common::tens;

tens!(&[zero::run, one::run]);