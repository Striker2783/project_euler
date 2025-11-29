pub mod ones;
pub mod tens;
pub mod twenties;

use common::hundreds;

hundreds!(&[ones::run, tens::run, twenties::run]);
