use crate::macros::tens;
mod seventy;
mod seventy_five;
mod seventy_four;
mod seventy_one;
mod seventy_three;
mod seventy_two;
tens!(&[
    seventy::run,
    seventy_one::run,
    seventy_two::run,
    seventy_three::run,
    seventy_four::run,
    seventy_five::run
]);
