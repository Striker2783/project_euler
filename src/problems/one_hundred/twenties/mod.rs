use crate::macros::tens;

pub mod twenty;
pub mod twenty_eight;
pub mod twenty_five;
pub mod twenty_four;
pub mod twenty_nine;
pub mod twenty_one;
pub mod twenty_seven;
pub mod twenty_six;
pub mod twenty_three;
pub mod twenty_two;

tens!(&[
    twenty::run,
    twenty_one::run,
    twenty_two::run,
    twenty_three::run,
    twenty_four::run,
    twenty_five::run,
    twenty_six::run,
    twenty_seven::run,
    twenty_eight::run,
    twenty_nine::run,
]);
