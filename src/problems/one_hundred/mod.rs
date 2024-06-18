pub mod eight;
pub mod eighteen;
pub mod eleven;
pub mod fifteen;
pub mod five;
pub mod forty;
pub mod forty_five;
pub mod forty_four;
pub mod forty_one;
pub mod forty_seven;
pub mod forty_six;
pub mod forty_three;
pub mod forty_two;
pub mod four;
pub mod fourteen;
pub mod nine;
pub mod nineteen;
pub mod seven;
pub mod seventeen;
pub mod six;
pub mod sixteen;
pub mod ten;
pub mod thirteen;
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
pub mod twelve;
pub mod twenty;
pub mod twenty_eight;
pub mod twenty_five;
pub mod twenty_four;
pub mod twenty_nine;
pub mod twenty_one;
pub mod twenty_seven;
pub mod twenty_six;
pub mod twenty_three;
pub mod twenty_two;
pub mod two;

const SOLVERS: &[fn()] = &[
    six::one,
    two::run,
    six::three,
    four::four,
    five::five,
    six::six,
    seven::seven,
    eight::eight,
    nine::nine,
    ten::ten,
    eleven::run,
    twelve::run,
    thirteen::run,
    fourteen::run,
    fifteen::run,
    sixteen::run,
    seventeen::run,
    eighteen::run,
    nineteen::run,
    twenty::run,
    twenty_one::run,
    twenty_two::run,
    twenty_three::run,
    twenty_four::run,
    twenty_five::run,
    twenty_six::run,
    twenty_seven::run,
    twenty_eight::run,
    twenty_nine::run,
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
    forty::run,
    forty_one::run,
    forty_two::run,
    forty_three::run,
    forty_four::run,
    forty_five::run,
    forty_six::run,
    forty_seven::run,
];

pub fn run(n: u32) {
    if n as usize > SOLVERS.len() {
        return;
    }
    SOLVERS[(n - 1) as usize]();
}
pub fn run_all() {
    for f in SOLVERS {
        f()
    }
}
