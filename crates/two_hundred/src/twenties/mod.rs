mod twenty;
mod twenty_one;

use common::tens;

tens!(&[twenty::run, twenty_one::run]);
