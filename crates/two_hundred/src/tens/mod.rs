mod ten;
mod eleven;
mod twelve;
mod thirteen;

use common::tens;

tens!(&[ten::run, eleven::run, twelve::run, thirteen::run]);