mod eighty;
mod eighty_five;
mod eighty_four;
mod eighty_one;
mod eighty_six;
mod eighty_three;
mod eighty_two;
mod eighty_seven;
mod eighty_eight;

use crate::macros::tens;

tens!(&[
    eighty::run,
    eighty_one::run,
    eighty_two::run,
    eighty_three::run,
    eighty_four::run,
    eighty_five::run,
    eighty_six::run,
    eighty_seven::run,
    eighty_eight::run,
]);
