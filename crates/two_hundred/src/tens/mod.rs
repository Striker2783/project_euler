mod ten;
mod eleven;
mod twelve;

use common::tens;

tens!(&[ten::run, eleven::run, twelve::run]);