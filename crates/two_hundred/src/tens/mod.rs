mod ten;
mod eleven;
mod twelve;
mod thirteen;
mod fourteen;

use common::tens;

tens!(&[ten::run, eleven::run, twelve::run, thirteen::run, fourteen::run]);