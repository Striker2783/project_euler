use crate::macros::tens;

pub mod forty;
pub mod forty_eight;
pub mod forty_five;
pub mod forty_four;
pub mod forty_nine;
pub mod forty_one;
pub mod forty_seven;
pub mod forty_six;
pub mod forty_three;
pub mod forty_two;

tens!(&[
    forty::run,
    forty_one::run,
    forty_two::run,
    forty_three::run,
    forty_four::run,
    forty_five::run,
    forty_six::run,
    forty_seven::run,
    forty_eight::run,
    forty_nine::run,
]);
