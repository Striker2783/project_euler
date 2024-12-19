use crate::macros::tens;
mod seventy;
mod seventy_one;
tens!(&[seventy::run, seventy_one::run]);
