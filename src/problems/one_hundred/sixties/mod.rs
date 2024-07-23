use crate::macros::tens;
pub mod sixty;
pub mod sixty_one;
pub mod sixty_two;

tens!(&[sixty::run, sixty_one::run, sixty_two::run]);
