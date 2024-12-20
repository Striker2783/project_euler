use crate::macros::tens;
mod seventy;
mod seventy_five;
mod seventy_four;
mod seventy_one;
mod seventy_six;
mod seventy_three;
mod seventy_two;
mod seventy_seven;
mod seventy_eight;
tens!(&[
    seventy::run,
    seventy_one::run,
    seventy_two::run,
    seventy_three::run,
    seventy_four::run,
    seventy_five::run,
    seventy_six::run,
    seventy_seven::run,
    seventy_eight::run,
]);
