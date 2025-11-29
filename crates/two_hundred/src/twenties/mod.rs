mod twenty;
mod twenty_one;
mod twenty_two;

use common::tens;

tens!(&[twenty::run, twenty_one::run, twenty_two::run]);
