#![feature(test)]

use common::hundreds;
extern crate test;

pub mod eighties;
pub mod fifties;
pub mod forties;
pub mod nineties;
pub mod ones;
pub mod seventies;
pub mod sixties;
pub mod tens;
pub mod thirties;
pub mod twenties;

hundreds!(&[
    ones::run,
    tens::run,
    twenties::run,
    thirties::run,
    forties::run,
    fifties::run,
    sixties::run,
    seventies::run,
    eighties::run,
    nineties::run
]);
