pub mod eighteen;
pub mod eleven;
pub mod fifteen;
pub mod fourteen;
pub mod nineteen;
pub mod seventeen;
pub mod sixteen;
pub mod ten;
pub mod thirteen;
pub mod twelve;

const F: &[fn()] = &[
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
];

pub unsafe fn run(n: usize) {
    F[n]();
}
