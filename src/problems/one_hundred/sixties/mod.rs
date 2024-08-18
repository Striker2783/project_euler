use crate::macros::tens;
pub mod sixty;
pub mod sixty_eight;
pub mod sixty_five;
pub mod sixty_four;
pub mod sixty_one;
pub mod sixty_seven;
pub mod sixty_six;
pub mod sixty_three;
pub mod sixty_two;

tens!(&[
    sixty::run,
    sixty_one::run,
    sixty_two::run,
    sixty_three::run,
    sixty_four::run,
    sixty_five::run,
    sixty_six::run,
    sixty_seven::run,
    sixty_eight::run,
]);
