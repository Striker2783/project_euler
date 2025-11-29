mod ten;
mod eleven;
mod twelve;
mod thirteen;
mod fourteen;
mod fifteen;
mod sixteen;

use common::tens;

tens!(&[ten::run, eleven::run, twelve::run, thirteen::run, fourteen::run, fifteen::run, sixteen::run]);