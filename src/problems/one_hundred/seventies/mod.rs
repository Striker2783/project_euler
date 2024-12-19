use crate::macros::tens;
mod seventy;
mod seventy_one;
mod seventy_two;
mod seventy_three;
mod seventy_four;
tens!(&[seventy::run, seventy_one::run, seventy_two::run, seventy_three::run, seventy_four::run]);
