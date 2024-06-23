use crate::macros::tens;

pub mod fifty;
pub mod fifty_five;
pub mod fifty_four;
pub mod fifty_one;
pub mod fifty_three;
pub mod fifty_two;

tens!(&[
    fifty::run,
    fifty_one::run,
    fifty_two::run,
    fifty_three::run,
    fifty_four::run,
    fifty_five::run,
]);
