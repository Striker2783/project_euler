mod ninety;
mod ninety_five;
mod ninety_four;
mod ninety_one;
mod ninety_seven;
mod ninety_six;
mod ninety_three;
mod ninety_two;
mod ninety_eight;

use crate::macros::tens;

tens!(&[
    ninety::run,
    ninety_one::run,
    ninety_two::run,
    ninety_three::run,
    ninety_four::run,
    ninety_five::run,
    ninety_six::run,
    ninety_seven::run,
    ninety_eight::run,
]);
