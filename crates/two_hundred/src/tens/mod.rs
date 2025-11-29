mod eleven;
mod fifteen;
mod fourteen;
mod seventeen;
mod sixteen;
mod ten;
mod thirteen;
mod twelve;

use common::tens;

tens!(&[
    ten::run,
    eleven::run,
    twelve::run,
    thirteen::run,
    fourteen::run,
    fifteen::run,
    sixteen::run,
    seventeen::run
]);
