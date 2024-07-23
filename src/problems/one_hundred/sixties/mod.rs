use crate::macros::tens;
pub mod sixty;
pub mod sixty_one;

tens!(&[sixty::run, sixty_one::run]);
