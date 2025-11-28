pub mod ones;
pub mod tens;

use common::hundreds;

hundreds!(&[ones::run, tens::run]);