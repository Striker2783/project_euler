pub mod thirty;
pub mod thirty_eight;
pub mod thirty_five;
pub mod thirty_four;
pub mod thirty_nine;
pub mod thirty_one;
pub mod thirty_seven;
pub mod thirty_six;
pub mod thirty_three;
pub mod thirty_two;

const F: &[fn()] = &[
    thirty::run,
    thirty_one::run,
    thirty_two::run,
    thirty_three::run,
    thirty_four::run,
    thirty_five::run,
    thirty_six::run,
    thirty_seven::run,
    thirty_eight::run,
    thirty_nine::run,
];

pub unsafe fn run(n: usize) {
    F[n]()
}
