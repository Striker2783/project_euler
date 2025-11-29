use common::tens;
mod seventy;
mod seventy_eight;
mod seventy_five;
mod seventy_four;
mod seventy_nine;
mod seventy_one;
mod seventy_seven;
mod seventy_six;
mod seventy_three;
mod seventy_two;
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
    seventy_nine::run,
]);
